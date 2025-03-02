use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

// API constants
const OPENROUTER_API_URL: &str = "https://openrouter.ai/api/v1/chat/completions";
const SONNET_MODEL: &str = "anthropic/claude-3-sonnet-20240229";

/// Represents a message in the chat conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
        }
    }
}

/// API request format for OpenRouter
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

/// API response format from OpenRouter
#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

/// Client for interacting with Sonnet 3.7 via OpenRouter
pub struct SonnetClient {
    client: Client,
    api_key: String,
}

impl SonnetClient {
    /// Create a new Sonnet client
    pub fn new() -> Result<Self> {
        let api_key = env::var("OPENROUTER_API_KEY")
            .map_err(|_| anyhow!("OPENROUTER_API_KEY environment variable not set"))?;

        Ok(Self {
            client: Client::new(),
            api_key,
        })
    }

    /// Send a message to the Sonnet model and get a response
    pub async fn send_message(&self, messages: Vec<ChatMessage>) -> Result<String> {
        let request = ChatRequest {
            model: SONNET_MODEL.to_string(),
            messages,
        };

        let response = self
            .client
            .post(OPENROUTER_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("API request failed: {}", error_text));
        }

        let chat_response: ChatResponse = response.json().await?;
        
        if chat_response.choices.is_empty() {
            return Err(anyhow!("No response choices received from API"));
        }

        Ok(chat_response.choices[0].message.content.clone())
    }
}