use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3::types::PyDict;
use std::sync::{Arc, Mutex};

#[pyclass]
pub struct Canvas {
    file_name: String,
    x_0: i32,             // x_min
    y_0: i32,             // y_min
    viewport_width: i32,  // x_max - x_min
    viewport_height: i32, // = y_max - y_min
    svg_x: i32,           // 0
    svg_y: i32,           // 0
    svg_width: i32,       // svg_width
    svg_height: i32,      // svg_height
    id: String,
    preambule: String,
    children: Arc<Mutex<Vec<PyObject>>>, //style = kwargs.get("style", SvgViewport.default_drawing_style)
                                         //text_style = kwargs.get("text_style", SvgViewport.default_text_style)

                                         // dont know what to do about it
                                         //bgcolor = kwargs.get("bgcolor", "transparent")
                                         //innerHTML = create_svg_header()
                                         //inner_viewports = []
                                         //is_inner_viewport = False
                                         //parent_viewport = None
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
        let (x_min, y_min, x_max, y_max) = (0, 0, svg_width, svg_height);
        let children = Arc::new(Mutex::new(Vec::new()));
        //still lacks kwargs
        Canvas {
            children,
            file_name,
            x_0: x_min,
            y_0: y_min,
            viewport_width: x_max - x_min,
            viewport_height: y_max - y_min,
            svg_x: 0,
            svg_y: 0,
            svg_width,
            svg_height,
            id: id.unwrap_or_else(|| "visualife_drawing".to_string()),
            preambule: "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n".to_string(),
        }
    }

    fn add_child(&self, py: Python, child: Py<PyAny>) {
        let mut children = self.children.lock().unwrap();
        children.push(child.clone_ref(py));
        println!("child adopted")
    }

    fn generate_string(&self, py: Python) -> PyResult<String> {
        let mut combined_svg = String::new();
        combined_svg.push_str(&self.preambule);
        //header
        let header1 = format!(
            r#"<svg id="{id}" viewBox="{x_0:.1} {y_0:.1} {viewport_width:.1} {viewport_height:.1}" xmlns="http://www.w3.org/2000/svg" version="1.1" width="{svg_width}" height="{svg_height}" x="{svg_x}" y="{svg_y}">"#,
            id = self.id,
            x_0 = self.x_0,
            y_0 = self.y_0,
            viewport_width = self.viewport_width,
            viewport_height = self.viewport_height,
            svg_width = self.svg_width,
            svg_height = self.svg_height,
            svg_x = self.svg_x,
            svg_y = self.svg_y,
        );
        let bgcolor = "black".to_string(); //temporary, to be introduced in style
        let header2 = format!(
            r#"<rect x="{x_0}" y="{y_0}" width="{viewport_width}" height="{viewport_height}" id="{id}-bg" fill="{bgcolor}"/>"#,
            x_0 = self.x_0,
            y_0 = self.y_0,
            viewport_width = self.viewport_width,
            viewport_height = self.viewport_height,
            id = self.id,
            bgcolor = bgcolor,
        );
        combined_svg.push_str(&header1);
        combined_svg.push_str(&header2);
        //if !self.file_name.is_empty() {
        // Append style element string
        //            combined_svg.push_str(&format!(
        //              r#"<style>
        //            .default_text_style {{{}}}
        //          .default_drawing_style {{{}}}
        //        </style>"#,
        //      self.text_style, self.style
        //));
        //}

        //the svgs of children
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
        }
        combined_svg.push_str("</svg>");
        Ok(combined_svg)
    }
}
