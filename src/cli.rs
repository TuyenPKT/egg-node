use clap::{Parser, Subcommand};

use crate::config::NodeConfig;
use crate::node::run_node;
use crate::chain::genesis_block;
use crate::chain::state::ChainState;
use crate::storage::sleddb::ChainDB;

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
        #[arg(long)]
        bind: Option<String>,

        /// Peer address, ví dụ: 180.93.1.235:8333
        #[arg(long)]
        peer: Vec<String>,
    },

    Mine {
        #[arg(long)]
        address: String,
    },

    Utxo,
}

impl Cli {
    pub fn execute(&self) {
        match &self.command {
            Commands::Init => {
                println!("Init node (genesis only)");
            }

            Commands::Run { bind, peer } => {
                let mut config = NodeConfig::default();

                if let Some(b) = bind {
                    config.bind_addr = b.clone();
                }

                if !peer.is_empty() {
                    config.peers = peer.clone();
                }

                let genesis = genesis_block();
                let db = ChainDB::open("./egg-chain");
                let chain = ChainState::load_or_init(genesis, db);

                run_node(config, chain);
            }

            Commands::Mine { address } => {
                println!("Mining to address: {}", address);
                // phần mine đã có trước đó, giữ nguyên
            }

            Commands::Utxo => {
                let genesis = genesis_block();
                let db = ChainDB::open("./egg-chain");
                let chain = ChainState::load_or_init(genesis, db);

                for utxo in chain.utxos.values() {
                    println!(
                        "UTXO {:x?}:{} value={} height={}",
                        utxo.txid,
                        utxo.vout,
                        utxo.value,
                        utxo.height
                    );
                }
            }
        }
    }
}
