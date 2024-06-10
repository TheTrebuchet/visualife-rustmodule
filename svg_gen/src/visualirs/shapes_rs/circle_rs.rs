use crate::StyleRs;
use std::sync::{Arc, Mutex};

pub struct CircleRs {
    id: String,
    radius: f32,
    cx: f32,
    cy: f32,
    pub style: Option<Arc<Mutex<StyleRs>>>,
}

impl CircleRs {
    pub fn new(id: String, radius: f32, cx: f32, cy: f32) -> Self {
        let circle = CircleRs {
            id,
            radius,
            cx,
            cy,
            style: None,
        };
        circle
    }
    pub fn to_svg(&self) -> String {
        let mut svg_string = format!(
            r#"<circle cx="{}" cy="{}" r="{}"#,
            self.cx, self.cy, self.radius
        );
        let style_string = self.style.expect("REASON").lock().unwrap().to_string();
        if !style_string.is_empty() {
            svg_string.push_str(&format!(r#" style="{}""#, style_string));

        }
        
        svg_string.push_str(r#" />"#);
        svg_string
    }

    pub fn set_style(&mut self, style: StyleRs) {
        self.style = Some(Arc::new(Mutex::new(style)));
    }
}
