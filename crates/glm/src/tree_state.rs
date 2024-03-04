use std::path::PathBuf;

use crate::tree::TreeNode;

#[derive(Default)]
pub struct TreeState {
    _items: Vec<TreeNode>,
    _current_dir: PathBuf,
}
