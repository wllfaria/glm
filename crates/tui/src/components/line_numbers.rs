use ratatui::{layout::Rect, style::Stylize, text::Span, widgets::List, Frame};

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

    pub fn update(&mut self, total_lines: usize) {
        self.total_lines = total_lines;
    }

    fn compose_list(&self) -> Vec<Span> {
        let mut lines = vec![];
        for i in 0..self.bounds.height as usize {
            if i < self.total_lines {
                let line = (i + 1).to_string();
                let line = format!("{}{}", " ".repeat(3 - line.len()), line);
                lines.push(line.gray().dim());
                continue;
            }
            lines.push("~".magenta().dim());
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
        self.bounds = size;
    }
}
