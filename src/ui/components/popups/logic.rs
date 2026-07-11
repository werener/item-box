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

            rsx! {
                if let Some(item) = inventory.read().get_item(id) {
                    DeleteConfirmation {
                        name: item.name.clone(),
                        callback: move |_| {
                            inventory.write().remove_item(id);
                        },
                    }
                } else {
                    DeleteConfirmation {
                        name: String::from("?Unknown Item?"),
                        callback: move |_| {},
                    }
                }
            }
        }

        FolderDelete => {
            let id = app_state.get_popup_source().unwrap_or_else(|| todo!());

            rsx! {
                if let Some(folder) = inventory.read().get_folder(id) {
                    DeleteConfirmation {
                        name: folder.name.clone(),
                        callback: move |_| {
                            inventory.write().remove_folder(id);
                        },
                    }
                } else {
                    DeleteConfirmation {
                        name: String::from("?Unknown Item?"),
                        callback: move |_| {},
                    }
                }
            }
        }

        ItemContextMenuFolders | HeaderContextMenuFolder | FolderUpdate => rsx! {
            Backdrop {}
        },

        ItemUpdate => {
            let id = app_state.get_popup_source().unwrap_or_else(|| todo!());
            rsx! {
                if let Some(item) = inventory().get_item(id) {
                    ItemEditorPopup {
                        item: item.clone(),
                        callback: move |new_item| {
                            inventory.write().change_item(id, new_item);
                        },
                    }
                }
            }
        }

        ItemAddTag | ItemCreate => rsx! {},
    }
}
