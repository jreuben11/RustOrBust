use futures::stream::{self, Stream, StreamExt};
use std::pin::Pin;
use std::io;
use futures::pin_mut;
use futures::executor::block_on;


async fn async_main() {
    let s1 = stream::iter(vec![1, 2, 3]).fuse();
    pin_mut!(s1);
    let x = sum_with_next(s1).await;
    println!("{x}"); 
    //TODO: construct an IO stream of results as input
}

fn main() {
    block_on(async_main());
}

// async fn send_recv() {

//     const BUFFER_SIZE: usize = 10;
//     let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);  // Tokio ???

//     tx.send(1).await.unwrap();
//     tx.send(2).await.unwrap();
//     drop(tx);

//     // `StreamExt::next` is similar to `Iterator::next`, but returns a
//     // type that implements `Future<Output = Option<T>>`.
//     assert_eq!(Some(1), rx.next().await);
//     assert_eq!(Some(2), rx.next().await);
//     assert_eq!(None, rx.next().await);
// }




async fn sum_with_next(mut stream: Pin<&mut dyn Stream<Item = i32>>) -> i32 {
    let mut sum = 0;
    while let Some(item) = stream.next().await {
        sum += item;
    }
    sum
}


#[allow(dead_code)]
async fn sum_with_try_next(
    mut stream: Pin<&mut dyn Stream<Item = Result<i32, io::Error>>>,
) -> Result<i32, io::Error> {
    use futures::stream::TryStreamExt; // for `try_next`
    let mut sum = 0;
    while let Some(item) = stream.try_next().await? {
        sum += item;
    }
    Ok(sum)
}

#[allow(dead_code)]
async fn jump_around(
    #[allow(unused_mut)]
    mut stream: Pin<&mut dyn Stream<Item = Result<u8, io::Error>>>,
) -> Result<(), io::Error> {
    use futures::stream::TryStreamExt; // for `try_for_each_concurrent`
    const MAX_CONCURRENT_JUMPERS: usize = 100;

    async fn jump_n_times(num: u8) -> Result<u8, io::Error> {
        Ok(num + 1)
    }
    async fn report_n_jumps(num: u8) -> Result<u8, io::Error> {
        println!("{num}");
        Ok(num)
    }
    stream.try_for_each_concurrent(MAX_CONCURRENT_JUMPERS, |num| async move {
        jump_n_times(num).await?;
        report_n_jumps(num).await?;
        Ok(())
    }).await?;

    Ok(())
}
