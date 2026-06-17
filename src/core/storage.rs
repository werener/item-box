use super::inventory::Inventory;
use crate::error::StorageError;
use std::path::PathBuf;

pub trait Storage {
    fn save(&self, inventory: &Inventory) -> Result<(), StorageError>;
    fn load(&self) -> Result<Inventory, StorageError>;
}

pub struct JsonFileStorage {
    path: PathBuf,
}

impl JsonFileStorage {
    pub fn new() -> Result<Self, StorageError> {
        Self::with_custom_path(None)
    }

    pub fn with_custom_path(custom_path: Option<PathBuf>) -> Result<Self, StorageError> {
        let path = custom_path.unwrap_or_else(Self::default_path);
        
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        Ok(Self { path })
    }

    fn default_path() -> PathBuf {
        directories::ProjectDirs::from("com", "vtoolbox", "vtoolbox")
            .map(|dirs| dirs.data_dir().join("inventory.json"))
            .unwrap_or_else(|| PathBuf::from("./inventory.json"))
    }
}

impl Storage for JsonFileStorage {
    fn save(&self, inventory: &Inventory) -> Result<(), StorageError> {
        let json = serde_json::to_string_pretty(inventory)?;
        std::fs::write(&self.path, json)?;
        Ok(())
    }

    fn load(&self) -> Result<Inventory, StorageError> {
        if !self.path.exists() {
            return Ok(Inventory::default());
        }
        let json = std::fs::read_to_string(&self.path)?;
        let inventory = serde_json::from_str(&json)?;
        Ok(inventory)
    }
}