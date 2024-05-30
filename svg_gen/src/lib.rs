use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::sync::{Arc, Mutex};

//this is the new strategy
//each object can hold an infinite number of references to other objects, hence we have a tree
//when creating an object we give it the "parent" as parameter, this parent then has a method executed 
// (in init) called .add_child which gives that parent a reference of this newly created object
// when we finally create the drawing we call the first "master" canvas,
// this canvas will have a method for creating the so-called tree and this will be a problematic operation, 
// hence here we will use the rust code completely


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
    children: Arc<Mutex<Vec<String>>>
}

#[pymethods]
impl Circle {
    #[new]
    fn new(_py: Python, parent: &PyAny, id: String, radius: f64, cx: f64, cy: f64) -> Self {
        let children = ;
        let circle = Circle{id, radius, cx, cy, children};
        parent.add_child(&circle);
        circle
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Circle{}", self.id))
    }
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
