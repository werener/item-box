// src/ui/components/control_panel.rs
use crate::core::inventory::Inventory;
use crate::core::storage::Storage;
use super::icons::FunnelIcon;
use dioxus::prelude::*;
use std::sync::Arc;

const BUTTON_CLASS: &str = "h-7 w-7 font-semibold text-primary-muted
            transition-colors select-none
            rounded-md flex justify-center items-center gap-1
            hover:bg-accent hover:text-primary-hover";

#[component]
pub fn ControlPanel(inventory: Signal<Inventory>) -> Element {
    let storage: Arc<dyn Storage> = use_context();

    let mut search_query = use_signal(|| String::new());

    rsx! {
        div { class: "flex items-center gap-2 p-1",
            // Left-aligned
            div { class: "flex items-center gap-2 flex-1 min-w-4",
                button {
                    class: "{BUTTON_CLASS} min-w-7 lg:w-20",
                    title: "Filter",
                    onclick: move |_| {},
                    FunnelIcon { class: "h-5 w-5" }
                    span { class: "hidden lg:inline", "Filter" }
                }

                // TODO: add styling to the search field
                input {
                    class: "px-2 py-1 text-sm text-primary bg-surface min-w-16",
                    class: "rounded-md shrink border-2 border-secondary-muted",
                    class: "focus:border-accent focus:outline-none",

                    placeholder: "Поиск...",
                    value: "{search_query}",
                    oninput: move |e| search_query.set(e.value()),
                }
            }

            // Right-aligned
            div { class: "flex items-center gap-2 shrink-0",
                button {
                    class: "{BUTTON_CLASS} md:w-14",
                    title: "New Tag",
                    onclick: move |_| {},
                    "#"
                    span { class: "hidden md:inline", "Tag" }
                }

                button {
                    class: "{BUTTON_CLASS} md:w-20",
                    title: "New folder",
                    onclick: move |_| {
                        inventory
                            .write()
                            .add_folder("New folder");
                    },
                    "🖿"
                    span { class: "hidden md:inline", "Folder" }
                }

                button {
                    class: "{BUTTON_CLASS} md:w-16",
                    title: "New item",
                    onclick: move |_| {
                        inventory
                            .write()
                            .add_item("Dummy Item", "", "", 0.0, None, vec![]);
                    },
                    "+"
                    span { class: "hidden md:inline", "Item" }
                }

                button {
                    class: "{BUTTON_CLASS} sm:w-24",
                    title: "Reverse",
                    onclick: {
                        let storage = storage.clone();
                        move |_| {
                            if let Ok(loaded) = storage.load() {
                                inventory.set(loaded);
                            }
                        }
                    },
                    "↶"
                    span { class: "hidden sm:inline", "Reverse" }
                }

                button {
                    class: "{BUTTON_CLASS} sm:w-16",
                    title: "Sync",
                    onclick: move |_| {
                        let _ = storage.save(&inventory());
                    },
                    "⟲"
                    span { class: "hidden sm:inline", "Sync" }
                }

                button {
                    class: "{BUTTON_CLASS} sm:w-20",
                    title: "Other",
                    onclick: move |_| {},
                    "⋯"
                    span { class: "hidden sm:inline", "Other" }
                }
            }
        }
    }
}
