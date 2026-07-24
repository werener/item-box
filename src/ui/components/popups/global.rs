use crate::{
    core::{
        inventory::Inventory,
        models::{Color, Item, Tag},
    },
    error::ItemError,
    ui::{
        components::{
            hooks::autofocus_by_id,
            popups::{local::ChoiceOfFolder, types::PopupType},
        },
        state::AppState,
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

#[component]
pub fn ItemEditorPopup(
    item: Item,
    inventory: Signal<Inventory>,
    callback: Callback<Item>,
) -> Element {
    let mut app_state: AppState = use_context();

    let item = use_signal(|| item);

    let mut popup_errors = ItemValidationErrors::new();

    let handle_save = Callback::new(move |_: ()| match item.read().is_valid() {
        Ok(_) => {
            callback.call(item.read().clone());
            app_state.close_popup();
        }
        Err(errors) => popup_errors.consume_errors(errors),
    });

    autofocus_by_id("Name-input");

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm",
            onkeydown: move |e| {
                if e.key() == Key::Escape {
                    app_state.close_popup();
                }
            },

            div {
                class: "flex flex-col bg-surface rounded-lg shadow-xl overflow-hidden",
                class: "w-[90%] max-w-sm sm:max-w-2xl max-h-[90vh]",
                onclick: move |e| e.stop_propagation(),

                form {
                    class: "flex flex-col flex-1 overflow-y-auto",
                    onsubmit: move |e| {
                        e.prevent_default();
                        handle_save.call(());
                    },

                    HeaderItemEditor {}

                    div { class: "grid grid-cols-1 sm:grid-cols-[1fr_1fr] gap-4 p-4",

                        div {
                            class: "flex items-center justify-center w-full aspect-video",
                            class: "bg-surface-accented-muted border-2 border-dashed border-secondary-muted rounded-lg",
                            span { class: "text-secondary text-sm", "Photo area" }
                        }

                        div { class: "flex flex-col gap-1 min-w-0 sm:row-span-2",
                            TextData { item, popup_errors }
                            FolderData { item, inventory }
                        }

                        div {
                            class: "flex items-center justify-center w-full min-h-[80px]",
                            class: "bg-surface-accented-muted border-2 border-dashed border-secondary-muted rounded-lg",
                            span { class: "text-secondary text-sm", "Tags area (dummy)" }
                        }
                    }

                    FooterItemEditor {}
                }
            }
        }
    }
}

type ItemValidationError = Signal<Option<String>>;
#[derive(Clone, Copy, PartialEq)]
struct ItemValidationErrors {
    name_error: ItemValidationError,
    price_error: ItemValidationError,
}
impl ItemValidationErrors {
    fn new() -> Self {
        ItemValidationErrors {
            name_error: use_signal(|| None::<String>),
            price_error: use_signal(|| None::<String>),
        }
    }

    /// Writes provided errors to the state.
    fn consume_errors(&mut self, errors: Vec<ItemError>) {
        for error in errors {
            match error {
                ItemError::InvalidName => {
                    self.name_error.set(Some(error.to_string()));
                }
                ItemError::InvalidPrice => self.price_error.set(Some(error.to_string())),
            }
        }
    }
}

