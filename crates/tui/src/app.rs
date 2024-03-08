use glm::{FileManager, FileType, FsOps, ListState};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

use crate::components::{file_list::FileListComponent, Component};

#[derive(Debug)]
pub struct App {
    file_list: FileListComponent,
    file_manager: FileManager<ListState>,
    pub is_running: bool,
}

impl App {
    pub fn new(file_manager: FileManager<ListState>, size: Rect) -> anyhow::Result<Self> {
        let list = file_manager.get_state().items.clone();

        Ok(Self {
            is_running: true,
            file_manager,
            file_list: FileListComponent::new(list, size),
        })
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

    pub fn select_current_item(&mut self) -> anyhow::Result<()> {
        let list_item = self.file_list.get_line_under_cursor();
        match list_item.item.file_type {
            FileType::Directory => {
                let new_state = self.file_manager.change_dir(&list_item.item.file_path)?;
                self.file_list.update(new_state.items.clone())?;
            }
            _ => todo!(), // TODO: we should open the file here
        };
        Ok(())
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) -> anyhow::Result<()> {
        match event.code {
            KeyCode::Enter => self.select_current_item()?,
            KeyCode::Char('q') => self.is_running = false,
            _ => self.file_list.handle_key_event(event)?,
        }

        Ok(())
    }

    pub fn tick(&mut self) -> anyhow::Result<()> {
        self.file_list.tick()?;
        Ok(())
    }
}
