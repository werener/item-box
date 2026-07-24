// src/ui/components/items/item_card.rs
use super::control_panel::ControlPanelItem;
use super::image_slider::ImageSlider;
use crate::core::inventory::Inventory;
use crate::core::models::{Item, Tag};
use dioxus::prelude::*;

#[component]
pub fn ItemCard(item: Item, inventory: Signal<Inventory>) -> Element {
    rsx! {
        div { class: "relative w-full aspect-[9/16] bg-surface-muted
            rounded-md shadow-[0_0_20px_rgba(0,0,0,0.3)]
            flex flex-col",

            div { class: "h-[60%]",
                ImageSlider { image_ids: item.clone().images }
            }

            div { class: "h-[30%] flex flex-col",
                ItemData { item: item.clone() }
                Tags { item: item.clone(), inventory }
            }

            div { class: "h-[10%]",
                ControlPanelItem { item: item.clone(), inventory }
            }
        }
    }
}

#[component]
fn Tags(item: Item, inventory: Signal<Inventory>) -> Element {
    let tags: Vec<Tag> = item
        .tags
        .iter()
        .filter_map(|id| (inventory()).get_tag(*id).cloned())
        .collect();

    rsx! {
        div { class: "flex px-0.5 gap-0.5 mt-auto items-center select-none",
            for tag in tags.clone().into_iter().take(3) {
                div { class: "truncate relative group flex-1 max-w-[10ch]",
                    div {
                        class: "text-[12px] px-1 py-0.25 group-hover:pr-5",
                        class: "rounded-xl truncate whitespace-nowrap block",
                        style: "background-color: #{tag.bg_color}; color: #{tag.fg_color}",
                        "{tag.name}"
                        button {
                            class: "absolute right-1 top-1/2 -translate-y-1/2",
                            class: "w-4 h-4 rounded-full flex items-center justify-center text-[10px]",
                            class: "bg-secondary-muted text-primary",
                            class: "transition-opacity opacity-0 group-hover:opacity-80 ",
                            onclick: move |_| {
                                inventory.write().remove_tag_from_item(item.id, tag.id);
                            },
                            "×"
                        }
                    }
                
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
