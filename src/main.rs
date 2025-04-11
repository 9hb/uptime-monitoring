#[tokio::main]
async fn main() {
    let config = Config::load();
    let monitor = Monitor::new();

    loop {
        for (url, webhook) in &config.monitorovane_stranky {
            match monitor.check_url(url).await {
                Ok(response_time) => {
                    println!("{} je ONLINE\n{}\n\n", url, format_response_time(response_time));
                }
                Err(err) => {
                    println!("{} je **OFFLINE**\n*doslo k chybe*: __{}__\n\n", url, err);

                    let discord_bot = DiscordBot::new(webhook.clone());
                    let _ = discord_bot.send_alert(
                        &format!("{} je **OFFLINE**\n*doslo k chybe*: __{}__\n\n", url, err)
                    ).await;
                }
            }
        }
        sleep(Duration::from_secs(30)).await; // interval mezi kontrolovanim stavu stranek
    }
}
