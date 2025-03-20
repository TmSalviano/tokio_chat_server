pub mod server;

use server::Server;
//matches many different Results for convenience
use anyhow::Result;
//logs at the info level
use log::info;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let ip_addr = "127.0.0.1:8080";

    let listener = match TcpListener::bind(ip_addr).await {
        Ok(the_listener) => {
            info!("TCP listener started on port 8080");
            the_listener
        }
        Err(e) => panic!("Failed to bind to localhost:8080"),
    };

    let server = Server::new(listener);
    Ok(())
}
