// app.rs
use crate::core::storage::create_storage;
use crate::ui::control_panel::ControlPanel;
use crate::ui::item_grid::ItemGrid;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let storage = use_hook(|| create_storage().unwrap());
    let inventory = use_signal(|| storage.load().unwrap_or_default());

    use_context_provider(|| storage);

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }

        div { class: "min-h-screen w-full bg-surface-muted",
            ControlPanel { inventory }
            ItemGrid { inventory }
        }
    }
}