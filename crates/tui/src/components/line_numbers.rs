use ratatui::{layout::Rect, style::Stylize, text::Span, widgets::List, Frame};

use super::Component;

#[derive(Debug)]
pub struct LineNumbersComponent {
    total_lines: usize,
    bounds: Rect,
    scroll: u16,
}

impl LineNumbersComponent {
    pub fn new(total_lines: usize, size: Rect, scroll: u16) -> Self {
        Self {
            scroll,
            total_lines,
            bounds: size,
        }
    }

    pub fn update(&mut self, total_lines: usize, scroll: u16) {
        self.scroll = scroll;
        self.total_lines = total_lines;
    }

    fn compose_list(&self) -> Vec<Span> {
        let mut lines = vec![];
        let mut starting_line = self.scroll + 1;
        for i in 0..self.bounds.height as usize {
            if i < self.total_lines {
                let line = starting_line.to_string();
                let line = format!("{}{}", " ".repeat(3 - line.len()), line);
                lines.push(line.gray().dim());
                starting_line += 1;
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
    fn resize(&mut self, size: Rect) -> anyhow::Result<()> {
        self.bounds = size;
        Ok(())
    }
}
