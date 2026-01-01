use clap::{Parser, Subcommand};

use crate::node::run_node;
use crate::config::NodeConfig;
use crate::chain::state::ChainState;
use crate::storage::sleddb::ChainDB;
use crate::chain::genesis_block;
use crate::chain::tx::{Transaction, TxInput, TxOutput};
use crate::chain::sign::sign_tx;


#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Run,
    Send {
        txid: String,
        vout: u32,
        to: String,
        value: u64,
    },
}

impl Cli {
    pub fn execute(&self) {
        match &self.command {
            Commands::Run => {
                let db = ChainDB::open("./egg-chain");
                let genesis = genesis_block();
                let chain = ChainState::load_or_init(genesis, db);
                let config = NodeConfig::default();
                run_node(config, chain);
            }

            Commands::Send { txid, vout, to, value } => {
                use secp256k1::SecretKey;

                let sk = SecretKey::from_slice(&[1u8; 32]).unwrap();
                let pubkey = secp256k1::PublicKey::from_secret_key(
                    &secp256k1::Secp256k1::new(),
                    &sk,
                );

                let mut tx = Transaction {
                    inputs: vec![TxInput {
                        prev_txid: hex::decode(txid).unwrap().try_into().unwrap(),
                        vout: *vout,
                        signature: vec![],
                        pubkey: pubkey.serialize().to_vec(),
                    }],
                    outputs: vec![TxOutput {
                        value: *value,
                        to_address: to.as_bytes().to_vec(),
                    }],
                    data: vec![],
                };

                let sig = sign_tx(&tx, &sk);
                tx.inputs[0].signature = sig;

                println!("Broadcast TX: {:?}", tx);
            }
        }
    }
}
