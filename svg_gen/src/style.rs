use pyo3::prelude::*;
// partially copied from github.com/dgront/rust-experiments/tree/master/vl_trial

#[pyclass]
#[derive(Clone)]
pub struct Style {
    #[pyo3(get, set)]
    pub fill: Option<String>,
    #[pyo3(get, set)]
    pub stroke: Option<String>,
    #[pyo3(get, set)]
    pub stroke_width: Option<f32>,
    #[pyo3(get, set)]
    pub opacity: Option<f32>,
    #[pyo3(get, set)]
    pub fill_opacity: Option<f32>,
    #[pyo3(get, set)]
    pub stroke_opacity: Option<f32>,
    #[pyo3(get, set)]
    pub angle: f32,
}
#[pymethods]
impl Style {
    #[new]
    pub fn new(_py: Python) -> Self {
        Style {
            fill: None,
            stroke: None,
            stroke_width: None,
            opacity: None,
            fill_opacity: None,
            stroke_opacity: None,
            angle: 0.0,
        }
    }

    pub fn to_string(&self) -> String {
        let mut style_string = String::new();

        if let Some(ref fill) = self.fill {
            style_string.push_str(&format!("fill:{};", fill));
        }

        if let Some(ref stroke) = self.stroke {
            style_string.push_str(&format!("stroke:{};", stroke));
        }

        if let Some(stroke_width) = self.stroke_width {
            style_string.push_str(&format!("stroke-width:{};", stroke_width));
        }

        if let Some(opacity) = self.opacity {
            style_string.push_str(&format!("opacity:{};", opacity));
        }

        if let Some(fill_opacity) = self.fill_opacity {
            style_string.push_str(&format!("fill-opacity:{};", fill_opacity));
        }

        if let Some(stroke_opacity) = self.stroke_opacity {
            style_string.push_str(&format!("stroke-opacity:{};", stroke_opacity));
        }

        style_string
    }
    // pub fn clone(&self) -> Style {
    //     Style {
    //         fill: self.fill,
    //         stroke: self.stroke,
    //         stroke_width: self.stroke_width,
    //         opacity: self.opacity,
    //         fill_opacity: self.fill_opacity,
    //         stroke_opacity: self.stroke_opacity,
    //         angle: self.angle,
    //     }
    //}
}
