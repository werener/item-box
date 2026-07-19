// src/ui/state.rs
use dioxus::prelude::*;
use super::components::popups::types::*;

#[derive(Clone, Debug, Copy, Default)]
#[fully_pub::fully_pub]
struct AppState {
    selected_folder: Signal<Option<FolderId>>,

    active_popup: Signal<PopupType>,
    notifications: Signal<NotificationState>
}

#[fully_pub::fully_pub]
impl AppState {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn get_popup(&self) -> PopupType {
        (self.active_popup)()
    }

    /// Opens a popup, closing any other open popup
    fn open_popup(&mut self, popup_type: PopupType) {
        self.active_popup.set(popup_type);
    }

    /// Closes currently open popup
    fn close_popup(&mut self) {
        self.active_popup.set(PopupType::Null);
    }

    /// Toggles a popup: if it's already open for this item, close it; otherwise open it
    fn toggle_popup(&mut self, popup_type: PopupType) {
        if self.is_popup_open(popup_type) {
            self.close_popup();
        } else {
            self.open_popup(popup_type);
        }
    }

    /// Checks if a specific popup is currently open with a specific ID attached
    fn is_popup_open(&self, popup_type: PopupType) -> bool {
        (self.active_popup)() == popup_type
    }
}
