use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

use crate::Style;

#[pyclass]
pub struct Circle {
    id: String,
    #[pyo3(get, set)]
    radius: f32,
    #[pyo3(get, set)]
    cx: f32,
    #[pyo3(get, set)]
    cy: f32,
    #[pyo3(get, set)]
    style: Style,
    children: Arc<Mutex<Vec<PyObject>>>,
}

#[pymethods]
impl Circle {
    #[new]
    fn new(_py: Python, id: String, radius: f32, cx: f32, cy: f32) -> Self {
        let children = Arc::new(Mutex::new(Vec::new()));
        let style = Style::new(_py);
        let circle = Circle {
            id,
            radius,
            cx,
            cy,
            children,
            style,
        };
        circle
    }
    fn add_child(&self, py: Python, child: Py<PyAny>) {
        let mut children = self.children.lock().unwrap();
        children.push(child.clone_ref(py));
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Circle{}", self.id))
    }
    fn to_svg(&self) -> String {
        let mut svg_string = format!(
            r#"<circle id="{}" cx="{}" cy="{}" r="{}""#,
            self.id, self.cx, self.cy, self.radius
        );
        let style_string = self.style.to_string();
        if !style_string.is_empty() {
            svg_string.push_str(&format!(r#" style="{}""#, style_string));
        };
        svg_string.push_str(r#" />"#);
        svg_string
    }
}

#[pyclass]
pub struct Rect {
    id: String,
    #[pyo3(get, set)]
    x: f32,
    #[pyo3(get, set)]
    y: f32,
    #[pyo3(get, set)]
    width: f32,
    #[pyo3(get, set)]
    height: f32,
    rx: Option<f32>,
    ry: Option<f32>,
    #[pyo3(get, set)]
    style: Style,
    children: Arc<Mutex<Vec<PyObject>>>,
}
#[pymethods]
impl Rect {
    #[new]
    fn new(_py: Python, id: &str, x: f32, y: f32, width: f32, height: f32) -> Self {
        let children = Arc::new(Mutex::new(Vec::new()));
        let style = Style::new(_py);
        let rect = Rect {
            id: id.to_string(),
            x,
            y,
            width,
            height,
            rx: None,
            ry: None,
            children,
            style,
        };
        rect
    }

    fn add_child(&self, py: Python, child: Py<PyAny>) {
        let mut children = self.children.lock().unwrap();
        children.push(child.clone_ref(py));
    }

    fn to_svg(&self) -> String {
        let mut svg_string = format!(
            r#"<rect id="{}" x="{}" y="{}" width="{}" height="{}""#,
            self.id, self.x, self.y, self.width, self.height
        );

        if let Some(rx) = self.rx {
            svg_string.push_str(&format!(r#" rx="{}""#, rx));
        }

        if let Some(ry) = self.ry {
            svg_string.push_str(&format!(r#" ry="{}""#, ry));
        }

        let style_string = self.style.to_string();
        if !style_string.is_empty() {
            svg_string.push_str(&format!(r#" style="{}""#, style_string));
        }

        if self.style.angle != 0.0 {
            svg_string.push_str(&format!(
                r#" transform="rotate({} {} {})""#,
                self.style.angle,
                self.x + self.width / 2.0,
                self.y + self.height / 2.0
            ));
        }
        svg_string.push_str(r#" />"#);
        svg_string
    }
}
