use crate::core::inventory::Inventory;
use crate::core::models::{Folder, Tag};
use crate::ui::item_card::ItemCard;
use dioxus::prelude::*;

#[component]
pub fn ItemGrid() -> Element {
    let inventory: Signal<Inventory> = use_context();
    rsx! {
        div { class: "grid grid-cols-[repeat(auto-fill,175px)] gap-2 p-2 justify-center",
            for item in &(inventory.read()).items {
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
