use tokio::net::TcpListener;


#[derive(Debug)]
struct Server {
   listener: TcpListener 
} 

impl Server {
    fn new(listener: TcpListener) -> Self {
        Self { listener}
    }
}