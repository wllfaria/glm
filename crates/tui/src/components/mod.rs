pub mod file_list;
pub mod line_numbers;

use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

pub trait Component {
    fn draw(&mut self, f: &mut Frame, area: Rect) -> anyhow::Result<()>;
    fn tick(&mut self) -> anyhow::Result<()>;
    fn handle_key_event(&mut self, event: KeyEvent) -> anyhow::Result<()>;
}
