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
use futures_timer::Delay;
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
    id: u64,
    message: String,
    notification_type: NotificationType,
}

impl Notification {
    pub fn new(message: impl Into<String>, notification_type: NotificationType) -> Self {
        Self {
            id: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
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
    fn notify(&mut self, message: impl Into<String>, notification_type: NotificationType) {
        let notification = Notification::new(message, notification_type);
        let duration = notification_type.duration();
        let id = notification.id;
        let mut notifications = self.notifications;
        
        spawn(async move {
            notifications.push(notification);
            if notifications.len() > MAX_NOTIFICATIONS {
                notifications.remove(0);
            }

            Delay::new(duration).await;
            notifications.write().retain(|n| n.id != id);
        });
    }

    pub fn send_success(&mut self, message: impl Into<String>) {
        self.notify(message, NotificationType::Success);
    }

    pub fn send_error(&mut self, message: impl Into<String>) {
        self.notify(message, NotificationType::Error);
    }

    pub fn dismiss(&mut self, id: u64) {
        self.notifications.write().retain(|n| n.id != id);
    }

    pub fn get_notifications(&self) -> Vec<Notification> {
        self.notifications.read().clone()
    }
}
