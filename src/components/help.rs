use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
};

use crate::app::Focus;

pub struct Help {
    help_text: String,
}

impl Help {
    pub fn new() -> Self {
        Self {
            help_text: String::new(),
        }
    }
    
    pub fn reset(&mut self) {
        self.help_text.clear();
    }

    pub fn update(&mut self, focus: Focus) {
        self.reset();
        match focus {
            Focus::Expr => {
                self.help_text = String::from("TODO: Help Expression Text");
            }
            Focus::Domain => {
                self.help_text = String::from("TODO: Help Domain Text");
            }
            Focus::Graph => {
                self.help_text = String::from("TODO: Help Graph Text");
            }
        }
    }

    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        let widget: Paragraph = Paragraph::new(self.help_text.as_str())
            .style(Style::default().fg(Color::Green));
        f.render_widget(widget, area);
    }
}