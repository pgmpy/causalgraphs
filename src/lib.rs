use pyo3::prelude::*;

mod dag;
mod utils;

use dag::RustDAG;

#[pymodule]
fn pgmpy_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RustDAG>()?;
    Ok(())
}