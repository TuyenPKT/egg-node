use rand::RngCore;
use sha2::{Sha256, Digest};

#[derive(Clone)]
pub struct MasterSeed {
    bytes: [u8; 32],
}

impl MasterSeed {
    pub fn generate() -> Self {
        let mut raw = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut raw);
        Self { bytes: Sha256::digest(raw).into() }
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self { bytes }
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }
}
