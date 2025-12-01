use base64::{Engine as _, engine::general_purpose};

pub fn decode_base64(encoded: &str) -> String {
    let padded_encoded = if !encoded.len().is_multiple_of(4) {
        let padding = 4 - (encoded.len() % 4);
        format!("{}{}", encoded, "=".repeat(padding))
    } else {
        encoded.to_string()
    };

    let decoded_bytes = general_purpose::STANDARD
        .decode(padded_encoded)
        .expect("Failed to decode base64");

    String::from_utf8(decoded_bytes).expect("Failed to convert to string")
}

pub fn encode_base64(input: &str) -> String {
    general_purpose::STANDARD.encode(input)
}
