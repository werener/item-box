// src/ui/components/items/control_panel.rs
use dioxus::prelude::*;
use uuid::Uuid;

use crate::core::{inventory::Inventory, models::Item};
use crate::ui::components::{
    icons::{PencilIcon, TrashIcon},
    popups::prelude::*,
};
use crate::ui::state::AppState;

#[derive(Clone, Copy, PartialEq, Debug)]
#[fully_pub::fully_pub]
struct ItemCardEvents {
    on_edit: Callback,
    on_delete: Callback,
    on_change_folder: Callback<Option<Uuid>>,
    on_add_tag: Callback<Uuid>,
    on_remove_tag: Callback<Uuid>,
}

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
        .unwrap_or("No Folder".to_string());

    let events = ItemCardEvents {
        on_edit: Callback::new(move |_| {}),

        on_delete: Callback::new(move |_| {
            inventory.write().remove_item(item_id);
        }),

        on_change_folder: Callback::new(move |new_folder_id: Option<Uuid>| {
            inventory.write().change_item_folder(item_id, new_folder_id);
        }),

        on_add_tag: Callback::new(move |tag_id: Uuid| {
            inventory.write().add_tag_to_item(item_id, tag_id);
        }),

        on_remove_tag: Callback::new(move |tag_id: Uuid| {
            inventory.write().remove_tag_from_item(item_id, tag_id);
        }),
    };

    rsx! {
        div {
            class: "flex items-center justify-between px-1 mt-1.5 select-none",
            class: "border-t-2 border-secondary-muted",
            div { class: "relative shrink-0",
                button {
                    class: "{BUTTON_CLASS} w-auto gap-1 px-1",
                    title: "Change folder",
                    onclick: move |_| app_state.toggle_popup(item_id, PopupType::ItemContextMenuFolders),
                    "🖿"
                    span { class: "text-xs truncate pt-0.5 max-w-[10ch]", "{folder_name}" }
                }
                if app_state.is_popup_open(item_id, PopupType::ItemContextMenuFolders) {
                    ChoiceOfFolder {
                        folder,
                        folders: inventory.read().folders.clone(),
                        callback: events.on_change_folder,
                    }
                }
            }

            div { class: "flex items-center gap-1",
                button {
                    class: "{BUTTON_CLASS}",
                    title: "Tags",
                    onclick: move |_| app_state.toggle_popup(item_id, PopupType::ItemAddTag),
                    "#"
                }
                button {
                    class: "{BUTTON_CLASS}",
                    title: "Edit",
                    onclick: move |_| app_state.toggle_popup(item_id, PopupType::ItemEdit),
                    PencilIcon {}
                }

                button {
                    class: "{BUTTON_CLASS} hover:text-red-500",
                    title: "Delete",
                    onclick: move |_| app_state.toggle_popup(item_id, PopupType::ItemDelete),
                    TrashIcon {}
                }
            
            }
        
        }
    }
}
