use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::event::{self, EnableMouseCapture, Event, KeyEventKind};
use crossterm::event::DisableMouseCapture;
use crossterm::event::KeyCode;

use ratatui::backend::CrosstermBackend;
//use ratatui::prelude::Backend;
use ratatui::Terminal;

//use std::error::Error;
use std::io;

use crate::ui::ui;

use crate::components::focus::{CurrentInput, CurrentScreen};

use crate::components::{graph::Graph, focus::Focus};

// application struct
pub struct App {
    pub graph: Graph,
    pub focus: Focus,
}

impl App {

    // default constructor method :: begin
    pub fn new() -> App {
        App {
            graph: Graph::new(),
            focus: Focus::new(),
        }
    }
    // default constructor method :: end

    // method to reset App to original state
    pub fn reset(&mut self) {
        self.graph.reset();
        self.focus.reset();
    }

    pub fn run(&mut self) -> io::Result<bool> {
        // setup terminal
        enable_raw_mode()?;
        let mut stderr = io::stderr();
        execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stderr);
        let mut terminal = Terminal::new(backend)?;

        loop {
            // draw is the ratatui comand to draw a Fram to the terminal
            // |f| ui(f, &app) tells draw that we want to take f: <Frame>
            // and pass it to our function ui, and ui will draw to that Frame.
            terminal.draw(|f| ui(f, self))?;
    
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    // essentially skil events that are not KeyEventKind::Press
                    continue;
                }
                match self.focus.current_screen {
                    CurrentScreen::Main if key.kind == KeyEventKind::Press => {
                        match key.code {
                            KeyCode::Enter => {
                                if let Some(input) = &self.focus.current_input {
                                    match input {
                                        CurrentInput::Expression => {
                                            self.focus.current_input = Some(CurrentInput::Xdomain);
                                        }
                                        CurrentInput::Xdomain => { //TODO
                                            // if Xdomain has been input then...
                                            // compute graph vector
                                            let result = self.graph.eval_expr();
                                            // TODO: research into handling errors in a better way
                                            match result {
                                                Ok(_vec) => {
                                                    self.focus.current_screen = CurrentScreen::Success; // on successful method call app.eval_expr()
                                                }, 
                                                Err(_err) => {
                                                    self.focus.current_screen = CurrentScreen::Failure; // on failed method call app.eval_expr()
                                                },
                                            };
                                        }
                                    }
                                }
                            }
                            KeyCode::Backspace => {
                                if let Some(input) = &self.focus.current_input {
                                    match input {
                                        CurrentInput::Expression => {
                                            self.graph.expression_input.pop();
                                        }
                                        CurrentInput::Xdomain => {
                                            self.graph.x_domain_input.pop();
                                        }
                                    }
                                }
                            }
                            KeyCode::Esc => {
                                self.focus.current_screen = CurrentScreen::Main;
                                self.focus.current_input = None;
                            }
                            KeyCode::Tab => {
                                self.focus.toggle_input();
                            }
                            KeyCode::Char('q') => {
                                self.focus.current_screen = CurrentScreen::Exiting;
                            }
                            KeyCode::Char(value) => {
                                if let Some(input) = &self.focus.current_input {
                                    match input {
                                        CurrentInput::Expression => {
                                            self.graph.expression_input.push(value);
                                        }
                                        CurrentInput::Xdomain => {
                                            self.graph.x_domain_input.push(value);
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    
                    CurrentScreen::Success if key.kind == KeyEventKind::Press => {
                        match key.code {
                            // use case: user wants to exit the application
                            KeyCode::Char('q') => {
                                self.focus.current_screen = CurrentScreen::Exiting;
                            }
                            // use case: user wants to reset the application and enter in a new equation and x domain
                            KeyCode::Char('r') => {
                                self.reset();
                            }
                            _ => {}
                        }
                    }
                    // use case: expression | x domain parsing failed => options to reset app or exit program
                    CurrentScreen::Failure if key.kind == KeyEventKind::Press => {
                        match key.code {
                            KeyCode::Char('q') => {
                                self.focus.current_screen = CurrentScreen::Exiting;
                            }
                            KeyCode::Char('r') => {
                                self.reset();
                            }
                            _ => {}
                        }
                    }
                    // use case: exit the program
                    CurrentScreen::Exiting if key.kind == KeyEventKind::Press => {
                        match key.code {
                            KeyCode::Char('y') => {
                                break;
                            }
                            KeyCode::Char('n') | KeyCode::Char('q') => {
                                break;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(true)
    }
}