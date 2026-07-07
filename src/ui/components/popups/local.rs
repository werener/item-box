use crate::{
    core::models::Folder,
    ui::{components::popups::types::PopupType, state::AppState},
};
use dioxus::prelude::*;
use uuid::Uuid;

const POPUP_LAYER: &str = "z-50";

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
            class: "absolute bottom-full mb-1 {POPUP_LAYER}
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
pub fn ActionsWithFolder(folder_id: Uuid) -> Element {
    let mut app_state: AppState = use_context();
    let (x, y) = (app_state.popup_position)();

    const BUTTON_CLASS: &str = "px-2 py-1 text-xs text-left rounded truncate";
    rsx! {
        div {
            class: "fixed {POPUP_LAYER} bg-surface p-1 flex flex-col gap-1 min-w-[120px]
                rounded-md shadow-lg border border-secondary-muted",
            style: "top: {y}px; left: {x}px;",
            onclick: move |e| e.stop_propagation(),

            button {
                class: "{BUTTON_CLASS} text-primary hover:bg-accent",
                onclick: move |_| {
                    app_state.open_popup(folder_id, PopupType::FolderEdit);
                },
                "Rename"
            }
            button {
                class: "{BUTTON_CLASS} text-error-primary",
                class: "hover:bg-error-primary-muted hover:text-error-primary-hover",
                onclick: move |_| {
                    app_state.open_popup(folder_id, PopupType::FolderDelete);
                },
                "Delete"
            }
        }
    }
}

#[component]
pub fn RenameFolder(folder_id: Uuid, folder_name: String, callback: Callback<String>) -> Element {
    let mut app_state: AppState = use_context();
    let mut new_name = use_signal(|| folder_name);
    let (x, y) = (app_state.popup_position)();

    use_effect(move || {
        spawn(async move {
            let _ = document::eval(
                r#"
                    const input = document.getElementById('rename-folder-input');
                    if (input) {
                        input.focus();
                    }
                "#,
            )
            .await;
        });
    });
    rsx! {
        div {
            class: "fixed {POPUP_LAYER} flex items-center gap-1 p-1",
            class: "bg-surface rounded-md shadow-xl border border-secondary-muted",
            style: "top: {y}px; left: {x}px;",
            onclick: move |e| e.stop_propagation(),

            input {
                id: "rename-folder-input",
                class: "px-2 py-1 text-xs w-32",
                class: "rounded-md bg-surface border border-secondary-muted",
                class: "focus:border-accent focus:outline-none",
                value: "{new_name}",
                oninput: move |e| new_name.set(e.value()),
                onkeydown: move |e| {
                    match e.key() {
                        Key::Enter => {
                            callback.call(new_name());
                            app_state.close_popup();
                        }
                        Key::Escape => {
                            app_state.close_popup();
                        }
                        _ => {}
                    }
                },
            }

            button {
                class: "w-6 h-6 flex items-center justify-center rounded-md ",
                class: "text-accent-hover hover:bg-accent transition-colors",
                onclick: move |_| {
                    callback.call(new_name());
                    app_state.close_popup();
                },
                "✓"
            }
        }
    }
}
