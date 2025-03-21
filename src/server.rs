use anyhow::Result;
use log::{error, info};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

/// The Server struct holds the tokio TcpListener and a shared map of active connections.
#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
    clients: Arc<Mutex<HashMap<usize, Arc<Mutex<TcpStream>>>>>,
}

impl Server {
    pub fn new(listener: TcpListener) -> Server {
        Server {
            listener,
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn connect(listener: &TcpListener) -> Result<TcpStream> {
        match listener.accept().await {
            Ok((socket, _)) => Ok(socket),
            Err(e) => Err(anyhow::anyhow!("connection failed: {}", e.to_string())),
        }
    }
    pub async fn run(&mut self) -> Result<()> {
        let mut client_id = 0; 
        loop {
            let sock = match Self::connect(&self.listener).await {
                Ok(sock) => sock,
                Err(_) => continue, 
            };
            let sock = Arc::new(Mutex::new(sock));
            let clients = Arc::clone(&self.clients);
            clients.lock().await.insert(client_id, Arc::clone(&sock));

            let clients = Arc::clone(&self.clients);
            tokio::spawn(async move {
                Self::handle_client(client_id, sock, clients).await;
            });
            client_id += 1;
        }
    }
    async fn send_to_group(
        sender_id: usize,
        message: &str,
        clients: &Arc<Mutex<HashMap<usize, Arc<Mutex<TcpStream>>>>>,
    ) {
        let clients = clients.lock().await;
        let other_clients: Vec<_> = clients
            .iter()
            .filter(|(id, _)| **id != sender_id)
            //Solution to the bottleneck issue: if I clone the socket I can drop the lock on the dict fast
            .map(|(id, client_sock)| (*id, Arc::clone(client_sock)))
            .collect();

        drop(clients);

        //write to the other clients
        for (id, client_sock) in other_clients {
            let message = message.to_string();
            tokio::spawn(async move {
                let mut client_sock = client_sock.lock().await;
                if let Err(e) = client_sock.write_all(message.as_bytes()).await {
                    error!("did not sent message to client {}. Error: {}", id, e);
                }
                if let Err(e) = client_sock.flush().await {
                    error!("failed to flush data to socket/client {}. Error: {}", id, e);
                }
            });
        }
    }
    async fn handle_client(
        client_id: usize,
        sock: Arc<Mutex<TcpStream>>,
        clients: Arc<Mutex<HashMap<usize, Arc<Mutex<TcpStream>>>>>,
    ) {
        let mut buf = [0; 1024];

        loop {
            if let Ok(n) = sock.lock().await.read(&mut buf).await {
                if n == 0 {
                    info!("Client {} disconnected", client_id);
                    break;
                }
                let msg = String::from_utf8_lossy(&buf[0..n]);
                // Skip telnet non-messages.
                if msg.is_empty() || msg == "\n" || msg == "\r" || msg == "\r\n" {
                    continue;
                }
                let message = String::from_utf8_lossy(&buf[0..n]).to_string();
                info!("Message was sent to Client{}: {}", client_id, message);

                Self::send_to_group(client_id, &message, &clients).await;
            }
        }

        //if they disconnect for whatever reason you have to remove them from the dict
        clients.lock().await.remove(&client_id);
    }
}
