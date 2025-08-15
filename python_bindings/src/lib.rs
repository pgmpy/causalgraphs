use pyo3::exceptions::{PyKeyError, PyValueError};
use pyo3::prelude::*;
use rust_core::{IndependenceAssertion, Independencies, RustDAG, RustPDAG};
use std::collections::{HashMap, HashSet};

#[pyclass(name = "PDAG")]
#[derive(Clone)]
pub struct PyRustPDAG {
    inner: RustPDAG,
}

#[pymethods]
impl PyRustPDAG {
    #[new]
    pub fn new() -> Self {
        PyRustPDAG {
            inner: RustPDAG::new(),
        }
    }

    /// Add a single node to the PDAG.
    pub fn add_node(&mut self, node: String, latent: Option<bool>) -> PyResult<()> {
        self.inner
            .add_node(node, latent.unwrap_or(false))
            .map_err(PyValueError::new_err)
    }

    /// Add multiple nodes to the PDAG.
    pub fn add_nodes_from(&mut self, nodes: Vec<String>, latent: Option<Vec<bool>>) -> PyResult<()> {
        self.inner
            .add_nodes_from(nodes, latent)
            .map_err(PyValueError::new_err)
    }

    /// Add a single edge (directed or undirected) to the PDAG.
    #[pyo3(signature = (u, v, weight = None, directed = true))]
    pub fn add_edge(&mut self, u: String, v: String, weight: Option<f64>, directed: bool) -> PyResult<()> {
        self.inner
            .add_edge(u, v, weight, directed)
            .map_err(PyValueError::new_err)
    }

    /// Add multiple edges (directed or undirected) to the PDAG.
    #[pyo3(signature = (ebunch, weights = None, directed = true))]
    pub fn add_edges_from(
        &mut self,
        ebunch: Vec<(String, String)>,
        weights: Option<Vec<f64>>,
        directed: bool,
    ) -> PyResult<()> {
        self.inner
            .add_edges_from(Some(ebunch), weights, directed)
            .map_err(PyValueError::new_err)
    }

    /// Get all neighbors (via directed or undirected edges) of a node.
    pub fn all_neighbors(&self, node: String) -> PyResult<Vec<String>> {
        let neighbors = self.inner
            .all_neighbors(&node)
            .map_err(PyKeyError::new_err)?;
        let mut result: Vec<String> = neighbors.into_iter().collect();
        result.sort(); // Ensure deterministic order
        Ok(result)
    }

    /// Get children of a node via directed edges.
    pub fn directed_children(&self, node: String) -> PyResult<Vec<String>> {
        let children = self.inner
            .directed_children(&node)
            .map_err(PyKeyError::new_err)?;
        let mut result: Vec<String> = children.into_iter().collect();
        result.sort();
        Ok(result)
    }

    /// Get parents of a node via directed edges.
    pub fn directed_parents(&self, node: String) -> PyResult<Vec<String>> {
        let parents = self.inner
            .directed_parents(&node)
            .map_err(PyKeyError::new_err)?;
        let mut result: Vec<String> = parents.into_iter().collect();
        result.sort();
        Ok(result)
    }

    /// Check if there is a directed edge u -> v.
    pub fn has_directed_edge(&self, u: String, v: String) -> bool {
        self.inner.has_directed_edge(&u, &v)
    }

    /// Check if there is an undirected edge u - v.
    pub fn has_undirected_edge(&self, u: String, v: String) -> bool {
        self.inner.has_undirected_edge(&u, &v)
    }

    /// Get neighbors connected via undirected edges.
    pub fn undirected_neighbors(&self, node: String) -> PyResult<Vec<String>> {
        let neighbors = self.inner
            .undirected_neighbors(&node)
            .map_err(PyKeyError::new_err)?;
        let mut result: Vec<String> = neighbors.into_iter().collect();
        result.sort();
        Ok(result)
    }

    /// Check if two nodes are adjacent (via any edge).
    pub fn is_adjacent(&self, u: String, v: String) -> bool {
        self.inner.is_adjacent(&u, &v)
    }

    /// Get all nodes in the PDAG.
    pub fn nodes(&self) -> Vec<String> {
        self.inner.nodes()
    }

    /// Get all edges in the PDAG.
    pub fn edges(&self) -> Vec<(String, String)> {
        self.inner.edges()
    }

    /// Apply Meek's rules to orient undirected edges.
    #[pyo3(signature = (apply_r4 = true, inplace = true))]
    pub fn apply_meeks_rules(&mut self, apply_r4: bool, inplace: bool) -> PyResult<Option<PyRustPDAG>> {
        self.inner
            .apply_meeks_rules(apply_r4, inplace)
            .map(|opt| opt.map(|pdag| PyRustPDAG { inner: pdag }))
            .map_err(PyValueError::new_err)
    }

