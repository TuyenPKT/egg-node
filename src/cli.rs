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

    Spend {
        #[arg(long)]
        txid: String,

        #[arg(long)]
        vout: u32,

        #[arg(long)]
        to: String,

        #[arg(long)]
        value: u64,
    },
}

impl Cli {
    pub fn execute(&self) {
        match &self.command {
            Commands::Init => {
                println!("Init node");
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
                use crate::pow::miner::mine_block;
                use crate::chain::hash::hash_header;

                let miner_addr = address.as_bytes().to_vec();

                let genesis = genesis_block();
                let db = ChainDB::open("./egg-chain");
                let mut chain = ChainState::load_or_init(genesis, db);

                let block = mine_block(chain.tip, 0, miner_addr);
                let hash = hash_header(&block.header);

                if chain.add_block(block) {
                    println!("Mined block: {:x?}", hash);
                } else {
                    println!("Mining failed");
                }
            }

            Commands::Spend { txid, vout, to, value } => {
                use crate::chain::tx::{Transaction, TxInput, TxOutput};
                use crate::chain::sign::sign_tx;
                use secp256k1::{Secp256k1, SecretKey};

                let secp = Secp256k1::new();
                let sk = SecretKey::from_slice(&[1u8; 32]).unwrap();
                let pk = secp256k1::PublicKey::from_secret_key(&secp, &sk);

                let mut tx = Transaction {
                    inputs: vec![TxInput {
                        prev_txid: hex::decode(txid).unwrap().try_into().unwrap(),
                        vout: *vout,
                        signature: vec![],
                        pubkey: pk.serialize().to_vec(),
                    }],
                    outputs: vec![TxOutput {
                        value: *value,
                        to_address: to.as_bytes().to_vec(),
                    }],
                    data: vec![],
                };

                let sig = sign_tx(&tx, &sk);
                tx.inputs[0].signature = sig;

                println!("Created transaction:");
                println!("{:#?}", tx);
            }

            Commands::Utxo => {
                let genesis = genesis_block();
                let db = ChainDB::open("./egg-chain");
                let chain = ChainState::load_or_init(genesis, db);

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
