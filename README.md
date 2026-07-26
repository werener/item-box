# vToolBox

# TODO:
items:
1) add skeleton-rendering for an image
2) add an "item-type" icon in the top-left corner
3) more interactive swipe photo-scrolling

popups:
1) divide local and global popups to allow reusing local ones inside the globals

tags:
1) Remove unnecessary context. Store not 3 fields, but just a "new tag style" or smth, similarly to item editing.
2) Move delete button to the top right of item editor. 
3) Remove save (v) button, and just save on any action. 
4) Add revert changes button to set the tag to how it was before editing

core:
1) Add a custom type for price to indicate currency
2) Add "Actions" to represent recent actions, performed on the inventory
3) Create new API, attached to Actions, to perform all CRUD operations and provide their reverses
4) Add setters for models' fields for normalizations (i.e. string trimming and common price handling)