use pyo3::prelude::*;
use rustworkx_core::petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, HashSet};

#[pyclass]
pub struct RustDAG  {
    graph: DiGraph<String, f64>,
    node_map: HashMap<String, NodeIndex>,
    reverse_node_map: HashMap<NodeIndex, String>,
    latents: HashSet<String>,
}

#[pymethods]
impl RustDAG  {
    #[new]
    pub fn new() -> Self {
        RustDAG {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
            reverse_node_map: HashMap::new(),
            latents: HashSet::new(),
        }
    }

    /// Add a single node to the graph
    pub fn add_node(&mut self, node: String, latent: Option<bool>) -> PyResult<()> {
        if !self.node_map.contains_key(&node) {
            let idx = self.graph.add_node(node.clone());
            self.node_map.insert(node.clone(), idx);
            self.reverse_node_map.insert(idx, node.clone());
            
            if latent.unwrap_or(false) {
                self.latents.insert(node);
            }
        }
        Ok(())
    }

    /// Add multiple nodes to the graph
    pub fn add_nodes_from(&mut self, nodes: Vec<String>, latent: Option<Vec<bool>>) -> PyResult<()> {
        let latent_flags = latent.unwrap_or_else(|| vec![false; nodes.len()]);
        
        if nodes.len() != latent_flags.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Length of nodes and latent flags must match"
            ));
        }

        for (node, is_latent) in nodes.iter().zip(latent_flags.iter()) {
            self.add_node(node.clone(), Some(*is_latent))?;
        }
        Ok(())
    }


    /// Add an edge between two nodes
    pub fn add_edge(&mut self, u: String, v: String, weight: Option<f64>) -> PyResult<()> {
        // Add nodes if they don't exist
        self.add_node(u.clone(), None)?;
        self.add_node(v.clone(), None)?;

        let u_idx = self.node_map[&u];
        let v_idx = self.node_map[&v];
        
        self.graph.add_edge(u_idx, v_idx, weight.unwrap_or(1.0));
        Ok(())
    }

    //** Stop here for now **/

}