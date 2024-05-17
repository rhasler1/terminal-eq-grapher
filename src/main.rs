use std::error::Error;

pub mod app;
pub mod ui;
pub mod components;

use crate::app::App;


//use terminal_eq_grapher::app::App;
//use terminal_eq_grapher::focus::{CurrentScreen, CurrentInput};
//use terminal_eq_grapher::ui::ui;

fn main() -> Result<(), Box<dyn Error>> {
    // create the app and run
    let mut app = App::new();
    app.run()?;
    Ok(())
}


//if let Ok(do_print) = res {
//    if do_print {
//        println!("Success!");
//    }
//}
//else if let Err(err) = res {
//    print!("{err:?}");
//}
//Ok(())

//
// Longer-term goals:
// 1. migrate to new repository: Equation-Grapher
// 2. document
// 3. create flow-chart
// 4. refactor
// 5. improve functionality
//