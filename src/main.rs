use dechdev_rs::messager::discord;
use dechdev_rs::utils::helper;
use dechdev_rs::utils::random;
use dechdev_rs::utils::string;

fn main() {
    helper::scroll_console();

    // app_random();
    // app_string_case();

    // app_send_message();
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
        "content": "Here's an embedded message:",
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
