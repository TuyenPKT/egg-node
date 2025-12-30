use serde::{Serialize, Deserialize};

use super::header::BlockHeader;
use super::tx::Transaction;

use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

pub fn merkle_root(txs: &[Transaction]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    for tx in txs {
        let encoded = bincode::serialize(tx).unwrap();
        hasher.update(encoded);
    }
    hasher.finalize().into()
}