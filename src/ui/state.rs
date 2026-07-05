// src/ui/state.rs
use dioxus::prelude::*;
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum PopupType {
    #[default]
    Null,
    ItemChangeFolder,
    ItemAddTag,
    ItemDelete,
    ItemEdit,
    HeaderContextMenuFolder,
    FolderDelete,
    FolderEdit,
}

#[derive(Clone, Debug, Copy, Default)]
#[fully_pub::fully_pub]
struct AppState {
    selected_folder: Signal<Option<uuid::Uuid>>,
    active_popup: Signal<(Option<Uuid>, PopupType)>,
    popup_position: Signal<(f64, f64)>,
}

#[fully_pub::fully_pub]
impl AppState {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn get_popup_type(&self) -> PopupType {
        PopupType::Null
    }

    /// Opens a popup, closing any other open popup
    fn open_popup(&mut self, id: Uuid, popup_type: PopupType) {
        self.active_popup.set((Some(id), popup_type));
    }

    /// Closes currently open popup
    fn close_popup(&mut self) {
        self.active_popup.set((None, PopupType::Null));
    }

    /// Toggles a popup: if it's already open for this item, close it; otherwise open it
    fn toggle_popup(&mut self, id: Uuid, popup_type: PopupType) {
        if self.is_popup_open(id, popup_type) {
            self.close_popup();
        } else {
            self.open_popup(id, popup_type);
        }
    }

    /// Checks if a specific popup is currently open with a specific ID attached
    fn is_popup_open(&self, id: Uuid, popup_type: PopupType) -> bool {
        (self.active_popup)() == (Some(id), popup_type)
    }

    /// Checks if a specific popup type is currently open
    fn is_popup_type_open(&self, popup_type: PopupType) -> Option<Uuid> {
        let (id, active_type) = (self.active_popup)();
        if active_type == popup_type {
            return id;
        }
        None
    }
}
