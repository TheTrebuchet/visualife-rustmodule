use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

// here im trying to implement this idea, that in python
// canvas = Canvas(...)
// circle = Circle(canvas, ...)

// so far it looks like I can't properly call a method of the python object and pass the newly created (for ex.) circle object to it
// intuitively I see that passing the circle to it as the circle is being created is not really possible
// like, this struct hasn't been converted to python object yet

pub trait ToSvg: Send{
    //fn as_any(&self) -> &dyn std::any::Any;
    fn to_svg(&self) -> String;
}

impl ToSvg for Circle {
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
struct Circle {
    id: String,
    radius: f64,
    cx: f64,
    cy: f64,
    children: Arc<Mutex<Vec<PyObject>>>
}

#[pymethods]
impl Circle {
    #[new]
    fn new(_py: Python, parent: &PyAny, id: String, radius: f64, cx: f64, cy: f64) -> Self {
        let children = Arc::new(Mutex::new(Vec::new()));
        let circle = Circle{id, radius, cx, cy, children};
        let parent_ref: Py<PyAny> = parent.into();
        //parent_ref.call_method1("add_child", (circle,)).unwrap(); ??????

        circle
    }
    fn add_child(&self, py: Python, child: Py<PyAny>) {
        let mut children = self.children.lock().unwrap();
        children.push(child.clone_ref(py));
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Circle{}", self.id))
    }
}

pub trait AddChild: {
    //fn as_any(&self) -> &dyn std::any::Any;
    fn add_child(&self, child: Py<PyAny>) -> &Self;
}

impl AddChild for Circle {
    fn add_child(&self, child: Py<PyAny>) -> &Self {
        let mut children = self.children.lock().unwrap();
        children.push(child);
        self
    }
}

impl AddChild for Canvas {
    fn add_child(&self, child: Py<PyAny>) -> &Self {
        let mut children = self.children.lock().unwrap();
        children.push(child);
        self
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

    fn generate_string(&self) -> PyResult<String> {
        let str = "svg".to_string();
        Ok(str)
    }
}

#[pymodule]
fn svg_gen(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Circle>()?;
    m.add_class::<Canvas>()?;
    Ok(())
}
