use pyo3::prelude::*;

#[pyclass]
struct SvgViewport {
    test_num: i32,
}

#[pymethods]
impl SvgViewport {
    fn set_method(&mut self, value: i32) -> PyResult<()> {
        self.test_num = value;
        Ok(())
    }
    }

/// A Python module implemented in Rust.
#[pymodule]
fn visualife_rustmodule(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SvgViewport>()?;
    Ok(())
}
