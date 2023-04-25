use async_std::{
    prelude::*, 
    task, 
    net::{TcpListener, ToSocketAddrs, TcpStream}, 
    io::BufReader,
};



type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>; 




fn main() -> Result<()> {
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
    broker_handle.await; // 5
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
    let (_shutdown_sender, shutdown_receiver) = mpsc::unbounded::<Void>(); // 3
    broker.send(Event::NewPeer { 
        name: name.clone(), 
        stream: Arc::clone(&stream),
        shutdown: shutdown_receiver,  // 4
    }).await.unwrap();

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
use futures::{select, FutureExt};

type Sender<T> = mpsc::UnboundedSender<T>; // 2
type Receiver<T> = mpsc::UnboundedReceiver<T>;

async fn connection_writer_loop(
    messages: &mut Receiver<String>,
    stream: Arc<TcpStream>, 
    shutdown: Receiver<Void>, // 1
) -> Result<()> {
    let mut stream = &*stream;
    let mut messages = messages.fuse();
    let mut shutdown = shutdown.fuse();
    // while let Some(msg) = messages.next().await {
    //     stream.write_all(msg.as_bytes()).await?;
    // }
    loop { // 2
        select! {
            msg = messages.next().fuse() => match msg { // 3
                Some(msg) => stream.write_all(msg.as_bytes()).await?,
                None => break,
            },
            void = shutdown.next().fuse() => match void {
                Some(void) => match void {}, // 4
                None => break,
            }
        }
    }
    Ok(())
}

use std::collections::hash_map::{Entry, HashMap};

#[derive(Debug)]
enum Void {} // 1

#[derive(Debug)]
enum Event {
    NewPeer {
        name: String,
        stream: Arc<TcpStream>,
        shutdown: Receiver<Void>, // 2
    },
    Message {
        from: String,
        to: Vec<String>,
        msg: String,
    },
}

async fn broker_loop(events: Receiver<Event>) /*-> Result<()>*/ {
    // let mut writers = Vec::new();
    let (disconnect_sender, mut disconnect_receiver) = // 1
        mpsc::unbounded::<(String, Receiver<String>)>();
    let mut peers: HashMap<String, Sender<String>> = HashMap::new(); 
    let mut events = events.fuse();

    // while let Some(event) = events.next().await {

    loop {
        let event = select! {
            event = events.next().fuse() => match event {
                None => break, // 2
                Some(event) => event,
            },
            disconnect = disconnect_receiver.next().fuse() => {
                let (name, _pending_messages) = disconnect.unwrap(); // 3
                assert!(peers.remove(&name).is_some());
                continue;
            },
        };
        match event {
            Event::Message { from, to, msg } => {  // 3
                for addr in to {
                    if let Some(peer) = peers.get_mut(&addr) {
                        let msg = format!("from {}: {}\n", from, msg);
                        peer.send(msg).await
                        .unwrap()
                    }
                }
            }
            Event::NewPeer { name, stream, shutdown } => {
                match peers.entry(name.clone()) {
                    Entry::Occupied(..) => (),
                    Entry::Vacant(entry) => {
                        let (client_sender, mut client_receiver) = mpsc::unbounded();
                        entry.insert(client_sender); // 4
                        let mut disconnect_sender = disconnect_sender.clone();
                        // let handle = spawn_and_log_error(connection_writer_loop(&mut client_receiver, stream, shutdown));
                        // writers.push(handle);
                        spawn_and_log_error(async move {
                            let res = connection_writer_loop(&mut client_receiver, stream, shutdown).await;
                            disconnect_sender.send((name, client_receiver)).await // 4
                                .unwrap();
                            res
                        });
                    }
                }
            }
        }
    }
    drop(peers); // 3
    // for writer in writers { // 4
    //     writer.await;
    // }
    drop(disconnect_sender); // 6
    while let Some((_name, _pending_messages)) = disconnect_receiver.next().await {
    }
    // Ok(())
}

