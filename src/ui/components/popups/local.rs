use crate::{core::models::Folder, ui::state::AppState};
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn ChoiceOfFolder(
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
                    app_state.close_popup();
                    callback.call(None);
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
                                app_state.close_popup();
                                callback.call(Some(folder_id));
                            },
                            "{folder_name}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ActionsWithFolder(edit_callback: Callback, delete_callback: Callback) -> Element {
    let mut app_state: AppState = use_context();
    let (x, y) = (app_state.popup_position)();

    const BUTTON_CLASS: &str = "px-2 py-1 text-xs text-left rounded truncate";
    rsx! {
        div {
            class: "fixed z-50 bg-surface p-1 flex flex-col gap-1 min-w-[120px]
                rounded-md shadow-lg border border-secondary-muted",
            style: "top: {y}px; left: {x}px;",
            onclick: move |e| e.stop_propagation(),

            button {
                class: "{BUTTON_CLASS} text-primary hover:bg-accent",
                onclick: move |_| {
                    app_state.close_popup();
                    edit_callback.call(())
                },
                "Rename"
            }
            button {
                class: "{BUTTON_CLASS} text-error-primary",
                class: "hover:bg-error-primary-muted hover:text-error-primary-hover",
                onclick: move |_| {
                    app_state.close_popup();
                    delete_callback.call(())
                },
                "Delete"
            }
        }
    }
}
