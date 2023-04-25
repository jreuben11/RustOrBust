use async_std::{
    prelude::*, 
    task, 
    net::{TcpListener, ToSocketAddrs, TcpStream}, 
    io::BufReader,
};



type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>; 

fn main() {
    run();
}

// main
fn run() -> Result<()> {
    let fut = accept_loop("127.0.0.1:8080");
    task::block_on(fut)
}

async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> { 

    let listener = TcpListener::bind(addr).await?; 
    let (broker_sender, broker_receiver) = mpsc::unbounded();   // 1
    let broker_handle = task::spawn(broker_loop(broker_receiver));
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await { 
        let stream = stream?;
        println!("Accepting from: {}", stream.peer_addr()?);
        spawn_and_log_error(connection_loop(broker_sender.clone(), stream));
    }
    drop(broker_sender); // 1
    broker_handle.await?; // 5
    Ok(())
}

async fn connection_loop(mut broker: Sender<Event>, stream: TcpStream) -> Result<()> {

    let stream = Arc::new(stream); 
    let reader = BufReader::new(&*stream); // 2
    let mut lines = reader.lines();

    let name = match lines.next().await { 
        None => Err("peer disconnected immediately")?,
        Some(line) => line?,
    };
    broker.send(Event::NewPeer { name: name.clone(), stream: Arc::clone(&stream) }).await // 3
        .unwrap();

    while let Some(line) = lines.next().await { 
        let line = line?;
        let (dest, msg) = match line.find(':') { 
            None => continue,
            Some(idx) => (&line[..idx], line[idx + 1 ..].trim()),
        };
        let dest: Vec<String> = dest.split(',').map(|name| name.trim().to_string()).collect();
        let msg: String = msg.to_string();
        broker.send(Event::Message { // 4
            from: name.clone(),
            to: dest,
            msg,
        }).await.unwrap();
    }
    Ok(())
}

fn spawn_and_log_error<F>(fut: F) -> task::JoinHandle<()>
where
    F: Future<Output = Result<()>> + Send + 'static,
{
    task::spawn(async move {
        if let Err(e) = fut.await {
            eprintln!("{}", e)
        }
    })
}

use futures::channel::mpsc; // 1
use futures::sink::SinkExt;
use std::sync::Arc;

type Sender<T> = mpsc::UnboundedSender<T>; // 2
type Receiver<T> = mpsc::UnboundedReceiver<T>;

async fn connection_writer_loop(
    mut messages: Receiver<String>,
    stream: Arc<TcpStream>, // 3
) -> Result<()> {
    let mut stream = &*stream;
    while let Some(msg) = messages.next().await {
        stream.write_all(msg.as_bytes()).await?;
    }
    Ok(())
}

use std::collections::hash_map::{Entry, HashMap};

#[derive(Debug)]
enum Event { // 1
    NewPeer {
        name: String,
        stream: Arc<TcpStream>,
    },
    Message {
        from: String,
        to: Vec<String>,
        msg: String,
    },
}

async fn broker_loop(mut events: Receiver<Event>) -> Result<()> {
    let mut writers = Vec::new();
    let mut peers: HashMap<String, Sender<String>> = HashMap::new(); 

    while let Some(event) = events.next().await {
        match event {
            Event::Message { from, to, msg } => {  // 3
                for addr in to {
                    if let Some(peer) = peers.get_mut(&addr) {
                        let msg = format!("from {}: {}\n", from, msg);
                        peer.send(msg).await?
                    }
                }
            }
            Event::NewPeer { name, stream } => {
                match peers.entry(name) {
                    Entry::Occupied(..) => (),
                    Entry::Vacant(entry) => {
                        let (client_sender, client_receiver) = mpsc::unbounded();
                        entry.insert(client_sender); // 4
                        let handle = spawn_and_log_error(connection_writer_loop(client_receiver, stream));
                        writers.push(handle);
                    }
                }
            }
        }
    }
    drop(peers); // 3
    for writer in writers { // 4
        writer.await;
    }
    Ok(())
}