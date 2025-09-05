use crate::utils::{crypto_aes, crypto_ase_gcm};

pub fn example_crypto_ase_gcm() {
    let plain_text = "This is a secret message";
    let secret_key = "password_123";

    // let mut encrypted_text = "".to_string();

    println!("--- AES Encryption/Decryption ---");
    let encrypted_text = match crypto_ase_gcm::encrypt_string(plain_text, secret_key) {
        Ok(encrypted) => {
            println!("Encrypted: {encrypted}");
            println!("Encrypted Text: Success");
            encrypted
        }
        Err(e) => {
            println!("Encryption failed: {e}");
            println!("Encrypted Text: Failed");
            return;
        }
    };

    println!("\n--- AES Decryption ---");
    match crypto_ase_gcm::decrypt_string(encrypted_text.as_str(), secret_key) {
        Ok(decrypted) => {
            println!("Decrypted: {decrypted}");
            println!("Decrypted Text: Success");
        }
        Err(e) => {
            println!("Decryption failed: {e}");
            println!("Decrypted Text: Failed");
        }
    };
}

pub fn example_crypto_aes() {
    let plain_text = "This is test program";
    let secret_key = "fds13rpowhls59wnpahmtps112456789"; // 32 characters for AES-256

    let encrypted_text = match crypto_aes::encrypt_string(plain_text, secret_key) {
        Ok(str1) => {
            println!("{str1}"); // Should output: YHPtkP6cNt5bZoi6vzxnWYn9Pq2CfjMi+siz/N6uYQQ=
            str1
        }
        Err(e) => {
            println!("Encryption failed: {e}");
            return;
        }
    };

    match crypto_aes::decrypt_string(&encrypted_text, secret_key) {
        Ok(str2) => {
            println!("{str2}"); // Should output: This is test program
        }
        Err(e) => {
            println!("Decryption failed: {e}");
        }
    }
}
