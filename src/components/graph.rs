use std::io;
use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
};
use meval::Error as MevalError;

pub struct Graph {
    pub coordinate_vector: Vec<(f64, f64)>,
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

impl Graph {
    // constructor
    pub fn new() -> Self {
        Self {
            coordinate_vector: Vec::new(),
            x_min: 0.0,
            x_max: 0.0,
            y_min: 0.0,
            y_max: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.coordinate_vector = Vec::new();
        self.x_min = 0.0;
        self.x_max = 0.0;
        self.y_min = 0.0;
        self.y_max = 0.0;
    }

    pub fn event(&mut self, key: KeyEvent, expr: &String, domain: &String) -> io::Result<bool> {
        if key.code == KeyCode::Enter {
            match self.eval_expr(&expr, &domain) {
                Ok(_state) => {
                    return Ok(true)
                }
                Err(_err) => { // 
                    return Ok(false)
                }
            }
        }
        Ok(false)
    }
    
    pub fn eval_expr(&mut self, expr: &String, domain: &String) -> Result<Vec<(f64,f64)>, MevalError> {

        let expr: meval::Expr = expr.parse()?;
        let func = expr.bind("x")?;

        let mut iter = domain.split("..");
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


    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        let dataset = vec![
            Dataset::default()
                .name("Graph")
                .marker(Marker::Dot)
                .graph_type(GraphType::Line)
                .style(Style::default().cyan())
                .data(&self.coordinate_vector)
        ];

        let x_axis = Axis::default()
            .title("X-Axis".blue())
            .style(Style::default().white())
            .bounds([self.x_min, self.x_max])
            .labels(vec![self.x_min.to_string().into(), self.x_max.to_string().into()]);

        let y_axis = Axis::default()
            .title("Y-Axis".blue())
            .style(Style::default().white())
            .bounds([self.y_min, self.y_max])
            .labels(vec![self.y_min.to_string().into(), self.y_max.to_string().into()]);

        let chart = Chart::new(dataset)
            .block(Block::default().title("Graph"))
            .x_axis(x_axis)
            .y_axis(y_axis);

        f.render_widget(chart, area)
    }
}