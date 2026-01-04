use serde::{Serialize, Deserialize};
use secp256k1::PublicKey;

use crate::wallet::role::Role;

#[derive(Clone, Serialize, Deserialize)]
pub struct MultisigPolicy {
    pub role: Role,
    pub threshold: u8,          // M
    pub signers: Vec<PublicKey>,// N
    pub timelock: Option<u64>,  // unix timestamp
}

impl MultisigPolicy {
    pub fn validate(&self) -> bool {
        self.threshold > 0
            && self.threshold as usize <= self.signers.len()
    }

    pub fn is_signer(&self, pk: &PublicKey) -> bool {
        self.signers.iter().any(|s| s == pk)
    }
}
