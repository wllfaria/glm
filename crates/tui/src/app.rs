use glm::{FileManager, ListState};

use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::Frame;

use crate::components::{file_list::FileListComponent, Component};

#[derive(Debug)]
pub struct App {
    file_list: FileListComponent,
    pub is_running: bool,
}

impl App {
    pub fn new(file_manager: FileManager<ListState>, size: Rect) -> Self {
        let list = file_manager
            .get_state()
            .items
            .iter()
            .map(|i| i.file_name.clone())
            .collect();

        Self {
            is_running: true,
            file_list: FileListComponent::new(list, size),
        }
    }

    pub fn draw(&mut self, f: &mut Frame) -> anyhow::Result<()> {
        self.file_list.resize(f.size());
        self.file_list.draw(
            f,
            Layout::default()
                .constraints([Constraint::Percentage(100)])
                .split(f.size())[0],
        )?;
        Ok(())
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) -> anyhow::Result<()> {
        self.file_list.handle_key_event(event)?;
        Ok(())
    }

    pub fn tick(&mut self) -> anyhow::Result<()> {
        self.file_list.tick()?;
        Ok(())
    }
}
