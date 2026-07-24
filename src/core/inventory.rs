// src/core/inventory.rs
use crate::core::builder::ItemBuilder;

use super::models::*;
use fully_pub::fully_pub;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Inventory represents a collection of all available models
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[fully_pub]
struct Inventory {
    items: Vec<Item>,
    folders: Vec<Folder>,
    tags: Vec<Tag>,
}

#[fully_pub]
/// CRUD
impl Inventory {
    // Create
    fn add_item<S: Into<String>>(
        &mut self,
        name: S,
        location: S,
        description: S,
        price: f64,
        folder_id: Option<Uuid>,
        tag_ids: Vec<Uuid>,
    ) -> Uuid {
        let item = ItemBuilder::new(name.into())
            .location(location.into())
            .description(description.into())
            .price(price)
            .folder(folder_id)
            .tags(tag_ids)
            .build();

        let id = item.id;
        self.items.push(item);
        id
    }

    fn add_item_struct(&mut self, item: Item) -> Uuid {
        let id = item.id;
        self.items.push(item);
        id
    }

    fn add_folder<S: Into<String>>(&mut self, name: S) -> Uuid {
        let folder = Folder::new(name.into());
        let id = folder.id;
        self.folders.push(folder);
        id
    }

    fn add_tag<S: Into<String>>(&mut self, name: S, color: Color) -> Uuid {
        let tag = Tag::new(name.into(), color);
        let id = tag.id;
        self.tags.push(tag);
        id
    }

    fn add_tag_struct(&mut self, tag: Tag) -> Uuid {
        let id = tag.id;
        self.tags.push(tag);
        id
    }

    // Read
    fn get_item(&self, id: Uuid) -> Option<&Item> {
        self.items.iter().find(|t| t.id == id)
    }

    fn get_folder(&self, id: Uuid) -> Option<&Folder> {
        self.folders.iter().find(|f| f.id == id)
    }

    fn get_tag(&self, id: Uuid) -> Option<&Tag> {
        self.tags.iter().find(|t| t.id == id)
    }

    fn get_item_mut(&mut self, id: Uuid) -> Option<&mut Item> {
        self.items.iter_mut().find(|t| t.id == id)
    }

    fn get_folder_mut(&mut self, id: Uuid) -> Option<&mut Folder> {
        self.folders.iter_mut().find(|f| f.id == id)
    }

    fn get_tag_mut(&mut self, id: Uuid) -> Option<&mut Tag> {
        self.tags.iter_mut().find(|t| t.id == id)
    }

    // Update
    // Items
    fn change_item(&mut self, item_id: Uuid, new_item: Item) -> bool {
        if let Some(item) = self.get_item_mut(item_id) {
            new_item.clone_into(item);
            true
        } else {
            false
        }
    }

    fn rename_item<S: Into<String>>(&mut self, item_id: Uuid, new_name: S) -> bool {
        if let Some(item) = self.get_item_mut(item_id) {
            item.name = new_name.into();
            true
        } else {
            false
        }
    }

    fn change_item_folder(&mut self, item_id: Uuid, new_folder_id: Option<Uuid>) -> bool {
        if let Some(item) = self.get_item_mut(item_id) {
            item.folder = new_folder_id;
            true
        } else {
            false
        }
    }

    fn add_tag_to_item(&mut self, item_id: Uuid, tag_id: Uuid) -> bool {
        if let Some(item) = self.get_item_mut(item_id) {
            if !item.tags.contains(&tag_id) {
                item.tags.push(tag_id);
            }
            true
        } else {
            false
        }
    }

    fn remove_tag_from_item(&mut self, item_id: Uuid, tag_id: Uuid) -> bool {
        if let Some(item) = self.get_item_mut(item_id) {
            item.tags.retain(|id| *id != tag_id);
            true
        } else {
            false
        }
    }

    // Folders
    fn rename_folder<S: Into<String>>(&mut self, folder_id: Uuid, new_name: S) -> bool {
        if let Some(folder) = self.get_folder_mut(folder_id) {
            folder.name = new_name.into();
            true
        } else {
            false
        }
    }

    // Tags
    fn rename_tag<S: Into<String>>(&mut self, tag_id: Uuid, new_name: S) -> bool {
        if let Some(tag) = self.get_tag_mut(tag_id) {
            tag.name = new_name.into();
            true
        } else {
            false
        }
    }

    fn recolor_tag(&mut self, tag_id: Uuid, new_color: Color) -> bool {
        if let Some(tag) = self.get_tag_mut(tag_id) {
            tag.color = new_color;
            true
        } else {
            false
        }
    }

    // Delete
    fn remove_item(&mut self, item_id: Uuid) {
        self.items.retain(|t| t.id != item_id)
    }

    fn remove_folder(&mut self, folder_id: Uuid) {
        self.folders.retain(|f| f.id != folder_id);

        for item in &mut self.items {
            if item.folder == Some(folder_id) {
                item.folder = None;
            }
        }
    }

    fn remove_tag(&mut self, tag_id: Uuid) {
        self.tags.retain(|t| t.id != tag_id);

        for item in &mut self.items {
            item.tags.retain(|id| *id != tag_id);
        }
    }
}
