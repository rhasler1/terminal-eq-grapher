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
// 6. add robustness to component::graph::Graph & Graph.eval_expr()
// 6.5 research the capabilities of Ratatui Chart

// Known Unhandled Errors:
// division by 0 results in unhandled error.

fn main() -> Result<(), Box<dyn Error>> {
    // create the app and run
    let mut app = App::new();
    app.run()?;
    Ok(())
}