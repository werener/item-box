use crate::core::inventory::Inventory;
use crate::core::models::*;
use crate::core::storage::create_storage;
use crate::ui::item_grid::ItemGrid;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let storage = use_hook(|| create_storage().unwrap());
    let mut inventory = use_signal(|| {
        storage.load().unwrap_or_default()
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }

        div { class: "min-h-screen w-full bg-surface-muted",
            button {
                class: "m-2 w-[90%] h-10 border-2 border-black",
                onclick: move |_| {
                    let _ = storage.save(&inventory());
                },
                "Save inventory"
            }
            ItemGrid { inventory: inventory() }
        }
    }
}