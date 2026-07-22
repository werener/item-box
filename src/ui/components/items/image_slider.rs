// src/ui/components/items/image_slider.rs
use crate::core::image::ImageProvider;
use dioxus::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq)]
struct ImageSliderState {
    current_index: Signal<usize>,
    current_image: Memo<String>,
    touch_start_x: Signal<Option<f64>>,
}

impl ImageSliderState {
    fn new(images: Vec<String>) -> Self {
        let current_index = use_signal(|| 0);
        let touch_start_x = use_signal(|| None);
        let current_image = use_memo(move || {
            images
                .get(current_index())
                .cloned()
                .unwrap_or_else(|| asset!("/assets/images/tool-placeholder.png").to_string())
        });

        Self {
            current_index,
            current_image,
            touch_start_x,
        }
    }

    fn next(&mut self, total_images: usize) {
        (self.current_index).set(((self.current_index)() + 1) % total_images);
    }

    fn prev(&mut self, total_images: usize) {
        let new_index = if (self.current_index)() == 0 {
            total_images - 1
        } else {
            (self.current_index)() - 1
        };
        (self.current_index).set(new_index);
    }

    fn handle_swipe(&mut self, start_x: f64, end_x: f64, total_images: usize) {
        let diff = end_x - start_x;

        if diff.abs() > 50.0 {
            if diff > 0.0 {
                self.prev(total_images);
            } else {
                self.next(total_images);
            }
        }
    }
}

/// ImageSlider is a component that renders images and allows to swipe thorough them.
#[component]
pub fn ImageSlider(images: Vec<Uuid>) -> Element {
    let image_provider: Arc<dyn ImageProvider> = use_context();
    let image_uris = use_signal(|| Vec::<String>::new());

    // Resolve image IDs to URIs asynchronously
    use_effect(move || {
        let provider = image_provider.clone();
        let ids = images.clone();
        let mut uris_signal = image_uris;

        spawn(async move {
            let mut uris = Vec::new();
            for id in ids {
                if let Ok(Some(uri)) = provider.get_image_uri(id).await {
                    uris.push(uri);
                }
            }
            // Fallback to placeholder if no images resolved
            if uris.is_empty() {
                uris.push(asset!("/assets/images/tool-placeholder.png").to_string());
            }
            uris_signal.set(uris);
        });
    });

    let images = image_uris.read().clone();
    let images_len = images.len();
    let state = ImageSliderState::new(images);

    rsx! {
        div { class: "w-full h-full flex flex-col select-none",
            Images { state, images_len }
            Controls { state, images_len }
        }
    }
}

#[component]
fn Images(state: ImageSliderState, images_len: usize) -> Element {
    rsx! {
        div {
            class: "flex-1 flex items-center justify-center overflow-hidden",

            // mobile implementation
            ontouchstart: move |e: Event<TouchData>| {
                if let Some(touch) = e.touches().first() {
                    state.touch_start_x.set(Some(touch.client_coordinates().x));
                }
            },
            ontouchend: move |e: Event<TouchData>| {
                if let Some(start_x) = (state.touch_start_x).take() {
                    if let Some(touch) = e.touches_changed().first() {
                        let end_x = touch.client_coordinates().x;
                        state.handle_swipe(start_x, end_x, images_len);
                    }
                }
            },

            // desktop implementation
            onmousedown: move |e: MouseEvent| {
                state.touch_start_x.set(Some(e.client_coordinates().x));
            },
            onmouseup: move |e: MouseEvent| {
                if let Some(start_x) = (state.touch_start_x).take() {
                    let end_x = e.client_coordinates().x;
                    state.handle_swipe(start_x, end_x, images_len);
                }
            },

            img {
                src: (state.current_image)(),
                draggable: "false",
                class: "w-full h-full object-cover object-center select-none",
            }
        }
    }
}

#[component]
fn Controls(state: ImageSliderState, images_len: usize) -> Element {
    rsx! {
        div { class: "flex justify-between items-center px-2 py-0.5",
            button {
                class: "text-sm text-primary font-semibold hover:text-accent-hover",
                onclick: move |_| state.prev(images_len),
                "<"
            }
            div { class: "flex gap-1",
                for index in (0..images_len) {
                    button {
                        class: "w-1.5 h-1.5 rounded-full ",
                        class: if index == (state.current_index)() { "bg-accent-hover" } else { "bg-secondary hover:bg-accent" },
                        onclick: move |_| state.current_index.set(index),
                    }
                }
            }
            button {
                class: "text-sm text-primary font-semibold hover:text-accent-hover",
                onclick: move |_| state.next(images_len),
                ">"
            }
        }
    }
}