    /// Convert the PDAG to a DAG.
    pub fn to_dag(&self) -> PyResult<PyRustDAG> {
        self.inner
            .to_dag()
            .map(|dag| PyRustDAG { inner: dag })
            .map_err(PyValueError::new_err)
    }

    /// Check if there is a directed path from source to target.
    pub fn has_directed_path(&self, source: String, target: String) -> PyResult<bool> {
        self.inner
            .has_directed_path(&source, &target)
            .map_err(PyKeyError::new_err)
    }

    /// Get the number of nodes in the PDAG.
    pub fn node_count(&self) -> usize {
        self.inner.node_map.len()
    }

    /// Get the number of edges in the PDAG (counting undirected edges once).
    pub fn edge_count(&self) -> usize {
        self.inner.directed_edges.len() + self.inner.undirected_edges.len()
    }

    /// Get the set of latent nodes.
    #[getter]
    pub fn latents(&self) -> Vec<String> {
        let mut result: Vec<String> = self.inner.latents.iter().cloned().collect();
        result.sort();
        result
    }
}

#[pyclass(name = "DAG")]
#[derive(Clone)]
pub struct PyRustDAG {
    inner: RustDAG,
}

#[pymethods]
impl PyRustDAG {
    #[new]
    pub fn new() -> Self {
        PyRustDAG {
            inner: RustDAG::new(),
        }
    }

    pub fn add_node(&mut self, node: String, latent: Option<bool>) -> PyResult<()> {
        self.inner
            .add_node(node, latent.unwrap_or(false))
            .map_err(PyValueError::new_err)
    }

    pub fn add_nodes_from(
        &mut self,
        nodes: Vec<String>,
        latent: Option<Vec<bool>>,
    ) -> PyResult<()> {
        self.inner
            .add_nodes_from(nodes, latent)
            .map_err(PyValueError::new_err)
    }

    pub fn add_edge(&mut self, u: String, v: String, weight: Option<f64>) -> PyResult<()> {
        self.inner
            .add_edge(u, v, weight)
            .map_err(PyValueError::new_err)
    }

    pub fn add_edges_from(
        &mut self,
        ebunch: Vec<(String, String)>,
        weights: Option<Vec<f64>>,
    ) -> PyResult<()> {
        self.inner
            .add_edges_from(ebunch, weights)
            .map_err(PyValueError::new_err)
    }

    pub fn get_parents(&self, node: String) -> PyResult<Vec<String>> {
        self.inner.get_parents(&node).map_err(PyKeyError::new_err)
    }

    pub fn get_children(&self, node: String) -> PyResult<Vec<String>> {
        self.inner.get_children(&node).map_err(PyKeyError::new_err)
    }

    pub fn get_ancestors_of(&self, nodes: Vec<String>) -> PyResult<HashSet<String>> {
        self.inner
            .get_ancestors_of(nodes)
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

    #[pyo3(signature = (variables, observed = None, include_latents = false))]
    pub fn active_trail_nodes(
        &self,
        variables: Vec<String>,
        observed: Option<Vec<String>>,
        include_latents: bool,
    ) -> PyResult<std::collections::HashMap<String, std::collections::HashSet<String>>> {
        self.inner
            .active_trail_nodes(variables, observed, include_latents)
            .map_err(PyValueError::new_err)
    }

    #[pyo3(signature = (start, end, observed = None, include_latents = false))]
    pub fn is_dconnected(
        &self,
        start: String,
        end: String,
        observed: Option<Vec<String>>,
        include_latents: bool,
    ) -> PyResult<bool> {
        self.inner
            .is_dconnected(&start, &end, observed, include_latents)
            .map_err(PyValueError::new_err)
    }

    pub fn are_neighbors(&self, start: String, end: String) -> PyResult<bool> {
        self.inner
            .are_neighbors(&start, &end)
            .map_err(PyValueError::new_err)
    }

    pub fn get_ancestral_graph(&self, nodes: Vec<String>) -> PyResult<PyRustDAG> {
        self.inner
            .get_ancestral_graph(nodes)
            .map(|dag| PyRustDAG { inner: dag })
            .map_err(PyValueError::new_err)
    }

    #[pyo3(signature = (start, end, include_latents=false))]
    pub fn minimal_dseparator(
        &self,
        start: String,
        end: String,
        include_latents: bool,
    ) -> PyResult<Option<std::collections::HashSet<String>>> {
        self.inner
            .minimal_dseparator(&start, &end, include_latents)
            .map_err(PyValueError::new_err)
    }

    #[getter]
    fn latents(&self) -> Vec<String> {
        let mut result: Vec<String> = self.inner.latents.iter().cloned().collect();
        result.sort();
        result
    }
}

// Existing PyIndependenceAssertion and PyIndependencies (unchanged, included for completeness)
#[pyclass(name = "IndependenceAssertion")]
#[derive(Clone)]
pub struct PyIndependenceAssertion {
    inner: IndependenceAssertion,
}

#[pymethods]
impl PyIndependenceAssertion {
    #[new]
    pub fn new(
        event1: Vec<String>,
        event2: Vec<String>,
        event3: Option<Vec<String>>,
    ) -> PyResult<Self> {
        let e1: HashSet<String> = event1.into_iter().collect();
        let e2: HashSet<String> = event2.into_iter().collect();
        let e3: Option<HashSet<String>> = event3.map(|v| v.into_iter().collect());
        let assertion = IndependenceAssertion::new(e1, e2, e3).map_err(PyValueError::new_err)?;
        Ok(PyIndependenceAssertion { inner: assertion })
    }

