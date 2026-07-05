// src/ui/components/mod.rs
mod control_panel;
mod header;
mod icons;
mod items;
mod popups;

pub mod prelude {
    pub use super::items::prelude::*;
    pub use super::{control_panel::ControlPanelInventory, header::Header, popups::*};
}
