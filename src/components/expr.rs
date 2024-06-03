use std::io;

use crossterm::event::KeyEvent;
use crossterm::event::KeyCode;

use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
};

pub struct Expr {
    pub expr_text: String,
}

impl Expr {
    pub fn new() -> Self {
        Self {
            expr_text: String::new(),
        }
    }

    pub fn reset(&mut self) {
        self.expr_text.clear();
    }

    pub fn event(&mut self, key: KeyEvent) -> io::Result<bool> {
        match key.code {
            KeyCode::Char(c) => {
                self.push(c);
                Ok(true)
            }
            KeyCode::Backspace => {
                self.pop();
                Ok(true)
            }
            _ => { Ok(false) }
        }
    }

    fn push(&mut self, c: char) {
        self.expr_text.push(c);
    }

    fn pop(&mut self) {
        self.expr_text.pop();
    }

    // app calls app.draw which will call expr.draw if expr is in focus...
    // app.draw will also call draw.help which will display a help msg dependent on the current focus
    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        let widget: Paragraph = Paragraph::new(self.expr_text.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Input Expression"));
        f.render_widget(widget, area);
    }
}