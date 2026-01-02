use rand::{RngCore, rngs::OsRng};
use argon2::{Argon2, PasswordHasher};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use sha2::{Sha256, Digest};

pub struct EncryptedBlob {
    pub ciphertext: Vec<u8>,
    pub salt: [u8; 16],
    pub nonce: [u8; 12],
}

fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    let argon = Argon2::default();

    argon.hash_password_into(
        password.as_bytes(),
        salt,
        &mut key,
    ).expect("argon2 failed");

    key
}

pub fn encrypt(password: &str, plaintext: &[u8]) -> EncryptedBlob {
    let mut salt = [0u8; 16];
    let mut nonce = [0u8; 12];

    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce);

    let key_bytes = derive_key(password, &salt);
    let cipher = Aes256Gcm::new(Key::from_slice(&key_bytes));

    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext)
        .expect("encrypt failed");

    EncryptedBlob {
        ciphertext,
        salt,
        nonce,
    }
}

pub fn decrypt(password: &str, blob: &EncryptedBlob) -> Vec<u8> {
    let key_bytes = derive_key(password, &blob.salt);
    let cipher = Aes256Gcm::new(Key::from_slice(&key_bytes));

    cipher
        .decrypt(
            Nonce::from_slice(&blob.nonce),
            blob.ciphertext.as_ref(),
        )
        .expect("invalid password or corrupted wallet")
}
