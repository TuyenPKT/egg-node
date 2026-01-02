use std::collections::HashMap;
use secp256k1::{Secp256k1, PublicKey};

use crate::wallet::seed::MasterSeed;
use crate::wallet::derive::{derive_key, DerivedKey};
use crate::wallet::address::pubkey_to_address;

pub struct Wallet {
    seed: MasterSeed,
    domain: String,
    next_index: u64,
    keys: HashMap<Vec<u8>, DerivedKey>, // address → key
}

impl Wallet {
    pub fn new(seed: MasterSeed, domain: &str) -> Self {
        Self {
            seed,
            domain: domain.to_string(),
            next_index: 0,
            keys: HashMap::new(),
        }
    }

    pub fn generate_address(&mut self) -> Vec<u8> {
        let dk = derive_key(&self.seed, &self.domain, self.next_index);
        self.next_index += 1;

        let secp = Secp256k1::new();
        let pk = PublicKey::from_secret_key(&secp, &dk.secret);
        let addr = pubkey_to_address(&pk);

        self.keys.insert(addr.clone(), dk);
        addr
    }

    pub fn get_key(&self, address: &[u8]) -> Option<&DerivedKey> {
        self.keys.get(address)
    }
}
