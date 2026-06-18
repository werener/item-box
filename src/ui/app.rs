use crate::core::builder::ItemBuilder;
use crate::core::inventory::Inventory;
use crate::core::models::*;
use crate::ui::item_grid::ItemGrid;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let mut inventory = Inventory::default();
    
    let tag1_id = inventory.add_tag("Power Tool", Color { r: 255, g: 0, b: 0 });
    let tag2_id = inventory.add_tag("Very very cool", Color { r: 0, g: 255, b: 0 });
    let tag3_id = inventory.add_tag("tag3", Color { r: 0, g: 255, b: 255 });
    let tag4_id = inventory.add_tag("tag4", Color { r: 0, g: 255, b: 255 });

    let folder1_id = inventory.add_folder("Garage");
    
    inventory.add_item(
        "Powerfull Stainless Cordless Drill",
        "Garage Shelf",
        "A powerful cordless drill with multiple speed settings and a long-lasting battery.",
        149.99,
        Some(folder1_id),
        vec![tag1_id, tag2_id, tag3_id, tag4_id],
    );
    
    inventory.add_item(
        "Circular Saw",
        "Garage Shelf",
        "",
        100.0,
        Some(folder1_id),
        vec![tag1_id, tag2_id, tag4_id],
    );

    rsx! {
        // metainfo
        document::Stylesheet { href: asset!("/assets/tailwind.css") }

        div { class: "min-h-screen w-full bg-surface",
            ItemGrid { inventory }
        }
    }
}