use crate::core::inventory::Inventory;
use dioxus::{document::eval, prelude::*};
use uuid::Uuid;

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
    let mut selected_folder = use_signal(|| None::<Uuid>);

    use_scroll_into_center(selected_folder, "folders-scroll", "folder-");

    rsx! {

        div {
            id: "folders-scroll",

            style: "scrollbar-width: none; -ms-overflow-style: none",
            class: "sticky top-0 z-10 bg-surface-accented",
            class: "flex items-center gap-2 p-2 overflow-x-auto scroll-smooth overscroll-contain",
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
            for folder in inventory.read().folders.iter() {
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
                            "{folder.name}"
                        }
                    }
                }
            }
        }

    }
}