    #[getter]
    pub fn event1(&self) -> Vec<String> {
        let mut result: Vec<String> = self.inner.event1.iter().cloned().collect();
        result.sort();
        result
    }

    #[getter]
    pub fn event2(&self) -> Vec<String> {
        let mut result: Vec<String> = self.inner.event2.iter().cloned().collect();
        result.sort();
        result
    }

    #[getter]
    pub fn event3(&self) -> Vec<String> {
        let mut result: Vec<String> = self.inner.event3.iter().cloned().collect();
        result.sort();
        result
    }

    #[getter]
    pub fn all_vars(&self) -> Vec<String> {
        let mut result: Vec<String> = self.inner.all_vars.iter().cloned().collect();
        result.sort();
        result
    }

    pub fn is_unconditional(&self) -> bool {
        self.inner.is_unconditional()
    }

    pub fn to_latex(&self) -> String {
        self.inner.to_latex()
    }

    fn __str__(&self) -> String {
        format!("{}", self.inner)
    }

    pub fn __eq__(&self, other: &PyIndependenceAssertion) -> bool {
        self.inner == other.inner
    }

    pub fn __ne__(&self, other: &PyIndependenceAssertion) -> bool {
        self.inner != other.inner
    }
}

#[pyclass(name = "Independencies")]
#[derive(Clone)]
pub struct PyIndependencies {
    inner: Independencies,
}

#[pymethods]
impl PyIndependencies {
    #[new]
    pub fn new() -> Self {
        PyIndependencies {
            inner: Independencies::new(),
        }
    }

    pub fn add_assertion(&mut self, assertion: &PyIndependenceAssertion) {
        self.inner.add_assertion(assertion.inner.clone());
    }

    pub fn add_assertions_from_tuples(
        &mut self,
        tuples: Vec<(Vec<String>, Vec<String>, Option<Vec<String>>)>,
    ) -> PyResult<()> {
        self.inner
            .add_assertions_from_tuples(tuples)
            .map_err(PyValueError::new_err)
    }

    pub fn get_assertions(&self) -> Vec<PyIndependenceAssertion> {
        self.inner
            .get_assertions()
            .iter()
            .map(|a| PyIndependenceAssertion { inner: a.clone() })
            .collect()
    }

    #[getter(independencies)]
    pub fn get_independencies(&self) -> Vec<PyIndependenceAssertion> {
        self.inner
            .get_assertions()
            .iter()
            .map(|a| PyIndependenceAssertion { inner: a.clone() })
            .collect()
    }

    pub fn get_all_variables(&self) -> Vec<String> {
        self.inner.get_all_variables().into_iter().collect()
    }

    pub fn contains(&self, assertion: &PyIndependenceAssertion) -> bool {
        self.inner.contains(&assertion.inner)
    }

    pub fn closure(&self) -> PyIndependencies {
        PyIndependencies {
            inner: self.inner.closure(),
        }
    }

    #[pyo3(signature = (inplace = false))]
    pub fn reduce(&mut self, inplace: bool) -> PyResult<Option<PyIndependencies>> {
        if inplace {
            self.inner.reduce_inplace();
            Ok(None)
        } else {
            Ok(Some(PyIndependencies {
                inner: self.inner.reduce(),
            }))
        }
    }

    pub fn entails(&self, other: &PyIndependencies) -> bool {
        self.inner.entails(&other.inner)
    }

    pub fn is_equivalent(&self, other: &PyIndependencies) -> bool {
        self.inner.is_equivalent(&other.inner)
    }

    pub fn __eq__(&self, other: &PyIndependencies) -> bool {
        self.inner == other.inner
    }

    pub fn __ne__(&self, other: &PyIndependencies) -> bool {
        self.inner != other.inner
    }
}

#[pymodule]
fn causalgraphs(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<PyRustDAG>()?;
    m.add_class::<PyRustPDAG>()?;
    m.add_class::<PyIndependenceAssertion>()?;
    m.add_class::<PyIndependencies>()?;
    Ok(())
}