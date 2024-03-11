pub mod file_list;
pub mod help;
pub mod line_numbers;

use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

pub trait Component {
    fn draw(&mut self, f: &mut Frame, area: Rect) -> anyhow::Result<()>;
    fn tick(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    fn handle_key_event(&mut self, _event: KeyEvent) -> anyhow::Result<()> {
        Ok(())
    }
    fn resize(&mut self, _size: Rect) -> anyhow::Result<()> {
        Ok(())
    }
}
