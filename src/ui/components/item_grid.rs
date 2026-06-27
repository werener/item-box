use crate::core::inventory::Inventory;
use crate::core::models::{Folder, Tag};
use crate::ui::state::AppState;
use super::item_card::ItemCard;
use dioxus::prelude::*;

#[component]
pub fn ItemGrid(inventory: Signal<Inventory>) -> Element {
    let app_state: AppState = use_context();

    let filtered_items = use_memo(move || {
        let inv = inventory.read();
        let selected_folder = app_state.selected_folder;
        
        match selected_folder() {
            Some(folder_id) => inv
                .items
                .iter()
                .filter(|item| item.folder == Some(folder_id))
                .cloned()
                .collect(),
            None => inv.items.clone(),
        }
    });
    
    rsx! {
        div { class: "grid grid-cols-[repeat(auto-fill,175px)] gap-2 p-2 justify-center",
            for item in filtered_items.read().iter() {
                {
                    let tags: Vec<Tag> = item
                        .tags
                        .iter()
                        .filter_map(|id| (inventory()).get_tag(*id).cloned())
                        .collect();

                    let folder: Option<Folder> = item
                        .folder
                        .and_then(|id| (inventory()).get_folder(id).cloned());
                    rsx! {
                        ItemCard { item: item.clone(), tags, folder }
                    }
                }
            }
        }
    }
}
