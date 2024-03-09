use glm::{FileType, Item};

use std::io;

use crossterm::cursor::MoveTo;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::widgets::List;
use ratatui::Frame;

use crate::components::Component;

#[derive(Debug, PartialEq, Clone)]
struct Position {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone)]
pub struct ListItem {
    pub display_name: String,
    pub item: Item,
}

#[derive(Debug)]
pub struct FileListComponent {
    items: Vec<ListItem>,
    bounds: Rect,
    last_pos: Option<Position>,
    pos: Position,
}

fn format_names(items: Vec<Item>) -> Vec<ListItem> {
    let mut items = items.clone();
    items.sort_by(|a, b| {
        match (
            a.file_type == FileType::Directory,
            b.file_type == FileType::Directory,
        ) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name.cmp(&b.file_name),
        }
    });

    items
        .iter()
        .map(|item| {
            let display_name = match item.file_type {
                FileType::Directory => item.file_name.clone() + "/",
                _ => item.file_name.clone(),
            };
            ListItem {
                display_name,
                item: item.clone(),
            }
        })
        .collect()
}

impl FileListComponent {
    pub fn new(items: Vec<Item>, bounds: Rect) -> Self {
        Self {
            last_pos: None,
            pos: Position { x: 0, y: 0 },
            bounds,
            items: format_names(items),
        }
    }

    pub fn update(&mut self, items: Vec<Item>) -> anyhow::Result<()> {
        self.items = format_names(items);
        self.pos = Position { x: 0, y: 0 };
        self.last_pos = None;
        self.draw_cursor()?;
        Ok(())
    }

    pub fn draw_cursor(&mut self) -> anyhow::Result<()> {
        self.last_pos = Some(self.pos.clone());
        let x = self.pos.x + self.bounds.x;
        let y = self.pos.y + self.bounds.y;
        crossterm::execute!(io::stdout(), MoveTo(x, y))?;
        Ok(())
    }

    fn constrain_to_line(&mut self) {
        let line_len = self.get_line_under_cursor().display_name.len() as u16;
        self.pos.x = u16::min(self.pos.x, line_len - 1);
    }

    pub fn get_line_under_cursor(&self) -> &ListItem {
        &self.items[self.pos.y as usize]
    }

    fn is_separator(&self, c: char) -> bool {
        matches!(c, '-' | '_' | '.' | ' ')
    }

    fn skip_to_next_line(&mut self) {
        let max_y = self.items.len() as u16 - 1;
        if self.pos.y == max_y {
            return;
        }
        self.pos.x = 0;
        self.pos.y = u16::min(max_y, self.pos.y + 1);
    }

    fn skip_to_separator(&mut self) {
        let line = &self.get_line_under_cursor().display_name;
        let char_at_cursor = line
            .chars()
            .nth(self.pos.x.into())
            .expect("cursor should never be out of bounds");
        let is_at_separator = self.is_separator(char_at_cursor);

        let mut chars_to_skip = 0;
        for c in line.chars().skip(self.pos.x.into()) {
            match (is_at_separator, self.is_separator(c)) {
                (true, false) => break,
                (false, true) => break,
                _ => chars_to_skip += 1,
            }
        }

        self.pos.x += chars_to_skip;
    }

    fn move_cursor_left(&mut self) -> anyhow::Result<()> {
        self.pos.x = self.pos.x.saturating_sub(1);
        self.constrain_to_line();
        Ok(())
    }

    fn move_cursor_down(&mut self) -> anyhow::Result<()> {
        let max_y = self.items.len() as u16 - 1;
        self.pos.y = u16::min(max_y, self.pos.y + 1);
        self.constrain_to_line();
        Ok(())
    }

    fn move_cursor_up(&mut self) -> anyhow::Result<()> {
        self.pos.y = self.pos.y.saturating_sub(1);
        self.constrain_to_line();
        Ok(())
    }

    fn move_cursor_right(&mut self) -> anyhow::Result<()> {
        self.pos.x += 1;
        self.constrain_to_line();
        Ok(())
    }

    fn move_cursor_to_line_start(&mut self) -> anyhow::Result<()> {
        self.pos.x = 0;
        Ok(())
    }

    fn move_cursor_to_line_end(&mut self) -> anyhow::Result<()> {
        let line_len = self.get_line_under_cursor().display_name.len() as u16;
        self.pos.x = line_len - 1;
        Ok(())
    }

    fn move_cursor_to_next_word(&mut self) -> anyhow::Result<()> {
        let item_len = self.get_line_under_cursor().display_name.len() - 1;
        let should_go_down = self.pos.x as usize == item_len;

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
        let list = self
            .items
            .iter()
            .map(|i| match i.item.file_type {
                FileType::Directory => i.display_name.clone().yellow().bold(),
                _ => i.display_name.clone().blue().dim(),
            })
            .collect::<Vec<_>>();
        f.render_widget(List::new(list), area);
        self.draw_cursor()?;
        Ok(())
    }

