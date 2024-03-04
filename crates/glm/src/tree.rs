use crate::file_manager::Item;

#[derive(Debug)]
pub struct TreeNode {
    item: Item,
    children: Vec<TreeNode>,
}
