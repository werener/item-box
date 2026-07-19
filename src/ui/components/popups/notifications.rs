// src/ui/components/popups/notifications.rs

use super::types::*;
use crate::ui::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn NotificationItem(notification: Notification) -> Element {
    let mut app_state: AppState = use_context();
    let id = notification.id;

    let icon = match notification.notification_type {
        NotificationType::Success => "✓",
        NotificationType::Error => "✕",
    };

    rsx! {
        div {
            class: "w-full p-3 rounded-md shadow-lg border border-secondary-muted",
            class: "bg-surface-accented text-primary cursor-pointer",
            class: "hover:bg-surface-accented-muted transition-colors",
            onclick: move |_| app_state.notifications.dismiss(id),

            div { class: "flex items-center justify-left gap-2 min-w-0",
                span { class: "text-lg shrink-0", "{icon}" }
                span { class: "text-sm font-medium break-words text-center", "{notification.message}" }
            }
        }
    }
}


#[component]
pub fn Success(notification: Notification) -> Element {
    let mut app_state: AppState = use_context();
    let id = notification.id;

    rsx! {
        div {
            class: "w-full p-3 rounded-md shadow-lg border border-secondary-muted",
            class: "bg-surface-accented text-primary cursor-pointer",
            class: "hover:bg-surface-accented-muted transition-colors",
            onclick: move |_| app_state.notifications.dismiss(id),

            div { class: "flex items-center justify-left gap-2 min-w-0",
                span { class: "text-lg text-green-700 shrink-0", "✓" }
                span { class: "text-sm font-medium break-words text-center", "{notification.message}" }
            }
        }
    }
}