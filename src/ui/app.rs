// src/ui/app.rs
use super::components::prelude::*;
use super::state::AppState;
use crate::core::inventory::Inventory;
use crate::core::storage::create_storage;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let storage = use_hook(|| create_storage().unwrap());
    let app_state = use_hook(|| AppState::new());

    let inventory = use_signal(|| storage.load().unwrap_or_default());

    use_context_provider(|| storage);
    use_context_provider(|| app_state);

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }

        div { class: "h-screen w-full bg-surface overflow-y-auto",
            Header { inventory }
            ControlPanelInventory { inventory }
            ItemGrid { inventory }
        }

        PopupWindow { inventory }

    }
}

#[component]
fn PopupWindow(inventory: Signal<Inventory>) -> Element {
    use crate::ui::state::PopupType::*;

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
                DeleteConfirmationPopup { name, callback }
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
                DeleteConfirmationPopup { name, callback }
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
