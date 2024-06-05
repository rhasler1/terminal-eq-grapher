use std::io::{self};
use crossterm::{
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
    terminal::{disable_raw_mode, LeaveAlternateScreen},
    event::{self, EnableMouseCapture, Event, KeyEvent, DisableMouseCapture, KeyCode}
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    prelude::*
};
use crate::components::expr::Expr;
use crate::components::domain::Domain;
use crate::components::graph::Graph;
use crate::components::title::Title;
use crate::components::help::Help;
use crate::components::home::Home;

#[derive(Clone, Copy)]
pub enum Focus {
    Home,
    Expr,
    Domain,
    Graph,
}

pub struct App {
    pub focus: Focus,
    pub title: Title,
    pub help: Help,
    pub home: Home,
    pub expr: Expr,
    pub domain: Domain,
    pub graph: Graph,
}

impl App {

    // default constructor method :: begin
    pub fn new() -> Self {
        Self {
            focus: Focus::Home,
            title: Title::new(),
            help: Help::new(),
            home: Home::new(),
            expr: Expr::new(),
            domain: Domain::new(),
            graph: Graph::new(),
        }
    }
    // default constructor method :: end

    // method to reset App to original state
    pub fn reset(&mut self) {
        self.focus = Focus::Home;
        self.title.reset();
        self.help.reset();
        //self.home.reset(); // no home.reset() method
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
            Focus::Home => {
                self.home.draw(f, chunks[1]);
            }
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
        // draw help::begin
        self.help.draw(f, chunks[2]);
        // draw help::end
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
            // update title and help blocks::begin
            self.title.update(self.focus);
            self.help.update(self.focus);
            // update title and help blocks::end

            // draw to terminal::begin
            terminal.draw(|f: &mut ratatui::prelude::Frame| {
                // TODO: implement error handling in draw
                match self.draw(f) {
                    Ok(_state) => {}//TODO
                    Err(_err) => {} //TODO
                }
            })?;
            // draw to terminal::end

            // process next event::begin
            if let Event::Key(key) = event::read()? {
                // exit application::begin
                if key.code == KeyCode::Char('q') {
                    // restore terminal
                    disable_raw_mode()?;
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    terminal.show_cursor()?;
                    return Ok(true)
                }
                // exit application::end

                if key.kind == event::KeyEventKind::Press {
                    match self.event(key) {
                        Ok(state) => {
                            // user wishes to quit app
                            if state && key.code == KeyCode::Char('q') {
                                // restore terminal
                                disable_raw_mode()?;
                                execute!(
                                    terminal.backend_mut(),
                                    LeaveAlternateScreen,
                                    DisableMouseCapture
                                )?;
                                terminal.show_cursor()?;
                                return Ok(true)
                            }
                        }
                        Err(_err) => {
                            // error occurred somewhere in event handling
                            // --this branch does not execute on parsing error in graph.eval_expr()
                            // --parsing error is currently being handled in graph.event
                            // TODO: propogate error to caller (here) and handle
                            self.reset();
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

    fn components_event(&mut self, key: KeyEvent) -> io::Result<bool> {
        match self.focus {
            Focus::Home => {
                self.home.event(key)?;
            }
            Focus::Expr => {
                self.expr.event(key)?;
            }
            Focus::Domain => {
                self.domain.event(key)?;
            }
            Focus::Graph => {
                self.graph.event(key, &self.expr.expr_text, &self.domain.domain_text)?;
            }
        }
        Ok(false)
    }

    fn move_focus(&mut self, key: KeyEvent) -> io::Result<bool> {
        match self.focus {
            Focus::Home => {
                if key.code == KeyCode::Tab {
                    self.focus = Focus::Expr;
                    return Ok(true)
                }
            }
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
}