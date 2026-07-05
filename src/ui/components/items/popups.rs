// src/ui/components/items/popups.rs
use dioxus::prelude::*;
use uuid::Uuid;

use crate::{core::models::Folder, ui::state::AppState};

#[component]
pub fn DeletePopup(item_name: String, callback: Callback) -> Element {
    let mut app_state: AppState = use_context();

    rsx! {

        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm",
            onclick: move |_| app_state.close_popup(),
            div {
                class: "flex flex-col bg-surface rounded-lg shadow-xl text-sm",
                class: "w-[60%] max-w-sm w-full p-4 mx-4 gap-2",
                onclick: move |e| e.stop_propagation(),

                p { class: "text-secondary line-clamp-2",
                    "Delete "
                    span { class: "font-semibold text-primary", "{item_name}" }
                }

                div { class: "flex justify-end mt-2 gap-2",
                    button {
                        class: "px-2 py-1 text-sm text-primary-muted  bg-surface-accented rounded-md transition-colors",
                        class: "hover:text-primary",
                        onclick: move |_| app_state.close_popup(),
                        "Cancel"
                    }
                    button {
                        class: "px-2 py-1 bg-error-primary text-error-secondary rounded-md transition-colors",
                        class: "hover:bg-error-hover hover:text-error-hover",
                        onclick: move |_| {
                            callback.call(());
                            app_state.close_popup();
                        },
                        "Delete"
                    }
                }
            }
        }
    }
}

#[component]
pub fn FolderPopup(
    folder: Option<Uuid>,
    folders: Vec<Folder>,
    callback: Callback<Option<Uuid>>,
) -> Element {
    const BUTTON_CLASS: &str =
        "px-2 py-1 text-xs min-h-4 text-left text-primary hover:bg-accent rounded truncate";

    let mut app_state: AppState = use_context();
    rsx! {

        div {
            class: "absolute bottom-full mb-1 z-50
                bg-surface rounded-md shadow-lg border border-secondary-muted 
                p-1 flex flex-col gap-1 max-h-40 overflow-y-auto min-w-[120px]",
            onclick: move |e| e.stop_propagation(),

            button {
                class: "{BUTTON_CLASS}",
                class: if folder.is_none() { "hover:bg-accent-hover bg-accent font-semibold text-primary" } else { "hover:bg-accent text-primary-muted" },
                onclick: move |_| {
                    callback.call(None);
                    app_state.close_popup();
                },
                "No folder"
            }

            for fd in folders {
                {
                    let folder_id = fd.id;
                    let folder_name = fd.name;

                    rsx! {
                        button {
                            class: "{BUTTON_CLASS}",
                            class: if folder == Some(folder_id) { "hover:bg-accent-hover bg-accent font-semibold text-primary" } else { "hover:bg-accent text-primary-muted" },
                            onclick: move |_| {
                                callback.call(Some(folder_id));
                                app_state.close_popup();
                            },
                            "{folder_name}"
                        }
                    }
                }
            }
        }
    }
}
