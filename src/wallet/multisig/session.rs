use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use secp256k1::{PublicKey, Message, Secp256k1};
use sha2::{Sha256, Digest};

use crate::wallet::multisig::policy::MultisigPolicy;

#[derive(Clone, Serialize, Deserialize)]
pub struct MultisigSession {
    pub policy: MultisigPolicy,
    pub payload_hash: [u8; 32],
    pub signatures: HashMap<Vec<u8>, Vec<u8>>, // pubkey -> sig
}

impl MultisigSession {
    pub fn new(policy: MultisigPolicy, payload: &[u8]) -> Self {
        let hash = Sha256::digest(payload).into();

        MultisigSession {
            policy,
            payload_hash: hash,
            signatures: HashMap::new(),
        }
    }

    pub fn add_signature(
        &mut self,
        pubkey: &PublicKey,
        sig: Vec<u8>,
    ) -> bool {
        if !self.policy.is_signer(pubkey) {
            return false;
        }

        self.signatures.insert(pubkey.serialize().to_vec(), sig);
        true
    }

    pub fn is_satisfied(&self, now: u64) -> bool {
        if let Some(tl) = self.policy.timelock {
            if now < tl {
                return false;
            }
        }

        self.signatures.len() >= self.policy.threshold as usize
    }

    pub fn verify(&self) -> bool {
        let secp = Secp256k1::new();
        let msg = Message::from_digest_slice(&self.payload_hash).unwrap();

        let mut valid = 0;

        for (pk_bytes, sig_bytes) in &self.signatures {
            let pk = match PublicKey::from_slice(pk_bytes) {
                Ok(p) => p,
                Err(_) => continue,
            };

            let sig = match secp256k1::ecdsa::Signature::from_der(sig_bytes) {
                Ok(s) => s,
                Err(_) => continue,
            };

            if secp.verify_ecdsa(&msg, &sig, &pk).is_ok() {
                valid += 1;
            }
        }

        valid >= self.policy.threshold as usize
    }
}
