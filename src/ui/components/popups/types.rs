#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum PopupType {
    #[default]
    Null,
    ItemContextMenuFolders,
    ItemAddTag,
    ItemDelete,
    ItemEdit,
    HeaderContextMenuFolder,
    FolderDelete,
    FolderEdit,
}
