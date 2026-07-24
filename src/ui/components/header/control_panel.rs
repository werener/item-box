// src/ui/components/control_panel.rs
use crate::core::{inventory::Inventory, storage::Storage};
use crate::ui::components::popups::types::PopupType::TagCreate;
use crate::ui::components::{icons::FunnelIcon, popups::types::PopupType};
use crate::ui::state::AppState;
use dioxus::prelude::*;
use std::sync::Arc;

#[component]
pub fn ControlPanelInventory(inventory: Signal<Inventory>) -> Element {
    let storage: Arc<dyn Storage> = use_context();
    let mut app_state: AppState = use_context();
    let mut search_query = use_signal(|| String::new());

    rsx! {
        div { class: "flex items-center gap-2 p-1 sticky top-0 z-10 bg-surface/80",
            // Left-aligned
            div { class: "flex items-center gap-2 flex-1",
                ControlButton {
                    icon: rsx! {
                        FunnelIcon { class: "h-5 w-5" }
                    },
                    text: "Filter",
                    title: "Filter",
                    min_screen_size: "lg",
                    onclick: move |_| {},
                }
                input {
                    class: "px-2 py-1 text-sm text-primary bg-surface/80 w-full",
                    class: "max-w-[29ch] sm:max-w-[32ch] md:max-w-[50ch] lg:max-w-[70%]",
                    class: "rounded-md shrink border-2 border-secondary-muted",
                    class: "focus:border-accent focus:outline-none",

                    placeholder: "Поиск...",
                    value: "{search_query}",
                    oninput: move |e| search_query.set(e.value()),
                }
            }

            // Right-aligned
            div { class: "flex items-center gap-2 lg:gap-5 shrink-0",
                ControlButton {
                    icon: rsx! { "#" },
                    text: "Tag",
                    title: "New Tag",
                    min_screen_size: "md",
                    onclick: move |_| {
                        app_state.toggle_popup(TagCreate);
                    },
                }

                ControlButton {
                    icon: rsx! { "🖿" },
                    text: "Folder",
                    title: "New Folder",
                    min_screen_size: "md",
                    onclick: move |_| {
                        inventory
                            .write()
                            .add_folder("New folder");
                    },
                }

                ControlButton {
                    icon: rsx! { "+" },
                    text: "Item",
                    title: "New item",
                    min_screen_size: "md",
                    onclick: move |_| app_state.open_popup(PopupType::ItemCreate),
                }

                ControlButton {
                    icon: rsx! { "↶" },
                    text: "Undo",
                    title: "Undo changes",
                    min_screen_size: "sm",
                    onclick: {
                        let storage = storage.clone();
                        move |_| {
                            if let Ok(loaded) = storage.load() {
                                inventory.set(loaded);
                            }
                        }
                    },
                }

                ControlButton {
                    icon: rsx! { "⟲" },
                    text: "Sync",
                    title: "Sync",
                    min_screen_size: "sm",
                    onclick: {
                        let storage = storage.clone();
                        move |_| {
                            let _ = storage.clone().save(&inventory());
                        }
                    },
                }

                ControlButton {
                    icon: rsx! { "⋯" },
                    text: "Other",
                    title: "Other",
                    min_screen_size: "sm",
                    onclick: |_| {},
                }
            }
        }
    }
}

#[component]
fn ControlButton(
    icon: Element,
    text: String,
    title: String,
    min_screen_size: String,
    onclick: Callback<Event<MouseData>>,
) -> Element {
    let full_button_size = text.len() + 1 + 2; // icon + text + padding
    let span_class = format!("hidden {}:inline", min_screen_size);

    rsx! {
        button {
            class: "h-[3ch] w-[{full_button_size}ch] px-1 min-w-7 font-semibold text-primary-muted rounded-md",
            class: "transition-colors select-none",
            class: "flex justify-center items-center gap-1",
            class: "hover:bg-accent hover:text-primary-hover",

            title: "{title}",
            onclick: move |e| onclick.call(e),

            {icon}
            span { class: "{span_class}", "{text}" }
        }
    }
}
