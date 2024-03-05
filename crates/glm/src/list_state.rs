use std::path::PathBuf;

use crate::file_manager::Item;

#[derive(Debug, Default)]
pub struct ListState {
    pub items: Vec<Item>,
    pub current_dir: PathBuf,
}
