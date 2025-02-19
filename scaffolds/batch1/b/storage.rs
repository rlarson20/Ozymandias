// storage.rs
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct Storage {
    data: HashMap<String, String>,
    storage_path: String,
}

impl Storage {
    pub fn new(storage_path: &str) -> Storage {
        Storage {
            data: HashMap::new(),
            storage_path: storage_path.to_string(),
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        // TO DO: implement saving to file
        Ok(())
    }

    pub fn load(&mut self) -> Result<(), std::io::Error> {
        // TO DO: implement loading from file
        Ok(())
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}
