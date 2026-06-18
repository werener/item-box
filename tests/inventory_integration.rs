use uuid::Uuid;
use v_tool_box::core::builder::ItemBuilder;
use v_tool_box::core::inventory::Inventory;
use v_tool_box::core::models::*;

fn dummy_color() -> Color {
    Color { r: 255, g: 0, b: 0 }
}

#[test]
fn test_create_and_read() {
    let mut inv = Inventory::default();
    let garage_id = inv.add_folder("Garage");
    let power_tag_id = inv.add_tag("Power Tool", dummy_color());

    let drill_id = inv.add_item_struct(
        ItemBuilder::new("Drill")
            .location("Shelf 1")
            .price(99.99)
            .folder(Some(garage_id))
            .tags(vec![power_tag_id])
            .build(),
    );

    assert!(
        inv.get_folder(garage_id).is_some(),
        "Folder should exist after creation"
    );
    assert!(
        inv.get_tag(power_tag_id).is_some(),
        "Tag should exist after creation"
    );

    let drill = inv
        .get_item(drill_id)
        .expect("Tool should exist after creation");
    assert_eq!(drill.name, "Drill", "Tool name mismatch");
    assert_eq!(drill.folder, Some(garage_id), "Tool folder mismatch");
    assert_eq!(drill.tags, vec![power_tag_id], "Tool tags mismatch");

    inv.remove_item(drill_id);
    assert!(
        inv.get_item(drill_id).is_none(),
        "Tool should not exist after deletion"
    );
}

#[test]
fn test_rename_operations() {
    let mut inv = Inventory::default();

    let tool_id = inv.add_item_struct(
        ItemBuilder::new("Old Drill")
            .location("Shelf")
            .price(50.0)
            .build(),
    );

    let folder_id = inv.add_folder("Old Garage");
    let tag_id = inv.add_tag("Old Tag", dummy_color());

    assert!(
        inv.rename_item(tool_id, "New Drill"),
        "Renaming existing tool should succeed"
    );
    assert_eq!(
        inv.get_item(tool_id).unwrap().name,
        "New Drill",
        "Tool name not updated"
    );

    assert!(
        inv.rename_folder(folder_id, "New Garage"),
        "Renaming existing folder should succeed"
    );
    assert_eq!(
        inv.get_folder(folder_id).unwrap().name,
        "New Garage",
        "Folder name not updated"
    );

    assert!(
        inv.rename_tag(tag_id, "New Tag"),
        "Renaming existing tag should succeed"
    );
    assert_eq!(
        inv.get_tag(tag_id).unwrap().name,
        "New Tag",
        "Tag name not updated"
    );

    let fake_id = Uuid::new_v4();
    assert!(
        !inv.rename_item(fake_id, "Ghost"),
        "Renaming non-existent tool should fail"
    );
    assert!(
        !inv.rename_folder(fake_id, "Ghost"),
        "Renaming non-existent folder should fail"
    );
    assert!(
        !inv.rename_tag(fake_id, "Ghost"),
        "Renaming non-existent tag should fail"
    );
}

#[test]
fn test_recolor_tag() {
    let mut inv = Inventory::default();
    let tag_id = inv.add_tag("Red Tag", dummy_color());
    let new_color = Color { r: 0, g: 255, b: 0 };

    assert!(
        inv.recolor_tag(tag_id, new_color.clone()),
        "Recoloring existing tag should succeed"
    );
    assert_eq!(
        inv.get_tag(tag_id).unwrap().color,
        new_color,
        "Tag color not updated"
    );

    let fake_id = Uuid::new_v4();
    assert!(
        !inv.recolor_tag(fake_id, new_color),
        "Recoloring non-existent tag should fail"
    );
}

#[test]
fn test_remove_folder_cleans_up_tools() {
    let mut inv = Inventory::default();
    let folder_id = inv.add_folder("Workshop");

    let tool_id = inv.add_item_struct(
        ItemBuilder::new("Saw")
            .location("Bench")
            .price(50.0)
            .folder(Some(folder_id))
            .build(),
    );

    assert_eq!(
        inv.get_item(tool_id).unwrap().folder,
        Some(folder_id),
        "Tool should be assigned to folder"
    );

    inv.remove_folder(folder_id);

    assert!(
        inv.get_folder(folder_id).is_none(),
        "Folder should be deleted"
    );
    assert_eq!(
        inv.get_item(tool_id).unwrap().folder,
        None,
        "Tool folder should be unassigned after folder deletion"
    );
}

#[test]
fn test_remove_tag_cleans_up_tools() {
    let mut inv = Inventory::default();
    let tag1_id = inv.add_tag("Heavy", dummy_color());
    let tag2_id = inv.add_tag("Expensive", dummy_color());

    let tool_id = inv.add_item_struct(
        ItemBuilder::new("Anvil")
            .location("Floor")
            .price(200.0)
            .tags(vec![tag1_id, tag2_id])
            .build(),
    );

    assert_eq!(
        inv.get_item(tool_id).unwrap().tags,
        vec![tag1_id, tag2_id],
        "Tool should have both tags"
    );

    inv.remove_tag(tag1_id);

    assert!(inv.get_tag(tag1_id).is_none(), "Tag should be deleted");
    assert_eq!(
        inv.get_item(tool_id).unwrap().tags,
        vec![tag2_id],
        "Tool should only retain the second tag"
    );
}

#[test]
fn test_serialization_roundtrip() {
    let mut inv = Inventory::default();
    let folder_id = inv.add_folder("Garage");

    inv.add_item_struct(
        ItemBuilder::new("Wrench")
            .location("Drawer")
            .price(25.5)
            .folder(Some(folder_id))
            .build(),
    );

    let json = serde_json::to_string(&inv).expect("Serialization should not fail");
    let loaded_inv: Inventory =
        serde_json::from_str(&json).expect("Deserialization should not fail");

    assert_eq!(
        inv.folders.len(),
        loaded_inv.folders.len(),
        "Folder count mismatch after roundtrip"
    );
    assert_eq!(
        inv.items.len(),
        loaded_inv.items.len(),
        "Tool count mismatch after roundtrip"
    );
    assert_eq!(
        inv.items[0].name, loaded_inv.items[0].name,
        "Tool name mismatch after roundtrip"
    );
}
