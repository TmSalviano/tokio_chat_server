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

#[tokio::main]
async fn main() -> Result<()> {
    let ip_addr = "127.0.0.1:8080";
    let server = TcpListener::bind(ip_addr).await?;
    println!("Passive open was established");

    let socket = match server.accept().await {
        Ok((socket, _)) => socket,
        Err(_) => panic!("Failed to establish an active connection"),
    };

    handle_socket(socket).await?;

    Ok(())
}

async fn handle_socket(mut socket: TcpStream) -> Result<()> {
    //1kb capacity for messages seems reasonable
    let mut buf = [0; 1024];
    loop {
        match socket.read(&mut buf).await {
            Ok(0) => {
                println!("client disconnected");
                return Ok(());
            },
            Ok(n) => {
                print!("Remote endpoint message: {}", String::from_utf8_lossy(&buf).trim());
            },
            Err(_) => {
                panic!("data transfer phase error")
            }

        }
    }
}
