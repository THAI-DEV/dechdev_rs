use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit, OsRng, rand_core::RngCore},
};
use base64::{Engine as _, engine::general_purpose};

///
/// Encrypts a string using AES-256-GCM and returns the base64 encoded ciphertext.
///
/// # Arguments
/// * `plain_text` - The plaintext string to encrypt.
/// * `secret_key` - The secret key used for encryption (should be kept secret).
///
pub fn encrypt_string(plain_text: &str, secret_key: &str) -> Result<String, String> {
    // Create a key from the secret key (in a real app, use proper key derivation)
    let mut key_bytes = [0u8; 32];
    let secret_bytes = secret_key.as_bytes();
    let key_len = std::cmp::min(secret_bytes.len(), 32);
    key_bytes[..key_len].copy_from_slice(&secret_bytes[..key_len]);

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Generate a random nonce
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt the data
    let ciphertext = cipher
        .encrypt(nonce, plain_text.as_bytes())
        .map_err(|e| format!("Encryption failed: {e}"))?;

    // Combine nonce and ciphertext, then base64 encode
    let mut combined = nonce_bytes.to_vec();
    combined.extend_from_slice(&ciphertext);

    Ok(general_purpose::STANDARD.encode(combined))
}

///
/// Decrypts a base64 encoded ciphertext using AES-256-GCM.
///
/// * `encrypt_text` - The base64 encoded ciphertext to decrypt.    
/// * `secret_key` - The secret key used for decryption (should match the one used for encryption).
///     
pub fn decrypt_string(encrypt_text: &str, secret_key: &str) -> Result<String, String> {
    // Decode from base64
    let combined = general_purpose::STANDARD
        .decode(encrypt_text)
        .map_err(|_| "Base64 decode failed".to_string())?;

    if combined.len() < 12 {
        return Err("Invalid ciphertext length".to_string());
    }

    // Split nonce and ciphertext
    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    // Create key from secret key
    let mut key_bytes = [0u8; 32];
    let secret_bytes = secret_key.as_bytes();
    let key_len = std::cmp::min(secret_bytes.len(), 32);
    key_bytes[..key_len].copy_from_slice(&secret_bytes[..key_len]);

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Decrypt the data
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption failed: {e}"))?;

    String::from_utf8(plaintext).map_err(|_| "UTF-8 conversion failed".to_string())
}
