use std::collections::HashMap;
use secp256k1::{Secp256k1, PublicKey};

use crate::wallet::seed::MasterSeed;
use crate::wallet::derive::{derive_key, DerivedKey};
use crate::wallet::address::pubkey_to_address;
use crate::wallet::role::Role;

pub struct Wallet {
    seed: Option<MasterSeed>, // None = locked
    next_index: HashMap<Role, u64>,
    keys: HashMap<(Role, Vec<u8>), DerivedKey>, // (role, address) → key
}

impl Wallet {
    pub fn new() -> Self {
        Self {
            seed: None,
            next_index: HashMap::new(),
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

    pub fn generate_address(&mut self, role: Role) -> Vec<u8> {
        let seed = self.seed.as_ref().expect("wallet locked");

        let index = self.next_index.entry(role.clone()).or_insert(0);
        let dk = derive_key(seed, &role, *index);
        *index += 1;

        let secp = Secp256k1::new();
        let pk = PublicKey::from_secret_key(&secp, &dk.secret);
        let addr = pubkey_to_address(&pk);

        self.keys.insert((role, addr.clone()), dk);
        addr
    }

    pub fn get_key(&self, role: &Role, address: &[u8]) -> Option<&DerivedKey> {
        self.keys.get(&(role.clone(), address.to_vec()))
    }
}
