use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::event::{self, EnableMouseCapture, Event, KeyEventKind};
use crossterm::event::DisableMouseCapture;
use crossterm::event::KeyCode;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use std::io;

use crate::ui::ui;
use crate::components::focus::{CurrentInput, CurrentScreen};
use crate::components::{graph::Graph, focus::Focus};
use crate::components::input::Input;
use crate::action::Action;

pub struct App {
    pub graph: Graph,
    pub focus: Focus,
    pub input: Input,
}

impl App {

    // default constructor method :: begin
    pub fn new() -> App {
        App {
            graph: Graph::new(),
            focus: Focus::new(),
            input: Input::new(),
        }
    }
    // default constructor method :: end

    // method to reset App to original state
    pub fn reset(&mut self) {
        self.graph.reset();
        self.focus.reset();
        self.input.reset();
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
            //terminal.draw(|f| ui(f, self))?;
            // testing component draw
            terminal.draw(|f| {
                ui(f, self);
                self.input.draw(f, f.size());
            })?;


            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    // skip events that are not KeyEventKind::Press
                    continue;
                }

                // get action and send to components
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Tab => {
                            self.input.update();
                            //Action::ChangeFocus
                        }
                        KeyCode::Enter => {
                            self.graph.update();
                            //Action::Submit
                        }
                        KeyCode::Char('r') => {
                            self.reset();
                            //Action::Reset
                        }
                        KeyCode::Char('q') => {
                            //self.quit();
                            //Action::Quit
                        }
                        KeyCode::Backspace => {
                            self.input.pop_input();
                        }
                        KeyCode::Char(c) => {
                            self.input.push_input(c);
                        }
                        _ => {}
                    }
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
                                        CurrentInput::Xdomain => {
                                            // if Xdomain has been input then compute graph vector
                                            let result = self.graph.eval_expr();
                                            match result {
                                                Ok(_vec) => {
                                                    self.focus.current_screen = CurrentScreen::Success; // on successful method call to app.eval_expr()
                                                }, 
                                                Err(_err) => {
                                                    self.focus.current_screen = CurrentScreen::Failure; // on failed method call to app.eval_expr()
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
                                            self.graph.pop_expression_input();
                                        }
                                        CurrentInput::Xdomain => {
                                            self.graph.pop_x_domain_input();
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
                                            self.graph.push_expression_input(value);
                                        }
                                        CurrentInput::Xdomain => {
                                            self.graph.push_x_domain_input(value);
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
                            KeyCode::Char('y' | 'q') => {
                                break;
                            }
                            KeyCode::Char('n') => {
                                self.reset();
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