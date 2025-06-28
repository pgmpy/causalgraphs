use petgraph::Direction;
use rustworkx_core::petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, HashSet, VecDeque};

// Conditional imports based on features
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};

// Core DAG structure - shared between Python and WASM
#[derive(Clone)]
#[cfg_attr(feature = "python", pyclass)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct RustDAG {
    graph: DiGraph<String, f64>,
    node_map: HashMap<String, NodeIndex>,
    reverse_node_map: HashMap<NodeIndex, String>,
    latents: HashSet<String>,
}

// Core implementation (shared)
impl RustDAG {
    /// Create a new empty DAG
    pub fn new() -> Self {
        RustDAG {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
            reverse_node_map: HashMap::new(),
            latents: HashSet::new(),
        }
    }

    /// Add a single node to the graph (internal implementation)
    fn add_node_internal(&mut self, node: String, latent: Option<bool>) -> Result<(), String> {
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

    /// Add multiple nodes to the graph (internal implementation)
    fn add_nodes_from_internal(&mut self, nodes: Vec<String>, latent: Option<Vec<bool>>) -> Result<(), String> {
        let latent_flags: Vec<bool> = latent.unwrap_or_else(|| vec![false; nodes.len()]);
        
        if nodes.len() != latent_flags.len() {
            return Err("Length of nodes and latent flags must match".to_string());
        }

        for (node, is_latent) in nodes.iter().zip(latent_flags.iter()) {
            self.add_node_internal(node.clone(), Some(*is_latent))?;
        }
        Ok(())
    }

    /// Add an edge between two nodes (internal implementation)
    fn add_edge_internal(&mut self, u: String, v: String, weight: Option<f64>) -> Result<(), String> {
        // Add nodes if they don't exist
        self.add_node_internal(u.clone(), None)?;
        self.add_node_internal(v.clone(), None)?;

        let u_idx: NodeIndex = self.node_map[&u];
        let v_idx: NodeIndex = self.node_map[&v];
        
        self.graph.add_edge(u_idx, v_idx, weight.unwrap_or(1.0));
        Ok(())
    }

    /// Get parents of a node (internal implementation)
    fn get_parents_internal(&self, node: &str) -> Result<Vec<String>, String> {
        let node_idx = self.node_map.get(node)
            .ok_or_else(|| format!("Node {} not found", node))?;

        let parents: Vec<String> = self.graph
            .neighbors_directed(*node_idx, Direction::Incoming)
            .map(|idx| self.reverse_node_map[&idx].clone())
            .collect();

        Ok(parents)
    }

    /// Get children of a node (internal implementation)
    fn get_children_internal(&self, node: &str) -> Result<Vec<String>, String> {
        let node_idx = self.node_map.get(node)
            .ok_or_else(|| format!("Node {} not found", node))?;

        let children: Vec<String> = self.graph
            .neighbors_directed(*node_idx, Direction::Outgoing)
            .map(|idx: NodeIndex| self.reverse_node_map[&idx].clone())
            .collect();

        Ok(children)
    }

    /// Get all ancestors of given nodes (internal implementation)
    fn get_ancestors_of_internal(&self, nodes: Vec<String>) -> Result<HashSet<String>, String> {
        let mut ancestors: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<NodeIndex> = VecDeque::new();

        // Initialize queue with input nodes
        for node in &nodes {
            if let Some(&node_idx) = self.node_map.get(node) {
                queue.push_back(node_idx);
                ancestors.insert(node.clone());
            } else {
                return Err(format!("Node {} not in graph", node));
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
    fn nodes_internal(&self) -> Vec<String> {
        self.node_map.keys().cloned().collect()
    }

    /// Get all edges in the graph
    fn edges_internal(&self) -> Vec<(String, String)> {
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
    fn node_count_internal(&self) -> usize {
        self.graph.node_count()
    }

    /// Get number of edges  
    fn edge_count_internal(&self) -> usize {
        self.graph.edge_count()
    }
}

// Python-specific methods
#[cfg(feature = "python")]
#[pymethods]
impl RustDAG {
    #[new]
    pub fn py_new() -> Self {
        Self::new()
    }

    /// Add a single node to the graph
    pub fn add_node(&mut self, node: String, latent: Option<bool>) -> PyResult<()> {
        self.add_node_internal(node, latent)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
    }

    /// Add multiple nodes to the graph
    pub fn add_nodes_from(&mut self, nodes: Vec<String>, latent: Option<Vec<bool>>) -> PyResult<()> {
        self.add_nodes_from_internal(nodes, latent)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
    }

    /// Add an edge between two nodes
    pub fn add_edge(&mut self, u: String, v: String, weight: Option<f64>) -> PyResult<()> {
        self.add_edge_internal(u, v, weight)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
    }

    /// Get parents of a node
    pub fn get_parents(&self, node: String) -> PyResult<Vec<String>> {
        self.get_parents_internal(&node)
            .map_err(|e| pyo3::exceptions::PyKeyError::new_err(e))
    }

    /// Get children of a node
    pub fn get_children(&self, node: String) -> PyResult<Vec<String>> {
        self.get_children_internal(&node)
            .map_err(|e| pyo3::exceptions::PyKeyError::new_err(e))
    }

    /// Get all ancestors of given nodes
    pub fn get_ancestors_of(&self, nodes: Vec<String>) -> PyResult<HashSet<String>> {
        self.get_ancestors_of_internal(nodes)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
    }

    /// Get all nodes in the graph
    pub fn nodes(&self) -> Vec<String> {
        self.nodes_internal()
    }

    /// Get all edges in the graph
    pub fn edges(&self) -> Vec<(String, String)> {
        self.edges_internal()
    }

    /// Get number of nodes
    pub fn node_count(&self) -> usize {
        self.node_count_internal()
    }

    /// Get number of edges  
    pub fn edge_count(&self) -> usize {
        self.edge_count_internal()
    }
}

// WASM-specific methods
#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl RustDAG {
    #[wasm_bindgen(constructor)]
    pub fn js_new() -> RustDAG {
        Self::new()
    }

    #[wasm_bindgen(js_name = addNode)]
    pub fn js_add_node(&mut self, node: String, latent: Option<bool>) -> Result<(), JsValue> {
        self.add_node_internal(node, latent)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = addNodesFrom)]
    pub fn js_add_nodes_from(&mut self, nodes: Vec<String>) -> Result<(), JsValue> {
        self.add_nodes_from_internal(nodes, None)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = addEdge)]
    pub fn js_add_edge(&mut self, u: String, v: String, weight: Option<f64>) -> Result<(), JsValue> {
        self.add_edge_internal(u, v, weight)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = getParents, catch)]
    pub fn js_get_parents(&self, node: String) -> Result<Vec<String>, JsValue> {
        self.get_parents_internal(&node)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = getChildren, catch)]
    pub fn js_get_children(&self, node: String) -> Result<Vec<String>, JsValue> {
        self.get_children_internal(&node)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = getAncestorsOf, catch)]
    pub fn js_get_ancestors_of(&mut self, nodes: Vec<String>) -> Result<Vec<String>, JsValue> {
        let ancestors = self.get_ancestors_of_internal(nodes)
            .map_err(|e| JsValue::from_str(&e))?;
        Ok(ancestors.into_iter().collect())
    }

    #[wasm_bindgen(js_name = nodes)]
    pub fn js_nodes(&self) -> Vec<String> {
        self.nodes_internal()
    }

    #[wasm_bindgen(js_name = edges, getter)]
    pub fn js_edges(&self) -> JsValue {
        let edges = self.edges_internal();
        serde_wasm_bindgen::to_value(&edges).unwrap()
    }

    #[wasm_bindgen(js_name = nodeCount, getter)]
    pub fn js_node_count(&self) -> usize {
        self.node_count_internal()
    }

    #[wasm_bindgen(js_name = edgeCount, getter)]
    pub fn js_edge_count(&self) -> usize {
        self.edge_count_internal()
    }
}