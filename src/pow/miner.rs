use sha2::{Sha256, Digest};
use crate::chain::header::BlockHeader;

pub fn mine(header: &mut BlockHeader, target: &[u8; 32]) {
    loop {
        let hash = hash_header(header);
        if &hash[..] <= target {
            break;
        }
        header.nonce += 1;
    }
}

fn hash_header(header: &BlockHeader) -> [u8; 32] {
    let encoded = bincode::serialize(header).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(encoded);
    hasher.finalize().into()
}
