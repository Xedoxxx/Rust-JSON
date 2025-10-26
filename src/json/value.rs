use std::collections::HashMap;
use std::fmt::Write;

#[derive(PartialEq, Clone)]
pub enum JsonValue {
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
    Boolean(bool),
    Null,
}

pub fn as_string(value: JsonValue) -> String {
    match value {
        JsonValue::String (string) => string,
        JsonValue::Number (float) => float.to_string(),
        JsonValue::Boolean (boolean) => boolean.to_string(),
        JsonValue::Null => "Null".to_string(),
        JsonValue::Array (_) => array_as_string(0, 2, value),
        JsonValue::Object (_) => object_as_string(0, 2, value),
    }
}

pub fn array_as_string(current_indent: u16, indent: u16, arr: JsonValue) -> String {
    let mut buffer = String::new();
    buffer.push('[');
    
    if let JsonValue::Array(items) = arr {
        for (i, item) in items.iter().enumerate() {
            let mut value = match item {
                JsonValue::Array(_) => array_as_string(current_indent + indent, indent, item.clone()),
                JsonValue::Object(_) => object_as_string(current_indent + indent, indent, item.clone()),
                _ => as_string(item.clone()),
            };
            if i > 0 {
                buffer.push_str(", ");
            }
            
            if let JsonValue::String(_) = item {
                value = format!("\"{}\"", value);
            }
            
            write!(buffer, "{}", value).unwrap();
        }
    }
    
    buffer.push(']');
    buffer
}


pub fn object_as_string(mut current_indent: u16, iindent: u16, object: JsonValue) -> String {
    let mut buffer = String::new();
    if current_indent < iindent {
        current_indent += iindent;
    }
    let indent = " ".repeat(current_indent as usize);
    buffer.push_str("{\n");
    if let JsonValue::Object(items) = object {
        for (key, item) in items {
            let mut value = match item {
                JsonValue::Array(_) => array_as_string(current_indent + iindent, iindent, item.clone()),
                JsonValue::Object(_) => object_as_string(current_indent + iindent, iindent, item.clone()),
                _ => as_string(item.clone()),
            };
            if let JsonValue::String(_) = item {
                value = format!("\"{}\"", value);
            }
            write!(buffer, "{}\"{}\": {},\n", indent, key, value).unwrap();
        }
    }
    if current_indent > iindent {
        current_indent += iindent;
        let num_of_ind: usize = (current_indent-iindent*2) as usize;
        let new_ind = " ".repeat(num_of_ind);
        write!(buffer, "{}}}", new_ind).unwrap();
    } else {
        write!(buffer, "}}").unwrap();
    }
    buffer
}