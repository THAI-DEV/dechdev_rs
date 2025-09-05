use crate::messager::discord;

pub fn example_send_message() {
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
pub async fn example_send_message_async() {
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

    match discord.send_message_async(json_message).await {
        Ok(result) => println!(
            "Is Success: {} \nStatus: {},\nMessage: {}",
            result.is_success, result.status, result.message
        ),
        Err(e) => println!("Failed to send message: {e}"),
    }
}
