use serde::{Serialize, Deserialize};
use secp256k1::{Secp256k1, SecretKey, PublicKey, Message};
use sha2::{Sha256, Digest};

use crate::wallet::role::Role;
use crate::wallet::derive::DerivedKey;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LeaseScope {
    Spend { max_amount: u64 },
    Mine,
    Vote,
    Custom(String),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct KeyLease {
    pub role: Role,
    pub lease_pubkey: Vec<u8>,
    pub scope: LeaseScope,
    pub expiry: u64, // unix timestamp
    pub issuer_sig: Vec<u8>, // signed by role key
}

impl KeyLease {
    /// role key issues a lease for AI/bot
    pub fn issue(
        role_key: &DerivedKey,
        lease_pubkey: &PublicKey,
        scope: LeaseScope,
        expiry: u64,
    ) -> Self {
        let secp = Secp256k1::new();
        let payload = lease_payload(&role_key.role, lease_pubkey, &scope, expiry);

        let msg = Message::from_digest_slice(&payload).unwrap();
        let sig = secp.sign_ecdsa(&msg, &role_key.secret);

        KeyLease {
            role: role_key.role.clone(),
            lease_pubkey: lease_pubkey.serialize().to_vec(),
            scope,
            expiry,
            issuer_sig: sig.serialize_der().to_vec(),
        }
    }

    /// verify lease authenticity
    pub fn verify(&self, role_pubkey: &PublicKey, now: u64) -> bool {
        if now > self.expiry {
            return false;
        }

        let secp = Secp256k1::new();
        let payload = lease_payload(
            &self.role,
            &PublicKey::from_slice(&self.lease_pubkey).unwrap(),
            &self.scope,
            self.expiry,
        );

        let msg = Message::from_digest_slice(&payload).unwrap();
        let sig = match secp256k1::ecdsa::Signature::from_der(&self.issuer_sig) {
            Ok(s) => s,
            Err(_) => return false,
        };

        secp.verify_ecdsa(&msg, &sig, role_pubkey).is_ok()
    }
}

fn lease_payload(
    role: &Role,
    lease_pubkey: &PublicKey,
    scope: &LeaseScope,
    expiry: u64,
) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(role.as_domain().as_bytes());
    hasher.update(lease_pubkey.serialize());
    hasher.update(bincode::serialize(scope).unwrap());
    hasher.update(expiry.to_le_bytes());
    hasher.finalize().into()
}
