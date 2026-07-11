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
