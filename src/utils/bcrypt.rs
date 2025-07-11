use bcrypt::{DEFAULT_COST, hash, verify};

///
/// Generates a bcrypt hash of a plain text string.
/// # Arguments
/// * `plain_text` - The plaintext string to hash.
///
pub fn bcrypt_hash_string(plain_text: &str) -> Result<String, bcrypt::BcryptError> {
    hash(plain_text, DEFAULT_COST)
}

///
/// Verifies a plain text string against a bcrypt hash.
/// # Arguments
/// * `plain_text` - The plaintext string to verify.
///     
pub fn bcrypt_verify(plain_text: &str, hash_string: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(plain_text, hash_string)
}
