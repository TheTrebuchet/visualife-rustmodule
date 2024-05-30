use pyo3::prelude::*;
use std::sync::{Arc, Mutex};
use pyo3::types::PyAny;

// here im trying to implement this idea, that in python
// canvas = Canvas(...)
// circle = Circle(...)
// canvas.add_child(circle)

// we should totally call them students and scientists or something

#[pyclass]
struct Circle {
    id: String,
    #[pyo3(get, set)]
    radius: f64,
    #[pyo3(get, set)]
    cx: f64,
    #[pyo3(get, set)]
    cy: f64,
    children: Arc<Mutex<Vec<PyObject>>>
}

#[pymethods]
impl Circle {
    #[new]
    fn new(_py: Python, id: String, radius: f64, cx: f64, cy: f64) -> Self {
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


#[pyclass]
struct Canvas {
    children: Arc<Mutex<Vec<PyObject>>>
}

#[pymethods]
impl Canvas {
    #[new]
    fn new() -> Self {
        let children = Arc::new(Mutex::new(Vec::new()));
        Canvas { children }
    }
    fn add_child(&self, py: Python, child: Py<PyAny>) {
        let mut children = self.children.lock().unwrap();
        children.push(child.clone_ref(py));
        println!("child adopted")
    }
    fn generate_string(&self, py: Python) -> PyResult<String> {
        let mut combined_svg = String::new();
        let locked_children = self.children.lock().unwrap();
        for obj in locked_children.iter() {
            let py_any: &PyAny = obj.as_ref(py);
            let result: Result<String, PyErr> = py_any.call_method0("to_svg").unwrap().extract();
            let svg_string = match result {
                Ok(svg) => svg,
                Err(err) => {
                    eprintln!("Error extracting SVG string: {}", err);
                    continue; 
                }
            };
            combined_svg.push_str(&svg_string)
        };
        Ok(combined_svg)
    }
}

#[pymodule]
fn svg_gen(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Circle>()?;
    m.add_class::<Canvas>()?;
    Ok(())
}
