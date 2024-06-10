use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use crate::visualirs::shapes_rs::ToSvg;

use crate::Style;

pub struct CanvasRs {
    file_name: String,
    x_0: i32,
    y_0: i32,
    viewport_width: i32,
    viewport_height: i32,
    svg_x: i32,
    svg_y: i32,
    svg_width: i32,
    svg_height: i32,

    id: String,
    preambule: String,
    style: Option<Style>,

    children: Arc<Mutex<Vec<Box<dyn ToSvg>>>>,
    combined_svg: String,
}

impl CanvasRs {
    pub fn new(
        file_name: String,
        svg_width: i32,
        svg_height: i32,
        id: Option<String>,
    ) -> Self {
        let (x_min, y_min, x_max, y_max) = (0, 0, svg_width, svg_height);
        let combined_svg = String::new();
        let id = id.unwrap_or_else(|| "visualife_drawing".to_string());
        //still lacks kwargs
        CanvasRs {
            children: Arc::new( Mutex::new(Vec::new())),
            file_name,
            x_0: x_min,
            y_0: y_min,
            viewport_width: x_max - x_min,
            viewport_height: y_max - y_min,
            svg_x: 0,
            svg_y: 0,
            svg_width,
            svg_height,
            id,
            preambule: r#"<?xml version="1.0" encoding="utf-8"?>"#.to_string(),
            combined_svg,
            style: None
        }
    }

           
    pub fn add_child<T: ToSvg + 'static>(&self, child: T) {
        let mut children = self.children.lock().unwrap();
        children.push(Box::new(child));
    }

    pub fn complete_svg(&mut self) -> String {
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
        combined_svg.push_str(&header1);

        // header 2
        let bgcolor = "transparent".to_string(); //temporary, to be introduced in style
        let header2 = format!(
            r#"<rect x="{x_0}" y="{y_0}" width="{viewport_width}" height="{viewport_height}" id="{id}-bg" fill="{bgcolor}"/>"#,
            x_0 = self.x_0,
            y_0 = self.y_0,
            viewport_width = self.viewport_width,
            viewport_height = self.viewport_height,
            id = self.id,
            bgcolor = bgcolor,
        );
        combined_svg.push_str(&header2);

        //the svgs of children
        let locked_children = self.children.lock().unwrap();
        for obj in locked_children.iter() {
            combined_svg.push_str(&obj.to_svg());
        }

        combined_svg.push_str("</svg>");
        self.combined_svg = combined_svg.clone();
        combined_svg
    }

    fn save_svg(&self) {
        let mut data_file = File::create(&self.file_name).expect("creation failed");
        data_file
            .write(self.combined_svg.as_bytes())
            .expect("write failed");
    }
}
