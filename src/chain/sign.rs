use secp256k1::{Secp256k1, SecretKey, PublicKey, Message};
use sha2::{Sha256, Digest};
use crate::chain::tx::Transaction;

pub fn sign_tx(tx: &Transaction, sk: &SecretKey) -> Vec<u8> {
    let secp = Secp256k1::new();
    let hash = tx_hash(tx);
    let msg = Message::from_digest_slice(&hash).unwrap();
    let sig = secp.sign_ecdsa(&msg, sk);
    sig.serialize_der().to_vec()
}

pub fn verify_tx(tx: &Transaction) -> bool {
    let secp = Secp256k1::new();
    let hash = tx_hash(tx);
    let msg = Message::from_digest_slice(&hash).unwrap();

    for inp in &tx.inputs {
        let pk = match PublicKey::from_slice(&inp.pubkey) {
            Ok(p) => p,
            Err(_) => return false,
        };
        let sig = match secp256k1::ecdsa::Signature::from_der(&inp.signature) {
            Ok(s) => s,
            Err(_) => return false,
        };
        if secp.verify_ecdsa(&msg, &sig, &pk).is_err() {
            return false;
        }
    }
    true
}

fn tx_hash(tx: &Transaction) -> [u8; 32] {
    let encoded = bincode::serialize(tx).unwrap();
    Sha256::digest(encoded).into()
}
