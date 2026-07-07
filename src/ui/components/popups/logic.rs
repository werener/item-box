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

            let name: String;
            let callback: Callback;

            if let Some(item) = inventory.read().get_item(id) {
                name = item.name.clone();
                callback = Callback::new(move |_| {
                    inventory.write().remove_item(id);
                });
            } else {
                name = String::from("?Unknown Item?");
                callback = Callback::new(move |_| {});
            }

            rsx! {
                DeleteConfirmation { name, callback }
                PopupBackdrop {}
            }
        }

        FolderDelete => {
            let id = app_state.get_popup_source().unwrap_or_else(|| todo!());

            let name: String;
            let callback: Callback;

            if let Some(folder) = inventory.read().get_folder(id) {
                name = folder.name.clone();
                callback = Callback::new(move |_| {
                    inventory.write().remove_folder(id);
                });
            } else {
                name = String::from("?Unknown Item?");
                callback = Callback::new(move |_| {});
            }

            rsx! {
                DeleteConfirmation { name, callback }
                PopupBackdrop {}
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
