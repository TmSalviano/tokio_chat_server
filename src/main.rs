pub mod server;

use core::panic;
use std::io::Read;

use server::Server;
//matches many different Results for convenience
use anyhow::Result;
//logs at the info level
use log::info;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

// Group Chat:
//DataStructure to store the sockets
//Task - listen to possible connections on a loop Client connections with the Server/Listener and add the
//socket to the socket_store if active open connection as established
//Tasks to handle the sockets (TCP Streams)
//in socket_handle() write the bytes from the data stream to all other clients

#[tokio::main]
async fn main() -> Result<()> {
    let ip_addr = "127.0.0.1:8080";
    let server = passive_open(ip_addr).await?;
    let socket = active_open(server).await;
    handle_socket(socket).await?;

    Ok(())
}

async fn passive_open(ip_addr: &str) -> Result<TcpListener>{
    let server = TcpListener::bind(ip_addr).await?;
    println!("Passive open was established");
    Ok(server)
}

async fn active_open(server: TcpListener) -> TcpStream {
    let socket = match server.accept().await {
        Ok((socket, _)) => socket,
        Err(_) => panic!("Failed to establish an active open connection"),
    };
    socket
}

async fn handle_socket(mut socket: TcpStream) -> Result<()> {
    //1kb capacity for messages seems reasonable
    let mut buf = [0; 1024];
    loop {
        match socket.read(&mut buf).await {
            Ok(0) => {
                println!("client disconnected");
                return Ok(());
            }
            Ok(_) => {
                print!(
                    "Remote endpoint message: {}",
                    String::from_utf8_lossy(&buf).trim()
                );
            }
            Err(_) => {
                panic!("data transfer phase error")
            }
        }
    }
}
