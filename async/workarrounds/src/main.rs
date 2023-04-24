fn main() {
    let _fut = async {
        foo().await?;
        bar().await?;
        Ok::<(), MyError>(()) // <- note the explicit type annotation here
    };

}

// explicit async results
struct MyError{}
async fn foo() -> Result<(), MyError> { Ok(())}
async fn bar() -> Result<(), MyError> { Err(MyError{})}

// send approximation
use std::rc::Rc;
#[derive(Default)]
struct NotSend(Rc<()>);

async fn bar2() {}
#[allow(dead_code)]
async fn foo2() {
    {
        let _x = NotSend::default();
    }
    bar2().await;
}

// recursive async
use futures::future::{BoxFuture, FutureExt};
#[allow(dead_code)]
fn recursive() -> BoxFuture<'static, ()> {
    async move {
        recursive().await;
        recursive().await;
    }.boxed()
}
