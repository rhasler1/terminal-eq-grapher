use std::error::Error;
pub mod app;
pub mod components;
use crate::app::App;

fn main() -> Result<(), Box<dyn Error>> {
    // create the app and run
    let mut app = App::new();
    app.run()?;
    Ok(())
}

//TODO:
// Add tui struct
// move main loop from app.run() to main.rs
// replace boolean return types with "is_consumed" idea
// improve error handling