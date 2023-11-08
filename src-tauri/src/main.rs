// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{command};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use dotenv::dotenv;

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct ChatPayload {
    messages: Vec<ChatMessage>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![chat])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[command]
async fn chat(payload: ChatPayload) -> Result<serde_json::Value, String> {
    let api_key = std::env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY");

    match send_chat_request(&api_key, payload).await {
        Ok(response) => Ok(response),
        Err(error) => Err(error.to_string()),
    }
}

async fn send_chat_request(api_key: &str, payload: ChatPayload) -> Result<serde_json::Value, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "messages": payload.messages,
            "model": "gpt-3.5-turbo-1106",
            "stream": false,
        }))
        .send()
        .await?;

    let response_json = response.json().await?;
    Ok(response_json)
}