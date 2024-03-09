use ratatui::{
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use super::Component;

#[derive(Debug)]
pub struct HelpComponent {}

impl HelpComponent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for HelpComponent {
    fn draw(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
    ) -> anyhow::Result<()> {
        let lines = vec![Line::from("this will eventually be help!".gray())];

        let help = Paragraph::new(lines).block(
            Block::new()
                .borders(Borders::TOP)
                .border_style(Style::new().fg(Color::Gray)),
        );
        f.render_widget(help, area);
        Ok(())
    }
    fn tick(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    fn resize(&mut self, size: ratatui::prelude::Rect) {}
    fn handle_key_event(&mut self, event: crossterm::event::KeyEvent) -> anyhow::Result<()> {
        Ok(())
    }
}
