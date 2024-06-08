use std::io;
use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    prelude::*,
    widgets::*,
};

pub struct Home {
    home_text: String,
}

impl Home {
    pub fn new() -> Self {
        Self {
            home_text: String::from("Welcome to Terminal EQ Grapher"),
        }
    }
    
    pub fn event(&mut self, _key: KeyEvent) -> io::Result<bool> {
        Ok(false)
    }

    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        let widget = Paragraph::new(self.home_text.as_str())
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(widget, area);
    }
}