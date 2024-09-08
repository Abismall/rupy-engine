pub mod engine;

use engine::RupyEngine;
use pyo3::prelude::*;

#[pymodule]
pub fn rupyengine(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RupyEngine>()?;
    Ok(())
}
