mod chat;
mod cli;
mod config;
mod gemini;

use anyhow::Result;

use chat::Chat;
use config::Config;
use gemini::GeminiClient;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;

    let client = GeminiClient::new(config)?;
    let chat = Chat::new(client);

    cli::run(chat).await?;

    Ok(())
}
