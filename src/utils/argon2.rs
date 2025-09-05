use argon2::PasswordHash;
use argon2::PasswordVerifier;
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};

pub fn argon2_hash_string(
    plain_text: &str,
    salt: &str,
) -> Result<String, argon2::password_hash::Error> {
    let argon2 = Argon2::default();
    let salt = SaltString::encode_b64(salt.as_bytes()).unwrap();
    let hash = argon2.hash_password(plain_text.as_bytes(), &salt).unwrap();
    Ok(hash.to_string())
}

pub fn argon2_verify(
    plain_text: &str,
    hash_string: &str,
) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash_string)?;
    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(plain_text.as_bytes(), &parsed_hash)
        .is_ok())
}
