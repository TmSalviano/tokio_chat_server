use tokio::net::TcpListener;

#[derive(Debug)]
pub struct Server {
    //this listen to connections
    listener: TcpListener,
}

impl Server {
    pub fn new(listener: TcpListener) -> Self {
        Self { listener }
    }
}
