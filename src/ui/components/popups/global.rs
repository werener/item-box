use crate::{
    core::models::Item,
    error::ItemError,
    ui::{
        components::hooks::autofocus_by_id, components::popups::types::PopupType, state::AppState,
    },
};
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

    autofocus_by_id("delete-confirmation-dialog");

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

type PopupError = Signal<Option<String>>;
struct EditorErrors {
    name_error: PopupError,
    price_error: PopupError,
}

#[component]
pub fn ItemEditorPopup(item: Item, callback: Callback<Item>) -> Element {
    let mut app_state: AppState = use_context();
    let is_new = if app_state.get_popup() == PopupType::ItemCreate {
        true
    } else {
        false
    };

    let mut item = use_signal(|| item);

    let mut popup_errors = EditorErrors {
        name_error: use_signal(|| None::<String>),
        price_error: use_signal(|| None::<String>),
    };

    let handle_save = Callback::new(move |_: ()| match item.read().is_valid() {
        Ok(_) => {
            callback.call(item.read().clone());
            app_state.close_popup();
        }
        Err(errors) => {
            for error in errors {
                match error {
                    ItemError::InvalidName => {
                        popup_errors.name_error.set(Some(error.to_string()));
                    }
                    ItemError::InvalidPrice => {
                        popup_errors.price_error.set(Some(error.to_string()))
                    }
                }
            }
        }
    });

    autofocus_by_id("submit-item");

    rsx! {
        div { class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm",
            div {
                class: "flex flex-col bg-surface rounded-lg shadow-xl w-[90%] max-w-md p-4 gap-4",
                onclick: move |e| e.stop_propagation(),
                id: "submit-item",
                tabindex: 0,
                onkeydown: move |e| {
                    match e.key() {
                        Key::Escape => {
                            app_state.close_popup();
                        }
                        _ => {}
                    }
                },
                h2 { class: "text-lg font-semibold text-primary",
                    if is_new {
                        "Create Item"
                    } else {
                        "Edit Item"
                    }
                }

                div {
                    class: "flex flex-col gap-3",
                    onsubmit: move |e| {
                        e.prevent_default();
                        handle_save.call(());
                    },

                    CustomLabel {
                        title: "Name",
                        value: item.read().name(),
                        oninput: move |e: Event<FormData>| item.write().name = e.value(),
                        error_signal: popup_errors.name_error,
                        placeholder: "Item name",
                        autofocus: true,
                    }

                    label { class: "flex flex-col gap-1 min-w-0",
                        span { class: "text-xs text-secondary", "Description" }

                        textarea {
                            class: "px-2 py-1.5 text-sm bg-surface resize-none h-20",
                            class: "border border-secondary-muted rounded",
                            class: "focus:border-accent focus:outline-none",

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

                    div { class: "flex gap-3",
                        CustomLabel {
                            title: "Price",
                            value: item.read().price.clone(),
                            oninput: move |e: Event<FormData>| {
                                item.write().price = e.value().parse::<f64>().unwrap_or(0.0);
                            },
                            error_signal: popup_errors.price_error,
                            r#type: "number",
                            step: "1",
                            min: "0",
                        }
                        CustomLabel {
                            title: "Location",
                            value: item.read().location(),
                            oninput: move |e: Event<FormData>| {
                                item.write().location = {
                                    let new_location = e.value();
                                    if new_location.is_empty() { None } else { Some(new_location) }
                                }
                            },
                            placeholder: "Item location",
                        }
                    }

                    div { class: "flex justify-end mt-2 gap-2",
                        button {
                            class: "px-3 py-1.5 rounded-md text-sm",
                            class: "bg-surface-accented-muted text-primary",
                            class: "hover:bg-accent hover:text-primary-hover transition-colors",

                            onclick: move |_| app_state.close_popup(),

                            "Cancel"
                        }
                        button {
                            class: "px-3 py-1.5 rounded-md text-sm",
                            class: "bg-surface-accented-muted text-primary",
                            class: "hover:bg-accent hover:text-primary-hover transition-colors",

                            onclick: move |_| handle_save(()),
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

#[component]
fn CustomLabel(
    title: String,
    value: String,
    oninput: Callback<Event<FormData>>,
    error_signal: Option<PopupError>,
    #[props(extends = input)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        label { class: "flex-1 flex flex-col gap-1 min-w-0",
            span { class: "text-xs text-secondary", "{title}" }

            input {
                class: "px-2 py-1.5 text-sm bg-surface",
                class: "border border-secondary-muted rounded",
                class: "focus:border-accent focus:outline-none",

                value,
                oninput: move |e: Event<FormData>| {
                    if let Some(mut signal) = error_signal {
                        signal.set(None);
                    }
                    oninput.call(e)
                },
                ..attributes,
            }
            if let Some(error) = error_signal {
                span { class: "text-xs text-error-primary",
                    {error.read().clone().unwrap_or_else(|| { "".to_string() })}
                }
            }
        }
    }
}
