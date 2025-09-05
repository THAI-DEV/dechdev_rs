use argon2::PasswordHash;
use argon2::PasswordVerifier;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};

pub fn argon2_hash_string(plain_text: &str) -> Result<String, argon2::password_hash::Error> {
    let algorithm = argon2::Algorithm::Argon2i;
    let version = argon2::Version::V0x13;
    let params = argon2::Params::default();
    let argon2 = Argon2::new(algorithm, version, params);
    let mut rng = OsRng;
    let salt = SaltString::generate(&mut rng);
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
