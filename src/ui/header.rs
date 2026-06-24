use crate::core::inventory::Inventory;
use dioxus::prelude::*;

#[component]
pub fn Header(inventory: Signal<Inventory>) -> Element {
    rsx! {
        div { class: "flex items-center gap-2 p-2 bg-surface border-b border-secondary",
            for folder in inventory.read().folders.iter() {
                div { class: "px-3 py-1 bg-secondary-muted text-primary rounded-md text-sm",
                    "{folder.name}"
                }
            }
        }
    }
}