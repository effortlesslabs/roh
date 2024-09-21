use core::str;
use std::vec;

use reqwest::Client;
use serde::{ Deserialize, Serialize };
use crate::config::Config;

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OpenAIRequest {
    messages: Vec<ChatMessage>,
    model: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Deserialize)]
struct OpenAIChoice {
    message: ResponseMessage,
}

pub async fn ask_openai(config: &Config, prompt: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let request = OpenAIRequest {
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are a helpful assistant. response should be in markdown".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }
        ],
        model: "gpt-4o".to_string(),
    };

    let response = client
        .post(&config.base_uri)
        .bearer_auth(&config.api_key)
        .json(&request)
        .send().await?
        .json::<OpenAIResponse>().await?;

    Ok(response.choices[0].message.content.clone())
}
