use crate::utils::random;
use crate::utils::string;

// #[allow(dead_code)]
pub fn example_random() {
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
