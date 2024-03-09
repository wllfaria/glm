use ratatui::{layout::Rect, widgets::List, Frame};

use super::Component;

#[derive(Debug)]
pub struct LineNumbersComponent {
    total_lines: usize,
    bounds: Rect,
}

impl LineNumbersComponent {
    pub fn new(total_lines: usize, size: Rect) -> Self {
        Self {
            total_lines,
            bounds: size,
        }
    }

    fn compose_list(&self) -> Vec<String> {
        let mut lines = vec![];
        for i in 0..self.bounds.height as usize {
            if i < self.total_lines {
                let line = (i + 1).to_string();
                let line = format!("{}{}", " ".repeat(3 - line.len()), line);
                lines.push(line);
                continue;
            }
            lines.push("~".into());
        }
        lines
    }
}

impl Component for LineNumbersComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect) -> anyhow::Result<()> {
        f.render_widget(List::new(self.compose_list()), area);
        Ok(())
    }
    fn resize(&mut self, size: Rect) {
        self.bounds = size
    }
    fn tick(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    fn handle_key_event(&mut self, event: crossterm::event::KeyEvent) -> anyhow::Result<()> {
        Ok(())
    }
}