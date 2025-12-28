use anyhow::Result;
use tracing::info;
use crate::config::Config;

pub async fn start(cfg: &Config) -> Result<()> {
    info!("RPC server on {}", cfg.rpc_port);
    Ok(())
}
