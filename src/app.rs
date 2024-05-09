
use meval::Error as MevalError;

// enum containing the current screen being displayed to the user
pub enum CurrentScreen {
    Main,
    Success,
    Failure,
    Exiting,
}

pub enum CurrentlyInputting {
    Expression,
    Xdomain,
}

// application struct
pub struct App {
    pub expression_input: String,
    pub x_domain_input: String,
    pub graph_vector: Vec<(f64, f64)>,
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub current_screen: CurrentScreen,
    pub currently_inputting: Option<CurrentlyInputting>,
}

impl App {

    // default constructor method
    pub fn new() -> App {
        App {
            expression_input: String::new(),
            x_domain_input: String::new(),
            graph_vector: Vec::new(),
            x_min: 0.0,
            x_max: 0.0,
            y_min: 0.0,
            y_max: 0.0,
            current_screen: CurrentScreen::Main,
            currently_inputting: None,
        }
    }

    // method to reset App to original state
    pub fn reset(&mut self) {
        self.expression_input = String::new();
        self.x_domain_input = String::new();
        self.graph_vector = Vec::new();
        self.current_screen = CurrentScreen::Main;
        self.currently_inputting = None;
    }
    // function to compute graph_vector

    pub fn eval_expr(&mut self) -> Result<Vec<(f64,f64)>, MevalError> {
        let expr: meval::Expr = match self.expression_input.parse() {
            Ok(expr) => expr,
            Err(error) => return Err(error),
        };
        let func = match expr.bind("x") {
            Ok(func) => func,
            Err(error) => return Err(error),
        };

        // unsafe code: I should explicitly handle potential errors, currently the program will panic! :: begin
        // set x_min and x_max :: begin
        let mut iter = self.x_domain_input.split("..");
        let start: usize = iter.next().unwrap().parse().unwrap();
        let end: usize = iter.next().unwrap().parse().unwrap();
        self.x_min = start as f64;
        self.x_max = end as f64;
        // set x_min and x_max :: end

        // compute y-values and update graph_vector :: begin
        self.graph_vector = (start .. end)
            .enumerate()
            .map(|(x, y)| (x as f64, func(y as f64)))
            .collect();
        // compute y-values and update graph_vector :: begin

        // get y_min and y_max :: begin
        let iter = self.graph_vector.iter();
        let min_y = iter.map(|(_, y)| y).min_by(|a, b| a.partial_cmp(b).unwrap());

        let iter = self.graph_vector.iter();
        let max_y = iter.map(|(_, y)| y).max_by(|a, b| a.partial_cmp(b).unwrap());
        // get y_min and y_max :: end

        self.y_min = match min_y {
            Some(value) => value.clone(),
            None => 0.0,
        };

        self.y_max = match max_y {
            Some(value) => value.clone(),
            None => 0.0,
        };

        // returning wrapped cloned graph vector on success
        let vs: Vec<_> = self.graph_vector.clone();
        Ok(vs)
    }

    
    
    // helper method to change the current input mode
    // if current mode is None change current input to Expression
    pub fn toggle_input(&mut self) {
        if let Some(edit_mode) = &self.currently_inputting {
            match edit_mode {
                CurrentlyInputting::Expression => {
                    self.currently_inputting = Some(CurrentlyInputting::Xdomain)
                }
                CurrentlyInputting::Xdomain => {
                    self.currently_inputting = Some(CurrentlyInputting::Expression)
                }
            };
        }
        else {
            self.currently_inputting = Some(CurrentlyInputting::Expression);
        }
    }
}