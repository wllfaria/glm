use ratatui::{layout::Rect, widgets::Paragraph, Frame};

use crate::components::Component;

#[derive(Debug)]
pub struct FileListComponent {}

impl FileListComponent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for FileListComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect) -> anyhow::Result<()> {
        f.render_widget(Paragraph::new("Hello, World!"), area);
        Ok(())
    }
}
