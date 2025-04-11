use reqwest::Client;
use std::time::Instant;

pub struct Monitor {
    client: Client,
}

impl Monitor {
    pub fn new() -> Self {
        Monitor {
            client: Client::new(),
        }
    }

    pub async fn check_url(&self, url: &str) -> Result<u128, String> {
        let start = Instant::now();
        match self.client.get(url).send().await {
            Ok(response) => {
                if response.status().is_success() || response.status().as_u16() == 403 {
                    Ok(start.elapsed().as_millis())
                } else {
                    // kod, ktery neni 200 OK nebo 403 Forbidden (ty jsou v poradku)
                    Err(format!("chybny kod: {}", response.status()))
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }
}
