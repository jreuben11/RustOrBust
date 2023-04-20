1. [getting started](async_await_intro/src/main.rs)
   - `async fn`
   - `futures::executor::block_on`
2. [futures and tasks](futures_and_tasks/src/main.rs)
   ```rust
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
    ```
    ```rust
    // allocation-free state machines
    trait Future {
        type Output;
        fn poll(self: Pin<&mut Self>,cx: &mut Context<'_>,) -> Poll<Self::Output>;
    }

    pub struct MyFuture {
        shared_state: Arc<Mutex<SharedState>>,
    }
    /// Shared state between the future and the waiting thread
    struct SharedState {
        completed: bool,
        waker: Option<Waker>,
    }
    impl Future for MyFuture {
        type Output = ();
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> { ... }
    }
    impl MyFuture {
        /// ctor spawns new thread with clone of SharedState
        pub fn new(duration: Duration) -> Self { ... }
    }
    
    /// A future that can reschedule itself to be polled by an `Executor`.
    struct Task {
        future: Mutex<Option<BoxFuture<'static, ()>>>,
        task_sender: SyncSender<Arc<Task>>,
    }
    impl ArcWake for Task {
        // Implement `wake` by sending this task back onto the task channel so that it will be polled again by the executor.
        fn wake_by_ref(arc_self: &Arc<Self>) { ... }
    }

    /// Task executor that receives tasks off of a channel and runs them.
    struct Executor {
        ready_queue: Receiver<Arc<Task>>,
    }
    /// Take the future, and if it has not yet completed (is still Some), poll it in an attempt to complete it.
    /// IF not done processing the future, enqueue it back in its task to be run again in the future.
    impl Executor {
        fn run(&self) { ... } // while let Ok(task) = self.ready_queue.recv(), if future.as_mut().poll(context).is_pending()
    }

    /// `Spawner` spawns new futures onto the task channel.
    #[derive(Clone)]
    struct Spawner {
        task_sender: SyncSender<Arc<Task>>,
    }
    impl Spawner {
        fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) { ... }
    }

    ...
    spawner.spawn(async {
        println!("howdy!");
        MyFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });
    drop(spawner);
    executor.run();

    ```
3. [async await / move async](async_await/src/main.rs)
- `std::future::Future`
4. pinning