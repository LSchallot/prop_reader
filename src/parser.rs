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
}