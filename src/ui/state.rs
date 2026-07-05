// src/ui/state.rs
use dioxus::prelude::*;
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PopupType {
    ItemChangeFolder,
    ItemAddTag,
    ItemDelete,
    ItemEdit,
    HeaderContextMenuFolder,
}

#[derive(Clone, Debug, Copy, Default)]
#[fully_pub::fully_pub]
struct AppState {
    selected_folder: Signal<Option<uuid::Uuid>>,
    active_popup: Signal<Option<(Uuid, PopupType)>>,
    popup_position: Signal<(f64, f64)>,
}

#[fully_pub::fully_pub]
impl AppState {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Opens a popup for a specific item, closing any other open popup
    fn open_popup(&mut self, id: Uuid, popup_type: PopupType) {
        self.active_popup.set(Some((id, popup_type)));
    }

    /// Closes any open popup
    fn close_popup(&mut self) {
        self.active_popup.set(None);
    }

    /// Toggles a popup: if it's already open for this item, close it; otherwise open it
    fn toggle_popup(&mut self, id: Uuid, popup_type: PopupType) {
        if self.is_popup_open(id, popup_type) {
            self.close_popup();
        } else {
            self.open_popup(id, popup_type);
        }
    }

    /// Checks if a specific popup is currently open for a specific item
    fn is_popup_open(&self, id: Uuid, popup_type: PopupType) -> bool {
        (self.active_popup)() == Some((id, popup_type))
    }
}
