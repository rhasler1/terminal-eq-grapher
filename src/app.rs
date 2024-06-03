use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::event::{self, EnableMouseCapture, Event, KeyEvent, KeyEventKind};
use crossterm::event::DisableMouseCapture;
use crossterm::event::KeyCode;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui::Frame;
use ratatui::prelude::Layout;

use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};

use std::io;

use crate::components::expr::Expr;
use crate::components::domain::Domain;
use crate::components::graph::Graph;
use crate::components::title::Title;
use crate::components::help::Help;
use crate::action::Action;

#[derive(Clone, Copy)]
pub enum Focus {
    Expr,
    Domain,
    Graph,
}

pub struct App {
    pub focus: Focus,
    pub title: Title,
    pub help: Help,
    pub graph: Graph,
    pub expr: Expr,
    pub domain: Domain,
}

impl App {

    // default constructor method :: begin
    pub fn new() -> App {
        App {
            focus: Focus::Expr,
            title: Title::new(),
            help: Help::new(),
            expr: Expr::new(),
            domain: Domain::new(),
            graph: Graph::new(),
        }
    }
    // default constructor method :: end

    // method to reset App to original state
    pub fn reset(&mut self) {
        self.focus = Focus::Expr;
        self.title.reset();
        self.help.reset();
        self.expr.reset();
        self.domain.reset();
        self.graph.reset();
    }

    pub fn draw(&mut self, f: &mut ratatui::prelude::Frame) -> io::Result<bool> {
        let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(1), // Component
            Constraint::Length(3), // Help
        ])
        .split(f.size());

        // draw title::begin
        self.title.draw(f, chunks[0]);
        // draw title::end

        // draw main component{Expr, Domain, or Graph}::begin
        match self.focus {
            Focus::Expr => {
                self.expr.draw(f, chunks[1]);
            }
            Focus::Domain => {
                self.domain.draw(f, chunks[1]);
            }
            Focus::Graph => { 
                // may need to pass self.expr.expr_text and self.domain.domain_text as parameters.
                self.graph.draw(f, chunks[1]);
            }
        }
        // draw main component{Expr, Domain, or Graph}::end

        self.help.draw(f, chunks[2]);
        Ok(true)
    }

    pub fn run(&mut self) -> io::Result<bool> {
        // set up terminal::begin
        enable_raw_mode()?;
        let mut stderr: io::Stderr = io::stderr();
        execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
        let backend: CrosstermBackend<io::Stderr> = CrosstermBackend::new(stderr);
        let mut terminal: Terminal<CrosstermBackend<io::Stderr>> = Terminal::new(backend)?;
        // set up terminal::end

        // run loop::begin
        loop {
            // draw to terminal::begin
            terminal.draw(|f: &mut ratatui::prelude::Frame| {
                // still need to implement error handling in self.draw, self.expr.draw, ...
                self.draw(f);
            })?;
            // draw to terminal::end

            // process next event::begin
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match self.event(key) {
                        Ok(state) => {
                            // user wishes to quit app
                            if !state && key.code == KeyCode::Char('q') {
                                return Ok(true)
                            }
                        }
                        Err(_err) => {
                            // error occurred somewhere in event handling
                            return Ok(false)
                        }
                    }
                }
            }
            // process next event::end
        }
    }

    fn event(&mut self, key: KeyEvent) -> io::Result<bool> {
        if self.components_event(key)? {
            return Ok(true);
        }
        if self.move_focus(key)? {
            return Ok(true);
        }
        Ok(false)
    }

    fn move_focus(&mut self, key: KeyEvent) -> io::Result<bool> {
        match self.focus {
            Focus::Expr => {
                if key.code == KeyCode::Tab {
                    self.focus = Focus::Domain;
                    return Ok(true)
                }
            }
            Focus::Domain => {
                if key.code == KeyCode::Tab {
                    self.focus = Focus::Graph;
                    return Ok(true)
                }
            }
            Focus::Graph => {
                if key.code == KeyCode::Tab {
                    self.reset();
                    return Ok(true)
                }
            }
        }
        return Ok(false)
    }

    fn components_event(&mut self, key: KeyEvent) -> io::Result<bool> {
        match self.focus {
            Focus::Expr => {
                self.expr.event(key)?;
            }
            Focus::Domain => {
                self.domain.event(key)?;
            }
            // TODO: graph.event
            Focus::Graph => {
                // on enter generate graph
                // self.graph.event(key, self.expr.expr_title.clone(), self.domain.domain_title.clone())?
                //     -> graph.eval_expr()?
                //         -> <stored coordinated vector on success>
                return Ok(true);
            }
        }
        Ok(false)
    }



}