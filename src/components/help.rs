use ratatui::{
    Frame,
    prelude::*,
    widgets::{*},
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
            Focus::Home => {
                self.help_text = String::from("Press <Tab> to continue or <Q> to quit");
            }
            Focus::Expr => {
                self.help_text = String::from("Enter an expression (ie: sin(x)), Press <Tab> to continue or <Q> to quit");
            }
            Focus::Domain => {
                self.help_text = String::from("Enter a domain (ie: -5..5), Press <Tab> to continue or <Q> to quit");
            }
            Focus::Graph => {
                self.help_text = String::from("Press <Enter> to render graph, <Tab> to reset application, or <Q> to quit");
            }
        }
    }

    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        let widget: Paragraph = Paragraph::new(self.help_text.as_str())
            .style(Style::default().fg(Color::Green));
        f.render_widget(widget, area);
    }
}