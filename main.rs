use anyhow::Result;
use tracing::info;

mod cli;
mod node;
mod p2p;
mod rpc;
mod config;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = cli::parse();
    info!("Starting egg-node with args: {:?}", args);

    let cfg = config::load(&args)?;
    let mut node = node::Runtime::new(cfg);

    node.start().await?;
    Ok(())
}
