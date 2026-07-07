use dioxus::prelude::*;
use crate::core::inventory::Inventory;
use super::{folder_panel::ControlPanelFolders, control_panel::ControlPanelInventory};

#[component]
pub fn Header(inventory: Signal<Inventory>) -> Element {
    rsx! {
        ControlPanelFolders { inventory }
        ControlPanelInventory { inventory }
    }
}