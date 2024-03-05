use glm::{FileManager, ListState};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Layout},
    Frame,
};

use crate::components::{file_list::FileListComponent, Component};

#[derive(Debug)]
pub struct App {
    file_list: FileListComponent,
    _file_manager: FileManager<ListState>,
    pub is_running: bool,
}

impl App {
    pub fn new(file_manager: FileManager<ListState>) -> Self {
        Self {
            _file_manager: file_manager,
            is_running: true,
            file_list: FileListComponent::new(),
        }
    }

    pub fn draw(&mut self, f: &mut Frame) -> anyhow::Result<()> {
        self.file_list.draw(
            f,
            Layout::default()
                .constraints([Constraint::Percentage(100)])
                .split(f.size())[0],
        )?;
        Ok(())
    }
}
