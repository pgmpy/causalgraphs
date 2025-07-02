use pyo3::prelude::*;
use pyo3::exceptions::{PyKeyError, PyValueError};
use rust_core::RustDAG;
use std::collections::HashSet;

#[pyclass]
#[derive(Clone)]
pub struct PyRustDAG {
    inner: RustDAG,
}

#[pymethods]
impl PyRustDAG {
    #[new]
    pub fn new() -> Self {
        PyRustDAG { inner: RustDAG::new() }
    }

    pub fn add_node(&mut self, node: String, latent: Option<bool>) -> PyResult<()> {
        self.inner.add_node(node, latent.unwrap_or(false))
            .map_err(PyValueError::new_err)
    }

    pub fn add_nodes_from(&mut self, nodes: Vec<String>, latent: Option<Vec<bool>>) -> PyResult<()> {
        self.inner.add_nodes_from(nodes, latent)
            .map_err(PyValueError::new_err)
    }

    pub fn add_edge(&mut self, u: String, v: String, weight: Option<f64>) -> PyResult<()> {
        self.inner.add_edge(u, v, weight)
            .map_err(PyValueError::new_err)
    }

    pub fn get_parents(&self, node: String) -> PyResult<Vec<String>> {
        self.inner.get_parents(&node)
            .map_err(PyKeyError::new_err)
    }

    pub fn get_children(&self, node: String) -> PyResult<Vec<String>> {
        self.inner.get_children(&node)
            .map_err(PyKeyError::new_err)
    }

    pub fn get_ancestors_of(&self, nodes: Vec<String>) -> PyResult<HashSet<String>> {
        self.inner.get_ancestors_of(nodes)
            .map_err(PyValueError::new_err)
    }

    pub fn nodes(&self) -> Vec<String> {
        self.inner.nodes()
    }

    pub fn edges(&self) -> Vec<(String, String)> {
        self.inner.edges()
    }

    pub fn node_count(&self) -> usize {
        self.inner.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.inner.edge_count()
    }
}

#[pymodule]
fn causalgraphs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyRustDAG>()?;
    Ok(())
}

