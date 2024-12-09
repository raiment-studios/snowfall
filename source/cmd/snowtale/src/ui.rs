use std::io::{self, Write};

use colored::Colorize;

pub fn parse_color(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches("#");
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        (r, g, b)
    } else if hex.len() == 3 {
        let r = u8::from_str_radix(&hex[0..1], 16).unwrap();
        let g = u8::from_str_radix(&hex[1..2], 16).unwrap();
        let b = u8::from_str_radix(&hex[2..3], 16).unwrap();
        let r = r * 16 + r;
        let g = g * 16 + g;
        let b = b * 16 + b;
        (r, g, b)
    } else {
        (255, 255, 255)
    }
}

pub fn cprintln<T>(color: &str, text: T)
where
    T: Into<String>,
{
    let (r, g, b) = parse_color(color);
    println!("{}", text.into().truecolor(r, g, b));
}

pub fn print_paragraph(color: &str, text: &str) {
    // Split the text into lines of at most 80 characters, splitting at word boundaries.
    let (r, g, b) = parse_color(color);

    // Split text by newline and process each line separately
    for line in text.lines() {
        let regex = regex::Regex::new(r".{1,78}(?:\s|$)").unwrap();
        let lines = regex
            .find_iter(line)
            .map(|m| m.as_str().trim()) // Trim any trailing whitespace
            .collect::<Vec<_>>();

        for line in lines {
            println!("  {}", line.truecolor(r, g, b));
        }
        println!();
    }
}

pub async fn prompt() -> String {
    print!("> ");
    io::stdout().flush().unwrap(); // Flush to show the prompt immediately

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return "".into();
    }
    input
}
