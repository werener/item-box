// src/ui/components/items/image_slider.rs
use crate::{
    core::{image::ImageProvider, inventory::Inventory},
    ui::components::popups::types::ItemId,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// ImageSlider is a component that renders images and allows swiping through them.
#[component]
pub fn ImageSlider(item_id: ItemId, inventory: Signal<Inventory>) -> Element {
    let image_provider: Signal<Option<Arc<dyn ImageProvider>>> = use_context();
    let images_resource = use_resource(move || {
        let images = inventory
            .read()
            .get_item(item_id)
            .map(|i| i.images.clone())
            .unwrap_or_default();
        let provider = image_provider.read().clone();

        async move {
            if images.is_empty() || provider.is_none() {
                return vec![asset!("/assets/images/tool-placeholder.png").to_string()];
            }

            let mut resolved_images = Vec::new();
            for image in images {
                if let Ok(Some(uri)) = provider.as_ref().unwrap().get_image_uri(image).await {
                    resolved_images.push(uri);
                }
            }

            if resolved_images.is_empty() {
                resolved_images.push(asset!("/assets/images/tool-placeholder.png").to_string());
            }

            resolved_images
        }
    });

    let mut current_index = use_signal(|| 0);
    let mut touch_start_x = use_signal(|| None::<f64>);

    use_effect(move || {
        let len = images_resource().map(|v| v.len()).unwrap_or(0);
        if len > 0 && current_index() >= len {
            current_index.set(len - 1);
        } else if len == 0 {
            current_index.set(0);
        }
    });

    let current_image = use_memo(move || {
        let imgs = images_resource().unwrap_or_else(|| {
            vec![asset!("/assets/images/tool-placeholder.png").to_string()]
        });
        let idx = current_index();
        imgs.get(idx)
            .cloned()
            .unwrap_or_else(|| asset!("/assets/images/tool-placeholder.png").to_string())
    });

    let images_len = use_memo(move || images_resource().map(|v| v.len()).unwrap_or(0));

    let mut next = move || {
        let len = images_len();
        if len > 0 {
            current_index.set((current_index() + 1) % len);
        }
    };

    let mut prev = move || {
        let len = images_len();
        if len > 0 {
            let new_index = if current_index() == 0 { len - 1 } else { current_index() - 1 };
            current_index.set(new_index);
        }
    };

    let mut handle_swipe = move |start_x: f64, end_x: f64| {
        let diff = end_x - start_x;
        if diff.abs() > 50.0 {
            if diff > 0.0 {
                prev();
            } else {
                next();
            }
        }
    };

    rsx! {
        div { class: "w-full h-full flex flex-col select-none",
            div {
                class: "flex-1 flex items-center justify-center overflow-hidden",
                ontouchstart: move |e: Event<TouchData>| {
                    if let Some(touch) = e.touches().first() {
                        touch_start_x.set(Some(touch.client_coordinates().x));
                    }
                },
                ontouchend: move |e: Event<TouchData>| {
                    if let Some(start_x) = touch_start_x.take() {
                        if let Some(touch) = e.touches_changed().first() {
                            handle_swipe(start_x, touch.client_coordinates().x);
                        }
                    }
                },
                onmousedown: move |e: MouseEvent| {
                    touch_start_x.set(Some(e.client_coordinates().x));
                },
                onmouseup: move |e: MouseEvent| {
                    if let Some(start_x) = touch_start_x.take() {
                        handle_swipe(start_x, e.client_coordinates().x);
                    }
                },
                img {
                    src: "{current_image}",
                    draggable: "false",
                    class: "w-full h-full object-cover object-center select-none",
                }
            }
            div { class: "flex justify-between items-center px-2 py-0.5",
                button {
                    class: "text-sm text-primary font-semibold hover:text-accent-hover",
                    onclick: move |_| prev(),
                    "<"
                }
                div { class: "flex gap-1",
                    for index in 0..images_len() {
                        button {
                            class: "w-1.5 h-1.5 rounded-full",
                            class: if index == current_index() { "bg-accent-hover" } else { "bg-secondary hover:bg-accent" },
                            onclick: move |_| current_index.set(index),
                        }
                    }
                }
                button {
                    class: "text-sm text-primary font-semibold hover:text-accent-hover",
                    onclick: move |_| next(),
                    ">"
                }
            }
        }
    }
}