use r_json::json::{JsonParser, JsonValue};
use std::fs;

use colored::Colorize;

#[test]
fn parse() {
    println!("{}", "=== PARSE FILE TEST ===".red());
    let content: String = fs::read_to_string("tests/data/data.json").expect("Failed to read test data");
    let mut parser = JsonParser::new();
    let parsed: JsonValue = parser.parse_json(content);
    
    println!("{}", parsed.to_string());
    println!("{}", "=== PARSE FILE TEST ===".red());
}