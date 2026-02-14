use rand::RngExt;

/// A configuration struct for customizing random string generation.
///
/// This struct allows fine-grained control over which character sets are included
/// in the generated random string, as well as options for excluding specific
/// characters and controlling leading zero behavior.
///
/// # Fields
///
/// * `include_lowercase` - Include lowercase letters (a-z)
/// * `include_uppercase` - Include uppercase letters (A-Z)
/// * `include_numeric` - Include numeric digits (0-9)
/// * `include_special_chars` - Include special characters from the `special_chars` field
/// * `special_chars` - String containing special characters to include when `include_special_chars` is true
/// * `exclude_custom_chars` - Whether to exclude characters specified in `custom_chars`
/// * `custom_chars` - String containing characters to exclude when `exclude_custom_chars` is true
/// * `is_leading_zero` - Whether to allow leading zeros in the generated string
///
/// # Default Values
///
/// By default, the struct includes lowercase, uppercase, and numeric characters,
/// excludes special characters, and allows leading zeros. The default special
/// characters include common symbols, and the default custom characters to exclude
/// are visually similar characters that can be difficult to distinguish.
///
pub struct RandomStringCustomOptions {
    pub include_lowercase: bool,
    pub include_uppercase: bool,
    pub include_numeric: bool,

    pub include_special_chars: bool,
    pub special_chars: String,

    pub exclude_custom_chars: bool,
    pub custom_chars: String,

    pub is_leading_zero: bool,
}

/// Default implementation for `RandomStringCustomOptions`.
///
/// Creates a new instance with the following default settings:
/// - **include_lowercase**: `true` - Includes lowercase letters (a-z)
/// - **include_uppercase**: `true` - Includes uppercase letters (A-Z)
/// - **include_numeric**: `true` - Includes numeric digits (0-9)
/// - **include_special_chars**: `false` - Excludes special characters by default
/// - **special_chars**: Contains a comprehensive set of special characters: `"!@#$%^&*()_+-={}[]:;\"'<>,.?/|~"`
/// - **exclude_custom_chars**: `false` - Does not exclude custom characters by default
/// - **custom_chars**: Contains characters that are visually similar or difficult to read: `"l10cCOopPsSuUvVxXwWzZ"`
/// - **is_leading_zero**: `true` - Allows leading zeros in generated strings
///
/// This configuration provides a balanced approach for generating random strings that are
/// both secure and readable, while avoiding potentially confusing character combinations.
impl Default for RandomStringCustomOptions {
    fn default() -> Self {
        Self {
            include_lowercase: true,
            include_uppercase: true,
            include_numeric: true,

            include_special_chars: false,
            special_chars: "!@#$%^&*()_+-={}[]:;\"'<>,.?/|~".to_string(),

            exclude_custom_chars: false,
            custom_chars: "l10cCOopPsSuUvVxXwWzZ".to_string(), // Characters that are same or difficult to read

            is_leading_zero: true,
        }
    }
}

/// Random number of integer between min and max
///
/// Example Result : 43
pub fn random_number(min: i32, max: i32) -> i32 {
    let mut rng = rand::rng();
    rng.random_range(min..=max)
}

/// Random number of integer as a string between min and max
///
/// Example Output : 43
pub fn random_number_string(min: i32, max: i32) -> String {
    random_number(min, max).to_string()
}

/// Generates a random string A-Z a-z 0-9
///
/// Example Output : bje3aMzzce1
pub fn random_string_alpha_numeric(length: u32) -> String {
    //A-Z a-z 0-9
    use rand::distr::Alphanumeric;
    let mut rng = rand::rng();
    (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
}

/// Generates a random string 0-9
///
/// Example Output : 0629133926
pub fn random_string_numeric(length: u32) -> String {
    use rand::distr::Uniform;
    let mut rng = rand::rng();
    let chars: Vec<char> = "0123456789".chars().collect();
    let uniform = Uniform::new(0, chars.len()).unwrap();
    (0..length).map(|_| chars[rng.sample(uniform)]).collect()
}

/// Generates a random string 0-9, first character is not zero
///
/// Example Output : 8629133926
pub fn random_string_number(length: u32) -> String {
    use rand::distr::Uniform;
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
/// Example Output : bjejaMzzce
pub fn random_string_alpha(length: u32) -> String {
    //A-Z a-z
    use rand::distr::Uniform;
    let mut rng = rand::rng();
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        .chars()
        .collect();
    let uniform = Uniform::new(0, chars.len()).unwrap();
    (0..length).map(|_| chars[rng.sample(uniform)]).collect()
}

/// Random string with custom options
///
/// Example Code :
/// ```rust
/// let opts = random::RandomStringCustomOptions {
///         exclude_custom_chars: true,
///         custom_chars: "1l0Oo".to_string(),
///
///         include_special_string: true,
///         special_string: "@#()".to_string(),
///
///         ..Default::default()
/// };
///
/// let random_string = random::random_string_custom(20, opts);
pub fn random_string_custom(length: u32, opts: RandomStringCustomOptions) -> String {
    use rand::distr::Uniform;

    let mut rng = rand::rng();

    if length == 0 {
        return String::new();
    }

    let mut chars = String::new();
    if opts.include_lowercase {
        chars.push_str("abcdefghijklmnopqrstuvwxyz");
    }
    if opts.include_uppercase {
        chars.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if opts.include_numeric {
        chars.push_str("0123456789");
    }

    if opts.exclude_custom_chars {
        // Exclude difficult-to-read characters
        chars = chars
            .chars()
            .filter(|c| !opts.custom_chars.contains(*c))
            .collect();
    }

    if !opts.special_chars.is_empty() && opts.include_special_chars {
        chars.push_str(&opts.special_chars);
    }

    if chars.is_empty() {
        return String::new(); // Return empty string if no character set is selected
    }

    let char_vec: Vec<char> = chars.chars().collect();
    let uniform = Uniform::new(0, char_vec.len()).unwrap();

    let mut result = String::new();

    // Handle first character based on is_zero_first option
    if opts.include_numeric && !opts.is_leading_zero {
        // First character cannot be zero
        let non_zero_chars: Vec<char> = char_vec.iter().filter(|&&c| c != '0').copied().collect();
        if !non_zero_chars.is_empty() {
            let non_zero_uniform = Uniform::new(0, non_zero_chars.len()).unwrap();
            result.push(non_zero_chars[rng.sample(non_zero_uniform)]);
        } else {
            result.push(char_vec[rng.sample(uniform)]);
        }

        // Remaining characters
        for _ in 1..length {
            result.push(char_vec[rng.sample(uniform)]);
        }
    } else {
        // All characters can be any from the set
        for _ in 0..length {
            result.push(char_vec[rng.sample(uniform)]);
        }
    }

    result
}
