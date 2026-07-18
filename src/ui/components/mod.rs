// src/ui/components/mod.rs
pub mod header;
mod hooks;
mod icons;
pub mod items;
pub mod popups;

pub mod prelude {
    pub use super::header::prelude::*;
    pub use super::items::prelude::*;
    pub use super::popups::prelude::*;
}
