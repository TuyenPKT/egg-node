use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

use crate::wallet::crypto::{EncryptedBlob, encrypt, decrypt};
use crate::wallet::seed::MasterSeed;

#[derive(Serialize, Deserialize)]
pub struct WalletFile {
    pub version: u32,
    pub encrypted_seed: Vec<u8>,
    pub salt: [u8; 16],
    pub nonce: [u8; 12],
}

pub fn save_wallet(
    path: &Path,
    seed: &MasterSeed,
    password: &str,
) {
    let blob = encrypt(password, seed.as_bytes());

    let file = WalletFile {
        version: 1,
        encrypted_seed: blob.ciphertext,
        salt: blob.salt,
        nonce: blob.nonce,
    };

    let encoded = bincode::serialize(&file).unwrap();
    fs::write(path, encoded).expect("write wallet failed");
}

pub fn load_wallet(
    path: &Path,
    password: &str,
) -> MasterSeed {
    let raw = fs::read(path).expect("wallet not found");
    let file: WalletFile = bincode::deserialize(&raw).unwrap();

    let blob = EncryptedBlob {
        ciphertext: file.encrypted_seed,
        salt: file.salt,
        nonce: file.nonce,
    };

    let seed_bytes = decrypt(password, &blob);
    let mut seed = [0u8; 32];
    seed.copy_from_slice(&seed_bytes[..32]);

    MasterSeed::from_bytes(seed)
}
