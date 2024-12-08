mod openai;
mod ui;
use std::collections::{HashMap, HashSet};

use colored::Colorize;
use openai::*;
use serde::{Deserialize, Serialize};
use ui::*;

#[derive(Debug, Default)]
struct ActionLog {
    entries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum ItemStatus {
    Immovable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum UseCondition {
    PlayerHas { item_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum UseEffects {
    GrantKnowledge { amount: i32 },
    SingleUse,
    Stationary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Item {
    id: String,
    aliases: Vec<String>,
    description: String,
    status: Vec<ItemStatus>,
    use_conditions: Vec<UseCondition>,
    use_effects: Vec<UseEffects>,
}

impl Item {
    fn new(id: &str) -> Self {
        Item {
            id: id.to_string(),
            aliases: Vec::new(),
            description: "".to_string(),
            status: Vec::new(),
            use_conditions: Vec::new(),
            use_effects: Vec::new(),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Player {
    position: (i32, i32),
    inventory: Vec<Item>,

    knownledge: i32,
}

impl Player {
    fn new() -> Player {
        Player {
            position: (0, 0),
            inventory: Vec::new(),
            knownledge: 0,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Room {
    name: String,
    description: String,
    items: Vec<Item>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Encyclopedia {
    markers: HashSet<String>,
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
    let mut encyclopedia = read_encyclopedia();

    loop {
        cprintln(
            "#3AC",
            format!(
                "POS: {},{}  KNOW: {}",
                player.position.0, player.position.1, player.knownledge
            )
            .as_str(),
        );
        println!();

        let room_position = player.position;
        let mut room = ensure_room(room_position, &mut encyclopedia, &mut world, &actions).await;
        cprintln("3ff", room.name.as_str());
        print_paragraph("eee", room.description.as_str());

        println!();
        for item in &room.items {
            cprintln("77f", item.description.as_str());
        }

        // Process the input
        let input = prompt().await;
        let input = input.trim().to_lowercase();

        // Split input by whitespace
        let words: Vec<&str> = input.split_whitespace().collect();
        let action = {
            match *words.get(0).unwrap_or(&"") {
                "n" => "north",
                "s" => "south",
                "e" => "east",
                "w" => "west",
                "q" => "quit",
                "i" => "inventory",
                s => s,
            }
            .to_string()
        };
        let subject = if words.len() > 1 {
            words[1..].join(" ")
        } else {
            "".to_string()
        };

        println!("You entered: {}", input);
        println!();

        let mut handled = true;
        match action.trim() {
            "north" => player.position.1 += 1,
            "south" => player.position.1 -= 1,
            "east" => player.position.0 += 1,
            "west" => player.position.0 -= 1,
            "regen" => {
                // TODO: this can break the encyclopedia markers as the prior
                // room may have set the marker and the new room may not have
                // the same items
                create_room(player.position, &mut encyclopedia, &mut world, &actions).await;
            }
            "get" => command_get(subject, &mut player, &mut world, &mut encyclopedia),
            "use" => command_use(subject, &mut player, &mut world, &mut encyclopedia),
            "inventory" => {
                for item in &player.inventory {
                    cprintln("0F5", item.description.as_str());
                }
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
        write_encyclopedia(&encyclopedia);
    }
}

fn command_get(
    subject: String,
    player: &mut Player,
    world: &mut World,
    encyclopedia: &mut Encyclopedia,
) {
    let mut room = world.rooms.get_mut(&player.position).unwrap().clone();
    let item = room
        .items
        .iter()
        .find(|item| item.aliases.contains(&subject));

    let Some(item) = item else {
        println!("You don't see that item here.");
        return;
    };
    let item = item.clone();

    if item.status.contains(&ItemStatus::Immovable) {
        cprintln("F50", "You can't take that item.");
        return;
    }

    // Remove the item from the room and add it to the player inventory
    room.items.retain(|i| i.id != item.id);
    player.inventory.push(item.clone());
    write_room(world, player.position, &room);
}

fn command_use(
    subject: String,
    player: &mut Player,
    world: &mut World,
    encyclopedia: &mut Encyclopedia,
) {
    cprintln("#FC3", format!("You used: {}", subject).as_str());

    let mut room = world.rooms.get_mut(&player.position).unwrap().clone();

    let item = room
        .items
        .iter()
        .find(|item| item.aliases.contains(&subject));

    let Some(item) = item else {
        println!("You don't see that item here.");
        return;
    };
    let item = item.clone();

    // Check the pre-conditions
    for condition in &item.use_conditions {
        match condition {
            UseCondition::PlayerHas { item_id } => {
                if !player.inventory.iter().any(|item| item.id == *item_id) {
                    cprintln("F50", "You don't have the required item to use this.");
                    return;
                }
            }
        }
    }

    cprintln("0F5", "You used the item!");

    for effect in &item.use_effects {
        match effect {
            UseEffects::GrantKnowledge { amount } => {
                player.knownledge += amount;
                cprintln("0F5", format!("You gained {} knowledge!", amount).as_str());
            }
            UseEffects::SingleUse => {
                room.items.retain(|i| i.id != item.id);
                cprintln("0F5", "The item was consumed.");
            }
            UseEffects::Stationary => {}
        }
    }

    write_room(world, player.position, &room);
}

fn read_encyclopedia() -> Encyclopedia {
    let contents = std::fs::read_to_string("store/_default/encyclopedia.yaml").ok();
    match contents {
        Some(contents) => serde_yaml::from_str(&contents).unwrap(),
        None => Encyclopedia::default(),
    }
}

fn write_encyclopedia(encyclopedia: &Encyclopedia) {
    let contents = serde_yaml::to_string(encyclopedia).unwrap();
    std::fs::write("store/_default/encyclopedia.yaml", contents).unwrap();
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

fn write_room(world: &mut World, position: (i32, i32), room: &Room) {
    // Ensure the memory-copy is in sync with the file
    world.rooms.insert(position, room.clone());

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
    encyclopedia: &mut Encyclopedia,
    world: &mut World, //
    actions: &ActionLog,
) -> Room {
    if let Some(room) = peek_room(position, world) {
        return room;
    }
    create_room(position, encyclopedia, world, actions).await
}

async fn create_room(
    position: (i32, i32), //
    encyclopedia: &mut Encyclopedia,
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

When writing text provide single, specific answers in plain English. Do not provide multiple
options.  Describe the room itself objectively and without any preface.  
Do not reference the player or group or any of their recent actions.  Use a writing 
style that reminds the reader of some combination of Jack Vance and JRR Tolkien.  
Use third-person and describe only the room and surroundings.

Always response in JSON format according to the schema provided in the question.

## SYSTEM

{sys_nearby}

{sys_recent_actions}

## USER

Please return a room object in JSON with exactly two fields: 
one called "name" and one called "description".  The "name" field should be
a short name for the room, at most 4 words in length.  The "description" field
should be a description of the room, at most 3 sentences in length.
Always, always provide a "name" field for the room in the JSON response.

    "#
    );

    let d = open_ai2(p.as_str()).await.unwrap_or_default();
    if !d.is_empty() {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Resp {
            // OpenAPI *insists* on returning a "title" rather than "name" field
            title: Option<String>,
            name: Option<String>,
            description: Option<String>,
            room_description: Option<String>,
        }

        let mut room = Room::default();
        room.description = "<undefined>".to_string();

        cprintln("555", &d);

        let resp = serde_json::from_str::<Resp>(d.as_str()).unwrap();
        if let Some(name) = resp.name {
            room.name = name;
        } else {
            room.name = resp.title.unwrap_or_default();
        }
        if let Some(desc) = resp.room_description {
            room.description = desc;
        } else if let Some(desc) = resp.description {
            room.description = desc;
        }

        // Randomly add a key to the room
        let r = rand::random::<u8>() % 100;
        if r < 50 {
            let colors = vec![
                "red", "green", "blue", "yellow", "purple", "orange", "cyan", "magenta",
            ];
            let color = colors[rand::random::<usize>() % colors.len()];
            let id = format!("key_{}", color);
            let desc = format!("A {} key", color);

            if !encyclopedia.markers.contains(id.as_str()) {
                let mut item = Item::new(id.as_str());
                item.description = desc.clone();
                item.aliases.push("key".to_string());
                item.aliases.push(format!("{} key", color));
                room.items.push(item);
                encyclopedia.markers.insert(id);
            }
        }

        // Randomly add a chest to the room
        let r = rand::random::<u8>() % 100;
        if r < 50 {
            let colors = vec![
                "red", "green", "blue", "yellow", "purple", "orange", "cyan", "magenta",
            ];
            let color = colors[rand::random::<usize>() % colors.len()];
            let id = format!("chest_{}", color);
            let desc = format!("A {} chest", color);

            let mut item = Item::new(id.as_str());
            item.description = desc.clone();
            item.aliases = vec![
                "chest".to_string(), //
                format!("{} chest", color),
            ];
            item.status.push(ItemStatus::Immovable);
            item.use_conditions.push(UseCondition::PlayerHas {
                item_id: format!("key_{}", color),
            });
            item.use_effects
                .push(UseEffects::GrantKnowledge { amount: 10 });
            item.use_effects.push(UseEffects::SingleUse);

            room.items.push(item);
        }

        write_room(world, position, &room);
        return room;
    } else {
        // TODO: need to implement so sort of retry with different
        // variations...and some fallback if N retries fail
        panic!("No description returned from OpenAI");
    }
}
