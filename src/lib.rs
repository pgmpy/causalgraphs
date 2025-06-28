// Python bindings
#[cfg(feature = "python")]
use pyo3::prelude::*;

// WASM bindings
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

mod dag;

// Export the DAG for different targets
pub use dag::RustDAG;

// Python module
#[cfg(feature = "python")]
#[pymodule]
fn causalgraphs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RustDAG>()?;
    Ok(())
}

// WASM initialization
#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn main() {
    // Optional: Set up panic hook for better error messages in browser
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}