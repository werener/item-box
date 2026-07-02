// src/ui/state.rs
use dioxus::prelude::*;

#[derive(Clone, Debug, Default)]
#[fully_pub::fully_pub]
struct AppState {
    selected_folder: Signal<Option<uuid::Uuid>>,
}

#[fully_pub::fully_pub]
impl AppState {
    fn new() -> Self {
        Self {
            selected_folder: Signal::new(None),
        }
    }
}
