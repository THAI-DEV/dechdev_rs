use dechdev_rs::utils::helper;
use dechdev_rs::utils::random;
use dechdev_rs::utils::string;

fn main() {
    let original = "This is a test string";

    let random_num = random::random_number(1, 100);
    let random_str = random::random_string_alpha_numeric(10);
    let random_str2 = random::random_string_alpha(10);
    let random_str3 = random::random_string_number(10);
    let random_str4 = random::random_string_numeric(10);

    let s1 = string::to_pascal_case(original, true);
    let s2 = string::to_snake_case(original);
    let s3 = string::to_kebab_case(original);
    let s4 = string::to_camel_case(original, true);

    helper::scroll_console();
    println!("Random Number: {}", random_num);
    println!("Random String: {}", random_str);
    println!("Random String 2: {}", random_str2);
    println!("Random String 3: {}", random_str3);
    println!("Random String 4: {}", random_str4);

    println!("Original String: {}", original);
    println!("Pascal Case: {}", s1);
    println!("Snake Case: {}", s2);
    println!("Kebab Case: {}", s3);
    println!("Camel Case: {}", s4);
}
