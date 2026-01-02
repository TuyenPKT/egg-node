use sha2::{Sha256, Digest};
use secp256k1::{PublicKey, Secp256k1};

pub fn pubkey_to_address(pubkey: &PublicKey) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(pubkey.serialize());
    hasher.finalize().to_vec()
}
