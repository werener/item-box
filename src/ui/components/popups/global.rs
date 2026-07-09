use crate::{core::models::Item, ui::state::AppState};
use dioxus::prelude::*;

#[component]
pub fn Backdrop() -> Element {
    let mut app_state: AppState = use_context();
    rsx! {
        div {
            class: "fixed inset-0 z-30",
            onclick: move |_| app_state.close_popup(),
            oncontextmenu: move |_| app_state.close_popup(),
        }
    }
}

#[component]
pub fn DeleteConfirmation(name: String, callback: Callback) -> Element {
    let mut app_state: AppState = use_context();
    const BUTTON_CLASS: &str = "px-2 py-1 text-sm rounded-md transition-colors";

    use_effect(move || {
        spawn(async move {
            let _ = document::eval(
                r#"
                const dialog = document.getElementById('delete-confirmation-dialog');
                if (dialog) {
                    dialog.focus();
                }
            "#,
            )
            .await;
        });
    });

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm",

            onclick: move |_| app_state.close_popup(),
            div {
                id: "delete-confirmation-dialog",
                tabindex: 0,
                onkeydown: move |e| {
                    match e.key() {
                        Key::Enter => {
                            app_state.close_popup();
                            callback.call(());
                        }
                        Key::Escape => {
                            app_state.close_popup();
                        }
                        _ => {}
                    }
                },
                class: "flex flex-col bg-surface rounded-lg shadow-xl text-sm",
                class: "w-[60%] max-w-sm w-full p-4 mx-4 gap-2",
                onclick: move |e| e.stop_propagation(),

                p { class: "text-secondary line-clamp-2",
                    "Confirm the deletion of "
                    span { class: "font-semibold text-primary", "{name}" }
                }

                div { class: "flex justify-end mt-2 gap-2",
                    button {
                        class: "{BUTTON_CLASS} text-primary-muted bg-surface-accented",
                        class: "hover:text-primary",
                        onclick: move |_| app_state.close_popup(),
                        "Cancel"
                    }
                    button {
                        class: "{BUTTON_CLASS} bg-error-primary text-error-secondary",
                        class: "hover:bg-error-primary-hover",
                        onclick: move |_| {
                            app_state.close_popup();
                            callback.call(());
                        },
                        "Delete"
                    }
                }
            }
        }
    }
}

const INPUT_CLASS: &str = "px-2 py-1.5 text-sm bg-surface-accented border border-secondary-muted rounded focus:border-accent focus:outline-none";

#[component]
pub fn ItemEditorPopup(item: Item, callback: Callback<Item>) -> Element {
    let mut app_state: AppState = use_context();

    let mut item = use_signal(|| item);

    let handle_save = Callback::new(move |_: ()| {
        callback.call(item.read().clone());
        app_state.close_popup();
    });

    let is_new = item.read().name().is_empty();

    rsx! {
        div { class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm",
            div {
                class: "flex flex-col bg-surface rounded-lg shadow-xl w-[90%] max-w-md p-4 gap-4",
                onclick: move |e| e.stop_propagation(),

                h2 { class: "text-lg font-semibold text-primary",
                    if is_new {
                        "Create Item"
                    } else {
                        "Edit Item"
                    }
                }

                form {
                    class: "flex flex-col gap-3",
                    onsubmit: move |e| {
                        e.prevent_default();
                        handle_save(());
                    },

                    // Name
                    label { class: "flex flex-col gap-1 min-w-0",
                        span { class: "text-xs text-secondary" }
                        "Name"
                        input {
                            class: "{INPUT_CLASS}",
                            value: "{item.read().name()}",
                            oninput: move |e| item.write().name = e.value(),
                            placeholder: "Item name",
                            autofocus: true,
                        }
                    }

                    // Description
                    label { class: "flex flex-col gap-1 min-w-0",
                        span { class: "text-xs text-secondary" }
                        "Description"
                        textarea {
                            class: "{INPUT_CLASS} resize-none h-20",
                            value: "{item.read().description()}",
                            oninput: move |e| {
                                item.write().description = {
                                    let new_descr = e.value();
                                    if new_descr.is_empty() { None } else { Some(new_descr) }
                                }
                            },
                            placeholder: "Optional description",
                        }
                    }

                    // Price & Location
                    div { class: "flex gap-3",
                        label { class: "flex-1 flex flex-col gap-1 min-w-0",
                            span { class: "text-xs text-secondary" }
                            "Price"
                            input {
                                class: "{INPUT_CLASS}",
                                value: "{item.read().price.clone()}",
                                oninput: move |e| item.write().price = e.value().parse::<f64>().unwrap_or(0.0),
                                r#type: "number",
                                step: "1",
                                min: "0",
                            }
                        }
                        label { class: "flex-1 flex flex-col gap-1 min-w-0",
                            span { class: "text-xs text-secondary" }
                            "Location"
                            input {
                                class: "{INPUT_CLASS}",
                                value: "{item.read().location()}",
                                oninput: move |e| {
                                    item.write().location = {
                                        let new_location = e.value();
                                        if new_location.is_empty() { None } else { Some(new_location) }
                                    }
                                },
                                placeholder: "Where the item is located",
                            }
                        }
                    }

                    // Buttons
                    div { class: "flex justify-end mt-2 gap-2",
                        button {
                            r#type: "button",
                            class: "px-3 py-1.5 text-sm rounded-md bg-surface-accented text-primary-muted hover:text-primary transition-colors",
                            onclick: move |_| app_state.close_popup(),
                            onkeydown: move |e| {
                                if e.key() == Key::Escape {
                                    app_state.close_popup()
                                }
                            },
                            "Cancel"
                        }
                        button {
                            r#type: "submit",
                            class: "px-3 py-1.5 text-sm rounded-md bg-accent text-primary hover:bg-accent-hover transition-colors",
                            onkeydown: move |e| {
                                if e.key() == Key::Enter {
                                    handle_save(());
                                }
                            },
                            "Save"
                        }
                    }
                }
            }
        }
    }
}
