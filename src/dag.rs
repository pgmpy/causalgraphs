use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PySet, PyTuple};
use rustworkx_core::petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, HashSet, VecDeque};

type NodeIndex = petgraph::graph::NodeIndex;
type EdgeIndex = petgraph::graph::EdgeIndex;

#[pyclass]
pub struct RustDAG  {
    graph: Graph<String, f64>,
    node_map: HashMap<String, NodeIndex>,
    reverse_node_map: HashMap<NodeIndex, String>,
    latents: HashSet<String>,
}

#[pymethods]
impl RustDAG  {
    #[new]
    pub fn new() -> Self {
        RustDAG {
            graph: Graph::new(),
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
            .map(|idx| self.reverse_node_map[&idx].clone())
            .collect();

        Ok(children)
    }

    /// Get all ancestors of given nodes (optimized Rust implementation)
    pub fn get_ancestors_of(&self, nodes: Vec<String>) -> PyResult<HashSet<String>> {
        let mut ancestors = AHashSet::new();
        let mut queue = VecDeque::new();

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

    /// Fast implementation of minimal d-separator
    pub fn minimal_dseparator(&self, start: String, end: String, include_latents: Option<bool>) -> PyResult<Option<HashSet<String>>> {
        let include_latents = include_latents.unwrap_or(false);
        
        // Check if nodes are adjacent
        if self.are_adjacent(&start, &end)? {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "No possible separators because start and end are adjacent"
            ));
        }

        // Get initial separator candidates (parents of both nodes)
        let start_parents: HashSet<String> = self.get_parents(start.clone())?.into_iter().collect();
        let end_parents: HashSet<String> = self.get_parents(end.clone())?.into_iter().collect();
        
        let mut separator: HashSet<String> = start_parents.union(&end_parents).cloned().collect();
        
        // Handle latents if not included
        if !include_latents {
            separator = self.resolve_latents(separator)?;
        }

        // Remove start and end nodes from separator
        separator.remove(&start);
        separator.remove(&end);

        // Check if initial set can d-separate
        if self.is_dconnected(&start, &end, Some(separator.iter().cloned().collect()))? {
            return Ok(None);
        }

        // Find minimal separator by removing unnecessary nodes
        let mut minimal_separator = separator.clone();
        
        for node in &separator {
            let mut test_separator = minimal_separator.clone();
            test_separator.remove(node);
            
            if !self.is_dconnected(&start, &end, Some(test_separator.iter().cloned().collect()))? {
                minimal_separator.remove(node);
            }
        }

        Ok(Some(minimal_separator))
    }

    /// Check if two nodes are adjacent
    fn are_adjacent(&self, node1: &str, node2: &str) -> PyResult<bool> {
        let idx1 = self.node_map.get(node1)
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err(format!("Node {} not found", node1)))?;
        let idx2 = self.node_map.get(node2)
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err(format!("Node {} not found", node2)))?;

        Ok(self.graph.find_edge(*idx1, *idx2).is_some() || 
           self.graph.find_edge(*idx2, *idx1).is_some())
    }

    /// Resolve latents by replacing them with their parents
    fn resolve_latents(&self, mut separator: HashSet<String>) -> PyResult<HashSet<String>> {
        while separator.iter().any(|node| self.latents.contains(node)) {
            let mut new_separator = HashSet::new();
            for node in &separator {
                if self.latents.contains(node) {
                    // Replace latent with its parents
                    let parents = self.get_parents(node.clone())?;
                    new_separator.extend(parents);
                } else {
                    new_separator.insert(node.clone());
                }
            }
            separator = new_separator;
        }
        Ok(separator)
    }

    /// Fast d-connection check
    pub fn is_dconnected(&self, start: &str, end: &str, observed: Option<Vec<String>>) -> PyResult<bool> {
        let observed_set: HashSet<String> = observed.unwrap_or_default().into_iter().collect();
        let ancestors = self.get_ancestors_of(observed_set.iter().cloned().collect())?;
        
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        // Start BFS from start node
        queue.push_back((start.to_string(), "up".to_string()));
        
        while let Some((node, direction)) = queue.pop_front() {
            let state = format!("{}_{}", node, direction);
            if visited.contains(&state) {
                continue;
            }
            visited.insert(state);
            
            if node == end {
                return Ok(true);
            }
            
            if direction == "up" && !observed_set.contains(&node) {
                // Can go up to parents
                for parent in self.get_parents(node.clone())? {
                    queue.push_back((parent, "up".to_string()));
                }
                // Can go down to children
                for child in self.get_children(node.clone())? {
                    queue.push_back((child, "down".to_string()));
                }
            } else if direction == "down" {
                if !observed_set.contains(&node) {
                    // Can continue down to children
                    for child in self.get_children(node.clone())? {
                        queue.push_back((child, "down".to_string()));
                    }
                }
                if ancestors.contains(&node) {
                    // Can go up to parents (collider)
                    for parent in self.get_parents(node.clone())? {
                        queue.push_back((parent, "up".to_string()));
                    }
                }
            }
        }
        
        Ok(false)
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