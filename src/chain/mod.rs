use crate::chain::block::{Block, merkle_root};
use crate::chain::header::BlockHeader;
use crate::chain::tx::Transaction;



use sha2::{Sha256, Digest};
use bincode;


pub mod block;
pub mod header;
pub mod tx;
pub mod validation;
pub mod hash;
pub mod state;
pub mod reward;





pub fn genesis_block() -> Block {
    let coinbase = Transaction::coinbase(
        b"genesis".to_vec(),
        0,
        "Egg Core Genesis — Re-establishing the right to run a node at home — 2025-01-01",
    );

    let txs = vec![coinbase];

    let header = BlockHeader {
        version: 1,
        prev_hash: [0u8; 32],
        merkle_root: merkle_root(&txs),
        timestamp: 1735689600, // hardcode UNIX time
        bits: 0x1f00ffff,      // difficulty cực thấp
        nonce: 0,
    };

    Block {
        header,
        transactions: txs,
    }
}

pub fn genesis_hash() -> [u8; 32] {
    let genesis = genesis_block();
    let encoded = bincode::serialize(&genesis.header).unwrap();
    Sha256::digest(encoded).into()
}
