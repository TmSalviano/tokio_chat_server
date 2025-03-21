mod server;

use crate::server::Server;
use anyhow::Result;
use log::info;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    //RUST_LOG = info to ouput info level and higher level logs (error level)
    env_logger::init();

    let addr = format!("127.0.0.1:8080");

    let listener = match TcpListener::bind(&addr).await {
        Ok(tcp_listener) => {
            info!("successfull passive open connection on port 8080");
            tcp_listener
        }
        Err(e) => panic!("Failed to boun to {} ip_addr. Err: {}", &addr, e),
    };

    let mut server = Server::new(listener);
    server.run().await?;
    Ok(())
}
