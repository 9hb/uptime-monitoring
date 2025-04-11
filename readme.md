# Uptime Monitor

A simple, efficient web service monitoring tool written in Rust that sends alerts to Discord when services go down.

## Features

- **Continuous Monitoring:** Checks website availability every 30 seconds
- **Response Time Tracking:** Measures and logs response times for all successful checks
- **Discord Notifications:** Sends alerts to Discord via webhooks when services become unavailable
- **Flexible Error Handling:** Considers HTTP 403 responses as acceptable (service still running)
- **Easy Configuration:** Simple configuration of multiple websites with separate Discord webhook URLs

## How It Works

The application loops through configured websites, checking their status periodically. For each website:

- When online: Logs status and response time to console
- When offline: Sends a Discord alert with details about the failure and logs to console

## Configuration

The application uses a simple configuration system defined in `config.rs`. Each monitored website is configured with:

- **Target URL**: The complete URL of the website to monitor (e.g., "http://example.com")
- **Discord Webhook URL**: Where alerts are sent when the site goes down

### How to Configure Websites

The configuration is currently defined in the `Config::load()` method. To add or modify monitored websites:

1. Open `src/config.rs`
2. Modify the `monitorovane_stranky` vector in the `load()` method
3. Each entry follows this format:
   ```rust
   (
       "http://your-website-to-monitor.com".to_string(),
       "https://discord.com/api/webhooks/your-webhook-id/your-webhook-token".to_string(),
   ),
   ```

### Discord Webhook Setup

1. In your Discord server, go to a channel's settings
2. Select "Integrations" → "Webhooks" → "New Webhook"
3. Copy the webhook URL and use it in your configuration

You can use different webhook URLs for different websites, or the same webhook for all sites.

## Usage

Simply build and run the application:

```
# build
cargo build --release

# run
cargo run
```

The monitor will start automatically and run continuously, checking all configured websites.
