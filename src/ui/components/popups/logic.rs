use super::global::*;
use crate::core::inventory::Inventory;
use crate::ui::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn PopupWindow(inventory: Signal<Inventory>) -> Element {
    use super::types::PopupType::*;

    let app_state: AppState = use_context();

    match app_state.get_popup_type() {
        Null => rsx! {},

        ItemDelete => {
            let id = app_state.get_popup_source().unwrap_or_else(|| todo!());

            rsx!{
                if let Some(item) = inventory.read().get_item(id) {
                    DeleteConfirmation {
                        name: item.name.clone(),
                        callback: move |_| {
                            inventory.write().remove_item(id);
                        },
                    }
                    PopupBackdrop {}
                } else {
                    DeleteConfirmation {
                        name: String::from("?Unknown Item?"),
                        callback: move |_| {},
                    }
                    PopupBackdrop {}
                }
            }
        }

        FolderDelete => {
            let id = app_state.get_popup_source().unwrap_or_else(|| todo!());

            rsx!{
                if let Some(folder) = inventory.read().get_folder(id) {
                    DeleteConfirmation {
                        name: folder.name.clone(),
                        callback: move |_| {
                            inventory.write().remove_folder(id);
                        },
                    }
                    PopupBackdrop {}
                } else {
                    DeleteConfirmation {
                        name: String::from("?Unknown Item?"),
                        callback: move |_| {},
                    }
                    PopupBackdrop {}
                }
            }
        }

        ItemContextMenuFolders | HeaderContextMenuFolder => rsx! {
            PopupBackdrop {}
        },
        ItemAddTag => rsx! {},
        ItemEdit => rsx! {},
        FolderEdit => rsx! {},
    }
}

#[component]
fn PopupBackdrop() -> Element {
    let mut app_state: AppState = use_context();
    rsx! {
        div {
            class: "fixed inset-0 z-30",
            onclick: move |_| app_state.close_popup(),
            oncontextmenu: move |_| app_state.close_popup(),
        }
    }
}
