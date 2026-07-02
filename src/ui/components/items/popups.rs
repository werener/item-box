use dioxus::prelude::*;

#[component]
pub fn DeleteConfirmation(show_delete_confirmation: Signal<bool>, item_name: String, callback: Callback) -> Element {
    rsx! {
        if show_delete_confirmation() {
            div { class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm",
                div {
                    class: "flex flex-col bg-surface rounded-lg shadow-xl text-sm",
                    class: "w-[60%] max-w-sm w-full p-4 mx-4 gap-2",
                    onclick: move |e| e.stop_propagation(),

                    p { class: "text-secondary line-clamp-2",
                        "Delete "
                        span { class: "font-semibold text-primary", "{item_name}" }
                    }

                    div { class: "flex justify-end mt-2 gap-2",
                        button {
                            class: "px-2 py-1 text-sm text-primary-muted  bg-surface-accented rounded-md transition-colors",
                            class: "hover:text-primary",
                            onclick: move |_| show_delete_confirmation.set(false),
                            "Cancel"
                        }
                        button {
                            class: "px-2 py-1 bg-error-primary text-error-secondary rounded-md transition-colors",
                            class: "hover:bg-error-hover hover:text-error-hover",
                            onclick: move |_| {
                                callback.call(());
                                show_delete_confirmation.set(false);
                            },
                            "Delete"
                        }
                    }
                }
            }
        }
    }
}
