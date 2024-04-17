# Tokio mini-redis
https://github.com/tokio-rs/mini-redis
```bash
cargo install mini-redis
sudo systemctl status redis-server
sudo systemctl stop redis-server
mini-redis-server
mini-redis-cli get foo
cargo new my-redis
cd my-redis
cargo add tokio --features full
cargo add mini-redis
cargo add bytes
```
## Hello Tokio
- `cargo run --example hello-redis`
- [client](my-redis/examples/hello-redis.rs)
```rust
#[tokio::main]
async fn main() { ... }

use tokio::net::ToSocketAddrs;
pub async fn connect<T: ToSocketAddrs>(addr: T) -> Result<mini_redis::client::Client> { ... }
```

## Spawning
```rust
use tokio::net::{TcpListener, TcpStream};
#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}
```
- tasks
```rust
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // Do some async work
        "return value"
    });

    // Do some other work

    let out = handle.await.unwrap();
    println!("GOT {}", out);
}
```
- Send bound
```rust
use tokio::task::yield_now;
use std::rc::Rc;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        // The scope forces `rc` to drop before `.await`.
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }

        // `rc` is no longer used. It is **not** persisted when
        // the task yields to the scheduler
        yield_now().await;
    });
}
```
## Shared State
```rust
use tokio::net::TcpListener;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // Clone the handle to the hash map.
        let db = db.clone();

        println!("Accepted");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}
```
- sharded Hashmap
```rust
type ShardedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;
fn new_sharded_db(num_shards: usize) -> ShardedDb {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}
let shard = db[hash(key) % db.len()].lock().unwrap();
shard.insert(key, value);
```
- Holding a MutexGuard across an .await
```rust
async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    {
        let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
        *lock += 1;
    } // lock goes out of scope here

    do_something_async().await;
}
```
- Restructure code to not hold the lock across an .await
```rust
use std::sync::Mutex;

struct CanIncrement {
    mutex: Mutex<i32>,
}
impl CanIncrement {
    // This function is not marked async.
    fn increment(&self) {
        let mut lock = self.mutex.lock().unwrap();
        *lock += 1;
    }
}

async fn increment_and_do_stuff(can_incr: &CanIncrement) {
    can_incr.increment();
    do_something_async().await;
}
```
- Use Tokio's asynchronous mutex
```rust
use tokio::sync::Mutex; // note! This uses the Tokio mutex

// This compiles! (but restructuring the code would be better in this case)
async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    let mut lock = mutex.lock().await;
    *lock += 1;

    do_something_async().await;
} // lock goes out of scope here
```
## Channels
- `tokio::sync::mpsc` : multi-producer, single-consumer channel. Many values can be sent.
- `tokio::sync::oneshot` : single-producer, single consumer channel. A single value can be sent.
- `tokio::sync::broadcast` : multi-producer, multi-consumer. Many values can be sent. Each receiver sees every value.
- `tokio::sync::watch` : single-producer, multi-consumer. Many values can be sent, but no history is kept. Receivers only see the most recent value.
- **async-channel** crate: multi-producer multi-consumer channel where only one consumer sees each message
```bash
cargo run --bin server
cargo run --bin my-redis
```
- [server](my-redis/src/bin/server.rs)
- [my-redis](my-redis/src/main.rs)
- create a channel:
```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();
    tokio::spawn(async move {
        tx.send("sending from first handle").await;
    });
    tokio::spawn(async move {
        tx2.send("sending from second handle").await;
    });
    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }
}
```
- Concurrency and queuing must be explicitly introduced. Ways to do this:
  - `tokio::spawn`
  - `select!`
  - `join!`
  - `mpsc::channel`
