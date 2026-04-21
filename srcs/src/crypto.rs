use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use std::io;

// AES-256-GCM uses a 32-byte key and a 12-byte nonce.
// We prepend the nonce to the ciphertext so we can recover it on decrypt.
const NONCE_LEN: usize = 12;

/// Generate a random 32-byte key and return it as a hex string (64 chars).
pub fn generate_key() -> String {
    let key = Aes256Gcm::generate_key(OsRng);
    hex::encode(key)
}

/// Parse a hex key string into 32 bytes.
fn parse_key(hex_key: &str) -> Result<[u8; 32], io::Error> {
    let bytes = hex::decode(hex_key).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid key (not valid hex): {}", e),
        )
    })?;

    if bytes.len() != 32 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Invalid key length: expected 32 bytes (64 hex chars), got {}",
                bytes.len()
            ),
        ));
    }

    let mut arr = [0u8; 32];
    arr.copy_from_slice(&bytes);
    Ok(arr)
}

/// Encrypt `plaintext` with the given hex key.
/// Returns `nonce (12 bytes) || ciphertext`.
pub fn encrypt(plaintext: &[u8], hex_key: &str) -> Result<Vec<u8>, io::Error> {
    let key_bytes = parse_key(hex_key)?;
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let nonce = Aes256Gcm::generate_nonce(OsRng);
    let ciphertext = cipher.encrypt(&nonce, plaintext).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Encryption failed: {}", e))
    })?;

    let mut result = Vec::with_capacity(NONCE_LEN + ciphertext.len());
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

/// Decrypt `data` (nonce || ciphertext) with the given hex key.
pub fn decrypt(data: &[u8], hex_key: &str) -> Result<Vec<u8>, io::Error> {
    if data.len() < NONCE_LEN {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Data too short to contain nonce",
        ));
    }

    let key_bytes = parse_key(hex_key)?;
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let (nonce_bytes, ciphertext) = data.split_at(NONCE_LEN);
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher.decrypt(nonce, ciphertext).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Decryption failed — wrong key or corrupted file",
        )
    })
}
