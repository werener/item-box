use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    core::models::Item, ui::components::{icons::{PencilIcon, TrashIcon}, items::popups::DeleteConfirmation},
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

#[component]
pub fn ControlPanel(events: ItemCardEvents, item: Signal<Item>) -> Element {
    let mut show_delete_confirmation = use_signal(|| false);
    let mut show_folder_menu = use_signal(|| false);
    let mut show_tag_menu = use_signal(|| false);

    rsx! {
        div { class: "flex items-center justify-end gap-2 px-1 pb-1 bg-surface",
            button {
                class: "w-5 h-5 text-primary-muted hover:text-accent-hover transition-colors flex items-center justify-center",
                title: "Move to folder",
                onclick: move |_| {
                    show_folder_menu.set(!show_folder_menu());
                    show_tag_menu.set(false);
                },
                "🖿"
            }
            button {
                class: "w-5 h-5 text-primary-muted hover:text-accent-hover transition-colors flex items-center justify-center",
                title: "Add tag",
                onclick: move |_| {
                    show_tag_menu.set(!show_tag_menu());
                    show_folder_menu.set(false);
                },
                "#"
            }
            button {
                class: "w-5 h-5 text-primary-muted hover:text-accent-hover transition-colors",
                title: "Edit item",
                onclick: move |_| events.on_edit.call(()),
                PencilIcon {}
            }
            button {
                class: "w-5 h-5 text-primary-muted hover:text-red-500 transition-colors",
                title: "Delete item",
                onclick: move |_| show_delete_confirmation.set(true),
                TrashIcon {}
            }
        }

        DeleteConfirmation {
            show_delete_confirmation,
            item_name: item.read().name.clone(),
            callback: events.on_delete,
        }
        // // folder menu
        // if show_folder_menu() {
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
