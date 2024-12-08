use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct OpenAIRequest<'a> {
    model: &'a str,
    messages: Vec<OpenAIMessage>,
    max_tokens: u32,
}

#[derive(Debug, Serialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessageContent,
}

#[derive(Debug, Deserialize)]
struct OpenAIMessageContent {
    content: String,
}

async fn open_ai(messages: Option<Vec<OpenAIMessage>>) -> Option<String> {
    let api_url = "https://api.openai.com/v1/chat/completions";
    let api_key = std::env::var("OPENAI_API_KEY").unwrap();

    // Default messages if none are provided
    let default_messages = vec![OpenAIMessage {
        role: "user".to_string(),
        content: "Hello, how can I use the OpenAI API?".to_string(),
    }];

    let request_body = OpenAIRequest {
        model: "gpt-4",
        messages: messages.unwrap_or(default_messages),
        max_tokens: 5000,
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();
    let response = client
        .post(api_url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await
        .unwrap();

    if !response.status().is_success() {
        println!("Error: {}", response.status());
        return None;
    }

    let data: OpenAIResponse = response.json().await.unwrap();
    let content = data.choices.get(0);

    match content {
        Some(content) => Some(content.message.content.clone()),
        None => None,
    }
}

pub async fn open_ai2(text: &str) -> Option<String> {
    #[derive(Debug, Clone)]
    struct Message {
        role: String,
        content: Vec<String>,
    }

    let mut messages: Vec<Message> = Vec::new();
    let mut current: Option<Message> = Some(Message {
        role: "system".to_string(),
        content: Vec::new(),
    });

    for line in text.trim().lines() {
        let line = line.trim();

        if line == "## SYSTEM" {
            messages.push(current.clone().unwrap());
            current = Some(Message {
                role: "system".to_string(),
                content: Vec::new(),
            });
        } else if line == "## USER" {
            messages.push(current.clone().unwrap());
            current = Some(Message {
                role: "user".to_string(),
                content: Vec::new(),
            });
        } else if let Some(ref mut msg) = current {
            msg.content.push(line.to_string());
        } else if line.is_empty() {
            // Ignore empty lines
        } else {
            eprintln!("Ignoring line: {}", line);
        }
    }

    let input_messages: Vec<OpenAIMessage> = messages
        .into_iter()
        .map(|m| OpenAIMessage {
            role: m.role,
            content: m.content.join("\n"),
        })
        .collect();

    let s = open_ai(Some(input_messages)).await;
    s
}
