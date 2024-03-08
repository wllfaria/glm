use std::io;

use crossterm::cursor::MoveTo;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::widgets::List;
use ratatui::Frame;

use crate::components::Component;

#[derive(Debug)]
pub struct FileListComponent {
    items: Vec<String>,
    bounds: Rect,
    y: u16,
    x: u16,
}

impl FileListComponent {
    pub fn new(items: Vec<String>, bounds: Rect) -> Self {
        Self {
            x: 0,
            y: 0,
            bounds,
            items,
        }
    }

    pub fn resize(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn draw_cursor(&mut self) -> anyhow::Result<()> {
        crossterm::execute!(io::stdout(), MoveTo(self.x, self.y))?;
        Ok(())
    }

    fn constrain_to_line(&mut self) {
        let line_len = self.get_line_under_cursor().len() as u16;
        self.x = u16::min(self.x, line_len - 1);
    }

    fn get_line_under_cursor(&self) -> &str {
        let y_as_list_index = self.y - self.bounds.y;
        &self.items[y_as_list_index as usize]
    }
}

impl Component for FileListComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect) -> anyhow::Result<()> {
        f.render_widget(List::new(self.items.to_vec()), area);
        Ok(())
    }

    fn handle_key_event(&mut self, event: KeyEvent) -> anyhow::Result<()> {
        match event.code {
            KeyCode::Char('h') => {
                self.x = u16::max(self.x.saturating_sub(1), self.bounds.x);
                self.constrain_to_line();
                self.draw_cursor()?;
            }
            KeyCode::Char('j') => {
                let max_y = self.bounds.y + self.items.len() as u16 - 1;
                self.y = u16::min(max_y, self.y + 1);
                self.constrain_to_line();
                self.draw_cursor()?;
            }
            KeyCode::Char('k') => {
                self.y = u16::max(self.y.saturating_sub(1), self.bounds.y);
                self.constrain_to_line();
                self.draw_cursor()?;
            }
            KeyCode::Char('l') => {
                self.x += 1;
                self.constrain_to_line();
                self.draw_cursor()?;
            }
            KeyCode::Char('0') => {
                self.x = self.bounds.x;
                self.draw_cursor()?;
            }
            KeyCode::Char('$') => {
                let line_len = self.get_line_under_cursor().len() as u16;
                self.x = line_len - 1;
                self.draw_cursor()?;
            }
            KeyCode::Char('q') => todo!(),
            _ => (),
        }
        Ok(())
    }
}