## I/O
- `AsyncRead` and `AsyncWrite` traits
- `AsyncReadExt::read` - read data into a buffer, returning the number of bytes read.
```rust
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer[..]).await?;

    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}
```
- `AsyncReadExt::read_to_end` - reads all bytes from the stream until EOF
```rust
use tokio::io::{self, AsyncReadExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer).await?;
    Ok(())
}
```
- `AsyncWriteExt::write` - writes a buffer into the writer, returning how many bytes were written.
```rust
use tokio::io::{self, AsyncWriteExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;

    // Writes some prefix of the byte string, but not necessarily all of it.
    let n = file.write(b"some bytes").await?;

    println!("Wrote the first {} bytes of 'some bytes'.", n);
    Ok(())
}
```
- `AsyncWriteExt::write_all` -  writes the entire buffer into the writer.
```rust
use tokio::io::{self, AsyncWriteExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;

    file.write_all(b"some bytes").await?;
    Ok(())
}
```
- `tokio::io::copy` - asynchronously copies the entire contents of a reader into a writer.
```rust
use tokio::fs::File;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut reader: &[u8] = b"hello";
    let mut file = File::create("foo.txt").await?;

    io::copy(&mut reader, &mut file).await?;
    Ok(())
}
```
- `tokio::net::TcpListener` - TCP server and needs an accept loop. A new task is spawned to process each accepted socket.
```rust
use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // Copy data here
        });
    }
}
```
-  `TcpStream::split` - split the socket into a reader handle and a writer handle.  can be used independently, including from separate tasks.
```rust
tokio::spawn(async move {
    let (mut rd, mut wr) = socket.split();
    
    if io::copy(&mut rd, &mut wr).await.is_err() {
        eprintln!("failed to copy");
    }
});
```
- [echo-server](my-redis/src/bin/echo-server.rs)
- [echo-server-copy](my-redis/src/bin/echo-server-copy.rs)
```bash
cargo run --bin echo-server
cargo run --bin echo-server-copy
```
## Framing
- [connection](my-redis/src/connection.rs)
```rust
use tokio::io::AsyncReadExt;
use bytes::Buf;
use bytes::BytesMut;
use std::io::Cursor;
use tokio::io::BufWriter;
use tokio::io::{self, AsyncWriteExt};

let mut buffer = BytesMut::with_capacity(4096);
...
if 0 == stream.read_buf(&mut buffer).await? {
    if buffer.is_empty() {
        return Ok(None);
    } else {
        return Err("connection reset by peer".into());
    }
}

fn parse_frame(&mut self)-> Result<Option<Frame>> {
    let mut buf = Cursor::new(&self.buffer[..]);
    match Frame::check(&mut buf) {
        Ok(_) => {
            let len = buf.position() as usize;
            buf.set_position(0);
            let frame = Frame::parse(&mut buf)?;
            self.buffer.advance(len);
            Ok(Some(frame))
        }
        Err(Incomplete) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}
impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(4096),
        }
    }
}

stream.write_u8(b'+').await?;
stream.write_all(val.as_bytes()).await?;
stream.write_all(b"\r\n").await?;
stream.flush().await;
```
## Async in Depth

# Axum on Tokio
## [main.rs](axum-tokio/src/main.rs)
-- dependencies:
```rust
use std::collections::HashMap;
use axum::routing::get;
use serde_json::{json, Value};
use std::thread;

mod book;
mod data;
use crate::book::Book;
use crate::data::DATA;

/// Use tracing crates for application-level tracing output.
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
use std::net::SocketAddr;
```
- server:
```rust
#[tokio::main]
pub async fn main() {
    // Start tracing. DOESNT SHOW ANYTHING !!!!
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    print_data().await;
     // Build our application by creating our router.
    let app = axum::Router::new()
        .fallback(fallback)
        .route("/",  axum::routing::get(|| async { "default!" }))
        .route("/hello", get(hello))
        .route("/demo.html", get(get_demo_html))
        .route("/demo-status", get(demo_status))
        .route("/demo-uri", get(demo_uri))
        .route("/demo.png", get(get_demo_png))
        .route("/foo",
            get(get_foo)
            .put(put_foo)
            .patch(patch_foo)
            .post(post_foo)
            .delete(delete_foo),
        )
        .route("/items/:id", get(get_items_id))
        .route("/items", get(get_items))
        .route("/demo.json",
            get(get_demo_json)
            .put(put_demo_json)
        )
        .route("/books",
            get(get_books)
            .put(put_books)
        )
        .route("/books/:id",
            get(get_books_id)
            .delete(delete_books_id)
        )
        .route("/books/:id/form",
            get(get_books_id_form)
            .post(post_books_id_form)
        )
        ;

    // Run our application as a hyper server on http://localhost:3000.
    let host = [127, 0, 0, 1];
    let port = 3000;
    let addr = SocketAddr::from((host, port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```
