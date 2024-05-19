use std::error::Error;

pub mod app;
pub mod ui;
pub mod components;

use crate::app::App;

// TODO:
// 1. implement tui struct
// 2. move render logic to components
// 3. implement tokio for async event handling
// 4. document
// 5. improve ui

fn main() -> Result<(), Box<dyn Error>> {
    // create the app and run
    let mut app = App::new();
    app.run()?;
    Ok(())
}