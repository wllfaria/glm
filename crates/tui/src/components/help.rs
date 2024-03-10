use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
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
        let lines = vec![
            Line::from(vec![" -".red(), " Move up to parent directory".blue()]),
            Line::from(vec![
                " Enter".red(),
                " Open the entry under the cursor".blue(),
            ]),
            Line::from(vec![" q".red(), " Quit glm".blue()]),
            Line::from(vec![" ?".red(), " Toggle this help pane".blue()]),
            Line::from(vec![" H".red(), " Toggle hidden files".blue()]),
        ];

        // _       Open oil in Neovim's current working directory
        // gs      Change the sort order
        // gx      Open the entry under the cursor in an external program
        // g.      Toggle hidden files and directories
        // <CR>    Open the entry under the cursor
        // <C-p>   Open the entry under the cursor in a preview window, or close the preview window if already open
        // <C-t>   Open the entry under the cursor in a new tab
        // -       Navigate to the parent path
        // <C-c>   Close oil and restore original buffer
        // g?      Show default keymaps
        // g\      Jump to and from the trash for the current directory
        // `       :cd to the current oil directory
        // <C-h>   Open the entry under the cursor in a horizontal split
        // <C-s>   Open the entry under the cursor in a vertical split
        // ~       :tcd to the current oil directory
        // <C-l>   Refresh current directory list
        let help = Paragraph::new(lines).block(
            Block::new()
                .borders(Borders::TOP)
                .border_style(Style::new().fg(Color::Gray)),
        );
        f.render_widget(help, area);
        Ok(())
    }
}
