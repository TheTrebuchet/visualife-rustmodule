use pyo3::prelude::*;
mod canvas;
mod shapes;
mod style;

pub use crate::canvas::Canvas;
pub use crate::shapes::Circle;
pub use crate::shapes::Rect;
pub use crate::style::Style;


// here im trying to implement this idea, that in python
// canvas = Canvas(...)
// circle = Circle(...)
// canvas.add_child(circle)

#[pymodule]
fn svg_gen(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Canvas>()?;
    m.add_class::<Circle>()?;
    m.add_class::<Rect>()?;
    m.add_class::<Style>()?;
    Ok(())
}
