use meval::Error as MevalError;

pub enum GraphState {
    CanDraw,
    CannotDraw,
}

pub struct Graph {
    pub graph_state: GraphState,
    pub coordinate_vector: Vec<(f64, f64)>,
    pub expression_input: String,
    pub x_domain_input: String,
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

impl Graph {
    // constructor
    pub fn new() -> Self {
        Self {
            graph_state: GraphState::CannotDraw,
            coordinate_vector: Vec::new(),
            expression_input: String::new(),
            x_domain_input: String::new(),
            x_min: 0.0,
            x_max: 0.0,
            y_min: 0.0,
            y_max: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.graph_state = GraphState::CannotDraw;
        self.coordinate_vector = Vec::new();
        self.expression_input = String::new();
        self.x_domain_input = String::new();
        self.x_min = 0.0;
        self.x_max = 0.0;
        self.y_min = 0.0;
        self.y_max = 0.0;
    }

    pub fn update(&mut self) {}
    
    pub fn eval_expr(&mut self) -> Result<Vec<(f64,f64)>, MevalError> {

        let expr: meval::Expr = self.expression_input.parse()?;
        let func = expr.bind("x")?;

        let mut iter = self.x_domain_input.split("..");
        let start: i64 = iter.next().unwrap_or_default().parse().unwrap_or_default();
        let end: i64 = iter.next().unwrap_or_default().parse().unwrap_or_default();
        self.x_min = start as f64;
        self.x_max = end as f64;

        self.coordinate_vector = (start ..= end)
            .enumerate()
            .map(|tuple| (tuple.1 as f64, func(tuple.1 as f64)))
            .collect();

        let iter = self.coordinate_vector.iter();
        let min_y = iter
            .map(|(_, y)| y)
            .min_by(|a, b| a.partial_cmp(b).unwrap());
        let iter = self.coordinate_vector.iter();
        let max_y = iter
            .map(|(_, y)| y)
            .max_by(|a, b| a.partial_cmp(b).unwrap());
        self.y_min = match min_y {
            Some(min_y) => min_y.clone(),
            None => 0.0,
        };
        self.y_max = match max_y {
            Some(max_y) => max_y.clone(),
            None => 0.0,
        };

        let vs: Vec<_> = self.coordinate_vector.clone();
        Ok(vs)
    }

    pub fn pop_input(&mut self) {
        
    }

    pub fn push_expression_input(&mut self, value: char) {
        self.expression_input.push(value);
    }

    pub fn push_x_domain_input(&mut self, value: char) {
        self.x_domain_input.push(value);
    }

    pub fn pop_expression_input(&mut self) {
        self.expression_input.pop();
    }

    pub fn pop_x_domain_input(&mut self) {
        self.x_domain_input.pop();
    }

    pub fn draw(&mut self) {
        //if the state is CanDraw then draw, else don't.
    }

}