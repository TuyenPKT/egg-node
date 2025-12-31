use clap::{Parser, Subcommand};




#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Run,
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

            Commands::Run => {
                use crate::storage::sleddb::ChainDB;
                use crate::chain::genesis_block;
                use crate::chain::state::ChainState;
                use crate::node::run_node;
                use crate::config::NodeConfig;

                let config = NodeConfig::default();

                let genesis = genesis_block();
                let db = ChainDB::open("./egg-chain");
                let chain = ChainState::load_or_init(genesis, db);

                run_node(config, chain);
            }

            Commands::Mine { address } => {
                use crate::storage::sleddb::ChainDB;
                use crate::chain::genesis_block;
                use crate::chain::state::ChainState;
                use crate::pow::miner::mine_block;
                use crate::chain::hash::hash_header;

                // 1. load chain
                let genesis = genesis_block();
                let db = ChainDB::open("./egg-chain");
                let mut chain = ChainState::load_or_init(genesis, db);

                // 2. mine block
                let miner_addr = address.as_bytes().to_vec();

                let block = mine_block(
                    chain.tip,
                    chain.blocks.get(&chain.tip).unwrap().height,
                    miner_addr,
                );

                let hash = hash_header(&block.header);

                // 3. add block
                if chain.add_block(block) {
                    println!("Mined new block: {:x?}", hash);
                } else {
                    println!("Mining failed (invalid block)");
                }
            }

            Commands::Utxo => {
                use crate::storage::sleddb::ChainDB;
                use crate::chain::genesis_block;
                use crate::chain::state::ChainState;

                let genesis = genesis_block();
                let db = ChainDB::open("./egg-chain");
                let chain = ChainState::load_or_init(genesis, db);

                if chain.utxos.is_empty() {
                    println!("No UTXO found");
                    return;
                }

                for utxo in chain.utxos.values() {
                    println!(
                        "UTXO {}:{} value={} height={}",
                        hex::encode(utxo.txid),
                        utxo.vout,
                        utxo.value,
                        utxo.height
                    );
                }
            }
        }
    }
}


