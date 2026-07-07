use crate::ui::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn PopupBackdrop() -> Element {
    let mut app_state: AppState = use_context();
    rsx! {
        div {
            class: "fixed inset-0 z-30",
            onclick: move |_| app_state.close_popup(),
            oncontextmenu: move |_| app_state.close_popup(),
        }
    }
}

#[component]
pub fn DeleteConfirmation(name: String, callback: Callback) -> Element {
    let mut app_state: AppState = use_context();
    const BUTTON_CLASS: &str = "px-2 py-1 text-sm rounded-md transition-colors";
    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm",
            onclick: move |_| app_state.close_popup(),
            div {
                class: "flex flex-col bg-surface rounded-lg shadow-xl text-sm",
                class: "w-[60%] max-w-sm w-full p-4 mx-4 gap-2",
                onclick: move |e| e.stop_propagation(),

                p { class: "text-secondary line-clamp-2",
                    "Confirm the deletion of "
                    span { class: "font-semibold text-primary", "{name}" }
                }

                div { class: "flex justify-end mt-2 gap-2",
                    button {
                        class: "{BUTTON_CLASS} text-primary-muted bg-surface-accented",
                        class: "hover:text-primary",
                        onclick: move |_| app_state.close_popup(),
                        "Cancel"
                    }
                    button {
                        class: "{BUTTON_CLASS} bg-error-primary text-error-secondary",
                        class: "hover:bg-error-primary-hover",
                        onclick: move |_| {
                            app_state.close_popup();
                            callback.call(());
                        },
                        "Delete"
                    }
                }
            }
        }
        PopupBackdrop {}
    }
}
