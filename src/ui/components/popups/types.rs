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
use futures_timer::Delay;

pub const MAX_NOTIFICATIONS: usize = 5;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NotificationType {
    Success,
    Error,
}

impl NotificationType {
    pub fn duration(&self) -> Duration {
        match self {
            NotificationType::Success => Duration::from_secs(3),
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

#[derive(Clone, Copy)]
pub struct NotificationState {
    notifications: Signal<Vec<Notification>>,
}

#[fully_pub::fully_pub]
impl NotificationState {
    fn new() -> Self {
        Self {
            notifications: Signal::new(Vec::new()),
        }
    }

    fn notify(&mut self, message: impl Into<String>, notification_type: NotificationType) {
        let notification = Notification::new(message, notification_type);
        let id = notification.id;
        let duration = notification_type.duration();

        let mut list = self.notifications;

        list.push(notification);

        // Remove oldest if over cap
        if list.len() > MAX_NOTIFICATIONS {
            list.remove(0);
        }

        // Auto-dismiss
        let mut notifications = self.notifications;
        spawn(async move {
            Delay::new(duration).await;
            notifications.write().retain(|n| n.id != id);
        });
    }

    fn success(&mut self, message: impl Into<String>) {
        self.notify(message, NotificationType::Success);
    }

    fn error(&mut self, message: impl Into<String>) {
        self.notify(message, NotificationType::Error);
    }

    fn dismiss(&mut self, id: u64) {
        self.notifications.write().retain(|n| n.id != id);
    }

    fn get_notifications(&self) -> Vec<Notification> {
        self.notifications.read().clone()
    }
}