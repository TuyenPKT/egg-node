use sha2::{Sha256, Digest};
use bincode;
use crate::chain::tx::Transaction;

pub fn txid(tx: &Transaction) -> [u8; 32] {
    let encoded = bincode::serialize(tx).unwrap();
    Sha256::digest(encoded).into()
}
