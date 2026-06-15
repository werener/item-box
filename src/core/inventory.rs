use super::models::*;
use fully_pub::fully_pub;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Inventory represents a collection of all available models
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[fully_pub]
struct Inventory {
    tools: Vec<Tool>,
    folders: Vec<Folder>,
    tags: Vec<Tag>,
}

#[fully_pub]
/// CRUD
impl Inventory {
    // Create
    fn add_tool(
        &mut self,
        name: String,
        location: String,
        price: f64,
        folder_id: Option<Uuid>,
        tag_ids: Vec<Uuid>,
    ) -> Uuid {
        let tool = Tool::new(name, location, price, folder_id, tag_ids);
        let id = tool.id;
        self.tools.push(tool);
        id
    }

    fn add_folder(&mut self, name: String) -> Uuid {
        let folder = Folder::new(name);
        let id = folder.id;
        self.folders.push(folder);
        id
    }

    fn add_tag(&mut self, name: String, color: Color) -> Uuid {
        let tag = Tag::new(name, color);
        let id = tag.id;
        self.tags.push(tag);
        id
    }

    // Read
    fn get_tool(&self, id: Uuid) -> Option<&Tool> {
        self.tools.iter().find(|t| t.id == id)
    }

    fn get_folder(&self, id: Uuid) -> Option<&Folder> {
        self.folders.iter().find(|f| f.id == id)
    }

    fn get_tag(&self, id: Uuid) -> Option<&Tag> {
        self.tags.iter().find(|t| t.id == id)
    }

    // Update
    fn rename_tool(&mut self, tool_id: Uuid, new_name: String) -> bool {
        if let Some(tool) = self.tools.iter_mut().find(|t| t.id == tool_id) {
            tool.name = new_name;
            true
        } else {
            false
        }
    }

    fn rename_folder(&mut self, folder_id: Uuid, new_name: String) -> bool {
        if let Some(folder) = self.folders.iter_mut().find(|f| f.id == folder_id) {
            folder.name = new_name;
            true
        } else {
            false
        }
    }

    fn rename_tag(&mut self, tag_id: Uuid, new_name: String) -> bool {
        if let Some(tag) = self.tags.iter_mut().find(|t| t.id == tag_id) {
            tag.name = new_name;
            true
        } else {
            false
        }
    }

    fn recolor_tag(&mut self, tag_id: Uuid, new_color: Color) -> bool {
        if let Some(tag) = self.tags.iter_mut().find(|t| t.id == tag_id) {
            tag.color = new_color;
            true
        } else {
            false
        }
    }

    // Delete
    fn remove_tool(&mut self, tool_id: Uuid) {
        self.tools.retain(|t| t.id != tool_id)
    }

    fn remove_folder(&mut self, folder_id: Uuid) {
        self.folders.retain(|f| f.id != folder_id);

        for tool in &mut self.tools {
            if tool.folder == Some(folder_id) {
                tool.folder = None;
            }
        }
    }

    fn remove_tag(&mut self, tag_id: Uuid) {
        self.tags.retain(|t| t.id != tag_id);

        for tool in &mut self.tools {
            tool.tags.retain(|id| *id != tag_id);
        }
    }
}
