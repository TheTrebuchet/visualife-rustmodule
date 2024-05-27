use pyo3::prelude::*;
use std::sync::Mutex;
use lazy_static::lazy_static;

pub trait Shape: Send {
    fn as_any(&self) -> &dyn std::any::Any;
    fn to_svg(&self) -> String;
}

impl Shape for Circle {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_svg(&self) -> String {
        let mut svg_string = format!(
            r#"<circle cx="{}" cy="{}" r="{}""#,
            self.cx, self.cy, self.radius
        );
        svg_string.push_str(r#" />"#);
        svg_string
    }
}

#[pyclass]
#[derive(Clone)]
struct Circle {
    radius: f64,
    cx: f64,
    cy: f64,
}

#[pymethods]
impl Circle {
    #[new]
    fn new(radius: f64, cx: f64, cy: f64) -> Self {
        let circle = Circle { radius, cx, cy };
        register_shape(Box::new(circle.clone()));
        circle
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Circle(radius: {}, cx: {}, cy: {})", self.radius, self.cx, self.cy))
    }
}


lazy_static! {
    static ref SHAPES: Mutex<Vec<Box<dyn Shape>>> = Mutex::new(Vec::new());
}

fn register_shape(shape: Box<dyn Shape>) {
    let mut shapes = SHAPES.lock().unwrap();
    shapes.push(shape);
}

#[pyclass]
struct Canvas;

#[pymethods]
impl Canvas {
    #[new]
    fn new() -> Self {
        Canvas
    }

    fn generate_string(&self) -> PyResult<String> {
        let shapes = SHAPES.lock().unwrap();
        let result = shapes.iter().map(|shape| shape.to_svg()).collect::<Vec<_>>().join("\n");
        Ok(result)
    }
}

#[pymodule]
fn svg_gen(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Circle>()?;
    m.add_class::<Canvas>()?;
    Ok(())
}
