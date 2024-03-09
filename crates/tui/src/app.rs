use glm::{FileManager, FileType, FsOps, ListState};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

use crate::components::line_numbers::LineNumbersComponent;
use crate::components::{file_list::FileListComponent, Component};

#[derive(Debug)]
pub struct App {
    file_list: FileListComponent,
    file_manager: FileManager<ListState>,
    line_numbers: LineNumbersComponent,
    is_help_open: bool,
    pub is_running: bool,
}

impl App {
    pub fn new(file_manager: FileManager<ListState>, size: Rect) -> anyhow::Result<Self> {
        let list = file_manager.get_state().items.clone();

        Ok(Self {
            is_running: true,
            file_manager,
            is_help_open: false,
            line_numbers: LineNumbersComponent::new(list.len(), size),
            file_list: FileListComponent::new(list, size),
        })
    }

    pub fn draw(&mut self, f: &mut Frame) -> anyhow::Result<()> {
        let bottom_pane_size = if self.should_open_bottom_pane() {
            Constraint::Length(10)
        } else {
            Constraint::Length(0)
        };
        let page = Layout::vertical([Constraint::Length(1), Constraint::Fill(1), bottom_pane_size])
            .split(f.size());

        let header = Layout::vertical([Constraint::Fill(1)]).split(page[0]);
        // TODO: we need to add scroll to our list
        let body = Layout::horizontal([Constraint::Length(4), Constraint::Fill(1)]).split(page[1]);
        let footer = Layout::vertical([Constraint::Fill(1)]).split(page[2]);

        let line_numbers = body[0];
        let list = body[1];

        self.file_list.resize(list);
        self.line_numbers.resize(line_numbers);

        self.line_numbers.draw(f, line_numbers)?;
        self.file_list.draw(f, list)?;
        Ok(())
    }

    fn should_open_bottom_pane(&self) -> bool {
        self.is_help_open
    }

    fn select_current_item(&mut self) -> anyhow::Result<()> {
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

    fn change_to_parent(&mut self) -> anyhow::Result<()> {
        let path = self.file_manager.get_state().current_dir.clone();
        if let Some(parent) = path.parent() {
            let new_state = self.file_manager.change_dir(parent)?;
            self.file_list.update(new_state.items.clone())?;
        }
        Ok(())
    }

    fn toggle_help(&mut self) {
        self.is_help_open = !self.is_help_open;
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) -> anyhow::Result<()> {
        match event.code {
            KeyCode::Enter => self.select_current_item()?,
            KeyCode::Char('q') => self.is_running = false,
            KeyCode::Char('-') => self.change_to_parent()?,
            KeyCode::Char('?') => self.toggle_help(),
            _ => self.file_list.handle_key_event(event)?,
        }

        Ok(())
    }

    pub fn tick(&mut self) -> anyhow::Result<()> {
        self.file_list.tick()?;
        Ok(())
    }
}
