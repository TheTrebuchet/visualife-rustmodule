use pyo3::prelude::*;
use std::sync::{Arc, Mutex};
use crate::visualirs::CircleRs;
use crate::Style;

#[pyclass]
pub struct Circle {
    id: String,
    radius: f32,
    cx: f32,
    cy: f32,
    pub style: Arc<Mutex<Style>>,
    pub rs_struct: Arc<Mutex<CircleRs>>,
}

#[pymethods]
impl Circle {
    #[new]
    fn new(_py: Python, id: String, radius: f32, cx: f32, cy: f32) -> Self {
        let child = CircleRs::new(id.clone(), radius, cx, cy);
        let style = Style::new(_py);
        let circle = Circle {
            id,
            radius,
            cx,
            cy,
            rs_struct: Arc::new(Mutex::new(child)),
            style: Arc::new(Mutex::new(style)),
        };
        circle
    }
    #[setter]
    fn set_style(&mut self, _py: Python, style: Style) -> PyResult<()> {
        self.style = Arc::new(Mutex::new(style.clone()));
        let cloned_style_rs = style.rs_struct.lock().unwrap().clone();
        self.rs_struct.lock().unwrap().set_style(cloned_style_rs);
        Ok(())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Circle{}", self.id))
    }
    fn to_svg(&self) -> String {
        self.rs_struct.lock().unwrap().to_svg()
    }
}

