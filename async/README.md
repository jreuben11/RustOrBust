1. [getting started](async_await_intro/src/main.rs)
   - async fn
   - futures::executor::block_on
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