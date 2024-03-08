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
    last_x: u16,
    last_y: u16,
    y: u16,
    x: u16,
}

impl FileListComponent {
    pub fn new(items: Vec<String>, bounds: Rect) -> Self {
        // HACK: we are starting last_x,y as different values so it fixes
        // on first render
        Self {
            last_x: 1,
            last_y: 1,
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
        self.last_x = self.x;
        self.last_y = self.y;
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

    fn is_separator(&self, c: char) -> bool {
        matches!(c, '-' | '_' | '.' | ' ')
    }

    fn skip_to_next_line(&mut self) {
        let max_y = self.bounds.y + self.items.len() as u16 - 1;
        if self.y == max_y {
            return;
        }
        self.x = self.bounds.x;
        self.y = u16::min(max_y, self.y + 1);
    }

    fn skip_to_separator(&mut self) {
        let x_as_index = self.x - self.bounds.x;
        let line = self.get_line_under_cursor();
        let char_at_cursor = line
            .chars()
            .nth(x_as_index.into())
            .expect("cursor should never be out of bounds");
        let is_at_separator = self.is_separator(char_at_cursor);

        let mut chars_to_skip = 0;
        for c in line.chars().skip(x_as_index.into()) {
            match (is_at_separator, self.is_separator(c)) {
                (true, false) => break,
                (false, true) => break,
                _ => chars_to_skip += 1,
            }
        }

        self.x += chars_to_skip;
    }

    fn move_cursor_left(&mut self) -> anyhow::Result<()> {
        self.x = u16::max(self.x.saturating_sub(1), self.bounds.x);
        self.constrain_to_line();
        Ok(())
    }

    fn move_cursor_down(&mut self) -> anyhow::Result<()> {
        let max_y = self.bounds.y + self.items.len() as u16 - 1;
        self.y = u16::min(max_y, self.y + 1);
        self.constrain_to_line();
        Ok(())
    }

    fn move_cursor_up(&mut self) -> anyhow::Result<()> {
        self.y = u16::max(self.y.saturating_sub(1), self.bounds.y);
        self.constrain_to_line();
        Ok(())
    }

    fn move_cursor_right(&mut self) -> anyhow::Result<()> {
        self.x += 1;
        self.constrain_to_line();
        Ok(())
    }

    fn move_cursor_to_line_start(&mut self) -> anyhow::Result<()> {
        self.x = self.bounds.x;
        Ok(())
    }

    fn move_cursor_to_line_end(&mut self) -> anyhow::Result<()> {
        let line_len = self.get_line_under_cursor().len() as u16;
        self.x = line_len - 1;
        Ok(())
    }

    fn move_cursor_to_next_word(&mut self) -> anyhow::Result<()> {
        let x_as_index = self.x - self.bounds.x;
        let should_go_down = x_as_index as usize == self.get_line_under_cursor().len() - 1;

        if should_go_down {
            self.skip_to_next_line();
        } else {
            self.skip_to_separator();
        }

        self.constrain_to_line();
        Ok(())
    }
}

impl Component for FileListComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect) -> anyhow::Result<()> {
        f.render_widget(List::new(self.items.to_vec()), area);
        Ok(())
    }

    fn handle_key_event(&mut self, event: KeyEvent) -> anyhow::Result<()> {
        match event.code {
            KeyCode::Char('h') => self.move_cursor_left()?,
            KeyCode::Char('j') => self.move_cursor_down()?,
            KeyCode::Char('k') => self.move_cursor_up()?,
            KeyCode::Char('l') => self.move_cursor_right()?,
            KeyCode::Char('0') => self.move_cursor_to_line_start()?,
            KeyCode::Char('$') => self.move_cursor_to_line_end()?,
            KeyCode::Char('w') => self.move_cursor_to_next_word()?,
            KeyCode::Char('q') => todo!(),
            _ => (),
        }
        self.draw_cursor()?;
        Ok(())
    }

    fn tick(&mut self) -> anyhow::Result<()> {
        match (self.last_x == self.x, self.last_y == self.y) {
            (true, true) => (),
            _ => self.draw_cursor()?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_sut(lines_number: u8) -> FileListComponent {
        let mut lines = vec![];
        for i in 0..lines_number {
            let line = i.to_string() + " Hello, World!";
            lines.push(line);
            println!()
        }
        let area = Rect::new(0, 0, 10, 10);
        FileListComponent::new(lines, area)
    }

    #[test]
    fn test_get_line_under_cursor() {
        let sut = make_sut(1);

        let line = sut.get_line_under_cursor();

        assert_eq!(line, "0 Hello, World!");
    }

    #[test]
    fn test_move_cursor_down() {
        let mut sut = make_sut(10);

        // should never be bigger than total lines
        for _ in 0..30 {
            _ = sut.move_cursor_down();
        }

        assert_eq!(sut.y, 9);
    }

    #[test]
    fn test_move_cursor_up() {
        let mut sut = make_sut(10);
        sut.y = 3;

        // should never go below 0
        for _ in 0..6 {
            _ = sut.move_cursor_up();
        }

        assert_eq!(sut.y, 0);
    }

    #[test]
    fn test_move_cursor_right() {
        let mut sut = make_sut(1);

        // should not go over end of line
        for _ in 0..30 {
            _ = sut.move_cursor_right();
        }

        assert_eq!(sut.x, 14);
    }

    #[test]
    fn test_move_cursor_left() {
        let mut sut = make_sut(1);
        sut.x = 10;

        // should not go below 0
        for _ in 0..30 {
            _ = sut.move_cursor_left();
        }

        assert_eq!(sut.x, 0);
    }

    #[test]
    fn test_move_cursor_to_next_word() {
        let mut sut = make_sut(2);

        // should go to the first character of second line.
        // 0 Hello, World!
        //  ^^     ^^    ^
        // These are the separatos it should stop. the 6th move to next line
        for _ in 0..6 {
            _ = sut.move_cursor_to_next_word();
        }

        assert_eq!(sut.x, 0);
        assert_eq!(sut.y, 1);
    }

    #[test]
    fn test_move_to_line_start() {
        let mut sut = make_sut(1);
        sut.x = 10;

        _ = sut.move_cursor_to_line_start();

        assert_eq!(sut.x, 0);
    }

    #[test]
    fn test_move_to_line_end() {
        let mut sut = make_sut(1);

        _ = sut.move_cursor_to_line_end();

        assert_eq!(sut.x, 14);
    }
}
