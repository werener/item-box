use super::global::*;
use crate::core::inventory::Inventory;
use crate::core::models::Item;
use crate::ui::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn PopupWindow(inventory: Signal<Inventory>) -> Element {
    use super::types::PopupType::*;

    let app_state: AppState = use_context();

    match app_state.get_popup() {
        Null => rsx! {},

        ItemContextMenuFolders(..) | HeaderContextMenuFolder(..) | FolderUpdate(..) => rsx! {
            Backdrop {}
        },

        ItemDelete(id) => {
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

        FolderDelete(id) => {
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

        ItemUpdate(id) => {
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

        ItemCreate => {
            let dummy_item = Item::default();

            rsx! {
                ItemEditorPopup {
                    item: dummy_item.clone(),
                    callback: move |new_item| {
                        inventory.write().add_item_struct(new_item);
                    },
                }
            }
        }

        ItemAddTag(_) => rsx! {},

        FolderCreate => rsx! {},
    }
}

#[component]
pub fn NotificationWindow() -> Element {
    rsx! {}
}
