use reqwest::Client;

pub struct DiscordBot {
    webhook_url: String,
    client: Client,
}

impl DiscordBot {
    pub fn new(webhook_url: String) -> Self {
        DiscordBot {
            webhook_url,
            client: Client::new(),
        }
    }

    pub async fn send_alert(&self, message: &str) -> Result<(), String> {
        let payload = serde_json::json!({
            "content": message,
        });

        match self.client.post(&self.webhook_url).json(&payload).send().await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
