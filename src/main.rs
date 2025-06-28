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

    let opts = random::RandomStringCustomOptions {
        // exclude_custom_chars: false,
        // custom_chars: "1l0Oo".to_string(),
        include_special_chars: true,
        // special_string: "()".to_string(),
        ..Default::default()
    };

    let random_str5 = random::random_string_custom(20, opts);

    let s1 = string::to_pascal_case(original, true);
    let s2 = string::to_snake_case(original);
    let s3 = string::to_kebab_case(original);
    let s4 = string::to_camel_case(original, true);

    helper::scroll_console();
    println!("Random Number: {random_num}");
    println!("Random String: {random_str}");
    println!("Random String 2: {random_str2}");
    println!("Random String 3: {random_str3}");
    println!("Random String 4: {random_str4}");
    println!("Random String 5: {random_str5}");

    let spell = string::to_thai_pronunciation(&random_str5);
    println!("Spell String: {spell}");

    println!("Original String: {original}");
    println!("Pascal Case: {s1}");
    println!("Snake Case: {s2}");
    println!("Kebab Case: {s3}");
    println!("Camel Case: {s4}");
}
