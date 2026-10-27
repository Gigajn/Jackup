use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2,
};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use rand::RngCore;
use thiserror::Error;
use zeroize::Zeroizing;

const NONCE_SIZE: usize = 12;

#[derive(Debug, Error)]
pub enum VaultError {
    #[error("password-based key derivation failed")]
    KeyDerivation,
    #[error("encryption failed")]
    Encryption,
    #[error("decryption failed")]
    Decryption,
    #[error("ciphertext is too short")]
    InvalidCiphertext,
}

/// Derives a 256-bit encryption key from a password and Argon2id salt.
///
/// # Errors
///
/// Returns [`VaultError::KeyDerivation`] if Argon2id cannot derive the key.
pub fn derive_key(password: &str, salt: &SaltString) -> Result<Zeroizing<[u8; 32]>, VaultError> {
    let mut key = Zeroizing::new([0_u8; 32]);
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt.as_str().as_bytes(), key.as_mut())
        .map_err(|_| VaultError::KeyDerivation)?;
    Ok(key)
}

/// Encrypts plaintext with ChaCha20-Poly1305 and prepends a random nonce.
///
/// # Errors
///
/// Returns [`VaultError::Encryption`] if authenticated encryption fails.
pub fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, VaultError> {
    let cipher = ChaCha20Poly1305::new(key.into());
    let mut nonce = [0_u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce);
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext)
        .map_err(|_| VaultError::Encryption)?;
    let mut result = nonce.to_vec();
    result.extend(ciphertext);
    Ok(result)
}

/// Decrypts a nonce-prefixed ChaCha20-Poly1305 payload.
///
/// # Errors
///
/// Returns [`VaultError::InvalidCiphertext`] for a truncated payload or
/// [`VaultError::Decryption`] when authentication fails.
pub fn decrypt(key: &[u8; 32], payload: &[u8]) -> Result<Vec<u8>, VaultError> {
    if payload.len() < NONCE_SIZE {
        return Err(VaultError::InvalidCiphertext);
    }
    let (nonce, ciphertext) = payload.split_at(NONCE_SIZE);
    ChaCha20Poly1305::new(key.into())
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|_| VaultError::Decryption)
}

pub fn new_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_encrypted_data() {
        let salt = new_salt();
        let key = derive_key("correct horse battery staple", &salt).unwrap();
        let encrypted = encrypt(&key, b"private configuration").unwrap();

        assert_ne!(encrypted, b"private configuration");
        assert_eq!(decrypt(&key, &encrypted).unwrap(), b"private configuration");
    }

    #[test]
    fn rejects_tampered_data() {
        let key = derive_key("password", &new_salt()).unwrap();
        let mut encrypted = encrypt(&key, b"secret").unwrap();
        let last_byte = encrypted.len() - 1;
        encrypted[last_byte] ^= 1;

        assert!(matches!(
            decrypt(&key, &encrypted),
            Err(VaultError::Decryption)
        ));
    }
}
