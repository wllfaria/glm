use std::path::PathBuf;

use crate::file_manager::Item;

#[derive(Default)]
pub struct ListState {
    items: Vec<Item>,
    current_dir: PathBuf,
}
