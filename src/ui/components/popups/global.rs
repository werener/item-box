use std::sync::Arc;

use crate::{
    core::{
        image::ImageProvider,
        inventory::Inventory,
        models::{Color, Item, Tag},
    },
    error::ItemError,
    ui::{
        components::{
            hooks::autofocus_by_id,
            icons::TrashIcon,
            popups::{local::ChoiceOfFolder, types::PopupType},
        },
        state::AppState,
    },
};
use dioxus::prelude::*;
use uuid::Uuid;

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
fn CustomLabel(
    title: String,
    value: String,
    oninput: Callback<Event<FormData>>,
    error_signal: Option<ItemValidationError>,
    #[props(extends = input)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        label { class: "flex flex-col gap-1 min-w-0",
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
fn TagView(tag: Tag, onclick: Callback) -> Element {
    rsx! {
        div {
            key: "{tag.id}",
            class: "relative flex items-center gap-1 px-2 py-1",
            class: "rounded-xl text-xs font-medium cursor-pointer",
            class: "hover:ring-2 hover:ring-accent",

            style: "background-color: #{tag.bg_color}; color: #{tag.fg_color};",
            onclick: move |_| { onclick.call(()) },
            span { class: "truncate max-w-[15ch] drop-shadow-sm", "{tag.name}" }
        }
    }
}

/// Global popup to confirm deletion
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

/// Global popup for editing item info
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

                    // Body
                    div { class: "grid grid-cols-1 sm:grid-cols-[1fr_1fr] gap-4 p-4",

                        PhotoData { item }

                        div { class: "flex flex-col gap-1 min-w-0 sm:row-span-2",
                            TextData { item, popup_errors }
                            FolderData { item, inventory }
                        }

                        TagData { item, inventory }
                    
                    }

                    FooterItemEditor {}
                }
            }
        }
    }
}

