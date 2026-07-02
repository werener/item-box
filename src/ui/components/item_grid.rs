// src/ui/components/item_grid.rs
use super::item_card::{ItemCard, ItemCardEvents};
use crate::core::inventory::Inventory;
use crate::core::models::{Folder, Tag};
use crate::ui::state::AppState;
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn ItemGrid(inventory: Signal<Inventory>) -> Element {
    let app_state: AppState = use_context();

    let filtered_items = use_memo(move || {
        let inv = inventory.read();

        match (app_state.selected_folder)() {
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
                    let item = item.clone();
                    let item_id = item.id;

                    let tags: Vec<Tag> = item
                        .tags
                        .iter()
                        .filter_map(|id| (inventory()).get_tag(*id).cloned())
                        .collect();

                    let folder: Option<Folder> = item
                        .folder
                        .and_then(|id| (inventory()).get_folder(id).cloned());

                    let events = ItemCardEvents {
                        on_edit: Callback::new(move |_| {}),

                        on_delete: Callback::new(move |_| {
                            inventory.write().remove_item(item_id);
                        }),

                        on_change_folder: Callback::new(move |new_folder_id: Option<Uuid>| {
                            inventory.write().change_item_folder(item_id, new_folder_id);
                        }),

                        on_add_tag: Callback::new(move |tag_id: Uuid| {
                            inventory.write().add_tag_to_item(item_id, tag_id);
                        }),

                        on_remove_tag: Callback::new(move |tag_id: Uuid| {
                            inventory.write().remove_tag_from_item(item_id, tag_id);
                        }),
                    };
                    rsx! {
                        ItemCard {
                            item,
                            tags,
                            folder,
                            events,
                        }
                    }
                }
            }
        }
    }
}
