use super::global::*;
use super::notifications::*;
use crate::core::inventory::Inventory;
use crate::core::models::Item;
use crate::ui::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn PopupWindow(inventory: Signal<Inventory>) -> Element {
    use super::types::PopupType::*;

    let app_state: AppState = use_context();
    let mut notifications = app_state.notifications;

    match app_state.get_popup() {
        Null => rsx! {},

        ItemContextMenuFolders(..) | HeaderContextMenuFolder(..) | FolderUpdate(..) => rsx! {
            Backdrop {}
        },

        ItemDelete(id) => {
            if let Some(item) = inventory.read().get_item(id) {
                let name = item.name();
                rsx! {
                    DeleteConfirmation {
                        name: item.name(),
                        callback: move |_| {
                            inventory.write().remove_item(id);
                            notifications
                                .send_success(format!("Deleted \"{name}\"!"));
                        },
                    }
                }
            } else {
                notifications.send_error(format!("Item not found. Cannot delete it."));
                rsx! {}
            }
        }

        FolderDelete(id) => {
            if let Some(folder) = inventory.read().get_folder(id) {
                let name = folder.name();
                rsx! {
                    DeleteConfirmation {
                        name: folder.name(),
                        callback: move |_| {
                            inventory.write().remove_folder(id);
                            notifications
                            .send_success(format!("Deleted \"{name}\"!"));
                        },
                    
                    }
                }
            } else {
                notifications.send_error(format!("Folder not found. Cannot delete it."));
                rsx! {}
            }
        }

        ItemUpdate(id) => {
            if let Some(item) = inventory().get_item(id) {
                rsx! {
                    ItemEditorPopup {
                        item: item.clone(),
                        callback: move |new_item: Item| {
                            let name = new_item.name();

                            inventory.write().change_item(id, new_item);
                            notifications
                            .send_success(format!("Edited \"{name}\"!"));
                        },
                    }
                }
            } else {
                notifications.send_error(format!("Item not found. Cannot edit it."));
                rsx! {}
            }
        }

        ItemCreate => {
            let dummy_item = Item::default();

            rsx! {
                ItemEditorPopup {
                    item: dummy_item.clone(),
                    callback: move |new_item: Item| {
                        let name = new_item.name();

                        inventory.write().add_item_struct(new_item);
                        notifications
                            .send_success(format!("Created a \"{name}\"!"));
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
    let app_state: AppState = use_context();
    let notifications = app_state.notifications.get_notifications();

    rsx! {
        div { class: "fixed bottom-4 right-4 z-30 w-[70%] max-w-xs flex flex-col gap-2",
            for notification in notifications.into_iter().rev() {
                NotificationComponent { notification }
            }
        }
    }
}
