use regex::Regex;

/// Converts a string to PascalCase.
///
/// Ex : This is a test string --> ThisIsATestString
pub fn to_pascal_case(input: &str, is_trim_spaces: bool) -> String {
    let re = Regex::new(r"\b[a-z]").unwrap();

    let result = re.replace_all(input, |caps: &regex::Captures| {
        caps.get(0).unwrap().as_str().to_uppercase()
    });

    if is_trim_spaces {
        result.replace(" ", "")
    } else {
        result.to_string()
    }
}

/// Converts a string to snake_case.
///
/// Ex : This is a test string --> this_is_a_test_string
pub fn to_snake_case(input: &str) -> String {
    let re = Regex::new(r"(?P<first>[A-Z])(?P<rest>[a-z]+)").unwrap();
    re.replace_all(input, |caps: &regex::Captures| {
        format!("{}{}", caps["first"].to_lowercase(), &caps["rest"])
    })
    .replace(" ", "_")
}

/// Converts a string to kebab-case.
///
/// Ex : This is a test string --> this-is-a-test-string
pub fn to_kebab_case(input: &str) -> String {
    let re = Regex::new(r"(?P<first>[A-Z])(?P<rest>[a-z]+)").unwrap();
    re.replace_all(input, |caps: &regex::Captures| {
        format!("{}{}", caps["first"].to_lowercase(), &caps["rest"])
    })
    .replace(" ", "-")
}

/// Converts a string to camelCase.
///
/// Ex : This is a test string --> thisIsATestString
pub fn to_camel_case(input: &str, is_trim_space: bool) -> String {
    let words: Vec<&str> = input.split_whitespace().collect();
    let mut result = String::new();

    for (i, word) in words.iter().enumerate() {
        if i == 0 {
            result.push_str(&word.to_lowercase());
        } else {
            let mut chars = word.chars();
            if let Some(first) = chars.next() {
                result.push(first.to_uppercase().next().unwrap_or(first));
                result.push_str(&chars.as_str().to_lowercase());
            }
        }

        if !is_trim_space && i < words.len() - 1 {
            result.push(' ');
        }
    }

    result
}
