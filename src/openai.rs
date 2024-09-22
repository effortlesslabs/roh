use core::str;
use std::vec;
use futures_util::StreamExt;
use reqwest::Client;
use serde::{ Deserialize, Serialize };
use crate::config::Config;
use crate::markdown::print_markdown;

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OpenAIRequest {
    messages: Vec<ChatMessage>,
    model: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct ChatChunkDelta {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChatChunkChoice {
    delta: ChatChunkDelta,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionChunk {
    choices: Vec<ChatChunkChoice>,
}

#[derive(Debug, Deserialize)]
struct APIError {
    message: String,
}

#[derive(Debug, Deserialize)]
struct APIErrorResponse {
    error: APIError,
}

pub async fn ask_openai(config: &Config, prompt: &str) -> Result<(), Box<dyn std::error::Error>> {
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
        stream: true,
    };

    let response = client
        .post(&config.base_uri)
        .bearer_auth(&config.api_key)
        .json(&request)
        .send().await?;

    let mut stream = response.bytes_stream();

    let streaming_completion_marker = "[DONE]";
    let mut previous_chunk_buffer = "".to_owned();

    let mut buffer = String::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;

        let chunk_string = match std::str::from_utf8(&chunk) {
            std::result::Result::Ok(value) => value,
            Err(_) => {
                eprintln!("Error converting chunk to string");
                break;
            }
        };
        let chunk_string = previous_chunk_buffer + chunk_string;
        previous_chunk_buffer = "".to_owned();

        let split_data = chunk_string.trim().split("data:");
        for (index, data_chunk) in split_data.to_owned().enumerate() {
            let data_chunk = data_chunk.trim();
            if data_chunk.is_empty() {
                continue;
            }
            if data_chunk == streaming_completion_marker {
                return Ok(());
            }

            let data_value = serde_json::from_str::<ChatCompletionChunk>(data_chunk);
            let data_value = match data_value {
                Ok(value) => value,
                Err(_) => {
                    match serde_json::from_str::<APIErrorResponse>(data_chunk) {
                        Ok(value) => {
                            eprintln!("Error from API: {}", value.error.message);
                            break;
                        }
                        Err(_) => {
                            if index == split_data.to_owned().count() - 1 {
                                previous_chunk_buffer = "data: ".to_owned() + &data_chunk;
                                break;
                            } else {
                                eprintln!("Error parsing data chunk");
                                break;
                            }
                        }
                    }
                }
            };
            let choice = data_value.choices.first().expect("No choices available");

            if let Some(content) = &choice.delta.content {
                buffer.push_str(content);
                while let Some(newline_pos) = buffer.find("\n\n") {
                    let line = buffer[..newline_pos].trim_end();
                    print_markdown(line);
                    buffer.drain(..=newline_pos);
                }
            }
        }
    }
    Ok(())
}