## testing
- Browse http://localhost:3000
- tester:
```bash
curl --request GET 'http://localhost:3000/foo'
curl --request PUT 'http://localhost:3000/foo'
curl --request PATCH 'http://localhost:3000/foo'
curl --request POST 'http://localhost:3000/foo'
curl --request DELETE 'http://localhost:3000/foo'
curl 'http://localhost:3000/items/1'
curl 'http://localhost:3000/items?a=b'
curl \
--header "Accept: application/json" \
--request GET 'http://localhost:3000/demo.json'
curl \
--request PUT 'http://localhost:3000/demo.json' \
--header "Content-Type: application/json" \
--data '{"a":"b"}'
curl 'http://localhost:3000/books'
curl 'http://localhost:3000/books/1'
curl 'http://localhost:3000/books/0'
curl \
--request PUT 'http://localhost:3000/books' \
--header "Content-Type: application/json" \
--data '{"id":4,"title":"Decameron","author":"Giovanni Boccaccio"}'
curl 'http://localhost:3000/books'
curl 'http://localhost:3000/books/1/form'
curl \
--request POST 'localhost:3000/books/1/form' \
--header "Content-Type: application/x-www-form-urlencoded" \
--data "id=1"  \
--data "title=Another Title" \
--data "author=Someone Else"
curl 'http://localhost:3000/books'
curl --request DELETE 'http://localhost:3000/books/1'
curl 'http://localhost:3000/books'
```
## [Cargo.toml](axum-tokio/Cargo.toml)
```toml
[dependencies]
axum = "0.7.5"
base64 = "0.22.0"
http = "1.1.0"
hyper = { version = "1.3.1", features = ["full"] }
once_cell = "1.19.0"
serde = { version = "1.0.198", features = ["derive"] }
serde_json = "1.0.116"
tokio = { version = "1.37.0", features = ["full"] }
tower = "0.4.13"
tracing = "0.1.40"
tracing-subscriber = { version = "0.3.18", features = ["env-filter"] }
```
## [data.rs](axum-tokio/src/data.rs)
```rust
use std::collections::HashMap;
use crate::book::Book;
/// Use once_cell for creating a global variable e.g. our DATA data.
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// Create a data store as a global variable with `Lazy` and `Mutex`.
/// This demo implementation uses a `HashMap` for ease and speed.
/// The map key is a primary key for lookup; the map value is a Book.
pub static DATA: Lazy<Mutex<HashMap<u32, Book>>> = Lazy::new(|| Mutex::new(HashMap::from([ ... ])));
```
## Axum
```rust
#[tokio::main]
async fn main() {
    // Build our application with a single route.
    let app = axum::Router::new().route("/",
        axum::routing::get(|| async { "Hello, World!" }));

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```
## Tower
```rust
pub trait Service<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>>;

    fn call(&mut self, req: Request) -> Self::Future;
}
---
use tower::{
    Service,
    ServiceExt,
};
let response = service
    // wait for the service to have capacity
    .ready().await?
    // send the request
    .call(request).await?;
```
## Hyper
```rust
use std::convert::Infallible;

async fn handle(
    _: hyper::Request<Body>
) -> Result<hyper::Response<hyper::Body>, Infallible> {
    Ok(hyper::Response::new("Hello, World!".into()))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = hyper::service::make_service_fn(|_conn| async {
        Ok::<_, Infallible>(hyper::service::service_fn(handle))
    });

    let server = hyper::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
```
## Tokio
```rust
// Demo tokio server
#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    loop {
        let (socket, _address) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: tokio::net::TcpStream) {
    println!("process socket");
}

...

// Demo tokio client
#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("localhost:3000").await?;
    println!("connected);
    Ok(())
}
```