use crossterm::event::{self, EnableMouseCapture, Event, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use crossterm::event::DisableMouseCapture;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::event::KeyCode;

use ratatui::backend::CrosstermBackend;
use ratatui::prelude::Backend;
use ratatui::Terminal;

use std::io;
use std::error::Error;

use terminal_queue::app::{App, CurrentScreen, CurrentlyInputting};
use terminal_queue::ui::ui;


fn main() -> Result<(), Box<dyn Error>> {

    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create the app and run
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);


    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            println!("Success!");
        }
    }
    else if let Err(err) = res {
        print!("{err:?}");
    }
    Ok(())
}


// inputs
// ------
// terminal: is the Terminal<Backend> that was instantiated in main
// App: is the Application State that was instantiated in main (defined in app.rs)
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App,) -> io::Result<bool> {

    loop {
        // draw is the ratatui comand to draw a Fram to the terminal
        // |f| ui(f, &app) tells draw that we want to take f: <Frame>
        // and pass it to our function ui, and ui will draw to that Frame.
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // essentially skil events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Enter => {
                            if let Some(inputting) = &app.currently_inputting {
                                match inputting {
                                    CurrentlyInputting::Expression => {
                                        app.currently_inputting = Some(CurrentlyInputting::Xdomain);
                                    }
                                    CurrentlyInputting::Xdomain => { //TODO
                                        // if Xdomain has been input then...
                                        // compute graph vector
                                        let result = app.eval_expr();
                                        // TODO: research into handling errors in a better way
                                        match result {
                                            Ok(_vec) => {
                                                app.current_screen = CurrentScreen::Success; // on successful method call app.eval_expr()
                                            }, 
                                            Err(_err) => {
                                                app.current_screen = CurrentScreen::Failure; // on failed method call app.eval_expr()
                                            },
                                        };
                                    }
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            if let Some(inputting) = &app.currently_inputting {
                                match inputting {
                                    CurrentlyInputting::Expression => {
                                        app.expression_input.pop();
                                    }
                                    CurrentlyInputting::Xdomain => {
                                        app.x_domain_input.pop();
                                    }
                                }
                            }
                        }
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                            app.currently_inputting = None;
                        }
                        KeyCode::Tab => {
                            app.toggle_input();
                        }
                        KeyCode::Char('q') => {
                            app.current_screen = CurrentScreen::Exiting;
                        }
                        KeyCode::Char(value) => {
                            if let Some(inputting) = &app.currently_inputting {
                                match inputting {
                                    CurrentlyInputting::Expression => {
                                        app.expression_input.push(value);
                                    }
                                    CurrentlyInputting::Xdomain => {
                                        app.x_domain_input.push(value);
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
                            app.current_screen = CurrentScreen::Exiting;
                        }
                        // use case: user wants to reset the application and enter in a new equation and x domain
                        KeyCode::Char('r') => {
                            app.reset();
                        }
                        _ => {}
                    }
                }
                // use case: expression | x domain parsing failed => options to reset app or exit program
                CurrentScreen::Failure if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Char('q') => {
                            app.current_screen = CurrentScreen::Exiting;
                        }
                        KeyCode::Char('r') => {
                            app.reset();
                        }
                        _ => {}
                    }
                }
                // use case: exit the program
                CurrentScreen::Exiting if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Char('y') => {
                            return Ok(true);
                        }
                        KeyCode::Char('n') | KeyCode::Char('q') => {
                            return Ok(false);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}


//
// Longer-term goals:
// 1. migrate to new repository: Equation-Grapher
// 2. document
// 3. create flow-chart
// 4. refactor
// 5. improve functionality
//