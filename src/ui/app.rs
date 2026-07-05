// src/ui/app.rs
use super::components::prelude::*;
use super::state::AppState;
use crate::core::storage::create_storage;
use crate::ui::state::PopupType;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let storage = use_hook(|| create_storage().unwrap());
    let mut app_state = use_hook(|| AppState::new());

    let inventory = use_signal(|| storage.load().unwrap_or_default());

    use_context_provider(|| storage);
    use_context_provider(|| app_state);

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }

        div { class: "h-screen w-full bg-surface overflow-y-auto",
            Header { inventory }
            ControlPanelInventory { inventory }
            ItemGrid { inventory }
        }
        if app_state.get_popup_type() != PopupType::Null {
            div {
                class: "fixed inset-0 z-30",
                onclick: move |_| app_state.close_popup(),
                oncontextmenu: move |_| app_state.close_popup(),
            }
        }
        // if app_state.is_popup_open(item_id, PopupType::ItemDelete) {
        //     DeleteConfirmationPopup { delete_target_name: item_name, callback: events.on_delete }
        // }
    }
}
