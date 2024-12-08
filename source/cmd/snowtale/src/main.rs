use std::{
    collections::HashMap,
    io::{self, Write},
};

#[derive(Debug, Default)]
struct ActionLog {
    entries: Vec<String>,
}

struct Player {
    position: (i32, i32),
    inventory: (),
}

impl Player {
    fn new() -> Player {
        Player {
            position: (0, 0),
            inventory: (),
        }
    }
}

#[derive(Debug, Default)]
struct Room {
    description: String,
}

#[derive(Default)]
struct World {
    rooms: HashMap<(i32, i32), Room>,
}

#[tokio::main]
async fn main() {
    let mut actions = ActionLog::default();
    let mut world = World::default();
    let mut player = Player::new();

    loop {
        let room = get_room(player.position, &mut world, &actions).await;
        cpara(room.as_str());

        // Process the input
        let input = prompt().await;
        actions.entries.push(input.clone());
        println!("You entered: {}", input.trim());
        println!();

        match input.trim() {
            "n" | "north" => player.position.1 += 1,
            "s" | "south" => player.position.1 -= 1,
            "e" | "east" => player.position.0 += 1,
            "w" | "west" => player.position.0 -= 1,
            "exit" | "quit" => break,
            _ => {}
        }
    }
}

fn cpara(text: &str) {
    // Split the text into lines of at most 80 characters, splitting at word boundaries.
    let regex = regex::Regex::new(r".{1,78}(?:\s|$)").unwrap();
    let lines = regex
        .find_iter(text)
        .map(|m| m.as_str().trim()) // Trim any trailing whitespace
        .collect::<Vec<_>>();

    for line in lines {
        println!("  {}", line);
    }
    println!();
}

async fn prompt() -> String {
    print!("> ");
    io::stdout().flush().unwrap(); // Flush to show the prompt immediately

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return "".into();
    }
    input
}

async fn get_room(
    position: (i32, i32),
    world: &mut World, //
    actions: &ActionLog,
) -> String {
    if world.rooms.contains_key(&position) {
        return world.rooms[&position].description.clone();
    }

    let recent_actions = actions.entries.join("\n");

    let p = format!(
        r#"

## SYSTEM

You are the mind behind a text adventure game and need to describe scenes and
rooms that the player enters.  Describe them in the fashion of a text adventure
like King's Quest or Zork or Daggerfall.  Be specific but also concise.

Please provide single, specific answers in plain English.  Do not provide multiple
options.  Describe the room itself in an objective way.  Do not explicitly include what
the player or group recently did.  In general, use high-fantasy as the setting such as 
elves, dwarves, dragons, etc. Use imaginary from Tolkien, Jack Vance, and generic 
D&D-esque settings.

Ensure answers are no more than 4 sentences long.

## SYSTEM

Please remember that these are the recent actions of the player:

{recent_actions}

## USER

Describe the current room or scene.
    "#
    );

    let d = open_ai2(p.as_str()).await.unwrap_or_default();
    if !d.is_empty() {
        world.rooms.insert(
            position,
            Room {
                description: d.clone(),
            },
        );
    }
    d
}

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

async fn open_ai2(text: &str) -> Option<String> {
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
