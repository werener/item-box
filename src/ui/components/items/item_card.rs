// src/ui/components/items/item_card.rs
use super::control_panel::ControlPanelItem;
use super::image_slider::ImageSlider;
use crate::core::inventory::Inventory;
use crate::core::models::{Folder, Item, Tag};
use dioxus::prelude::*;

#[component]
pub fn ItemCard(item: Item, inventory: Signal<Inventory>) -> Element {
    let tags: Vec<Tag> = item
        .tags
        .iter()
        .filter_map(|id| (inventory()).get_tag(*id).cloned())
        .collect();

    let folder: Option<Folder> = item
        .folder
        .and_then(|id| (inventory()).get_folder(id).cloned());

    let images = vec![
        asset!("/assets/images/tool-placeholder-2.png").to_string(),
        asset!("/assets/images/tool-placeholder.png").to_string(),
        asset!("/assets/images/tool-placeholder.png").to_string(),
    ];

    
    rsx! {
        div { class: "relative w-full aspect-[9/16] bg-surface-muted
            rounded-md shadow-[0_0_20px_rgba(0,0,0,0.3)]
            flex flex-col",

            div { class: "h-[60%]",
                ImageSlider { images }
            }

            div { class: "h-[30%] flex flex-col",
                ItemData { item: item.clone() }
                Tags { tags }
            }

            div { class: "h-[10%]",
                ControlPanelItem { item: item.clone(), inventory }
            }
        }
    }
}

#[component]
fn Tags(tags: Vec<Tag>) -> Element {
    rsx! {
        div { class: "flex px-0.5 gap-0.5 mt-auto items-center select-none",
            for tag in tags.iter().take(3) {
                span {
                    class: "text-[12px] px-1 py-0.25",
                    class: "rounded-xl truncate whitespace-nowrap flex-1 max-w-[10ch]",
                    style: "background-color: #{tag.color.to_hex()};",
                    "{tag.name}"
                }
            }
            if tags.len() > 3 {
                button {
                    class: "text-[12px] text-primary w-[18px] py-0.5 h-[18px]",
                    class: "bg-surface hover:text-primary-hover hover:bg-accent",
                    class: "flex items-center justify-center shrink-0 rounded-full",
                    "+{tags.len() - 3}"
                }
            }
        }
    }
}

#[component]
fn ItemData(item: Item) -> Element {
    rsx! {
        div { class: "px-2 mb-1",
            h3 { class: "text-base font-semibold text-primary truncate", "{item.name}" }

            div { class: "flex items-center gap-2",
                p { class: "text-xs font-medium text-primary max-w-[7ch] overflow-hidden",
                    "${item.price:.2}"
                }
                p { class: "text-xs text-secondary truncate flex-1 text-right", "{item.location()}" }
            }

            p { class: "text-xs text-secondary line-clamp-2", "{item.description()}" }
        }
    }
}
