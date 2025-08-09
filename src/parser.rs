use std::collections::HashMap;

pub struct Parser;

impl Parser {
    pub fn parse(contents: String) -> HashMap<String, String> {
        let mut hashed: HashMap<String, String> = HashMap::new();
        for line in contents.lines() {
            // Skip lines that are empty or comments 
            if line.is_empty() || line.trim().starts_with('#') {
                continue;
            }
            // Split the str only once as the value could contain colons and/or equals
            let mut keyval = line.splitn(2,[':', '=']);
            let key = keyval.next().unwrap().trim();
            let mut value = keyval.next().unwrap().trim();
            // Check for in-line comment and remove it from the final value
            if value.contains('#') {
                let mut cleanval = value.split('#');
                value = cleanval.next().unwrap().trim();
            }
            // Attempt to remove the leading and trailing double-quotes if they exist
             if let Some(stripped_value) = value.strip_prefix('"').and_then(|value| value.strip_suffix('"')) {
                 value = stripped_value; // Convert the &str slice back to a String if successful
             }

            hashed.insert(key.to_string(), value.to_string());
        }
        hashed
    }

    pub fn parse_xml(contents: String) -> HashMap<String, String> {
        let mut in_comments = false;
        let mut hashed: HashMap<String, String> = HashMap::new();
        for line in contents.lines() {
            // Perform checks for multi-line comments, empty lines, and lines that won't matter.
            if in_comments && line.contains("-->") {
                in_comments = false;
                continue;
            } else if line.contains("<!--") {
                in_comments = true;
                continue;
            } else if line.is_empty() || !line.trim().starts_with("<entry") || in_comments {
                continue;
            }

            // First we need to extract the key
            let mut key = line.split("key=\"").last().unwrap().trim();
            key = key.split("\">").next().unwrap().trim();

            // Now extract the value
            let mut value = line.split("</entry>").next().unwrap().trim();
            value = value.split(">").last().unwrap().trim();

            if let Some(stripped_key) = key.strip_prefix('"').and_then(|key| key.strip_suffix('"')) {
                key = stripped_key;
            }
            hashed.insert(key.to_string(), value.to_string());
        }
        hashed
    }
}
