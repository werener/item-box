use std::path::PathBuf;
use uuid::Uuid;
use v_tool_box::core::builder::ToolBuilder;
use v_tool_box::core::inventory::Inventory;
use v_tool_box::core::models::*;
use v_tool_box::core::storage::{JsonFileStorage, Storage};

fn temp_path() -> PathBuf {
    std::env::temp_dir().join(format!("vtoolbox_test_{}.json", Uuid::new_v4()))
}

struct TempFile {
    path: PathBuf,
}

impl TempFile {
    fn new() -> Self {
        Self { path: temp_path() }
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

#[test]
fn test_save_and_load_roundtrip() {
    let temp = TempFile::new();
    let storage = JsonFileStorage::with_custom_path(Some(temp.path.clone())).unwrap();
    let mut inventory = Inventory::default();
    
    let folder_id = inventory.add_folder("Garage");
    let tag_id = inventory.add_tag("Power Tool", Color { r: 255, g: 0, b: 0 });
    
    inventory.add_tool_struct(
        ToolBuilder::new("Drill")
            .location("Shelf 1")
            .price(99.99)
            .folder(Some(folder_id))
            .tags(vec![tag_id])
            .build(),
    );

    storage.save(&inventory).expect("save should succeed");

    let loaded = storage.load().expect("load should succeed");

    assert_eq!(loaded.folders.len(), 1, "folder count mismatch");
    assert_eq!(loaded.tags.len(), 1, "tag count mismatch");
    assert_eq!(loaded.tools.len(), 1, "tool count mismatch");
    assert_eq!(loaded.tools[0].name, "Drill", "tool name mismatch");
    assert_eq!(loaded.tools[0].price, 99.99, "tool price mismatch");
}

#[test]
fn test_load_nonexistent_returns_default() {
    let temp = TempFile::new();
    let storage = JsonFileStorage::with_custom_path(Some(temp.path.clone())).unwrap();
    let loaded = storage.load().expect("load should succeed even if file doesn't exist");

    assert!(loaded.tools.is_empty(), "default inventory should have no tools");
    assert!(loaded.folders.is_empty(), "default inventory should have no folders");
    assert!(loaded.tags.is_empty(), "default inventory should have no tags");
}

#[test]
fn test_save_creates_parent_directories() {
    let temp_dir = std::env::temp_dir().join(format!("vtoolbox_test_dir_{}", Uuid::new_v4()));
    let file_path = temp_dir.join("subdir").join("inventory.json");
    let storage = JsonFileStorage::with_custom_path(Some(file_path.clone())).unwrap();
    let inventory = Inventory::default();

    storage.save(&inventory).expect("save should succeed");

    assert!(file_path.exists(), "file should be created");
    assert!(temp_dir.join("subdir").exists(), "parent directory should be created");

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_save_overwrites_existing_file() {
    let temp = TempFile::new();
    let storage = JsonFileStorage::with_custom_path(Some(temp.path.clone())).unwrap();
    let mut inventory = Inventory::default();
    
    storage.save(&inventory).expect("first save should succeed");

    inventory.add_tool_struct(
        ToolBuilder::new("Hammer")
            .location("Box")
            .price(15.0)
            .build(),
    );
    
    storage.save(&inventory).expect("second save should succeed");

    let loaded = storage.load().expect("load should succeed");
    assert_eq!(loaded.tools.len(), 1, "should have one tool after overwrite");
    assert_eq!(loaded.tools[0].name, "Hammer", "tool name should match");
}