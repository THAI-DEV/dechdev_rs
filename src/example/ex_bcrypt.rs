use crate::utils::bcrypt;

pub fn example_bcrypt() {
    let plain_text = "This is a secret message";
    let mut hash_string = "".to_string();

    println!("\n--- Bcrypt Hashing ---");
    match bcrypt::bcrypt_hash_string(plain_text) {
        Ok(hashed) => {
            println!("Hashed: {hashed}");
            println!("Hashed Text: Success");
            hash_string = hashed;
        }
        Err(e) => {
            println!("Hashing failed: {e}");
            println!("Hashed Text: Failed");
        }
    };

    println!("\n--- Bcrypt Verification ---");
    match bcrypt::bcrypt_verify(plain_text, hash_string.as_str()) {
        Ok(is_valid) => {
            println!(
                "Verification: {}",
                if is_valid { "Valid" } else { "Invalid" }
            );
        }
        Err(e) => {
            println!("Verification failed: {e}");
        }
    };
}
