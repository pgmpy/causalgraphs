use ahash::AHashSet;
use petgraph::Direction;
use pyo3::prelude::*;
use rustworkx_core::petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, HashSet, VecDeque};

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
            let idx: NodeIndex = self.graph.add_node(node.clone());
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
        let latent_flags: Vec<bool> = latent.unwrap_or_else(|| vec![false; nodes.len()]);
        
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

        let u_idx: NodeIndex = self.node_map[&u];
        let v_idx: NodeIndex = self.node_map[&v];
        
        self.graph.add_edge(u_idx, v_idx, weight.unwrap_or(1.0));
        Ok(())
    }

    /// Get parents of a node
    pub fn get_parents(&self, node: String) -> PyResult<Vec<String>> {
        let node_idx = self.node_map.get(&node)
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err(format!("Node {} not found", node)))?;

        let parents: Vec<String> = self.graph
            .neighbors_directed(*node_idx, Direction::Incoming)
            .map(|idx| self.reverse_node_map[&idx].clone())
            .collect();

        Ok(parents)
    }

    /// Get children of a node
    pub fn get_children(&self, node: String) -> PyResult<Vec<String>> {
        let node_idx = self.node_map.get(&node)
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err(format!("Node {} not found", node)))?;

        let children: Vec<String> = self.graph
            .neighbors_directed(*node_idx, Direction::Outgoing)
            .map(|idx: NodeIndex| self.reverse_node_map[&idx].clone())
            .collect();

        Ok(children)
    }

    /// Get all ancestors of given nodes (optimized Rust implementation)
    pub fn get_ancestors_of(&self, nodes: Vec<String>) -> PyResult<HashSet<String>> {
        let mut ancestors: AHashSet<String> = AHashSet::new();
        let mut queue: VecDeque<NodeIndex> = VecDeque::new();

        // Initialize queue with input nodes
        for node in &nodes {
            if let Some(&node_idx) = self.node_map.get(node) {
                queue.push_back(node_idx);
                ancestors.insert(node.clone());
            } else {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    format!("Node {} not in graph", node)
                ));
            }
        }

        // BFS to find all ancestors
        while let Some(current_idx) = queue.pop_front() {
            for parent_idx in self.graph.neighbors_directed(current_idx, Direction::Incoming) {
                let parent_name = &self.reverse_node_map[&parent_idx];
                if ancestors.insert(parent_name.clone()) {
                    queue.push_back(parent_idx);
                }
            }
        }

        Ok(ancestors.into_iter().collect())
    }

    /// Get all nodes in the graph
    pub fn nodes(&self) -> Vec<String> {
        self.node_map.keys().cloned().collect()
    }

    /// Get all edges in the graph
    pub fn edges(&self) -> Vec<(String, String)> {
        self.graph
            .edge_indices()
            .map(|edge_idx| {
                let (source, target) = self.graph.edge_endpoints(edge_idx).unwrap();
                (
                    self.reverse_node_map[&source].clone(),
                    self.reverse_node_map[&target].clone(),
                )
            })
            .collect()
    }

    /// Get number of nodes
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Get number of edges  
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }


}