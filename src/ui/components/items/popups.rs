// src/ui/components/items/popups.rs
use dioxus::prelude::*;
use uuid::Uuid;

use crate::core::models::Folder;

#[component]
pub fn DeleteConfirmation(
    show_delete_confirmation: Signal<bool>,
    item_name: String,
    callback: Callback,
) -> Element {
    rsx! {
        if show_delete_confirmation() {
            div { class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm",
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
                            onclick: move |_| show_delete_confirmation.set(false),
                            "Cancel"
                        }
                        button {
                            class: "px-2 py-1 bg-error-primary text-error-secondary rounded-md transition-colors",
                            class: "hover:bg-error-hover hover:text-error-hover",
                            onclick: move |_| {
                                callback.call(());
                                show_delete_confirmation.set(false);
                            },
                            "Delete"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn FolderPopup(
    show_folder_menu: Signal<bool>,
    current_folder: Option<Uuid>,
    folders: Vec<Folder>,
    callback: Callback<Option<Uuid>>,
) -> Element {
    rsx! {
        if show_folder_menu() {
            div {
                class: "absolute bottom-8 right-2 z-30 bg-surface rounded-md shadow-lg border border-secondary-muted p-1 flex flex-col gap-1 max-h-40 overflow-y-auto min-w-[120px]",
                onclick: move |e| e.stop_propagation(),

                button {
                    class: "px-2 py-1 text-xs text-left text-primary hover:bg-accent rounded truncate",
                    class: if current_folder.is_none() { "bg-accent-hover font-semibold" } else { "" },
                    onclick: move |_| {
                        callback.call(None);
                        show_folder_menu.set(false);
                    },
                    "None"
                }

                for folder in folders {
                    {
                        let folder_id = folder.id;
                        let folder_name = folder.name;

                        rsx! {
                            button {
                                class: "px-2 py-1 text-xs text-left text-primary hover:bg-accent rounded truncate",
                                class: if current_folder == Some(folder_id) { "bg-accent-hover font-semibold" } else { "" },
                                onclick: move |_| {
                                    callback.call(Some(folder_id));
                                    show_folder_menu.set(false);
                                },
                                "{folder_name}"
                            }
                        }
                    }
                }
            }
        }
    }
}
