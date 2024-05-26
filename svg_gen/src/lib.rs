use pyo3::prelude::*;
use std::fmt;
use std::sync::Mutex;
use lazy_static::lazy_static;

#[pyclass]
struct Circle {
    radius: f64,
    x: f64,
    y: f64,
}

#[pymethods]
impl Circle {
    #[new]
    fn new(radius: f64, x: f64, y: f64) -> Self {
        let circ = Circle { radius, x, y };
        register_shape(format!("{}", circ));
        circ
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Circle(radius: {}, x: {}, y: {})", self.radius, self.x, self.y))
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle(radius: {}, x: {}, y: {})", self.radius, self.x, self.y)
    }
}

fn register_shape(shape: String) {
    let mut shapes = SHAPES.lock().unwrap();
    shapes.push(shape);
}

lazy_static! {
    static ref SHAPES: Mutex<Vec<String>> = Mutex::new(Vec::new());
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
        Ok(shapes.join(", "))
    }
}

#[pymodule]
fn svg_gen(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Circle>()?;
    m.add_class::<Canvas>()?;
    Ok(())
}