#[component]
fn PhotoData(item: Signal<Item>) -> Element {
    rsx! {
        style {
        }
        div {
            class: "flex flex-row justify-between w-full gap-1",
            class: "p-2 rounded border border-secondary-muted",
            div {
                class: "flex flex-1 flex-wrap content-start hide-scrollbar overflow-y-auto",
                class: "gap-1 p-1 max-h-40 shrink-0",
                class: "rounded border-r border-secondary-muted",
                style: "scrollbar-width: none; -ms-overflow-style: none",

                if item.read().images.is_empty() {
                    div { class: "flex items-center justify-center w-full h-full min-h-[60px]",
                        span { class: "text-xs text-secondary-muted italic", "No photos attached" }
                    }
                } else {
                    for image_id in item.read().images.clone() {
                        PhotoThumbnail {
                            image_id,
                            on_remove: move |id: Uuid| {
                                item.write().images.retain(|x| *x != id);
                            },
                        }
                    }
                }
            }

            div { class: "flex flex-col gap-2 justify-center",
                ImageUploader { item }

                button {
                    r#type: "button",
                    class: "flex items-center justify-center min-h-4 aspect-square px-3 py-2",
                    class: "text-sm text-primary rounded-md",
                    class: "hover:text-error-primary-hover transition-colors",

                    class: if item.read().images.is_empty() { "pointer-events-none opacity-40" },
                    onclick: move |_| {
                        item.write().images.clear();
                    },
                    "x"
                }
            
            }
        }
    }
}
#[component]
fn PhotoThumbnail(image_id: Uuid, on_remove: Callback<Uuid>) -> Element {
    let image_provider = use_context::<Signal<Option<Arc<dyn ImageProvider>>>>();
    
    let uri_resource = use_resource(move || {
        let provider = image_provider.read().clone();
        async move {
            if let Some(provider) = provider {
                provider.get_image_uri(image_id).await.ok().flatten()
            } else {
                None
            }
        }
    });

    rsx! {
        div {
            key: "{image_id}",
            class: "group relative w-16 h-16 rounded overflow-hidden border border-secondary-muted shrink-0 bg-surface-accented-muted",

            if let Some(Some(actual_uri)) = uri_resource() {
                img {
                    src: "{actual_uri}",
                    class: "w-full h-full object-cover",
                    alt: "Item photo",
                }
            } else {
                div { class: "w-full h-full flex items-center justify-center text-xs text-secondary-muted",
                    "..."
                }
            }

            div { class: "absolute inset-0 bg-black/60 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity",
                button {
                    r#type: "button",
                    class: "text-white hover:text-error-primary transition-colors p-1 text-lg leading-none",
                    title: "Remove photo",
                    onclick: move |_| {
                        on_remove.call(image_id);
                    },
                    "×"
                }
            }
        }
    }
}
#[component]
fn ImageUploader(item: Signal<Item>) -> Element {
    let mut is_uploading = use_signal(|| false);
    let image_provider = use_context::<Signal<Option<Arc<dyn ImageProvider>>>>();

    let handle_file_change = move |e: Event<FormData>| {
        let files = e.files();

        if files.is_empty() {
            return;
        }

        is_uploading.set(true);

        spawn(async move {
            if let Some(provider) = image_provider() {
                for file_data in files {
                    if let Ok(bytes) = file_data.read_bytes().await {
                        if let Ok(new_image_id) = provider.save_image(&bytes).await {
                            item.write().images.push(new_image_id);
                        }
                    }
                }
            }
            is_uploading.set(false);
        });
    };

    rsx! {
        label {
            r#for: "photo-file-picker",
            class: "flex items-center justify-center cursor-pointer disabled:opacity-50",
            class: "flex items-center justify-center cursor-pointer disabled:opacity-50",
            class: "hover:text-accent-hover transition-colors",

            title: "Add photo",
            if is_uploading() {
                "..."
            } else {
                "+"
            }
        }

        input {
            id: "photo-file-picker",
            r#type: "file",
            multiple: true,
            accept: "image/*",
            class: "hidden",
            onchange: handle_file_change,
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
            name_error: use_signal(|| None),
            price_error: use_signal(|| None),
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
                class: "rounded border border-secondary-muted",
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
                class: "rounded border border-secondary-muted",
                class: "hover:font-semibold hover:border-accent hover:text-accent-hover transition-colors",

                title: "Choose folder",

                "🖿"
                span { class: "text-xs truncate pt-0.5", "{folder_name}" }
            }
        }
    }
}
#[component]
fn TagData(item: Signal<Item>, inventory: Signal<Inventory>) -> Element {
    let available_tags: Memo<Vec<Tag>> = use_memo(move || {
        inventory
            .read()
            .tags
            .iter()
            .filter(|t| !item.read().tags.clone().contains(&t.id))
            .cloned()
            .collect()
    });
    rsx! {
        div {
            class: "flex flex-col w-full p-2 gap-2",
            class: "rounded border border-secondary-muted",
            // Selected tags
            div { class: "flex flex-col min-w-0 gap-1",
                span { class: "text-xs font-semibold text-secondary", "Selected" }
                div { class: "flex flex-wrap gap-1 max-h-12 overflow-y-auto p-1",
                    if item.read().tags.is_empty() {
                        span { class: "text-xs text-secondary-muted italic px-1",
                            "Attach tags to this item"
                        }
                    } else {
                        for tag_id in item.read().tags.clone() {
                            if let Some(tag) = inventory.read().get_tag(tag_id) {
                                TagView {
                                    tag: tag.clone(),
                                    onclick: move |_| {
                                        item.write().tags.retain(|t| *t != tag_id);
                                    },
                                }
                            }
                        }
                    }
                }
            }

            // Available tags
            div { class: "flex flex-col min-w-0",
                span { class: "text-xs font-semibold text-secondary", "Available" }
                div { class: "flex flex-wrap gap-1 max-h-12 overflow-y-auto p-1",

                    if available_tags.read().is_empty() {
                        span { class: "text-xs text-secondary-muted italic px-1", "No available tags" }
                    } else {
                        for tag in available_tags.read().clone() {
                            TagView {
                                tag: tag.clone(),
                                onclick: move |_| {
                                    item.write().tags.push(tag.id);
                                },
                            }
                        }
                    }
                }
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

/// Global popup for managing tags
#[component]
pub fn TagEditorPopup(inventory: Signal<Inventory>, callback: Callback<Tag>) -> Element {
    let mut app_state: AppState = use_context();
    let mut popup_errors = TagValidationErrors::new();

    let mut context = TagContext::new();

    use_effect(move || {
        if let Some(tag) = context.selected_tag() {
            context.name.set(tag.name.clone());
            context.bg_color.set(tag.bg_color.clone());
            context.fg_color.set(tag.fg_color.clone());

            popup_errors.name_error.set(None);
            popup_errors.bg_color_error.set(None);
        } else {
            context.name.set(String::new());
            context.bg_color.set(Color::default_bg());
            context.fg_color.set(Color::default_fg())
        }
    });
    autofocus_by_id("popup-window-tags");

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm",
            onkeydown: move |e| {
                if e.key() == Key::Escape {
                    app_state.close_popup();
                }
            },
            div {
                id: "popup-window-tags",
                tabindex: 0,
                class: "flex flex-col bg-surface rounded-lg shadow-xl overflow-hidden w-[90%] max-w-sm p-4 gap-4",
                onclick: move |e| e.stop_propagation(),

                HeaderTagEditor {}

                // Body
                ListingTagEditor { context, inventory }
                FormTagEditor { context, inventory, popup_errors }

                FooterTagEditor {}
            }
        }
    }
}

type TagValidationError = Signal<Option<String>>;

#[derive(Clone, Copy, PartialEq)]
struct TagValidationErrors {
    name_error: TagValidationError,
    bg_color_error: TagValidationError,
    fg_color_error: TagValidationError,
}
impl TagValidationErrors {
    fn new() -> Self {
        TagValidationErrors {
            name_error: use_signal(|| None),
            bg_color_error: use_signal(|| None),
            fg_color_error: use_signal(|| None),
        }
    }
}

#[derive(Clone, PartialEq, Copy)]
struct TagContext {
    name: Signal<String>,
    bg_color: Signal<Color>,
    fg_color: Signal<Color>,
    selected_tag: Signal<Option<Tag>>,
}
impl TagContext {
    fn new() -> Self {
        Self {
            name: use_signal(|| String::new()),
            bg_color: use_signal(|| Color::default_bg()),
            fg_color: use_signal(|| Color::default_fg()),
            selected_tag: use_signal(|| Option::<Tag>::None),
        }
    }

    fn selected_tag(self) -> Option<Tag> {
        (self.selected_tag)()
    }
}

#[component]
fn HeaderTagEditor() -> Element {
    let mut app_state: AppState = use_context();
    rsx! {
        div { class: "flex items-center justify-between",
            h2 { class: "text-lg font-semibold text-primary", "Manage Tags" }
            button {
                r#type: "button",
                class: "p-1 text-lg text-secondary hover:text-accent-hover transition-colors",
                onclick: move |_| app_state.close_popup(),
                "×"
            }
        }
    }
}
#[component]
fn ListingTagEditor(context: TagContext, inventory: Signal<Inventory>) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2 border-b border-secondary-muted",
            div { class: "flex justify-between items-center",
                h3 { class: "text-sm font-semibold text-primary", "Available Tags" }
                button {
                    r#type: "button",
                    class: "px-2 py-1 text-xs rounded-md bg-accent text-primary hover:bg-accent-hover transition-colors",
                    onclick: move |_| {
                        let mut new_tag = Tag::default();
                        new_tag.name = "New tag".to_string();
                        inventory.write().add_tag_struct(new_tag.clone());
                        context.selected_tag.set(Some(new_tag));
                    },
                    "+ Add New"
                }
            }
            div { class: "flex flex-wrap gap-2 max-h-20 overflow-y-auto p-1",
                if inventory.read().tags.is_empty() {
                    span { class: "text-xs text-secondary-muted italic w-full text-center py-2",
                        "No tags yet. Create one below!"
                    }
                } else {
                    for tag in inventory.read().tags.clone() {
                        if Some(tag.id) == context.selected_tag().map(|t| t.id) {
                            div {
                                key: "{tag.id}",
                                class: "group relative flex items-center gap-1 px-2 py-1",
                                class: "rounded-xl text-xs font-medium cursor-pointer ",
                                class: "ring-2 ring-accent hover:ring-accent-hover",

                                style: "background-color: #{context.bg_color}; color: #{context.fg_color};",
                                onclick: move |_| { context.selected_tag.set(None) },
                                span { class: "truncate max-w-[15ch] drop-shadow-sm",
                                    "{context.name}"
                                }
                            }
                        } else {
                            TagView {
                                tag: tag.clone(),
                                onclick: move |_| { context.selected_tag.set(Some(tag.clone())) },
                            }
                        }
                    }
                }
            }
        }
    }
}
#[component]
fn FormTagEditor(
    context: TagContext,
    inventory: Signal<Inventory>,
    popup_errors: TagValidationErrors,
) -> Element {
    let mut handle_tag_save = move || {
        if let Some(tag) = context.selected_tag() {
            if !(context.name)().trim().is_empty() {
                inventory.write().update_tag(
                    tag.id,
                    Tag::new((context.name)(), (context.bg_color)(), (context.fg_color)()),
                );
                context.selected_tag.set(None);
            } else {
                popup_errors
                    .name_error
                    .set(Some("Name has to be non-empty".to_string()));
            }
        }
    };
    let mut handle_tag_delete = move || {
        inventory.write().remove_tag(
            context
                .selected_tag()
                .map_or_else(|| Uuid::default(), |t| t.id),
        );
        context.selected_tag.set(None);
    };

    rsx! {
        form {
            class: "flex flex-col gap-4",
            class: if context.selected_tag().is_none() { "pointer-events-none opacity-40" },
            onsubmit: move |e| {
                e.prevent_default();
                handle_tag_save()
            },

            // Name
            CustomLabel {
                title: "Name",
                value: context.name,
                oninput: move |e: Event<FormData>| {
                    context.name.set(e.value());
                    popup_errors.name_error.set(None)
                },
                error_signal: popup_errors.name_error,
                placeholder: "Tag name",
            }

            div { class: "flex flex-row justify-between",
                // Colors
                div {
                    class: "flex gap-8 px-2 py-1",
                    class: "rounded border border-secondary-muted",
                    ColorPicker {
                        title: "Background color",
                        color: context.bg_color,
                        error_signal: popup_errors.bg_color_error,
                    }
                    ColorPicker {
                        title: "Text color",
                        color: context.fg_color,
                        error_signal: popup_errors.fg_color_error,
                    }
                }
                // Buttons
                div {
                    class: "flex flex-col justify-center select-none items-end p-0.5",
                    class: "rounded border border-secondary-muted",
                    button {
                        r#type: "submit",
                        class: "w-8 h-8 flex items-center justify-center rounded-lg",
                        class: "text-green-500 hover:bg-green-500/10 transition-colors text-xl",
                        "✓"
                    }

                    button {
                        r#type: "button",
                        class: "w-8 h-8 flex items-center justify-center rounded-lg",
                        class: "text-error-primary hover:bg-error-primary/10 transition-colors",
                        onclick: move |_| { handle_tag_delete() },
                        TrashIcon { class: "w-4 h-4" }
                    }
                }
            }
        }
    }
}
#[component]
fn ColorPicker(title: String, color: Signal<Color>, error_signal: TagValidationError) -> Element {
    rsx! {
        label { class: "flex flex-col gap-1",
            span { class: "text-xs text-secondary", "{title}" }
            div { class: "flex items-center  gap-2",
                input {
                    r#type: "color",
                    class: "w-12 h-10 p-0.5 cursor-pointer bg-transparent",
                    class: "rounded border border-secondary-muted",

                    value: "#{color()}",
                    oninput: move |e| {
                        match Color::from_hex(&e.value()) {
                            Ok(c) => {
                                color.set(c);
                                error_signal.set(None);
                            }
                            Err(err) => error_signal.set(Some(err.to_string())),
                        }
                    },
                }
                span { class: "text-sm text-primary-muted font-mono", "{color}" }
            }
            if let Some(msg) = error_signal() {
                span { class: "text-xs text-error-primary", "{msg}" }
            }
        }
    }
}
#[component]
fn FooterTagEditor() -> Element {
    let mut app_state: AppState = use_context();
    rsx! {
        div { class: "flex justify-end gap-2 mt-2",
            button {
                class: "px-3 py-1.5 rounded-md text-sm bg-surface-accented-muted text-primary",
                class: "hover:bg-accent-hover hover:text-primary-hover transition-colors",
                onclick: move |_| app_state.close_popup(),
                "Cancel"
            }
            button {
                class: "px-3 py-1.5 rounded-md text-sm bg-accent text-primary",
                class: "hover:bg-accent-hover hover:text-primary-hover transition-colors",
                "Save"
            }
        }
    }
}
