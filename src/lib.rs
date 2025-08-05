#![warn(clippy::all)]
#![warn(clippy::cargo)] 

use std::{collections::HashMap, fs::read_to_string};
mod parser;
use parser::Parser;

#[derive(Clone, Debug, PartialEq)]
pub struct PropReader {
    properties: HashMap<String, String>,
}

impl PropReader {
    pub fn new(filename: &str) -> Self {
        let contents = read_to_string(filename).expect("Could not find properties file.");
        Self { properties: Parser::parse(contents) }
    }

    // Return a value based on the given key
    pub fn get(&self, key: &str) -> &str {
        match self.properties.get(key) {
            Some(data) => data,
            None => ""
        }
    }

    // Return all properties data in key, value formed as a Vector
    pub fn get_all_data(self) -> HashMap<String, String> {
        self.properties.clone()
    }

    // Return all of the keys in the properties file as a Vector of Strings
    pub fn get_all_keys(self) -> Vec<String> {
        self.properties.into_keys().collect()
    }

    // Return all of the values in the properties file as a Vector of Strings
    pub fn get_all_values(self) -> Vec<String> {
        self.properties.into_values().collect()
    }
}

impl Default for PropReader {
    // Default values
    fn default() -> Self {
        Self::new("application.properties")
    }
}
