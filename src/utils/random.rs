/// Random number of integer between min and max
//
/// Ex : 43
pub fn random_number(min: i32, max: i32) -> i32 {
    use rand::Rng;
    let mut rng = rand::rng();
    rng.random_range(min..=max)
}

/// Generates a random string A-Z a-z 0-9
///
/// Ex : bje3aMzzce1
pub fn random_string_alpha_numeric(length: u32) -> String {
    //A-Z a-z 0-9
    use rand::{Rng, distr::Alphanumeric};
    let mut rng = rand::rng();
    (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
}

/// Generates a random string 0-9
///
/// Ex : 0629133926
pub fn random_string_numeric(length: u32) -> String {
    use rand::{Rng, distr::Uniform};
    let mut rng = rand::rng();
    let chars: Vec<char> = "0123456789".chars().collect();
    let uniform = Uniform::new(0, chars.len()).unwrap();
    (0..length).map(|_| chars[rng.sample(uniform)]).collect()
}

/// Generates a random string 0-9, first character is not zero
///
/// Ex : 8629133926
pub fn random_string_number(length: u32) -> String {
    use rand::{Rng, distr::Uniform};
    let mut rng = rand::rng();

    if length == 0 {
        return String::new();
    }

    let mut result = String::new();

    // First character: 1-9
    let first_chars: Vec<char> = "123456789".chars().collect();
    result.push(first_chars[rng.sample(Uniform::new(0, first_chars.len()).unwrap())]);

    // Remaining characters: 0-9
    let all_chars: Vec<char> = "0123456789".chars().collect();
    for _ in 1..length {
        result.push(all_chars[rng.sample(Uniform::new(0, all_chars.len()).unwrap())]);
    }

    result
}

/// Generates a random string A-Z a-z
///
/// Ex : bjejaMzzce

pub fn random_string_alpha(length: u32) -> String {
    //A-Z a-z
    use rand::{Rng, distr::Uniform};
    let mut rng = rand::rng();
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        .chars()
        .collect();
    let uniform = Uniform::new(0, chars.len()).unwrap();
    (0..length).map(|_| chars[rng.sample(uniform)]).collect()
}
