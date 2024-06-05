use std::io;
use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
};

pub struct Domain {
    pub domain_text: String,
}

impl Domain {
    pub fn new() -> Self {
        Self {
            domain_text: String::new(),
        }
    }

    pub fn reset(&mut self) {
        self.domain_text.clear();
    }

    pub fn event(&mut self, key: KeyEvent) -> io::Result<bool> {
        match key.code {
            KeyCode::Char(c) => {
                self.push(c);
                Ok(true)
            }
            KeyCode::Backspace => {
                self.pop();
                Ok(false)
            }
            _ => { Ok(false) }
        }
    }

    fn push(&mut self, c: char) {
        self.domain_text.push(c);
    }

    fn pop(&mut self) {
        self.domain_text.pop();
    }

    // app calls app.draw which will call domain.draw if expr is in focus...
    // app.draw will also call draw.help which will display a help msg dependent on the current focus
    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        let widget: Paragraph = Paragraph::new(self.domain_text.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Input Domain"));
        f.render_widget(widget, area);
    }
}