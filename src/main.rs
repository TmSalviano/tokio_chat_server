use anyhow::Result;


mod server;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    Ok(())
}