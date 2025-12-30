use sha2::{Sha256, Digest};
use bincode;
use super::header::BlockHeader;

pub fn hash_header(header: &BlockHeader) -> [u8; 32] {
    let encoded = bincode::serialize(header).unwrap();
    Sha256::digest(encoded).into()
}
