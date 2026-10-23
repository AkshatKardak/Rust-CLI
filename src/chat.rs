use anyhow::Result;

use crate::gemini::{Content, GeminiClient};

pub struct Chat {
    client: GeminiClient,
    history: Vec<Content>,
}

impl Chat {
    pub fn new(client: GeminiClient) -> Self {
        Self {
            client,
            history: Vec::new(),
        }
    }

    pub async fn send_message(&mut self, message: &str) -> Result<()> {
        let message = message.trim();

        if message.is_empty() {
            return Ok(());
        }

        // Include earlier turns without modifying stored history
        // until the API call succeeds.
        let mut contents = self.history.clone();
        contents.push(Content::user(message));

        let answer = self.client.stream_response(&contents).await?;

        // Store only successful exchanges.
        self.history.push(Content::user(message));
        self.history.push(Content::model(answer));

        Ok(())
    }

    pub fn clear(&mut self) {
        self.history.clear();
    }

    pub fn history(&self) -> &[Content] {
        &self.history
    }

    pub fn model(&self) -> &str {
        self.client.model()
    }

    pub fn set_model(&mut self, model: String) -> Result<()> {
        self.client.set_model(model)
    }
}
