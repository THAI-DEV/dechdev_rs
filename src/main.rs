use dechdev_rs::messager::discord;
use dechdev_rs::utils::bcrypt;
use dechdev_rs::utils::crypto_aes;
use dechdev_rs::utils::crypto_ase_gcm;
use dechdev_rs::utils::helper;
use dechdev_rs::utils::random;
use dechdev_rs::utils::string;

fn main() {
    helper::scroll_console();
    println!("Welcome to DechDev-RS!");

    // app_random();
    // app_string_case();
    // app_send_message();

    // app_crypto_aes();
    // app_ase_gcm()
    // app_bcrypt();
}

#[allow(dead_code)]
fn app_random() {
    let random_num = random::random_number(1, 100);
    let random_str = random::random_string_alpha_numeric(10);
    let random_str2 = random::random_string_alpha(10);
    let random_str3 = random::random_string_number(10);
    let random_str4 = random::random_string_numeric(10);

    let opts = random::RandomStringCustomOptions {
        // exclude_custom_chars: false,
        // custom_chars: "1l0Oo".to_string(),
        include_special_chars: true,
        // special_string: "()".to_string(),
        ..Default::default()
    };

    let random_str5 = random::random_string_custom(20, opts);

    println!("Random Number: {random_num}");
    println!("Random String: {random_str}");
    println!("Random String 2: {random_str2}");
    println!("Random String 3: {random_str3}");
    println!("Random String 4: {random_str4}");
    println!("Random String 5: {random_str5}");

    let spell = string::to_thai_pronunciation(&random_str5);
    println!("Spell String: {spell}");
}

#[allow(dead_code)]
fn app_string_case() {
    let original = "This is a test string";

    let s1 = string::to_pascal_case(original, true);
    let s2 = string::to_snake_case(original);
    let s3 = string::to_kebab_case(original);
    let s4 = string::to_camel_case(original, true);

    println!("Original String: {original}");
    println!("Pascal Case: {s1}");
    println!("Snake Case: {s2}");
    println!("Kebab Case: {s3}");
    println!("Camel Case: {s4}");
}

#[allow(dead_code)]
fn app_send_message() {
    let webhook_id = "1322089545617117204";
    let webhook_token = "NJnSR3V39bSphUkehwsB4ptc_he5y60OCV5pcKDrtlNzk7fyMmo9gqdjniBgQq0jm9pH";
    let discord = discord::DiscordMessager::new(webhook_id, webhook_token);

    // let json_message = r#"{"content": "Hello, Discord!"}"#;

    let json_message = r#"
    {
        "content": "สวัสดีครับ \r\nนี่คือข้อความทดสอบจาก DechDev-RS",
        "embeds": [
            {
                "title": "Embed Title",
                "description": "This is the description of the embed.",
                "color": 16711680,
                "footer": {
                    "text": "remark"
                },
                "timestamp": "2025-06-26T23:44:00.123+07:00"
            },
            {
                "title": "Second Embed",
                "description": "This is another embed description.",
                "color": 15258703,
                "footer": {
                    "text": "remark"
                },
                "thumbnail": {
                    "url": "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png"
                }
            }
        ]
    }
  "#;

    match discord.send_message(json_message) {
        Ok(result) => println!(
            "Is Success: {} \nStatus: {},\nMessage: {}",
            result.is_success, result.status, result.message
        ),
        Err(e) => println!("Failed to send message: {e}"),
    }
}

#[allow(dead_code)]
fn app_ase_gcm() {
    let plain_text = "This is a secret message";
    let secret_key = "password_123";

    let mut encrypted_text = "".to_string();

    println!("--- AES Encryption/Decryption ---");
    match crypto_ase_gcm::encrypt_string(plain_text, secret_key) {
        Ok(encrypted) => {
            println!("Encrypted: {encrypted}");
            println!("Encrypted Text: Success");
            encrypted_text = encrypted;
        }
        Err(e) => {
            println!("Encryption failed: {e}");
            println!("Encrypted Text: Failed");
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

#[allow(dead_code)]
fn app_crypto_aes() {
    let plain_text = "This is test program";
    let secret_key = "fds13rpowhls59wnpahmtps112456789"; // 32 characters for AES-256

    let mut encrypted_text = "".to_string();

    match crypto_aes::encrypt_string(plain_text, secret_key) {
        Ok(str1) => {
            println!("{str1}"); // Should output: YHPtkP6cNt5bZoi6vzxnWYn9Pq2CfjMi+siz/N6uYQQ=
            encrypted_text = str1;
        }
        Err(e) => {
            println!("Encryption failed: {e}");
        }
    }

    match crypto_aes::decrypt_string(&encrypted_text, secret_key) {
        Ok(str2) => {
            println!("{str2}"); // Should output: This is test program
        }
        Err(e) => {
            println!("Decryption failed: {e}");
        }
    }
}

#[allow(dead_code)]
fn app_bcrypt() {
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
