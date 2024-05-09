
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

    // default constructor method :: begin
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
    // default constructor method :: end

    // method to reset App to original state
    pub fn reset(&mut self) {
        self.expression_input = String::new();
        self.x_domain_input = String::new();
        self.graph_vector = Vec::new();
        self.x_min = 0.0;
        self.x_max = 0.0;
        self.y_min = 0.0;
        self.y_max = 0.0;
        self.current_screen = CurrentScreen::Main;
        self.currently_inputting = None;
    }
    // function to compute graph_vector

    // seperate logic in eval_expr to multiple methods
    pub fn eval_expr(&mut self) -> Result<Vec<(f64,f64)>, MevalError> {

        // handling expr parsing errors :: begin
        let expr: meval::Expr = match self.expression_input.parse() {
            Ok(expr) => expr,
            Err(error) => return Err(error),
        };
        let func = match expr.bind("x") {
            Ok(func) => func,
            Err(error) => return Err(error),
        };
        // handling expr parsing errors :: end

        // UNSAFE CODE BELOW: I should explicitly handle potential errors, currently the program will panic! :: begin
        // Note-- I am having a hard time making the program panic!
        // look into unwrap_or_else
        // set x_min and x_max :: begin
        let mut iter = self.x_domain_input.split(".."); // iter must be mut b/c of next() func.
        let start: i64 = iter.next().unwrap_or_default().parse().unwrap_or_default(); // Range value must be a integer type
        let end: i64 = iter.next().unwrap_or_default().parse().unwrap_or_default(); // Range value must be a interger type

        // We should not be losing precision here, the user is required to enter signed integers for the x min and max values
        self.x_min = start as f64; // storing in app state as f64
        self.x_max = end as f64; // storing in app state as f64
        // set x_min and x_max :: end

        // compute y-values and update graph_vector :: begin
        self.graph_vector = (start ..= end)
            .enumerate()
            // we want to map x and f(x) -- tuple.0 accesses the counter (we do not want this), tuple.1 accesses x
            .map(|tuple| (tuple.1 as f64, func(tuple.1 as f64)))
            // collect transforms iterator into relevant collection
            .collect();
        // compute y-values and update graph_vector :: begin

        // get y_min and y_max :: begin
        // ghosting iter
        let iter = self.graph_vector.iter();
        // computing min y value
        let min_y = iter
            .map(|(_, y)| y)
            .min_by(|a, b| a.partial_cmp(b).unwrap()); // look into unwrap_or_else
        // ghosting iter
        let iter = self.graph_vector.iter();
        // computing max y value
        let max_y = iter
            .map(|(_, y)| y)
            .max_by(|a, b| a.partial_cmp(b).unwrap()); // look into unwrap_or_else
        // get y_min and y_max :: end

        // unwrap min_y
        self.y_min = match min_y {
            Some(min_y) => min_y.clone(), // could dereference min_y instead of cloning (not using min_y after this point)
            None => 0.0, // if None set self.y_min to 0.0 (default value)
        };
        // unwrap max_y
        self.y_max = match max_y {
            Some(max_y) => max_y.clone(), // could dereference max_y instead of cloning (not using max_y after this point)
            None => 0.0, // if None set self.y_max to 0.0 (default value)
        };

        // returning wrapped cloned graph vector on success (this cloned vector is only used to signal success to the caller)
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