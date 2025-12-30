use clap::{Parser, Subcommand};

use crate::config::NodeConfig;
use crate::node::run_node;
use crate::chain::genesis_block;
use crate::chain::state::ChainState;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,

    Run {
        /// Bind address, ví dụ: 0.0.0.0:8333
        #[arg(long, default_value = "0.0.0.0:8333")]
        bind: String,

        /// Peer address, ví dụ: 180.93.1.235:8333
        #[arg(long)]
        peer: Vec<String>,
    },
}

impl Cli {
    pub fn execute(&self) {
        match &self.command {
            Commands::Init => {
                println!("Init node (genesis only)");
            }

            Commands::Run { bind, peer } => {
                let mut config = NodeConfig::default();
                config.bind_addr = bind.clone();
                config.peers = peer.clone();

                let genesis = genesis_block();
                let chain = ChainState::load_or_init("./egg-data", genesis);


                run_node(config, chain);
            }
        }
    }
}
