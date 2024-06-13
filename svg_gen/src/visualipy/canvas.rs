use crate::visualirs::{CanvasRs, CircleRs, RectRs};
use pyo3::prelude::*;
use pyo3::types::PyAny;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};

use crate::Style;

#[pyclass]
pub struct Canvas {
    file_name: String,
    svg_width: i32,
    svg_height: i32,
    id: Option<String>,
    style: Option<Style>,
    rs_struct: Arc<Mutex<CanvasRs>>,
}

#[pymethods]
impl Canvas {
    #[new]
    fn new(
        _py: Python,
        file_name: String,
        svg_width: i32,
        svg_height: i32,
        id: Option<String>,
    ) -> Self {
        let rs_struct = Arc::new(Mutex::new(CanvasRs::new(
            file_name, svg_width, svg_height, id,
        )));
        Canvas {
            id,
            file_name,
            svg_width,
            svg_height,
            style: None,
            rs_struct,
        }
    }

    fn add_child(&self, py: Python, child: Py<PyAny>) {
        match child.getattr(py, "rs_struct") {
            Ok(rs_struct) => {
                // I need to add at least the clone of rs_struct to the chidren attribute in the CanvasRs struct
                // for that I need to check if the rs_struct is an Arc<Mutex<...>>
                // ideally I wouldn't have to clone it, but I couldn't find a way to do that
                // I would probably have to check every type of object the module introduces (Rect, Cricle, etc.)
                let arc_mutex: Arc<Mutex<RectRs>> = rs_struct.extract(py)?; 
                }
            Err(_) => {}
        };
    }

    fn complete_svg(&mut self, py: Python) -> PyResult<String> {
        Ok(self.rs_struct.lock().unwrap().complete_svg())
    }

    fn save_svg(&self) {
        let output = self.rs_struct.lock().unwrap().complete_svg();
        let mut data_file = File::create(&self.file_name).expect("creation failed");
        data_file.write(output.as_bytes()).expect("write failed");
    }
}
