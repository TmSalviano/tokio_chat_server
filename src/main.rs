
use std::sync::Arc;
//matches many different Results for convenience
use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::sync::mpsc::{Receiver, Sender, channel};

// Group Chat:
//DataStructure to store the sockets -> Done
//Task - listen to possible connections on a loop Client connections with the Server/Listener and add the
//socket to the socket_store if active open connection as established.
//  Change the active_open() function to do this and spawn a dedicated task for this.
//Tasks to handle the sockets (TCP Streams)
//in socket_handle() write the bytes from the data stream to all other clients

#[tokio::main]
async fn main() -> Result<()> {
    let ip_addr = "127.0.0.1:8080";
    let server = passive_open(ip_addr).await?;
    let (tx, rx) = channel(1024);
    //(id, socket) collection
    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    tokio::spawn(send_to_group(rx, clients.clone()));

    loop {
        let (mut socket, _) = server.accept().await?;
        let tx = tx.clone();
        let clients = clients.clone();

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            clients.lock().await.push(socket);
            if let Ok(n) = socket.read(&mut buf).await {
                if n == 0 {
                    println!("client disconnected");
                };
            }
            tx.send(String::from_utf8_lossy(&buf).to_string())
                .await
                .unwrap();
        });
    }
}

async fn passive_open(ip_addr: &str) -> Result<TcpListener> {
    let server = TcpListener::bind(ip_addr).await?;
    println!("Passive open was established");
    Ok(server)
}

async fn send_to_group(
    mut consumer: Receiver<String>,
    ref mut clients: Arc<Mutex<Vec<TcpStream>>>,
) {
    while let Some(message) = consumer.recv().await {
        let mut clients = clients.lock().await;
        for client in &mut *clients {
            client.write_all(message.as_bytes());
        }
    }
}
