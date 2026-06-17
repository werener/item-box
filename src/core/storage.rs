// src/core/storage.rs

use super::inventory::Inventory;
use std::path::PathBuf;

pub trait Storage {
    fn save(&self, inventory: &Inventory) -> Result<(), Box<dyn std::error::Error>>;
    fn load(&self) -> Result<Inventory, Box<dyn std::error::Error>>;
}

pub struct JsonFileStorage {
    path: PathBuf,
}

impl JsonFileStorage {
    pub fn new() -> Self {
        Self {
            path: PathBuf::from("inventory.json"),
        }
    }
}

impl Storage for JsonFileStorage {
    fn save(&self, inventory: &Inventory) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(inventory)?;
        std::fs::write(&self.path, json)?;
        Ok(())
    }
    
    fn load(&self) -> Result<Inventory, Box<dyn std::error::Error>> {
        if !self.path.exists() {
            return Ok(Inventory::default());
        }
        let json = std::fs::read_to_string(&self.path)?;
        let inventory = serde_json::from_str(&json)?;
        Ok(inventory)
    }
}