use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph},
};

use super::Component;

#[derive(Debug)]
pub struct HelpComponent {
    keys: [Line<'static>; 5],
    lines: [Line<'static>; 5],
}

impl HelpComponent {
    pub fn new() -> Self {
        Self {
            keys: [
                "-".into(),
                "Enter".into(),
                "q".into(),
                "?".into(),
                "H".into(),
            ],
            lines: [
                "Move up to parent directory".into(),
                "Open the entry under the cursor".into(),
                "Quit glm".into(),
                "Toggle this help pane".into(),
                "Toggle hidden files".into(),
            ],
        }
    }
}

impl Component for HelpComponent {
    fn draw(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
    ) -> anyhow::Result<()> {
        let help_block = Block::new()
            .borders(Borders::TOP)
            .border_style(Style::new().fg(Color::Gray))
            .padding(Padding::left(1));

        let keys = Paragraph::new(self.keys.to_vec())
            .red()
            .block(help_block.clone());
        let lines = Paragraph::new(self.lines.to_vec()).blue().block(help_block);

        let layout = Layout::horizontal([Constraint::Length(6), Constraint::Fill(1)]).split(area);

        f.render_widget(keys, layout[0]);
        f.render_widget(lines, layout[1]);
        Ok(())
    }
}
