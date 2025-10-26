mod json;
use json::JsonParser;
use json::JsonValue;
use json::object_as_string;
use std::time::Instant;
use std::collections::HashMap;

fn test_from_json() {
    println!("=== JSON Parsing Test ===");
    
    let test_json = r#"
    {
        "user": {
            "name": "John Doe",
            "age": 30,
            "active": true,
            "skills": ["Rust", "Python", "JavaScript"],
            "address": {
                "city": "New York",
                "zipcode": 10001
            }
        },
        "metadata": {
            "version": "1.0.0",
            "count": 1
        }
    }"#.to_string();
    
    let mut parser = JsonParser::new();
    let object = parser.parse_json(test_json);
    println!("{}", object_as_string(0, 2, object));
}

fn test_to_json() {
    println!("=== JSON Construction Test ===");
    
    let mut user_profile = HashMap::new();
    user_profile.insert("username".to_string(), JsonValue::String("alice_smith".to_string()));
    user_profile.insert("score".to_string(), JsonValue::Number(95.5));
    user_profile.insert("premium".to_string(), JsonValue::Boolean(true));
    user_profile.insert("last_login".to_string(), JsonValue::Null);
    
    // Create nested preferences object
    let mut preferences = HashMap::new();
    preferences.insert("theme".to_string(), JsonValue::String("dark".to_string()));
    preferences.insert("notifications".to_string(), JsonValue::Boolean(false));
    preferences.insert("language".to_string(), JsonValue::String("en".to_string()));
    user_profile.insert("preferences".to_string(), JsonValue::Object(preferences));
    
    // Create array of tags
    let tags = JsonValue::Array(vec![
        JsonValue::String("developer".to_string()),
        JsonValue::String("rustacean".to_string()),
        JsonValue::String("opensource".to_string()),
    ]);
    user_profile.insert("tags".to_string(), tags);
    
    println!("{}", object_as_string(0, 2, JsonValue::Object(user_profile)));
}

fn main() {
    let start = Instant::now();
    
    test_from_json();
    println!();
    test_to_json();
    
    let elapsed = start.elapsed();
    println!("\nCompleted in {:.3} seconds", elapsed.as_secs_f64());
}