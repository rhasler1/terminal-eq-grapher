use ratatui::{
    Frame,
    prelude::*,
    widgets::*,
};
use crate::app::Focus;

pub struct Title {
    title_text: String,
}

impl Title {
    pub fn new() -> Self {
        Self {
            title_text: String::new(),
        }
    }

    pub fn reset(&mut self) {
        self.title_text.clear();
    }

    pub fn update(&mut self, focus: Focus) {
        self.reset();
        match focus {
            Focus::Home => {
                self.title_text = String::from("HOME")
            }
            Focus::Expr => {
                self.title_text = String::from("EXPRESSION");
            }
            Focus::Domain => {
                self.title_text = String::from("DOMAIN");
            }
            Focus::Graph => {
                self.title_text = String::from("GRAPH");
            }
        }
    }

    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        let widget: Paragraph = Paragraph::new(self.title_text.as_str())
            .style(Style::default().fg(Color::Green));
        f.render_widget(widget, area);
    }
}