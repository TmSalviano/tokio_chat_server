use std::{
    collections::HashMap,
    ops::Mul,
    sync::{Arc},
};

use anyhow::Result;
use log::{error, info};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex}

#[derive(Debug)]
struct Server {
    listener: TcpListener,
    clients: Arc<Mutex<HashMap<usize, Arc<Mutex<TcpStream>>>>>,
}

impl Server {
    fn new(listener: TcpListener) -> Self {
        Self {
            listener,
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn connect(listener: &TcpListener) -> Result<(TcpStream)> {
        todo!()
    }

    async fn send_to_group(
        sender_id: usize,
        message: &str,
        clients: &Arc<Mutex<HashMap<usize, Arc<Mutex<TcpStream>>>>>,
    ) {
        todo!()
    }

    async fn handle_client(
        client_id: usize,
        sock: Arc<Mutex<TcpStream>>,
        clients: Arc<Mutex<HashMap<usize, Arc<Mutex<TcpStream>>>>>,
    ) {
        //You are going to send_to_group() somewhere here
        todo!()
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut client_id = 0; // Unique ID for each client.
        loop {
            // connect to TCP connection.
            todo!();

            // sock == connect() output
            let sock = Arc::new(Mutex::new(sock));

            let clients = Arc::clone(&self.clients);
            clients.lock().await.insert(client_id, Arc::clone(&sock));

            let clients = Arc::clone(&self.clients);
            tokio::spawn(async move {
                //client_handle() the stream/sock of each client
                todo!()
            });

            client_id += 1; 
        }
    }
}
