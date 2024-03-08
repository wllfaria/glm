use glm::{FileManager, ListState};

use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

use crate::components::{file_list::FileListComponent, Component};

#[derive(Debug)]
pub struct App {
    file_list: FileListComponent,
    pub is_running: bool,
}

impl App {
    pub fn new(file_manager: FileManager<ListState>, size: Rect) -> Self {
        let list = file_manager.get_state().items.clone();

        Self {
            is_running: true,
            file_list: FileListComponent::new(list, size),
        }
    }

    pub fn draw(&mut self, f: &mut Frame) -> anyhow::Result<()> {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(4), Constraint::Fill(1)])
            .split(f.size());
        let _lines = layout[0];
        let list = layout[1];
        self.file_list.resize(list);
        self.file_list.draw(f, list)?;
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
