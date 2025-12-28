use anyhow::Result;
use tracing::info;

use crate::{p2p, rpc, config::Config};

pub struct Runtime {
    config: Config,
}

impl Runtime {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("Node starting…");

        p2p::start(&self.config).await?;
        rpc::start(&self.config).await?;

        info!("Node is running");
        futures::future::pending::<()>().await;
        Ok(())
    }
}
