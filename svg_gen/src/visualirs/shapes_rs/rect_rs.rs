use crate::StyleRs;
use crate::visualirs::shapes_rs::draw_svg::ToSvg;

pub struct RectRs {
    id: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    rx: Option<f32>,
    ry: Option<f32>,
    style: StyleRs,
}

impl RectRs {
    pub fn new(id: &str, x: f32, y: f32, width: f32, height: f32) -> Self {
        let style = StyleRs::new();
        let rect = RectRs {
            id: id.to_string(),
            x,
            y,
            width,
            height,
            rx: None,
            ry: None,
            style,
        };
        rect
    }
}
impl ToSvg for RectRs {
    fn to_svg(&self) -> String {
        let mut svg_string = format!(
            r#"<rect id="{}" x="{}" y="{}" width="{}" height="{}"#,
            self.id, self.x, self.y, self.width, self.height
        );

        if let Some(rx) = self.rx {
            svg_string.push_str(&format!(r#" rx="{}""#, rx));
        }

        if let Some(ry) = self.ry {
            svg_string.push_str(&format!(r#" ry="{}""#, ry));
        }

        let style_string = self.style.to_string();
        if !style_string.is_empty() {
            svg_string.push_str(&format!(r#" style="{}""#, style_string));
        }

        if self.style.angle != 0.0 {
            svg_string.push_str(&format!(
                r#" transform="rotate({} {} {})""#,
                self.style.angle,
                self.x + self.width / 2.0,
                self.y + self.height / 2.0
            ));
        }
        svg_string.push_str(r#" />"#);
        svg_string
    }
}
