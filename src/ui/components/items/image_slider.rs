// src/ui/components/items/image_slider.rs
use crate::{
    core::{image::ImageProvider, inventory::Inventory},
    ui::components::popups::types::{ImageId, ItemId},
};
use dioxus::prelude::*;
use std::{ops::Range, sync::Arc};

const MAX_CIRCLES: usize = 9;

/// ImageSlider is a component that renders images and allows swiping through them.
#[component]
pub fn ImageSlider(item_id: ItemId, inventory: Signal<Inventory>) -> Element {
    let image_provider = use_context::<Resource<Option<Arc<dyn ImageProvider>>>>();

    let images_resource = use_resource(move || {
        let images = inventory
            .read()
            .get_item(item_id)
            .map(|i| i.images.clone())
            .unwrap_or_default();

        let provider = image_provider().flatten();

        resolve_images(images, provider)
    });
    let mut state = SliderState::new(images_resource);

    rsx! {
        div { class: "w-full h-full flex flex-col select-none",
            div {
                class: "flex-1 flex items-center justify-center overflow-hidden",
                ontouchstart: move |e: Event<TouchData>| {
                    if let Some(touch) = e.touches().first() {
                        state.touch_start_x.set(Some(touch.client_coordinates().x));
                    }
                },
                ontouchend: move |e: Event<TouchData>| {
                    if let Some(start_x) = state.touch_start_x.take() {
                        if let Some(touch) = e.touches_changed().first() {
                            state.handle_swipe(start_x, touch.client_coordinates().x);
                        }
                    }
                },
                onmousedown: move |e: MouseEvent| {
                    state.touch_start_x.set(Some(e.client_coordinates().x));
                },
                onmouseup: move |e: MouseEvent| {
                    if let Some(start_x) = state.touch_start_x.take() {
                        state.handle_swipe(start_x, e.client_coordinates().x);
                    }
                },
                img {
                    src: "{state.current_image}",
                    draggable: "false",
                    class: "w-full h-full object-cover object-center select-none",
                }
            }
            div { class: "flex justify-between items-center px-2 py-0.5",
                button {
                    class: "text-sm text-primary font-semibold hover:text-accent-hover",
                    onclick: move |_| state.prev(),
                    "<"
                }
                div { class: "flex gap-1",
                    for index in state.get_range() {
                        button {
                            class: "w-1.5 h-1.5 rounded-full",
                            class: "{circle_color(index, state)}",
                            onclick: move |_| state.current_index.set(index),
                        }
                    }
                }
                button {
                    class: "text-sm text-primary font-semibold hover:text-accent-hover",
                    onclick: move |_| state.next(),
                    ">"
                }
            }
        }
    }
}

fn circle_color(index: usize, state: SliderState) -> &'static str {
    match index {
        i if i == state.current_index() => "bg-accent-hover",
        i if (i == 0 || i == state.images_len() - 1) => "bg-primary-muted hover:bg-accent",
        _ => "bg-secondary hover:bg-accent",
    }
}

async fn resolve_images(
    images: Vec<ImageId>,
    provider: Option<Arc<dyn ImageProvider>>,
) -> Vec<String> {
    if images.is_empty() {
        return vec![asset!("/assets/images/tool-placeholder.png").to_string()];
    }

    // Clean early return if provider is not yet initialized
    let Some(provider) = provider else {
        return vec![asset!("/assets/images/tool-placeholder.png").to_string()];
    };

    let mut resolved_images = Vec::new();
    for image in images {
        if let Ok(Some(uri)) = provider.get_image_uri(image).await {
            resolved_images.push(uri);
        }
    }

    if resolved_images.is_empty() {
        resolved_images.push(asset!("/assets/images/tool-placeholder.png").to_string());
    }

    resolved_images
}

#[derive(Clone, Copy)]
struct SliderState {
    current_index: Signal<usize>,
    touch_start_x: Signal<Option<f64>>,
    current_image: Memo<String>,
    images_len: Memo<usize>,
}

impl SliderState {
    fn new(images_resource: Resource<Vec<String>>) -> Self {
        let mut current_index = use_signal(|| 0);
        let touch_start_x = use_signal(|| None::<f64>);

        let current_image = use_memo(move || {
            let imgs = images_resource()
                .unwrap_or_else(|| vec![asset!("/assets/images/tool-placeholder.png").to_string()]);
            let idx = current_index();
            imgs.get(idx)
                .cloned()
                .unwrap_or_else(|| asset!("/assets/images/tool-placeholder.png").to_string())
        });

        let images_len = use_memo(move || images_resource().map(|v| v.len()).unwrap_or(0));

        use_effect(move || {
            let len = images_len();
            let idx = current_index();

            if len > 0 && idx >= len {
                current_index.set(len - 1);
            } else if len == 0 {
                current_index.set(0);
            }
        });

        Self {
            current_index,
            touch_start_x,
            current_image,
            images_len,
        }
    }

    fn current_index(&self) -> usize {
        (self.current_index)()
    }
    fn images_len(&self) -> usize {
        (self.images_len)()
    }

    fn next(&mut self) {
        let len = self.images_len();
        let idx = self.current_index();
        if len > 0 {
            self.current_index.set((idx + 1) % len);
        }
    }

    fn prev(&mut self) {
        let len = self.images_len();
        let idx = self.current_index();
        if len > 0 {
            let new_index = if idx == 0 { len - 1 } else { idx - 1 };
            self.current_index.set(new_index);
        }
    }

    fn handle_swipe(&mut self, start_x: f64, end_x: f64) {
        let diff = end_x - start_x;
        if diff.abs() > 50.0 {
            if diff > 0.0 {
                self.prev();
            } else {
                self.next();
            }
        }
    }

    fn get_range(&self) -> Range<usize> {
        let len = self.images_len();

        let capacity = std::cmp::min(MAX_CIRCLES, len);
        let right = std::cmp::min(len, self.current_index().saturating_add(MAX_CIRCLES.div_ceil(2)).max(capacity));
        let left = right - capacity;
        left..right
    }
}
