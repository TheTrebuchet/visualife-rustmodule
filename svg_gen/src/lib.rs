use pyo3::prelude::*;
mod visualipy;
mod visualirs;


pub use crate::visualirs::*;
pub use crate::visualipy::{Canvas, Style};
pub use crate::visualipy::shapes::{Rect, Circle};


// here im trying to implement this idea, that in python
// canvas = Canvas(...)
// circle = Circle(...)
// canvas.add_child(circle)

// we should totally call them students and researchgroups or something

#[pymodule]
fn svg_gen(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Canvas>()?;
    m.add_class::<Circle>()?;
    m.add_class::<Rect>()?;
    m.add_class::<Style>()?;
    Ok(())
}
