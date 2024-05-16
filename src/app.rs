use crate::graph::Graph;
use crate::focus::Focus;

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
}