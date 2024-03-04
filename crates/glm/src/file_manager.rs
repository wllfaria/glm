use crate::list_state::ListState;
use crate::tree_state::TreeState;

#[derive(Debug)]
pub struct Item {}

pub struct FileManager<S> {
    pub state: S,
}

impl FileManager<ListState> {
    pub fn new() -> FileManager<ListState> {
        FileManager {
            state: ListState::default(),
        }
    }
}

impl FileManager<TreeState> {
    pub fn new() -> FileManager<TreeState> {
        FileManager {
            state: TreeState::default(),
        }
    }
}
