use pyo3::prelude::*;
use std::sync::{Arc, Mutex};
use crate::visualirs::StyleRs;
// partially copied from github.com/dgront/rust-experiments/tree/master/vl_trial

#[pyclass]
#[derive(Clone)]
pub struct Style {
    fill: Option<String>,
    stroke: Option<String>,
    stroke_width: Option<f32>,
    opacity: Option<f32>,
    fill_opacity: Option<f32>,
    stroke_opacity: Option<f32>,
    angle: f32,
    pub rs_struct: Arc<Mutex<StyleRs>>
}
#[pymethods]
impl Style {
    #[new]
    pub fn new(_py: Python) -> Self {
        let child = StyleRs::new();
        Style {
            fill: None,
            stroke: None,
            stroke_width: None,
            opacity: None,
            fill_opacity: None,
            stroke_opacity: None,
            angle: 0.0,
            rs_struct: Arc::new(Mutex::new(child))
        }
    }
    pub fn set_fill(&mut self, fill: &str) {
        self.fill = Some(fill.to_string());
    }

    pub fn set_stroke(&mut self, stroke: &str) {
        self.stroke = Some(stroke.to_string());
    }

    pub fn set_stroke_width(&mut self, stroke_width: f32) {
        self.stroke_width = Some(stroke_width);
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = Some(opacity);
    }

    pub fn set_fill_opacity(&mut self, fill_opacity: f32) {
        self.fill_opacity = Some(fill_opacity);
    }

    pub fn set_stroke_opacity(&mut self, stroke_opacity: f32) {
        self.stroke_opacity = Some(stroke_opacity);
    }

    pub fn set_angle(&mut self, angle_deg: f32) {
        self.angle = angle_deg;
    }
}