#[component]
fn CustomLabel(
    title: String,
    value: String,
    oninput: Callback<Event<FormData>>,
    error_signal: Option<ItemValidationError>,
    #[props(extends = input)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        label { class: "flex-1 flex flex-col gap-1 min-w-0",
            span { class: "text-xs text-secondary", "{title}" }

            input {
                id: "{title}-input",
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

#[component]
fn HeaderItemEditor() -> Element {
    let mut app_state: AppState = use_context();

    rsx! {
        div {
            class: "flex items-center justify-between",
            class: "sticky top-0 z-10 p-4",
            class: "bg-surface/60 border-b border-secondary-muted/60 backdrop-blur-sm",
            h2 { class: "text-lg font-semibold text-primary",
                if app_state.get_popup() == PopupType::ItemCreate {
                    "Create Item"
                } else {
                    "Edit Item"
                }
            }
            button {
                r#type: "button",
                class: "p-1 text-secondary hover:text-accent transition-colors",
                onclick: move |_| app_state.close_popup(),
                title: "Close",
                "×"
            }
        }
    }
}

#[component]
fn TextData(item: Signal<Item>, popup_errors: ItemValidationErrors) -> Element {
    rsx! {
        CustomLabel {
            title: "Name",
            value: item.read().name(),
            oninput: move |e: Event<FormData>| item.write().name = e.value(),
            error_signal: popup_errors.name_error,
            placeholder: "Item name",
        }

        label { class: "flex flex-col gap-2 min-w-0",
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
                step: "0.01",
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
    }
}

#[component]
fn FolderData(item: Signal<Item>, inventory: Signal<Inventory>) -> Element {
    let mut selecting_folder = use_signal(|| false);
    let folder_name = item
        .read()
        .folder
        .and_then(|id| inventory.read().get_folder(id).map(|f| f.name.clone()))
        .unwrap_or_else(|| "Choose folder".to_string());

    rsx! {
        span { class: "text-xs text-secondary", "Folder" }
        div { class: "relative shrink-0",
            if selecting_folder() {
                ChoiceOfFolder {
                    current_folder: item.read().folder,
                    folders: inventory.read().folders.clone(),
                    callback: move |new_folder| {
                        item.write().folder = new_folder;
                        selecting_folder.set(false)
                    },
                }
                div {
                    class: "fixed inset-0 z-30",
                    onclick: move |_| selecting_folder.set(false),
                    oncontextmenu: move |_| selecting_folder.set(false),
                }
            }
            button {
                r#type: "button",
                onclick: move |_| selecting_folder.set(true),
                class: "flex items-center gap-1 px-1",
                class: "w-[40%] h-8 text-primary-muted text-xl",
                class: "border border-secondary-muted rounded",
                class: "hover:font-semibold hover:border-accent hover:text-accent-hover transition-colors",

                title: "Choose folder",

                "🖿"
                span { class: "text-xs truncate pt-0.5", "{folder_name}" }
            }
        }
    }
}

#[component]
fn FooterItemEditor() -> Element {
    let mut app_state: AppState = use_context();

    rsx! {
        div {
            class: "flex justify-end gap-2 p-4",
            class: "sticky bottom-0 z-10",
            class: "bg-surface/70 border-t border-secondary-muted/70 backdrop-blur-sm",

            button {
                r#type: "button",
                class: "px-3 py-1.5 rounded-md text-sm",
                class: "bg-surface-accented-muted text-primary",
                class: "hover:bg-accent-hover hover:text-primary-hover transition-colors",
                onclick: move |_| app_state.close_popup(),
                "Cancel"
            }
            button {
                r#type: "submit",
                class: "px-3 py-1.5 rounded-md text-sm",
                class: "bg-accent text-primary",
                class: "hover:bg-accent-hover hover:text-primary-hover transition-colors",
                "Save"
            }
        }
    }
}

type TagValidationError = Signal<Option<String>>;
#[derive(Clone, Copy, PartialEq)]
struct TagValidationErrors {
    name_error: TagValidationError,
    color_error: TagValidationError,
}
impl TagValidationErrors {
    fn new() -> Self {
        TagValidationErrors {
            name_error: use_signal(|| None::<String>),
            color_error: use_signal(|| None::<String>),
        }
    }
}

#[component]
pub fn TagEditorPopup(callback: Callback<Tag>) -> Element {
    let mut app_state: AppState = use_context();
    let mut popup_errors = TagValidationErrors::new();

    let mut name = use_signal(|| String::new());
    let mut color = use_signal(|| Color::default());

    autofocus_by_id("tag-name-input");

    let mut handle_save = move |_: ()| {
        if !name().trim().is_empty() {
            callback.call(Tag::new(name(), color()));
            app_state.close_popup();
        } else {
            popup_errors
                .name_error
                .set(Some("Name has to be non-empty".to_string()));
        }
    };

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm",
            onkeydown: move |e| {
                if e.key() == Key::Escape {
                    app_state.close_popup();
                }
            },
            div {
                class: "flex flex-col bg-surface rounded-lg shadow-xl overflow-hidden w-[90%] max-w-sm p-4 gap-4",
                onclick: move |e| e.stop_propagation(),

                // Header
                div { class: "flex items-center justify-between",
                    h2 { class: "text-lg font-semibold text-primary", "Create Tag" }
                    button {
                        r#type: "button",
                        class: "p-1 text-secondary hover:text-accent transition-colors",
                        onclick: move |_| app_state.close_popup(),
                        "×"
                    }
                }

                // Form
                form {
                    class: "flex flex-col gap-4",
                    onsubmit: move |e| {
                        e.prevent_default();
                        handle_save(());
                    },

                    // Name Input
                    label { class: "flex flex-col gap-1",
                        span { class: "text-xs text-secondary", "Tag Name" }
                        input {
                            id: "tag-name-input",
                            class: "px-2 py-1.5 text-sm bg-surface border border-secondary-muted rounded focus:border-accent focus:outline-none",
                            value: "{name}",
                            oninput: move |e| {
                                name.set(e.value());
                                popup_errors.name_error.set(None)
                            },
                            placeholder: "Enter tag name",
                        }
                        if let Some(msg) = (popup_errors.name_error)() {
                            span { class: "text-xs text-error-primary", "{msg}" }
                        }
                    }

                    // Color Picker
                    label { class: "flex flex-col gap-1",
                        span { class: "text-xs text-secondary", "Tag Color" }
                        div { class: "flex items-center gap-3",
                            input {
                                r#type: "color",
                                class: "w-12 h-10 p-0.5 border border-secondary-muted rounded cursor-pointer bg-transparent",
                                value: "#{color().to_hex()}",
                                oninput: move |e| {
                                    match Color::from_hex(&e.value()) {
                                        Ok(c) => {
                                            color.set(c);
                                            popup_errors.color_error.set(None);
                                        }
                                        Err(err) => popup_errors.color_error.set(Some(err.to_string())),
                                    }
                                },
                            }
                            span { class: "text-sm text-primary-muted font-mono", "{color}" }
                        }
                        if let Some(msg) = (popup_errors.color_error)() {
                            span { class: "text-xs text-error-primary", "{msg}" }
                        }
                    }

                    // Footer Buttons
                    div { class: "flex justify-end gap-2 mt-2",
                        button {
                            r#type: "button",
                            class: "px-3 py-1.5 rounded-md text-sm bg-surface-accented-muted text-primary hover:bg-accent-hover hover:text-primary-hover transition-colors",
                            onclick: move |_| app_state.close_popup(),
                            "Cancel"
                        }
                        button {
                            r#type: "submit",
                            class: "px-3 py-1.5 rounded-md text-sm bg-accent text-primary hover:bg-accent-hover hover:text-primary-hover transition-colors",
                            "Create"
                        }
                    }
                }
            }
        }
    }
}
