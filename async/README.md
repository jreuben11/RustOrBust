# Futures book
- https://rust-lang.github.io/async-book
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
4. [Pinning](pinning/src/main.rs)
   ```rust
    use std::pin::Pin;
    use std::marker::PhantomPinned;
    use pin_utils::pin_mut;

    naive::swap();
    stack_pinned::swap();
    heap_pinned::swap();
    pass_unpinable_futures();
   ```
5. [Stream Trait](stream_trait/src/main.rs)
   - `futures::Stream`
   - `futures::stream::StreamExt` -> `next().await`
   - `futures::stream::TryStreamExt` -> `try_next().await?`, `try_for_each_concurrent(n, |x| async move`
  ```rust
    use futures::stream::{self, Stream, StreamExt};
    use std::pin::Pin;
    use std::io;
    use futures::pin_mut;
    use futures::executor::block_on;
    use futures::channel::mpsc;
    use futures::SinkExt;

    send_recv().await;
    let s1 = stream::iter(vec![1, 2, 3]).fuse();
    pin_mut!(s1);
    sum_with_next(s1).await;
    //TODO: construct an IO stream of results as input

  ```
6. [multiple asynchronous operations](multi_async_ops/src/main.rs)
   ```rust
    use futures::{ join, try_join,select, pin_mut};
    use futures::future::{self, Fuse, FusedFuture, FutureExt, TryFutureExt };
    use futures::stream::{self, Stream, StreamExt, FusedStream};
    use futures::executor::block_on;

    async fn async_main() {
        serial().await;
        parallel_join().await;
        parallel_try_join().await.unwrap();
        match parallel_try_join_consolidate_error_type().await {
            Err(s) => println!("{s}"),
            _ => println!("OK"),
        }
        race_tasks().await;
        select_fused_mutable().await;
        loop_select_count().await;
        let s1 = stream::iter(vec![1, 2]).fuse();
        let s2 = stream::iter(vec![1, 2]).fuse();
        add_two_fused_streams(s1, s2).await;  
        let s3 = stream::iter(vec![(),(),()]).fuse();   
        timer_loop_select_next_some(s3, 10).await;
    }

    fn main() {
        block_on(async_main());
    }
    ```
7. [Workarrounds](workarrounds/src/main.rs)
   - explicit async results
   - send approximation
   - async recursive
9. [Concurrent Web Server](concurrent_web_server/src/main.rs)
    ```rust
    use async_std::net::{TcpListener, /*TcpStream*/};
    use async_std::prelude::*;
    use async_std::task::{self, spawn};
    use async_std::io::{Read, Write};
    use futures::stream::StreamExt;

    #[async_std::main]
    async fn main() { ... }

    use super::*;
    use futures::io::Error;
    use futures::task::{Context, Poll};
    use std::cmp::min;
    use std::pin::Pin;

    impl Read for MockTcpStream { ... }
    impl Write for MockTcpStream { ... }
    impl Unpin for MockTcpStream { ... }

    #[async_std::test]
    async fn test_handle_connection() { ... }
    ```

# async-std book
    - https://book.async.rs/
[async_std chat](async_std_chat/src/main.rs)

```rust
use async_std::{
    io::BufReader,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};
use futures::{
    channel::mpsc,
    sink::SinkExt;
    select, 
    FutureExt,
};
use std::{
    collections::hash_map::{Entry, HashMap},
    future::Future,
    sync::Arc,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

enum Void {}

fn main() -> Result<()> { ... }
async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> { ... }
async fn connection_loop(mut broker: Sender<Event>, stream: TcpStream) -> Result<()> { ... }
async fn connection_writer_loop( messages: &mut Receiver<String>, stream: Arc<TcpStream>, shutdown: Receiver<Void>,) -> Result<()> { ... }

enum Event {
    NewPeer { name: String, stream: Arc<TcpStream>, shutdown: Receiver<Void>, },
    Message { from: String, to: Vec<String>, msg: String, },
}

async fn broker_loop(events: Receiver<Event>) { ... }

fn spawn_and_log_error<F>(fut: F) -> task::JoinHandle<()> where F: Future<Output = Result<()>> + Send + 'static, { ... }

```