// partially copied from github.com/dgront/rust-experiments/tree/master/vl_trial
#[derive(Clone)]
pub struct StyleRs {
    pub fill: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: Option<f32>,
    pub opacity: Option<f32>,
    pub fill_opacity: Option<f32>,
    pub stroke_opacity: Option<f32>,
    pub angle: f32,
}

impl StyleRs {
    pub fn new() -> Self {
        StyleRs {
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
}
