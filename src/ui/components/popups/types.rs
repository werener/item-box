#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum PopupType {
    #[default]
    Null,
    ItemCreate,
    ItemUpdate,
    ItemDelete,

    FolderUpdate,
    FolderDelete,

    ItemAddTag,

    ItemContextMenuFolders,
    HeaderContextMenuFolder,
}
