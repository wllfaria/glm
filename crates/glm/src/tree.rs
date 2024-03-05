use crate::file_manager::Item;

#[derive(Debug)]
pub struct TreeNode {
    _item: Item,
    _children: Vec<TreeNode>,
}
