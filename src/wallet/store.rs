use std::collections::HashMap;
use secp256k1::{Secp256k1, PublicKey};

use crate::wallet::seed::MasterSeed;
use crate::wallet::derive::{derive_key, DerivedKey};
use crate::wallet::address::pubkey_to_address;
use crate::wallet::persist::{save_wallet, load_wallet};

pub struct Wallet {
    seed: Option<MasterSeed>, // None = locked
    domain: String,
    next_index: u64,
    keys: HashMap<Vec<u8>, DerivedKey>,
}

impl Wallet {
    pub fn new(domain: &str) -> Self {
        Self {
            seed: None,
            domain: domain.to_string(),
            next_index: 0,
            keys: HashMap::new(),
        }
    }

    pub fn unlock(&mut self, seed: MasterSeed) {
        self.seed = Some(seed);
    }

    pub fn lock(&mut self) {
        self.seed = None;
        self.keys.clear();
    }

    pub fn generate_address(&mut self) -> Vec<u8> {
        let seed = self.seed.as_ref().expect("wallet locked");

        let dk = derive_key(seed, &self.domain, self.next_index);
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
