// src/ui/components/items/control_panel.rs
use dioxus::prelude::*;

use crate::core::{inventory::Inventory, models::Item};
use crate::ui::components::{
    icons::{PencilIcon, TrashIcon},
    popups::prelude::*,
};
use crate::ui::state::AppState;

const BUTTON_CLASS: &str = "w-5 h-5 text-primary-muted font-semibold
                    hover:text-accent-hover transition-colors
                    flex items-center justify-center";

#[component]
pub fn ControlPanelItem(item: Item, inventory: Signal<Inventory>) -> Element {
    let mut app_state: AppState = use_context();

    let item_id = item.id;
    let folder = item.folder;

    let folder_name = folder
        .and_then(|id| (inventory()).get_folder(id).map(|f| f.name.clone()))
        .unwrap_or_else(|| "No Folder".to_string());

    rsx! {
        div {
            class: "flex items-center justify-between px-1 mt-1.5 select-none",
            class: "border-t-2 border-secondary-muted",
            div { class: "relative shrink-0",
                button {
                    class: "{BUTTON_CLASS} w-auto gap-1 px-1",
                    title: "Change folder",
                    onclick: move |_| app_state.toggle_popup(PopupType::ItemContextMenuFolders(item_id)),
                    "🖿"
                    span { class: "text-xs truncate pt-0.5 max-w-[10ch]", "{folder_name}" }
                }
                if app_state.is_popup_open(PopupType::ItemContextMenuFolders(item_id)) {
                    ChoiceOfFolder {
                        current_folder: folder,
                        folders: inventory.read().folders.clone(),
                        callback: move |new_folder_id: Option<FolderId>| {
                            inventory.write().change_item_folder(item_id, new_folder_id);
                            app_state.close_popup();
                        },
                    }
                }
            }

            div { class: "flex items-center gap-1",
                button {
                    class: "{BUTTON_CLASS}",
                    title: "Edit",
                    onclick: move |_| app_state.toggle_popup(PopupType::ItemUpdate(item_id)),
                    PencilIcon {}
                }

                button {
                    class: "{BUTTON_CLASS} hover:text-red-500",
                    title: "Delete",
                    onclick: move |_| app_state.toggle_popup(PopupType::ItemDelete(item_id)),
                    TrashIcon {}
                }
            
            }
        
        }
    }
}
