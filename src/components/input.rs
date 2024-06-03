use ratatui::Frame;
use ratatui::prelude::Layout;

use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};

use crate::action::Action;

pub enum InputState {
    InputExpr,
    InputDomain,
}

pub struct Input {
    pub input_state: Option<InputState>,
    pub input_expr: String,
    pub input_domain: String,
}

impl Input {
    pub fn new() -> Input {
        Self {
            input_state: None,
            input_expr: String::new(),
            input_domain: String::new(),
        }
    }

    pub fn reset(&mut self) {
        self.input_state = None;
        self.input_expr = String::new();
        self.input_domain = String::new();
    }

    // maybe pass in action as an argument as well.
    // function to change state.
    pub fn update(&mut self) {
        if let Some(input_mode) = &self.input_state {
            match input_mode {
                InputState::InputExpr => {
                    self.input_state = Some(InputState::InputDomain)
                }
                InputState::InputDomain => {
                    self.input_state = Some(InputState::InputExpr)
                }
            };
        }
        else { self.input_state = Some(InputState::InputExpr); } // case input_state is None.
    }

    pub fn pop_input(&mut self) {
        if let Some(input_mode) = &self.input_state {
            match input_mode {
                InputState::InputExpr => {
                    self.input_expr.pop();
                }
                InputState::InputDomain => {
                    self.input_domain.pop();
                }
            }
        }
    }

    pub fn push_input(&mut self, c: char) {
        if let Some(input_mode) = &self.input_state {
            match input_mode {
                InputState::InputExpr => {
                    self.input_expr.push(c);
                }
                InputState::InputDomain => {
                    self.input_domain.push(c);
                }
            }
        }
    }






    // #    DRAWING / RENDERING BELOW
    //
    //

    // helper functions for draw
    pub fn draw_input_state(&mut self, f: &mut Frame, footer_chunks: Rect) {
        let input_text = {
            if let Some(input) = &self.input_state {
                match input {
                    InputState::InputExpr => Span::styled(
                        "Input Expression",
                        Style::default().fg(Color::Green),
                    ),
                    InputState::InputDomain => Span::styled(
                        "Input Domain",
                        Style::default().fg(Color::Green),
                    ),
                }
            } else {
                Span::styled(
                    "Input None",
                    Style::default().fg(Color::Green),
                )
            }
        };
        f.render_widget(input_text, footer_chunks)
    }

    pub fn draw_input_expr(&mut self, f: &mut Frame, footer_chunks: Rect) {
        let expr_text = Paragraph::new(self.input_expr.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Expr Input"));
        f.render_widget(expr_text, footer_chunks);
    }

    pub fn draw_input_domain(&mut self, f: &mut Frame, footer_chunks: Rect) {
        let domain_text = Paragraph::new(self.input_domain.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Domain Input"));
        f.render_widget(domain_text, footer_chunks)
    }

    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        let chunks: std::rc::Rc<[Rect]> = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(1), // Graph
                Constraint::Length(3), // Inputs
            ])
            .split(area);

        let footer_chunks: std::rc::Rc<[Rect]> = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25), // state
                Constraint::Percentage(25), // expr
                Constraint::Percentage(25), // domain
                Constraint::Percentage(25), // help
            ])
            .split(chunks[2]);

        self.draw_input_state(f, footer_chunks[0]);
        self.draw_input_expr(f, footer_chunks[1]);
        self.draw_input_domain(f, footer_chunks[2]);

        // temp code below to render a title block,
        // move to a different componenet.
        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let title = Paragraph::new(Text::styled(
            "Test Me!",
            Style::default().fg(Color::Green),
        ))
        .block(title_block);
        f.render_widget(title, chunks[0]);
    }
}