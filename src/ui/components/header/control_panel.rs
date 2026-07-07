// src/ui/components/control_panel.rs
use crate::core::inventory::Inventory;
use crate::core::storage::Storage;
use crate::ui::components::icons::FunnelIcon;
use dioxus::prelude::*;
use std::sync::Arc;

const BUTTON_CLASS: &str = "h-[3ch] w-[3ch] font-semibold text-primary-muted
            transition-colors select-none
            rounded-md flex justify-center items-center gap-1
            hover:bg-accent hover:text-primary-hover";

#[component]
pub fn ControlPanelInventory(inventory: Signal<Inventory>) -> Element {
    let storage: Arc<dyn Storage> = use_context();

    let mut search_query = use_signal(|| String::new());

    rsx! {
        div { class: "flex items-center gap-2 p-1 sticky top-0 z-10 bg-surface/80",
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
                    class: "px-2 py-1 text-sm text-primary bg-surface/80 w-full max-w-[29ch] sm:max-w-[32ch] md:max-w-[50ch] lg:max-w-[70%]",
                    class: "rounded-md shrink border-2 border-secondary-muted",
                    class: "focus:border-accent focus:outline-none",

                    placeholder: "Поиск...",
                    value: "{search_query}",
                    oninput: move |e| search_query.set(e.value()),
                }
            }

            // Right-aligned
            div { class: "flex items-center gap-2 lg:gap-5 shrink-0",
                button {
                    class: "{BUTTON_CLASS} md:w-[6ch]",
                    title: "New Tag",
                    onclick: move |_| {},
                    "#"
                    span { class: "hidden md:inline", "Tag" }
                }

                button {
                    class: "{BUTTON_CLASS} md:w-[9ch]",
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
                    class: "{BUTTON_CLASS} md:w-[7ch]",
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
                    class: "{BUTTON_CLASS} sm:w-[8ch]",
                    title: "Undo changes",
                    onclick: {
                        let storage = storage.clone();
                        move |_| {
                            if let Ok(loaded) = storage.load() {
                                inventory.set(loaded);
                            }
                        }
                    },
                    "↶"
                    span { class: "hidden sm:inline", "Undo" }
                }

                button {
                    class: "{BUTTON_CLASS} sm:w-[7ch]",
                    title: "Sync",
                    onclick: move |_| {
                        let _ = storage.save(&inventory());
                    },
                    "⟲"
                    span { class: "hidden sm:inline", "Sync" }
                }

                button {
                    class: "{BUTTON_CLASS} sm:w-[9ch]",
                    title: "Other",
                    onclick: move |_| {},
                    "⋯"
                    span { class: "hidden sm:inline", "Other" }
                }
            }
        }
    }
}