    fn resize(&mut self, bounds: Rect) {
        self.bounds = bounds;
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
            _ => (),
        }
        self.draw_cursor()?;
        Ok(())
    }

    fn tick(&mut self) -> anyhow::Result<()> {
        match &self.last_pos {
            Some(last_pos) => {
                if *last_pos != self.pos {
                    self.draw_cursor()?;
                }
            }
            None => self.draw_cursor()?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn make_sut(lines_number: u8) -> FileListComponent {
        let mut lines = vec![];
        for i in 0..lines_number {
            lines.push(Item {
                is_hidden: false,
                file_type: FileType::File,
                file_ext: Some(".txt".into()),
                file_name: i.to_string() + "hello_world",
                file_path: PathBuf::new(),
            });
        }
        // start not in 0,0 to test bounds
        let area = Rect::new(10, 10, 10, 10);
        FileListComponent::new(lines, area)
    }

    #[test]
    fn test_get_line_under_cursor() {
        let sut = make_sut(1);

        let line = &sut.get_line_under_cursor().display_name;

        assert_eq!(line, "0hello_world");
    }

    #[test]
    fn test_move_cursor_down() {
        let mut sut = make_sut(10);

        // should never be bigger than total lines
        for _ in 0..30 {
            _ = sut.move_cursor_down();
        }

        assert_eq!(sut.pos.y, 9);
    }

    #[test]
    fn test_move_cursor_up() {
        let mut sut = make_sut(10);
        sut.pos.y = 3;

        // should never go below 0
        for _ in 0..6 {
            _ = sut.move_cursor_up();
        }

        assert_eq!(sut.pos.y, 0);
    }

    #[test]
    fn test_move_cursor_right() {
        let mut sut = make_sut(1);

        // should not go over end of line
        for _ in 0..30 {
            _ = sut.move_cursor_right();
        }

        assert_eq!(sut.pos.x, 11);
    }

    #[test]
    fn test_move_cursor_left() {
        let mut sut = make_sut(1);
        sut.pos.x = 10;

        // should not go below 0
        for _ in 0..30 {
            _ = sut.move_cursor_left();
        }

        assert_eq!(sut.pos.x, 0);
    }

    #[test]
    fn test_move_cursor_to_next_word() {
        let mut sut = make_sut(2);

        // should go to the first character of second line.
        // 0hello_world
        //       ^^   ^
        // These are the separators it should stop. the 4th move to next line
        for _ in 0..4 {
            _ = sut.move_cursor_to_next_word();
        }

        assert_eq!(sut.pos.x, 0);
        assert_eq!(sut.pos.y, 1);
    }

    #[test]
    fn test_move_to_line_start() {
        let mut sut = make_sut(1);
        sut.pos.x = 10;

        _ = sut.move_cursor_to_line_start();

        assert_eq!(sut.pos.x, 0);
    }

    #[test]
    fn test_move_to_line_end() {
        let mut sut = make_sut(1);

        _ = sut.move_cursor_to_line_end();

        assert_eq!(sut.pos.x, 11);
    }

    #[test]
    fn test_sort_correctly() {
        let mut lines = vec![];
        lines.push(Item {
            is_hidden: false,
            file_type: FileType::File,
            file_ext: Some(".txt".into()),
            file_name: String::from("6hello_world"),
            file_path: PathBuf::new(),
        });
        for i in 0..3 {
            lines.push(Item {
                is_hidden: false,
                file_type: FileType::File,
                file_ext: Some(".txt".into()),
                file_name: i.to_string() + "hello_world",
                file_path: PathBuf::new(),
            });
        }
        for i in 3..6 {
            lines.push(Item {
                is_hidden: false,
                file_type: FileType::Directory,
                file_ext: Some(".txt".into()),
                file_name: i.to_string() + "hello_world",
                file_path: PathBuf::new(),
            });
        }
        let area = Rect::new(10, 10, 10, 10);
        let sut = FileListComponent::new(lines, area);

        let items = sut.items;

        assert_eq!(items.len(), 7);
        assert_eq!(items[0].item.file_type, FileType::Directory);
        assert_eq!(items[1].item.file_type, FileType::Directory);
        assert_eq!(items[2].item.file_type, FileType::Directory);
        assert_eq!(items[0].display_name, "3hello_world/");
        assert_eq!(items[3].display_name, "0hello_world");
        assert_eq!(items[4].item.file_type, FileType::File);
        assert_eq!(items[5].item.file_type, FileType::File);
        assert_eq!(items[5].display_name, "2hello_world");
        assert_eq!(items[6].item.file_type, FileType::File);
        assert_eq!(items[6].display_name, "6hello_world");
    }
}
