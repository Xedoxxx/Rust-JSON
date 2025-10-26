use std::collections::HashMap;
use crate::json::JsonValue;

pub struct JsonParser {
    position: usize,
    source: String,
}

impl JsonParser {
    pub fn new() -> Self {
        Self { 
            position: 0, 
            source: String::new(),
        }
    }
    
    pub fn parse_json(&mut self, source: String) -> JsonValue {
        self.position = 0;
        self.source = source;
        self.parse_value()
    }
    
    fn peek(&self, relative: i32) -> char {
        let target = self.position as i32 + relative;
        if target >= 0 && target < self.source.len() as i32 {
            self.source.chars().nth(target as usize).unwrap()
        } else {
            panic!("Index out of bounds: {}", target);
        }
    }
    
    fn skip_whitespace(&mut self) {
        while self.position < self.source.len() {
            let c = self.peek(0);
            if c.is_whitespace() {
                self.position += 1;
            } else {
                break;
            }
        }
    }
    
    fn consume(&mut self, expected: char) -> bool {
        self.skip_whitespace();
        if self.position < self.source.len() && self.peek(0) == expected {
            self.position += 1;
            true
        } else {
            false
        }
    }
    
    fn is_start_of_boolean(&self, c: char) -> bool {
        let lowercase_char = c.to_lowercase().next().unwrap();
        lowercase_char == 't' || lowercase_char == 'f'
    }
    
    fn parse_string(&mut self) -> JsonValue {
        self.skip_whitespace();
        let bracket_type = self.peek(0);
        if bracket_type != '"' && bracket_type != '\'' {
            panic!("Expected string delimiter, found: {}", bracket_type);
        }
        
        self.position += 1;
        let mut buffer = String::new();
        
        while self.position < self.source.len() {
            let current = self.peek(0);
            if current == bracket_type {
                self.position += 1;
                break;
            }
            buffer.push(current);
            self.position += 1;
        }
        
        JsonValue::String(buffer)
    }
    
    fn parse_number(&mut self) -> JsonValue {
        self.skip_whitespace();
        let mut buffer = String::new();
        
        while self.position < self.source.len() {
            let current = self.peek(0);
            if current != '.' && !current.is_ascii_digit() {
                break;
            }
            buffer.push(current);
            self.position += 1;
        }
        
        match buffer.parse::<f64>() {
            Ok(num) => JsonValue::Number(num),
            Err(_) => panic!("Invalid number: {}", buffer),
        }
    }
    
    fn parse_boolean(&mut self) -> JsonValue {
        self.skip_whitespace();
        
        if self.position + 4 <= self.source.len() && 
           self.source[self.position..self.position + 4].to_lowercase() == "true" {
            self.position += 4;
            JsonValue::Boolean(true)
        } else if self.position + 5 <= self.source.len() && 
                  self.source[self.position..self.position + 5].to_lowercase() == "false" {
            self.position += 5;
            JsonValue::Boolean(false)
        } else {
            panic!("Invalid boolean value");
        }
    }
    
    fn parse_null(&mut self) -> JsonValue {
        self.skip_whitespace();
        
        if self.position + 4 <= self.source.len() && 
           self.source[self.position..self.position + 4].to_lowercase() == "null" {
            self.position += 4;
            JsonValue::Null
        } else {
            panic!("Invalid null value");
        }
    }
    
    fn parse_array(&mut self) -> JsonValue {
        if !self.consume('[') {
            panic!("Expected '[' for array");
        }
        
        let mut array: Vec<JsonValue> = Vec::new();
        self.skip_whitespace();
        
        if self.peek(0) == ']' {
            self.position += 1;
            return JsonValue::Array(array);
        }
        
        loop {
            self.skip_whitespace();
            let value = self.parse_value();
            array.push(value);
            
            self.skip_whitespace();
            if self.peek(0) == ']' {
                self.position += 1;
                break;
            }
            
            if !self.consume(',') {
                panic!("Expected ',' or ']' in array");
            }
        }
        
        JsonValue::Array(array)
    }
    
    fn parse_object(&mut self) -> JsonValue {
        if !self.consume('{') {
            panic!("Expected '{{' for object");
        }
        
        let mut object: HashMap<String, JsonValue> = HashMap::new();
        self.skip_whitespace();
        
        if self.peek(0) == '}' {
            self.position += 1;
            return JsonValue::Object(object);
        }
        
        loop {
            self.skip_whitespace();
            
            // Parse key
            let key_value = self.parse_string();
            let key = match key_value {
                JsonValue::String(s) => s,
                _ => panic!("Expected string key"),
            };
            
            self.skip_whitespace();
            if !self.consume(':') {
                panic!("Expected ':' after key");
            }
            
            // Parse value
            let value = self.parse_value();
            object.insert(key, value);
            
            self.skip_whitespace();
            if self.peek(0) == '}' {
                self.position += 1;
                break;
            }
            
            if !self.consume(',') {
                panic!("Expected ',' or '}}' in object");
            }
        }
        
        JsonValue::Object(object)
    }
    
    fn parse_value(&mut self) -> JsonValue {
        self.skip_whitespace();
        
        if self.position >= self.source.len() {
            panic!("Unexpected end of input");
        }
        
        let current = self.peek(0);
        
        if current == '"' || current == '\'' {
            self.parse_string()
        } else if current == '[' {
            self.parse_array()
        } else if current == '{' {
            self.parse_object()
        } else if current.is_ascii_digit() || current == '-' {
            self.parse_number()
        } else if self.is_start_of_boolean(current) {
            self.parse_boolean()
        } else if current == 'n' {
            self.parse_null()
        } else {
            panic!("Unknown value type: '{}' at position {}", current, self.position);
        }
    }
}