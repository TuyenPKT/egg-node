use anyhow::Result;
use tracing::info;
use crate::config::Config;

pub async fn start(cfg: &Config) -> Result<()> {
    info!("P2P listening on {}", cfg.p2p_port);
    Ok(())
}
