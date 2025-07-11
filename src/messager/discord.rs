use core::str;

static WEBHOOK_URL: &str = "https://discordapp.com/api/webhooks";

pub struct ResultDiscordMessage {
    pub is_success: bool,
    pub status: String,
    pub message: String,
}

pub struct DiscordMessager {
    webhook_url: String,
    pub webhook_id: String,
    pub webhook_token: String,
}

impl DiscordMessager {
    pub fn new(webhook_id: &str, webhook_token: &str) -> Self {
        Self {
            webhook_id: webhook_id.to_string(),
            webhook_token: webhook_token.to_string(),
            webhook_url: format!("{WEBHOOK_URL}/{webhook_id}/{webhook_token}"),
        }
    }

    pub fn send_message(&self, json_message: &str) -> Result<ResultDiscordMessage, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(&self.webhook_url)
            .header("Content-Type", "application/json")
            .body(json_message.to_string())
            .send();

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    let result = ResultDiscordMessage {
                        is_success: true,
                        status: resp.status().to_string(),
                        message: resp.text().unwrap_or_default(),
                    };

                    Ok(result)
                } else {
                    let result = ResultDiscordMessage {
                        is_success: false,
                        status: resp.status().to_string(),
                        message: convert_json_string_to_string(resp.text().unwrap_or_default()),
                    };
                    Ok(result)
                }
            }
            Err(e) => Err(e),
        }
    }

    pub async fn send_message_async(
        &self,
        json_message: &str,
    ) -> Result<ResultDiscordMessage, reqwest::Error> {
        let client = reqwest::Client::new();
        let response = client
            .post(&self.webhook_url)
            .header("Content-Type", "application/json")
            .body(json_message.to_string())
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    let result = ResultDiscordMessage {
                        is_success: true,
                        status: resp.status().to_string(),
                        message: resp.text().await.unwrap_or_default(),
                    };

                    Ok(result)
                } else {
                    let result = ResultDiscordMessage {
                        is_success: false,
                        status: resp.status().to_string(),
                        message: convert_json_string_to_string(
                            resp.text().await.unwrap_or_default(),
                        ),
                    };
                    Ok(result)
                }
            }
            Err(e) => Err(e),
        }
    }
}

fn convert_json_string_to_string(s: String) -> String {
    if s.is_empty() {
        return String::new();
    }

    let chars_to_remove = ['{', '}', '"', '\\', '[', ']'];
    s.chars().filter(|c| !chars_to_remove.contains(c)).collect()
}
