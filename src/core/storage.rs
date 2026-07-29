// src/core/storage.rs
use super::inventory::Inventory;
use crate::error::StorageError;
use std::{path::PathBuf, sync::Arc};

const PROJECT_NAME: &str =
    option_env!("PROJECT_NAME").expect("env variable 'PROJECT_NAME' not found (image)");

pub trait Storage {
    fn save(&self, inventory: &Inventory) -> Result<(), StorageError>;
    fn load(&self) -> Result<Inventory, StorageError>;
}

/// Desktop implementation of Storage trait.
///
/// Saves files into `json` format.
/// Uses a default system path under organization and application names "vtoobox".
#[cfg(not(target_arch = "wasm32"))]
pub struct JsonFileStorage {
    path: PathBuf,
}

#[cfg(not(target_arch = "wasm32"))]
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
        directories::ProjectDirs::from("com", PROJECT_NAME, PROJECT_NAME)
            .map(|dirs| dirs.data_dir().join("inventory.json"))
            .unwrap_or_else(|| PathBuf::from("./inventory.json"))
    }
}

#[cfg(not(target_arch = "wasm32"))]
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

/// Web implementation of Storage trait.
///
/// Saves files, using browsers' built-in API for file storage.
#[cfg(target_arch = "wasm32")]
pub struct WebStorage {
    key: String,
}

#[cfg(target_arch = "wasm32")]
impl WebStorage {
    pub fn new() -> Self {
        Self {
            key: "vtoolbox_inventory".to_string(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Storage for WebStorage {
    fn save(&self, inventory: &Inventory) -> Result<(), StorageError> {
        let json = serde_json::to_string(inventory)?;

        let window = web_sys::window()
            .ok_or_else(|| StorageError::WebStorage("No window object".to_string()))?;

        let storage = window
            .local_storage()
            .map_err(|_| StorageError::WebStorage("Failed to get localStorage".to_string()))?
            .ok_or_else(|| StorageError::WebStorage("localStorage not available".to_string()))?;

        storage
            .set_item(&self.key, &json)
            .map_err(|_| StorageError::WebStorage("Failed to save to localStorage".to_string()))?;

        Ok(())
    }

    fn load(&self) -> Result<Inventory, StorageError> {
        let window = web_sys::window()
            .ok_or_else(|| StorageError::WebStorage("No window object".to_string()))?;

        let storage = window
            .local_storage()
            .map_err(|_| StorageError::WebStorage("Failed to get localStorage".to_string()))?
            .ok_or_else(|| StorageError::WebStorage("localStorage not available".to_string()))?;

        match storage.get_item(&self.key) {
            Ok(Some(json)) => {
                let inventory = serde_json::from_str(&json)?;
                Ok(inventory)
            }
            Ok(None) => Ok(Inventory::default()),
            Err(_) => Err(StorageError::WebStorage(
                "Failed to read from localStorage".to_string(),
            )),
        }
    }
}

// Factory function
pub fn create_storage() -> Result<Arc<dyn Storage>, StorageError> {
    #[cfg(target_arch = "wasm32")]
    {
        Ok(Arc::new(WebStorage::new()))
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        Ok(Arc::new(JsonFileStorage::new()?))
    }
}
