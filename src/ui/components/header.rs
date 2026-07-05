// src/ui/components/header.rs
use crate::{
    core::{
        inventory::{self, Inventory},
        models::Folder,
    },
    ui::state::{AppState, PopupType},
};
use dioxus::{document::eval, prelude::*};

/// Centers the container view around the chosen element.
pub fn use_scroll_into_center<T: ToString + Clone + 'static>(
    selected: Signal<Option<T>>,
    container_id: &'static str,
    element_prefix: &'static str,
) {
    use_effect(move || {
        if let Some(id) = selected() {
            let element_id = format!("{element_prefix}{}", id.to_string());
            spawn(async move {
                let _ = eval(&format!(
                    r#"
                    const btn = document.getElementById('{element_id}');
                    const container = document.getElementById('{container_id}');
                    if (!btn || !container) return;
                    
                    const targetScroll = btn.offsetLeft - container.offsetWidth / 2 + btn.offsetWidth / 2;
                    
                    container.scrollTo({{
                        left: targetScroll,
                        behavior: 'smooth'
                    }});
                    "#
                )).await;
            });
        }
    });
}

#[component]
pub fn Header(inventory: Signal<Inventory>) -> Element {
    let mut app_state: AppState = use_context();
    let mut selected_folder = app_state.selected_folder;

    let mut active_popup = app_state.active_popup;
    let folders = inventory.read().folders.clone();

    use_scroll_into_center(selected_folder, "folders-scroll", "folder-");

    rsx! {

        div {
            id: "folders-scroll",

            style: "scrollbar-width: none; -ms-overflow-style: none",
            class: "sticky top-0 z-10 bg-surface-accented",
            class: "flex items-center gap-2 p-2 overflow-x-auto overflow-y-none scroll-smooth overscroll-contain",
            onwheel: move |e: Event<WheelData>| {
                spawn(async move {
                    let delta_y = e.data().delta().strip_units().y;

                    let _ = document::eval(
                            &format!(
                                r#"
                                                        const container = document.getElementById('folders-scroll'); 
                                                        if (container) {{ 
                                                            container.scrollBy({{
                                                                left: {delta_y},
                                                                behavior: 'smooth'
                                                            }});
                                                        }}
                                                        "#,
                            ),
                        )
                        .await;
                });
            },
            for folder in folders.iter() {
                {
                    let folder_id = folder.id;
                    rsx! {
                        button {
                            key: "{folder_id}",
                            id: "folder-{folder_id}",
                            class: "px-3 py-1 rounded-lg text-sm shrink-0 font-semibold",
                            class: if selected_folder() == Some(folder_id) { "hover:bg-accent-hover bg-accent text-primary" } else { "hover:bg-accent bg-surface-accented-muted text-primary-muted" },
                            onclick: move |_| {
                                selected_folder
                                    .set(
                                        if selected_folder() == Some(folder_id) { None } else { Some(folder_id) },
                                    );
                            },
                            oncontextmenu: move |e| {
                                e.prevent_default();
                                let pos = e.client_coordinates();
                                active_popup
                                    .set(Some((folder_id, PopupType::HeaderContextMenuFolder)));
                                app_state.popup_position.set((pos.x - 30., pos.y + 30.));
                            },
                            "{folder.name}"
                        }
                    }
                }
            }
        }
        if let Some((folder_id, PopupType::HeaderContextMenuFolder)) = (active_popup)() {
            {
                let (x, y) = (app_state.popup_position)();
                let btnClass = "px-2 py-1 text-xs text-left rounded truncate";
                rsx! {
                    div {
                        class: "fixed z-50 bg-surface rounded-md shadow-lg border border-secondary-muted p-1 flex flex-col gap-1 min-w-[120px]",
                        style: "top: {y}px; left: {x}px;",

                        button { class: "{btnClass} text-primary hover:bg-accent", "Rename" }
                        button {
                            class: "{btnClass} text-error-primary",
                            class: "hover:bg-error-primary-muted hover:text-error-primary-hover",
                            onclick: move |_| {
                                inventory.write().remove_folder(folder_id);
                                app_state.close_popup();
                            },
                            "Delete"
                        }
                    }
                }
            }
        }
    }
}
