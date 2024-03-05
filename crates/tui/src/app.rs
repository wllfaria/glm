use glm::{FileManager, ListState};

#[derive(Debug)]
pub struct App {
    _file_manager: FileManager<ListState>,
    pub is_running: bool,
}

impl App {
    pub fn new(file_manager: FileManager<ListState>) -> Self {
        Self {
            _file_manager: file_manager,
            is_running: true,
        }
    }
}
