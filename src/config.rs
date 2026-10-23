use anyhow::{Context, Result};
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub api_key: String,
    pub model: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        dotenvy::dotenv().ok();

        let api_key = env::var("GEMINI_API_KEY")
            .context("GEMINI_API_KEY is missing. Add it to your .env file.")?;

        if api_key.trim().is_empty() {
            anyhow::bail!("GEMINI_API_KEY cannot be empty.");
        }

        let model = env::var("GEMINI_MODEL").unwrap_or_else(|_| "gemini-3.8-flash".to_string());

        if model.trim().is_empty() {
            anyhow::bail!("GEMINI_MODEL cannot be empty.");
        }

        Ok(Self { api_key, model })
    }
}
