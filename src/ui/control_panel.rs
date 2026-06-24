// ui/control_panel.rs
use crate::core::inventory::Inventory;
use crate::core::storage::Storage;
use dioxus::prelude::*;
use std::sync::Arc;

#[component]
pub fn ControlPanel(inventory: Signal<Inventory>) -> Element {
    let storage: Arc<dyn Storage> = use_context();

    let mut search_query = use_signal(|| String::new());

    rsx! {
        div { class: "flex items-center gap-2 p-1",
            // Left-aligned
            div { class: "flex items-center gap-2 flex-1 min-w-0",
                button {
                    class: "h-7 w-7 p-1.5 text-primary-muted font-semibold",
                    class: "rounded-md flex justify-center items-center gap-1",
                    class: "hover:bg-accent hover:text-primary-hover",
                    title: "Filter",

                    FunnelIcon { class: "h-7 w-7 text-inherit" }
                }

                // TODO: add styling to the search field
                input {
                    class: "px-2 py-1 text-sm text-primary bg-surface min-w-16 border-2 border-secondary-muted",
                    class: "rounded-md shrink",
                    class: "focus:border-accent focus:outline-none",
                    placeholder: "Поиск...",
                    value: "{search_query}",
                    oninput: move |e| search_query.set(e.value()),
                }
            }

            // Right-aligned
            div { class: "flex items-center gap-2 shrink-0",
                button {
                    class: "h-7 w-7 md:w-14 text-primary-muted font-semibold",
                    class: "rounded-md flex justify-center items-center gap-1",
                    class: "hover:bg-accent hover:text-primary-hover",
                    title: "New Tag",
                    "#"
                    span { class: "hidden md:inline", "Tag" }
                }

                button {
                    class: "h-7 w-7 md:w-20 text-primary-muted font-semibold",
                    class: "rounded-md flex justify-center items-center gap-1",
                    class: "hover:bg-accent hover:text-primary-hover",
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
                    class: "h-7 w-7 md:w-16 text-primary-muted ",
                    class: "rounded-md flex justify-center items-center gap-1 font-semibold",
                    class: "hover:bg-accent hover:text-primary-hover",
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
                    class: "h-7 w-7 sm:w-24 text-primary-muted font-semibold",
                    class: "rounded-md flex justify-center items-center gap-1",
                    class: "hover:bg-accent hover:text-primary-hover",
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
                    class: "h-7 w-7 sm:w-16 text-primary-muted font-semibold",
                    class: "rounded-md flex justify-center items-center gap-1",
                    class: "hover:bg-accent hover:text-primary-hover",
                    title: "Sync",
                    onclick: move |_| {
                        let _ = storage.save(&inventory());
                    },
                    "⟲"
                    span { class: "hidden sm:inline", "Sync" }
                }

                button {
                    class: "h-7 w-7 sm:w-20 text-primary-muted font-semibold",
                    class: "rounded-md flex justify-center items-center gap-1",
                    class: "hover:bg-accent hover:text-primary-hover",
                    title: "Other",
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
