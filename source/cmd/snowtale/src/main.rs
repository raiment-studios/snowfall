mod openai;
mod ui;
use std::collections::HashMap;

use colored::Colorize;
use openai::*;
use serde::{Deserialize, Serialize};
use ui::*;

#[derive(Debug, Default)]
struct ActionLog {
    entries: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Room {
    description: String,
}

#[derive(Default)]
struct World {
    rooms: HashMap<(i32, i32), Room>,
}

#[tokio::main]
async fn main() {
    println!();
    println!("{}", "Welcome to snowtale!".truecolor(64, 192, 220));
    println!("{}", "~".repeat(80).truecolor(128, 128, 160));
    println!();

    let mut actions = ActionLog::default();
    let mut world = World::default();
    let mut player = read_player();

    loop {
        println!("POS: {},{}", player.position.0, player.position.1);
        println!();

        let room = ensure_room(player.position, &mut world, &actions).await;
        print_paragraph("eee", room.description.as_str());

        // Process the input
        let input = prompt().await;
        let input = input.trim().to_lowercase();
        let input = regex::Regex::new(r"\s+")
            .unwrap()
            .replace_all(&input, " ")
            .to_string();
        let action = {
            match input.as_str() {
                "n" | "north" | "move n" => "move north",
                "s" | "south" | "move s" => "move south",
                "e" | "east" | "move e" => "move east",
                "w" | "west" | "move w" => "move west",
                "q" => "quit",
                s => s,
            }
            .to_string()
        };

        println!("You entered: {}", input);
        println!();

        let mut handled = true;
        match action.trim() {
            "move north" => player.position.1 += 1,
            "move south" => player.position.1 -= 1,
            "move east" => player.position.0 += 1,
            "move west" => player.position.0 -= 1,
            "regen" => {
                create_room(player.position, &mut world, &actions).await;
            }
            "quit" => break,
            _ => {
                println!("I don't understand that command.");
                handled = false;
            }
        }
        if handled {
            actions.entries.push(action.clone());
        }
        write_player(&player);
    }
}

fn read_player() -> Player {
    let contents = std::fs::read_to_string("store/_default/player.yaml").ok();
    match contents {
        Some(contents) => serde_yaml::from_str(&contents).unwrap(),
        None => Player::new(),
    }
}

fn write_player(player: &Player) {
    let contents = serde_yaml::to_string(player).unwrap();
    std::fs::write("store/_default/player.yaml", contents).unwrap();
}

fn get_room_filename(position: (i32, i32)) -> String {
    let dirname = "store/_default/rooms";
    let filename = format!(
        "room-{}{:03}-{}{:03}.yaml",
        if position.0 >= 0 { "e" } else { "w" },
        position.0.abs(),
        if position.1 >= 0 { "n" } else { "s" },
        position.1.abs(),
    );
    format!("{}/{}", dirname, filename)
}

fn read_room(position: (i32, i32)) -> Option<Room> {
    let fullpath = get_room_filename(position);
    let contents = std::fs::read_to_string(fullpath).ok()?;
    let room: Room = serde_yaml::from_str(&contents).ok()?;
    Some(room)
}

fn write_room(position: (i32, i32), room: &Room) {
    let fullpath = get_room_filename(position);

    let dirname = std::path::Path::new(&fullpath).parent().unwrap();
    std::fs::create_dir_all(dirname).unwrap();

    let contents = serde_yaml::to_string(room).unwrap();
    std::fs::write(fullpath, contents).unwrap();
}

fn peek_room(position: (i32, i32), world: &mut World) -> Option<Room> {
    if world.rooms.contains_key(&position) {
        Some(world.rooms[&position].clone())
    } else if let Some(room) = read_room(position) {
        world.rooms.insert(position, room.clone());
        Some(room)
    } else {
        None
    }
}

async fn ensure_room(
    position: (i32, i32),
    world: &mut World, //
    actions: &ActionLog,
) -> Room {
    if let Some(room) = peek_room(position, world) {
        return room;
    }
    create_room(position, world, actions).await
}

async fn create_room(
    position: (i32, i32), //
    world: &mut World,
    actions: &ActionLog,
) -> Room {
    let sys_nearby = {
        let mut nearby = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let neighbor = (position.0 + dx, position.1 + dy);
                if let Some(room) = peek_room(neighbor, world) {
                    nearby.push(format!(
                        "To the {} is a room with this description:\n {}\n\n",
                        if dx > 0 {
                            "east"
                        } else if dx < 0 {
                            "west"
                        } else if dy > 0 {
                            "north"
                        } else if dy < 0 {
                            "south"
                        } else {
                            "???"
                        },
                        room.description.trim()
                    ));
                }
            }
        }
        if nearby.is_empty() {
            "".to_string()
        } else {
            format!(
                r#"
When describing the new room, please make sure it follows naturally
and logically from the nearby rooms.  Here are the descriptions of 
the nearby rooms:

{}

Please include a small, subtle reference to one of the neighboring rooms
when describing this new room.
"#,
                nearby.join("\n")
            )
        }
    };

    let sys_recent_actions = {
        if actions.entries.is_empty() {
            "".to_string()
        } else {
            format!(
                r#"
Please remember that these are the recent actions of the player:
- {}
"#,
                actions.entries.join("\n- ")
            )
        }
    };

    let p = format!(
        r#"

## SYSTEM

You are the mind behind a text adventure game and need to describe scenes and
rooms that the player enters.  Describe them in the fashion of a text adventure
like King's Quest or Zork or Daggerfall.  Be specific but also concise.
This is a world of low-fantasy with magical elements and mythical creatures, but
they are rare and the majority of the story is based on humans and their struggles.

Please provide single, specific answers in plain English. Do not provide multiple
options.  Describe the room itself objectively and without any preface.  
Do not reference the player or group or any of their recent actions.  Use a writing 
style that reminds the reader of some combination of Jack Vance and JRR Tolkien.  
Use third-person and describe only the room and surroundings.

Ensure answers are no more than 4 sentences long.

## SYSTEM

{sys_nearby}

{sys_recent_actions}

## USER

Describe the current room or scene.
    "#
    );

    let d = open_ai2(p.as_str()).await.unwrap_or_default();
    if !d.is_empty() {
        let room = Room {
            description: d.clone(),
        };
        world.rooms.insert(position, room.clone());
        write_room(position, &room);
        return room;
    } else {
        // TODO: need to implement so sort of retry with different
        // variations...and some fallback if N retries fail
        panic!("No description returned from OpenAI");
    }
}
