use aes::Aes256;
use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
use base64::{Engine as _, engine::general_purpose};

//   "Secret key must be 32 characters long for AES-256",
pub fn encrypt_string(plain_text: &str, secret_key: &str) -> Result<String, String> {
    if secret_key.len() != 32 {
        return Err("Secret key must be 32 characters long for AES-256".to_string());
    }

    let key = secret_key.as_bytes();
    let result = aes_encrypt(plain_text.as_bytes(), key).expect("Encryption failed");
    Ok(general_purpose::STANDARD.encode(result))
}

pub fn decrypt_string(encrypt_text: &str, secret_key: &str) -> Result<String, String> {
    if secret_key.len() != 32 {
        return Err("Secret key must be 32 characters long for AES-256".to_string());
    }

    let encrypted_data = general_purpose::STANDARD
        .decode(encrypt_text)
        .map_err(|e| format!("Base64 decode failed: {e}"))?;

    let key = secret_key.as_bytes();
    let orig_data = aes_decrypt(&encrypted_data, key)?;

    String::from_utf8(orig_data).map_err(|e| format!("UTF-8 conversion failed: {e}"))
}

fn aes_encrypt(orig_data: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    let iv = *b"1234567812345678"; // 16 bytes IV (same as Go version)

    let cipher = Aes256::new_from_slice(key).map_err(|e| format!("Invalid key length: {e}"))?;

    // Apply PKCS7 padding manually
    let block_size = 16;
    let padding_len = block_size - (orig_data.len() % block_size);
    let mut padded_data = orig_data.to_vec();
    for _ in 0..padding_len {
        padded_data.push(padding_len as u8);
    }

    let mut encrypted = Vec::new();
    let mut prev_block = iv;

    // Manual CBC mode encryption
    for chunk in padded_data.chunks(block_size) {
        let mut block = [0u8; 16];
        for i in 0..16 {
            block[i] = chunk[i] ^ prev_block[i];
        }

        let mut enc_block = aes::Block::from(block);
        cipher.encrypt_block(&mut enc_block);

        encrypted.extend_from_slice(&enc_block);
        prev_block = *enc_block.as_ref();
    }

    Ok(encrypted)
}

fn aes_decrypt(crypto: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    let iv = *b"1234567812345678"; // 16 bytes IV (same as Go version)

    let cipher = Aes256::new_from_slice(key).map_err(|e| format!("Invalid key length: {e}"))?;

    let block_size = 16;
    let mut decrypted = Vec::new();
    let mut prev_block = iv;

    // Manual CBC mode decryption
    for chunk in crypto.chunks(block_size) {
        let mut enc_block = *aes::Block::from_slice(chunk);
        cipher.decrypt_block(&mut enc_block);

        let mut block = [0u8; 16];
        for i in 0..16 {
            block[i] = enc_block[i] ^ prev_block[i];
        }

        decrypted.extend_from_slice(&block);
        prev_block.copy_from_slice(chunk);
    }

    // Remove PKCS7 padding
    let padding_len = decrypted[decrypted.len() - 1] as usize;
    decrypted.truncate(decrypted.len() - padding_len);

    Ok(decrypted)
}
