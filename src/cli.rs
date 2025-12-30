use clap::{Parser, Subcommand};

use crate::config::NodeConfig;
use crate::node::run_node;
use crate::chain::genesis_block;
use crate::chain::state::ChainState;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Bind address, ví dụ: 0.0.0.0:8333
    #[arg(long)]
    pub bind: Option<String>,

    /// Peer address, ví dụ: 180.93.1.235:8333
    #[arg(long)]
    pub peer: Vec<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Run,
}

impl Cli {
    pub fn execute(&self) {
        match self.command {
            Commands::Init => {
                println!("Init node (genesis only)");
            }
            Commands::Run => {
                let mut config = NodeConfig::default();

                if let Some(bind) = &self.bind {
                    config.bind_addr = bind.clone();
                }

                if !self.peer.is_empty() {
                    config.peers = self.peer.clone();
                }

                let genesis = genesis_block();
                let chain = ChainState::new(genesis);

                run_node(config, chain);
            }
        }
    }
}
