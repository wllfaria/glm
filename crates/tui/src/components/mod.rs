use ratatui::{layout::Rect, Frame};

pub mod file_list;

pub trait Component {
    fn draw(&mut self, f: &mut Frame, area: Rect) -> anyhow::Result<()>;
}
