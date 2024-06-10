use pyo3::prelude::*;
use std::sync::{Arc, Mutex};
use crate::visualirs::RectRs;
use crate::Style;
use crate::visualirs::shapes_rs::ToSvg;

#[pyclass]
pub struct Rect {
    id: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    rx: Option<f32>,
    ry: Option<f32>,
    style: Style,
    rs_struct: Arc<Mutex<RectRs>>,
}
#[pymethods]
impl Rect {
    #[new]
    fn new(_py: Python, id: &str, x: f32, y: f32, width: f32, height: f32) -> Self {
        let child = RectRs::new(id, x, y, width, height);
        let style = Style::new(_py);
        let rect = Rect {
            id: id.to_string(),
            x,
            y,
            width,
            height,
            rx: None,
            ry: None,
            style,
            rs_struct: Arc::new(Mutex::new(child)),
        };
        rect
    }
    pub fn to_svg(&self) -> String {
        self.rs_struct.lock().unwrap().to_svg()
    }
}
