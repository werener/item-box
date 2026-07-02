// src/ui/components/items/control_panel.rs
use dioxus::prelude::*;
use uuid::Uuid;

use crate::core::{inventory::Inventory, models::Item};
use crate::ui::components::{
    icons::{PencilIcon, TrashIcon},
    items::popups::*,
};

#[derive(Clone, Copy, PartialEq, Debug)]
#[fully_pub::fully_pub]
struct ItemCardEvents {
    on_edit: Callback,
    on_delete: Callback,
    on_change_folder: Callback<Option<Uuid>>,
    on_add_tag: Callback<Uuid>,
    on_remove_tag: Callback<Uuid>,
}

const BUTTON_CLASS: &str = "w-5 h-5 text-primary-muted 
                    hover:text-accent-hover transition-colors 
                    flex items-center justify-center";

#[component]
pub fn ControlPanel(item: Item, inventory: Signal<Inventory>) -> Element {
    let item_id = item.id;
    let item_name = item.name.clone();
    let current_folder = item.folder;
    
    let mut show_delete_confirmation = use_signal(|| false);
    let mut show_folder_menu = use_signal(|| false);
    let mut show_tag_menu = use_signal(|| false);

    let events = ItemCardEvents {
        on_edit: Callback::new(move |_| {}),

        on_delete: Callback::new(move |_| {
            inventory.write().remove_item(item_id);
        }),

        on_change_folder: Callback::new(move |new_folder_id: Option<Uuid>| {
            inventory
                .write()
                .change_item_folder(item_id, new_folder_id);
        }),

        on_add_tag: Callback::new(move |tag_id: Uuid| {
            inventory.write().add_tag_to_item(item_id, tag_id);
        }),

        on_remove_tag: Callback::new(move |tag_id: Uuid| {
            inventory
                .write()
                .remove_tag_from_item(item_id, tag_id);
        }),
    };

    rsx! {
        div { class: "flex items-center justify-end gap-2 px-1 pb-1 bg-surface",
            button {
                class: "{BUTTON_CLASS}",
                title: "Move to folder",
                onclick: move |_| {
                    show_folder_menu.set(!show_folder_menu());
                    show_tag_menu.set(false);
                },
                "🖿"
            }
            button {
                class: "{BUTTON_CLASS}",
                title: "Add tag",
                onclick: move |_| {
                    show_tag_menu.set(!show_tag_menu());
                    show_folder_menu.set(false);
                },
                "#"
            }
            button {
                class: "{BUTTON_CLASS}",
                title: "Edit item",
                onclick: move |_| events.on_edit.call(()),
                PencilIcon {}
            }
            button {
                class: "{BUTTON_CLASS} hover:text-red-500",
                title: "Delete item",
                onclick: move |_| show_delete_confirmation.set(true),
                TrashIcon {}
            }
            DeleteConfirmation {
                show_delete_confirmation,
                item_name,
                callback: events.on_delete,
            }
        }

        FolderPopup {
            show_folder_menu,
            current_folder,
            folders: inventory.read().folders.clone(),
            callback: events.on_change_folder,
        }

        // // tag menu
        // if show_tag_menu() {
        //     div { class: "absolute bottom-10 right-2 z-30 bg-surface rounded-md shadow-lg border border-secondary-muted p-1 flex flex-col gap-1 max-h-40 overflow-y-auto min-w-[120px]",
        //         button {
        //             class: "px-2 py-1 text-xs text-left text-primary hover:bg-accent rounded truncate",
        //             class: if folder.is_none() { "bg-accent-hover font-semibold" } else { "" },
        //             onclick: move |_| {
        //                 events.on_change_folder.call(None);
        //                 show_folder_menu.set(false);
        //             },
        //             "None"
        //         }
        //     }
        // }
    }
}
