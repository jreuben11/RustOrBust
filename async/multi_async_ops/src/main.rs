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

struct Book {}
struct Music {}

async fn get_book() -> Book {Book{}}
async fn get_music() -> Music {Music{}}

async fn serial() -> (Book, Music) {
    let book = get_book().await;
    let music = get_music().await;
    (book, music)
}

async fn parallel_join() -> (Book, Music) {
    let book_fut = get_book();
    let music_fut = get_music();
    join!(book_fut, music_fut)
}



async fn get_book2() -> Result<Book, String> { /* ... */ Ok(Book{}) }
async fn get_music2() -> Result<Music, String> { /* ... */ Ok(Music{}) }

async fn parallel_try_join() -> Result<(Book, Music), String> {
    let book_fut = get_book2();
    let music_fut = get_music2();
    try_join!(book_fut, music_fut)
}

async fn get_book3() -> Result<Book, ()> { /* ... */ Err(()) }

async fn parallel_try_join_consolidate_error_type() -> Result<(Book, Music), String> {
    let book_fut = get_book3().map_err(|()| "Unable to get book".to_string());
    let music_fut = get_music2();
    try_join!(book_fut, music_fut)
}

async fn task_one() -> u8 { 1 }
async fn task_two() -> u8 { 2 }

async fn race_tasks() {
     let t1 = task_one().fuse();     
     let t2 = task_two().fuse();

    pin_mut!(t1, t2);
    
    select! {
        i = t1 => println!("task one completed first with {i}"),
        i = t2 => println!("task two completed first  with {i}"),
    }
}

async fn do_thing(foo: &mut i32) {println!("{foo}");}
async fn something_else() {}

async fn select_fused_mutable() {
    
    let mut foo = 1;
    select! {
        _x = do_thing(&mut foo).fuse() => {
            foo += 1;
        },
        _y = something_else().fuse() => {
            foo += 2;
        },
    }
    println!("foo:{foo}");
    
    
}

async fn loop_select_count() {
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(6);
    let mut total = 0;

    loop {
        select! {
            a = a_fut => total += a,
            b = b_fut => total += b,
            complete => break,
            default => unreachable!(), // never runs (futures are ready, then complete)
        };
    }
    assert_eq!(total, 10);
}



async fn add_two_fused_streams(
    mut s1: impl Stream<Item = u8> + FusedStream + Unpin,
    mut s2: impl Stream<Item = u8> + FusedStream + Unpin,
) -> u8 {
    let mut total = 0;

    loop {
        let item = select! {
            x = s1.next() => x,
            x = s2.next() => x,
            complete => break,
        };
        if let Some(next_num) = item {
            total += next_num;
        }
    }

    println!("total: {total}");
    total
}

async fn get_new_num() -> u8 { /* ... */ 5 }
async fn run_on_new_num(i: u8) { println!("{i}"); }

async fn timer_loop_select_next_some(
    mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin,
    starting_num: u8,
) {
    let run_on_new_num_fut = run_on_new_num(starting_num).fuse();
    let get_new_num_fut = Fuse::terminated();
    pin_mut!(run_on_new_num_fut, get_new_num_fut);
    loop {
        select! {
            () = interval_timer.select_next_some() => {
                println!("The timer has elapsed. Start a new `get_new_num_fut` if one was not already running");
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num().fuse());
                }
            },
            new_num = get_new_num_fut => {
                println!("A new number has arrived -- start a new `run_on_new_num_fut`, dropping the old one.");
                run_on_new_num_fut.set(run_on_new_num(new_num).fuse());
            },
            // Run the `run_on_new_num_fut`
            () = run_on_new_num_fut => {},
            // panic if everything completed, since the `interval_timer` should
            // keep yielding values indefinitely.
            complete => {
                println!("`interval_timer` stream completed");
                break;
            }
        }
    }
}