use crate::utils::string;

pub fn example_string_case() {
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
