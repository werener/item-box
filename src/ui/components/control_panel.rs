// ui/control_panel.rs
use crate::core::inventory::Inventory;
use crate::core::storage::Storage;
use dioxus::prelude::*;
use std::sync::Arc;

const BUTTON_CLASS: &str = "h-7 w-7 font-semibold text-primary-muted
            rounded-md flex justify-center items-center gap-1
            hover:bg-accent hover:text-primary-hover";

#[component]
pub fn ControlPanel(inventory: Signal<Inventory>) -> Element {
    let storage: Arc<dyn Storage> = use_context();

    let mut search_query = use_signal(|| String::new());

    rsx! {
        div { class: "flex items-center gap-2 p-1",
            // Left-aligned
            div { class: "flex items-center gap-2 flex-1 min-w-0",
                button {
                    class: "{BUTTON_CLASS} lg:w-20",
                    title: "Filter",
                    onclick: move |_| {},
                    FunnelIcon { class: "h-4 w-4 text-inherit" }
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

/// Custom icon of a filter/funnel (8x8).
#[component]
pub fn FunnelIcon(#[props(extends = SvgAttributes)] attributes: Vec<Attribute>) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 8 8",
            width: "100%",
            height: "100%",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            ..attributes,
            polygon { points: "7.33 1 0.67 1 3.33 4.15 3.33 6.33 4.67 7 4.67 4.15 7.33 1" }
        }
    }
}
