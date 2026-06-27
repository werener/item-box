// app.rs
use crate::core::storage::create_storage;
use super::components::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let storage = use_hook(|| create_storage().unwrap());
    let inventory = use_signal(|| storage.load().unwrap_or_default());

    use_context_provider(|| storage);

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }

        div { class: "h-screen w-full bg-surface overflow-y-auto",
            Header { inventory }
            ControlPanel { inventory }
            ItemGrid { inventory }
        }
    }
}