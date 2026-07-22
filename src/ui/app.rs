// src/ui/app.rs
use super::components::hooks;
use super::components::prelude::*;
use super::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let app_state = use_hook(|| AppState::new());
    use_context_provider(|| app_state);
    hooks::use_image_provider();
    let storage = hooks::use_storage();

    let inventory = use_signal(|| storage.load().unwrap_or_default());

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }

        div { class: "h-screen w-full bg-surface overflow-y-auto",
            Header { inventory }
            ItemGrid { inventory }
        }
        NotificationWindow {}
        PopupWindow { inventory }
    }
}
