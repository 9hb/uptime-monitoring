pub struct Config {
    pub monitorovane_stranky: Vec<(String, String)>, // (url, webhook)
}

impl Config {
    pub fn load() -> Self {
        Config {
            monitorovane_stranky: vec![
                ("".to_string(), "https://discord.com/api/webhooks/".to_string()),
                ("".to_string(), "https://discord.com/api/webhooks/".to_string()),
                ("".to_string(), "https://discord.com/api/webhooks/".to_string()),
                ("".to_string(), "https://discord.com/api/webhooks/".to_string())
            ],
        }
    }
}
