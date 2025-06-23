use pyo3::prelude::*;

mod dag;

use dag::RustDAG;

#[pymodule]
fn causalgraphs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RustDAG>()?;
    Ok(())
}