use sha2::{Sha256, Digest};
use secp256k1::SecretKey;

use crate::wallet::seed::MasterSeed;

#[derive(Clone)]
pub struct DerivedKey {
    pub index: u64,
    pub secret: SecretKey,
}

pub fn derive_key(
    seed: &MasterSeed,
    domain: &str,
    index: u64,
) -> DerivedKey {
    let mut hasher = Sha256::new();

    hasher.update(seed.as_bytes());
    hasher.update(domain.as_bytes());
    hasher.update(index.to_le_bytes());

    let hash = hasher.finalize();
    let sk = SecretKey::from_slice(&hash).expect("invalid key");

    DerivedKey {
        index,
        secret: sk,
    }
}
