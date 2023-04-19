use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};

use std::{
    future::Future,
    pin::Pin,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};



pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// Shared state between the future and the waiting thread
struct SharedState {
    /// Whether or not the sleep time has elapsed
    completed: bool,

    /// The waker for the task that `TimerFuture` is running on.
    /// The thread can use this after setting `completed = true` to tell
    /// `TimerFuture`'s task to wake up, see that `completed = true`, and
    /// move forward.
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Look at the shared state to see if the timer has already completed.
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            // Set waker so that the thread can wake up the current task
            // when the timer has completed, ensuring that the future is polled
            // again and sees that `completed = true`.
            //
            // It's tempting to do this once rather than repeatedly cloning
            // the waker each time. However, the `TimerFuture` can move between
            // tasks on the executor, which could cause a stale waker pointing
            // to the wrong task, preventing `TimerFuture` from waking up
            // correctly.
            //
            // N.B. it's possible to check for this using the `Waker::will_wake`
            // function, but we omit that here to keep things simple.
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    /// Create a new `TimerFuture` which will complete after the provided
    /// timeout.
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // Spawn the new thread
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            // Signal that the timer has completed and wake up the last
            // task on which the future was polled, if one exists.
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}

/// Task executor that receives tasks off of a channel and runs them.
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

/// `Spawner` spawns new futures onto the task channel.
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

/// A future that can reschedule itself to be polled by an `Executor`.
struct Task {
    /// In-progress future that should be pushed to completion.
    ///
    /// The `Mutex` is not necessary for correctness, since we only have
    /// one thread executing tasks at once. However, Rust isn't smart
    /// enough to know that `future` is only mutated from one thread,
    /// so we need to use the `Mutex` to prove thread-safety. A production
    /// executor would not need this, and could use `UnsafeCell` instead.
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// Handle to place the task itself back onto the task queue.
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    // Maximum number of tasks to allow queueing in the channel at once.
    // This is just to make `sync_channel` happy, and wouldn't be present in
    // a real executor.
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // Implement `wake` by sending this task back onto the task channel
        // so that it will be polled again by the executor.
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // Take the future, and if it has not yet completed (is still Some),
            // poll it in an attempt to complete it.
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // Create a `LocalWaker` from the task itself
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);
                // `BoxFuture<T>` is a type alias for
                // `Pin<Box<dyn Future<Output = T> + Send + 'static>>`.
                // We can get a `Pin<&mut dyn Future + Send + 'static>`
                // from it by calling the `Pin::as_mut` method.
                if future.as_mut().poll(context).is_pending() {
                    // We're not done processing the future, so put it
                    // back in its task to be run again in the future.
                    *future_slot = Some(future);
                }
            }
        }
    }
}

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // Spawn a task to print before and after waiting on a timer.
    spawner.spawn(async {
        println!("howdy!");
        // Wait for our timer future to complete after two seconds.
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.
    drop(spawner);

    // Run the executor until the task queue is empty.
    // This will print "howdy!", pause, and then print "done!".
    executor.run();
}


#[allow(dead_code)]
mod naive {

    trait SimpleFuture {
        type Output;
        fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
    }
    
    enum Poll<T> {
        Ready(T),
        Pending,
    }



    /// A SimpleFuture that runs two other futures to completion concurrently.
    ///
    /// Concurrency is achieved via the fact that calls to `poll` each future
    /// may be interleaved, allowing each future to advance itself at its own pace.
    pub struct Join<FutureA, FutureB> {
        // Each field may contain a future that should be run to completion.
        // If the future has already completed, the field is set to `None`.
        // This prevents us from polling a future after it has completed, which
        // would violate the contract of the `Future` trait.
        a: Option<FutureA>,
        b: Option<FutureB>,
    }

    impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
    where
        FutureA: SimpleFuture<Output = ()>,
        FutureB: SimpleFuture<Output = ()>,
    {
        type Output = ();
        fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
            // Attempt to complete future `a`.
            if let Some(a) = &mut self.a {
                if let Poll::Ready(()) = a.poll(wake) {
                    self.a.take();
                }
            }

            // Attempt to complete future `b`.
            if let Some(b) = &mut self.b {
                if let Poll::Ready(()) = b.poll(wake) {
                    self.b.take();
                }
            }

            if self.a.is_none() && self.b.is_none() {
                // Both futures have completed -- we can return successfully
                Poll::Ready(())
            } else {
                // One or both futures returned `Poll::Pending` and still have
                // work to do. They will call `wake()` when progress can be made.
                Poll::Pending
            }
        }
    }

    /// A SimpleFuture that runs two futures to completion, one after another.
    //
    // Note: for the purposes of this simple example, `AndThenFut` assumes both
    // the first and second futures are available at creation-time. The real
    // `AndThen` combinator allows creating the second future based on the output
    // of the first future, like `get_breakfast.and_then(|food| eat(food))`.
    pub struct AndThenFut<FutureA, FutureB> {
        first: Option<FutureA>,
        second: FutureB,
    }

    impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
    where
        FutureA: SimpleFuture<Output = ()>,
        FutureB: SimpleFuture<Output = ()>,
    {
        type Output = ();
        fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
            if let Some(first) = &mut self.first {
                match first.poll(wake) {
                    // We've completed the first future -- remove it and start on
                    // the second!
                    Poll::Ready(()) => self.first.take(),
                    // We couldn't yet complete the first future.
                    Poll::Pending => return Poll::Pending,
                };
            }
            // Now that the first future is done, attempt to complete the second.
            self.second.poll(wake)
        }
    }

}
