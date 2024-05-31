use pyo3::prelude::*;
use std::sync::{Arc, Mutex};
use pyo3::types::PyAny;
mod canvas;

pub use crate::canvas::Canvas;

// here im trying to implement this idea, that in python
// canvas = Canvas(...)
// circle = Circle(...)
// canvas.add_child(circle)

// we should totally call them students and scientists or something

#[pyclass]
struct Circle {
    id: String,
    #[pyo3(get, set)]
    radius: f32,
    #[pyo3(get, set)]
    cx: f32,
    #[pyo3(get, set)]
    cy: f32,
    children: Arc<Mutex<Vec<PyObject>>>
}

#[pymethods]
impl Circle {
    #[new]
    fn new(_py: Python, id: String, radius: f32, cx: f32, cy: f32) -> Self {
        let children = Arc::new(Mutex::new(Vec::new()));
        let circle = Circle{id, radius, cx, cy, children};
        circle
    }
    fn add_child(&self, py: Python, child: Py<PyAny>) {
        let mut children = self.children.lock().unwrap();
        children.push(child.clone_ref(py));
        println!("child adopted")
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Circle{}", self.id))
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

#[pymodule]
fn svg_gen(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Canvas>()?;
    m.add_class::<Circle>()?;
    Ok(())
}
