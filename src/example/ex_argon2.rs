use crate::utils::argon2;

pub fn example_argon2() {
    let plain_text = "This is a secret message";
    let mut hash_string = "".to_string();

    println!("\n--- Argon2 Hashing ---");
    match argon2::argon2_hash_string(plain_text, "random_salt") {
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

    println!("\n--- Argon2 Verification ---");

    match argon2::argon2_verify(plain_text, hash_string.as_str()) {
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
