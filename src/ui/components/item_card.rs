use crate::core::models::{Folder, Item, Tag};
use dioxus::prelude::*;

#[component]
pub fn ItemCard(item: Item, tags: Vec<Tag>, folder: Option<Folder>) -> Element {
    let images = vec![
        asset!("/assets/images/tool-placeholder-2.png").to_string(),
        asset!("/assets/images/tool-placeholder.png").to_string(),
        asset!("/assets/images/tool-placeholder.png").to_string(),
    ];
    rsx! {
        div { class: "w-full aspect-[9/14] bg-surface-muted
            rounded-md shadow-[0_0_20px_rgba(0,0,0,0.3)]
            flex flex-col overflow-hidden",

            div { class: "h-[65%] w-full overflow-hidden select-none",
                ImageSlider { images }
            }

            div { class: "h-[35%] flex flex-col",
                ItemData { item }
                Tags { tags }
            }
        }
    }
}

#[component]
fn Tags(tags: Vec<Tag>) -> Element {
    rsx! {
        div { class: "flex px-0.5 pb-0.5 gap-0.5 mt-auto items-center",
            for tag in tags.iter().take(3) {
                span {
                    class: "text-[12px] px-1 py-0.25",
                    class: "rounded-xl truncate whitespace-nowrap flex-1 max-w-[10ch]",
                    style: "background-color: #{tag.color.to_hex()};",
                    "{tag.name}"
                }
            }
            if tags.len() > 3 {
                button {
                    class: "text-[12px] text-primary w-[18px] py-0.5 h-[18px]",
                    class: "bg-surface hover:text-primary-hover hover:bg-accent",
                    class: "flex items-center justify-center shrink-0 rounded-full",
                    "+{tags.len() - 3}"
                }
            }
        }
    }
}

#[component]
fn ItemData(item: Item) -> Element {
    rsx! {
        div { class: "px-2",
            h3 { class: "text-base font-semibold text-primary truncate", "{item.name}" }

            div { class: "flex items-center gap-2",
                p { class: "text-xs font-medium text-primary max-w-[7ch] overflow-hidden",
                    "${item.price:.2}"
                }
                p { class: "text-xs text-secondary truncate flex-1 text-right", "{item.location()}" }
            }

            p { class: "text-xs text-secondary line-clamp-2", "{item.description()}" }
        }
    }
}

#[derive(Clone, Copy)]
struct ImageSliderState {
    current_index: Signal<usize>,
    current_image: Memo<String>,
    touch_start_x: Signal<Option<f64>>,
}

impl ImageSliderState {
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

fn use_image_slider(images: Vec<String>) -> ImageSliderState {
    let current_index = use_signal(|| 0);
    let touch_start_x = use_signal(|| None);
    let current_image = use_memo(move || {
        images
            .get(current_index())
            .cloned()
            .unwrap_or_else(|| asset!("/assets/images/tool-placeholder.png").to_string())
    });

    ImageSliderState {
        current_index,
        current_image,
        touch_start_x,
    }
}

/// ImageSlider is a component that renders an of images and a representation of currently chosen one.
///
/// TODO:
/// 1) add skeleton-rendering for an image
/// 2) add an "item-type" icon in the top-left corner
/// 3) more interactive swipe photo-scrolling
#[component]
pub fn ImageSlider(images: Vec<String>) -> Element {
    let images_len = images.len();
    let mut state = use_image_slider(images.clone());

    rsx! {
        div { class: "w-full h-full flex flex-col",

            // Image with a swiping feature
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
            // slider controls
            div { class: "flex justify-between items-center px-2 py-0.5",
                button {
                    class: "text-sm text-primary font-semibold hover:text-accent-hover",
                    onclick: move |_| state.prev(images_len),
                    "<"
                }
                div { class: "flex gap-1",
                    for (index , _) in images.iter().enumerate() {
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
}
