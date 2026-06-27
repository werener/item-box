// app.rs
use super::components::prelude::*;
use super::state::AppState;
use crate::core::storage::create_storage;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let storage = use_hook(|| create_storage().unwrap());
    let app_state = use_hook(|| AppState::new());

    let inventory = use_signal(|| storage.load().unwrap_or_default());

    let folders = inventory.read().folders.clone();

    use_context_provider(|| storage);
    use_context_provider(|| app_state);

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }

        div { class: "h-screen w-full bg-surface overflow-y-auto",
            Header { folders }
            ControlPanel { inventory }
            ItemGrid { inventory }
        }
    }
}
