use anyhow::{Context, Result};
use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::io::{self, Write};

use crate::config::Config;

const API_BASE: &str = "https://generativelanguage.googleapis.com/v1beta/models";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Part {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub role: String,
    pub parts: Vec<Part>,
}

impl Content {
    pub fn user(text: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            parts: vec![Part { text: text.into() }],
        }
    }

    pub fn model(text: impl Into<String>) -> Self {
        Self {
            role: "model".to_string(),
            parts: vec![Part { text: text.into() }],
        }
    }

    pub fn text(&self) -> String {
        self.parts
            .iter()
            .map(|part| part.text.as_str())
            .collect::<Vec<_>>()
            .join("")
    }
}

pub struct GeminiClient {
    client: Client,
    api_key: String,
    model: String,
}

impl GeminiClient {
    pub fn new(config: Config) -> Result<Self> {
        let client = Client::builder()
            .user_agent("rust-cli-project/0.1.0")
            .build()
            .context("Failed to initialize HTTP client")?;

        Ok(Self {
            client,
            api_key: config.api_key,
            model: config.model,
        })
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn set_model(&mut self, model: String) -> Result<()> {
        let model = model.trim().to_string();

        if model.is_empty() || model.contains('/') || model.contains('?') || model.contains('#') {
            anyhow::bail!("Invalid model name.");
        }

        self.model = model;
        Ok(())
    }

    pub async fn stream_response(&self, contents: &[Content]) -> Result<String> {
        let url = format!("{}/{}:streamGenerateContent?alt=sse", API_BASE, self.model);

        let request_body = json!({
            "contents": contents,
            "generationConfig": {
                "temperature": 0.4,
                "maxOutputTokens": 8192
            }
        });

        let response = self
            .client
            .post(&url)
            .header("x-goog-api-key", &self.api_key)
            .header("Accept", "text/event-stream")
            .json(&request_body)
            .send()
            .await
            .context("Could not connect to Gemini. Check your internet connection.")?;

        let status = response.status();

        if !status.is_success() {
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "No error details returned.".to_string());

            anyhow::bail!("Gemini API returned HTTP {}: {}", status, error_body);
        }

        let mut stream = response.bytes_stream();
        let mut buffer = String::new();
        let mut full_response = String::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.context("Error reading Gemini response stream")?;

            buffer.push_str(&String::from_utf8_lossy(&chunk));

            while let Some(newline) = buffer.find('\n') {
                let line = buffer[..newline].trim().to_string();
                buffer.drain(..=newline);

                if let Some(text) = parse_sse_line(&line)? {
                    print!("{text}");
                    io::stdout().flush()?;
                    full_response.push_str(&text);
                }
            }
        }

        // Process any final event that does not end with a newline.
        if !buffer.trim().is_empty() {
            if let Some(text) = parse_sse_line(buffer.trim())? {
                print!("{text}");
                io::stdout().flush()?;
                full_response.push_str(&text);
            }
        }

        if full_response.trim().is_empty() {
            anyhow::bail!(
                "Gemini returned no text. Check the selected model, \
                 API availability, and response safety status."
            );
        }

        Ok(full_response)
    }
}

fn parse_sse_line(line: &str) -> Result<Option<String>> {
    let Some(data) = line.strip_prefix("data:") else {
        // Ignore blank lines and SSE metadata such as event:.
        return Ok(None);
    };

    let data = data.trim();

    if data.is_empty() || data == "[DONE]" {
        return Ok(None);
    }

    let value: Value = serde_json::from_str(data).context("Invalid JSON in Gemini stream")?;

    // Gemini response:
    // candidates[0].content.parts[*].text
    let Some(candidates) = value.get("candidates").and_then(Value::as_array) else {
        // Some stream events may contain metadata instead of text.
        return Ok(None);
    };

    let mut text = String::new();

    for candidate in candidates {
        let Some(parts) = candidate
            .get("content")
            .and_then(|content| content.get("parts"))
            .and_then(Value::as_array)
        else {
            continue;
        };

        for part in parts {
            if let Some(part_text) = part.get("text").and_then(Value::as_str) {
                text.push_str(part_text);
            }
        }
    }

    if text.is_empty() {
        Ok(None)
    } else {
        Ok(Some(text))
    }
}
