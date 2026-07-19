// Popups
use uuid::Uuid;

pub type ItemId = Uuid;
pub type FolderId = Uuid;
pub type Coords = (f64, f64);

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum PopupType {
    #[default]
    Null,

    ItemCreate,
    ItemUpdate(ItemId),
    ItemDelete(ItemId),

    FolderCreate,
    FolderUpdate(FolderId, Coords),
    FolderDelete(FolderId),

    ItemAddTag(ItemId),

    ItemContextMenuFolders(ItemId),
    HeaderContextMenuFolder(FolderId, Coords),
}

// Notifications
use dioxus::prelude::*;
use std::time::Duration;

pub const MAX_NOTIFICATIONS: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NotificationType {
    Success,
    Error,
}

impl NotificationType {
    pub fn duration(&self) -> Duration {
        match self {
            NotificationType::Success => Duration::from_secs(5),
            NotificationType::Error => Duration::from_secs(5),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[fully_pub::fully_pub]
struct Notification {
    id: Uuid,
    message: String,
    notification_type: NotificationType,
}

impl Notification {
    pub fn new(message: impl Into<String>, notification_type: NotificationType) -> Self {
        Self {
            id: Uuid::new_v4(),
            message: message.into(),
            notification_type,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NotificationState {
    notifications: Signal<Vec<Notification>>,
}

impl NotificationState {
    pub fn notify(&mut self, message: impl Into<String>, notification_type: NotificationType) {
        let notification = Notification::new(message, notification_type);
        let id = notification.id;
        let duration = notification_type.duration();
        let mut notifications = self.notifications;

        notifications.push(notification);
        if notifications.len() > MAX_NOTIFICATIONS {
            notifications.remove(0);
        }

        gloo_timers::callback::Timeout::new(duration.as_millis() as u32, move || {
            notifications.write().retain(|n| n.id != id);
        })
        .forget();
    }

    pub fn send_success(&mut self, message: impl Into<String>) {
        self.notify(message, NotificationType::Success);
    }

    pub fn send_error(&mut self, message: impl Into<String>) {
        self.notify(message, NotificationType::Error);
    }

    pub fn dismiss(&mut self, id: Uuid) {
        self.notifications.write().retain(|n| n.id != id);
    }

    pub fn get_notifications(&self) -> Vec<Notification> {
        self.notifications.read().clone()
    }
}
