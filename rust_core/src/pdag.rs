use petgraph::Direction;
use rustworkx_core::petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, HashSet};

use crate::RustDAG;


#[derive(Debug, Clone)]
pub struct RustPDAG {
    pub graph: DiGraph<String, f64>,
    pub node_map: HashMap<String, NodeIndex>,
    pub reverse_node_map: HashMap<NodeIndex, String>,
    pub directed_edges: HashSet<(String, String)>,
    pub undirected_edges: HashSet<(String, String)>,
    pub latents: HashSet<String>,
}
impl RustPDAG {
    pub fn new() -> Self {
        RustPDAG {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
            reverse_node_map: HashMap::new(),
            directed_edges: HashSet::new(),
            undirected_edges: HashSet::new(),
            latents: HashSet::new(),
        }
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

    /// Get all nodes in the graph
    pub fn nodes(&self) -> Vec<String> {
        let mut nodes: Vec<String> = self.node_map.keys().cloned().collect();
        nodes.sort(); // Sort alphabetically for deterministic order
        nodes
    }
    /// Adds a single node to the PDAG.
    pub fn add_node(&mut self, node: String, latent: bool) -> Result<(), String> {
        if !self.node_map.contains_key(&node) {
            let idx: NodeIndex = self.graph.add_node(node.clone());
            self.node_map.insert(node.clone(), idx);
            self.reverse_node_map.insert(idx, node.clone());

            if latent {
                self.latents.insert(node);
            }
        }
        Ok(())
    }

    /// Adds multiple nodes to the PDAG.
    pub fn add_nodes_from(&mut self, nodes: Vec<String>, latent: Option<Vec<bool>>) -> Result<(), String> {
        let latent_flags: Vec<bool> = latent.unwrap_or_else(|| vec![false; nodes.len()]);

        if nodes.len() != latent_flags.len() {
            return Err("Length of nodes and latent flags must match".to_string());
        }

        for (node, is_latent) in nodes.iter().zip(latent_flags.iter()) {
            self.add_node(node.clone(), *is_latent)?;
        }
        Ok(())
    }

    /// Adds a single edge (directed or undirected) to the PDAG.
    pub fn add_edge(&mut self, u: String, v: String, weight: Option<f64>, directed: bool) -> Result<(), String> {
        // Add nodes if they don't exist
        self.add_node(u.clone(), false)?;
        self.add_node(v.clone(), false)?;

        let u_idx = self.node_map[&u];
        let v_idx = self.node_map[&v];

        if directed {
            // Check for cycles before adding directed edge
            let mut temp_graph = self.graph.clone();
            temp_graph.add_edge(u_idx, v_idx, weight.unwrap_or(1.0));
            if petgraph::algo::is_cyclic_directed(&temp_graph) {
                return Err(format!("Adding directed edge {} -> {} creates a cycle", u, v));
            }
            self.graph.add_edge(u_idx, v_idx, weight.unwrap_or(1.0));
            self.directed_edges.insert((u.clone(), v.clone()));
        } else {
            // Add undirected edge (bidirectional in graph)
            self.graph.add_edge(u_idx, v_idx, weight.unwrap_or(1.0));
            self.graph.add_edge(v_idx, u_idx, weight.unwrap_or(1.0));
            self.undirected_edges.insert((u.clone(), v.clone()));
        }
        Ok(())
    }

    /// Adds multiple edges (directed or undirected) to the PDAG.
    pub fn add_edges_from(
        &mut self,
        ebunch: Option<Vec<(String, String)>>,
        weights: Option<Vec<f64>>,
        directed: bool,
    ) -> Result<(), String> {
        let ebunch = ebunch.unwrap_or_default();
        let weights = weights.unwrap_or_else(|| vec![1.0; ebunch.len()]);

        if ebunch.len() != weights.len() {
            return Err("The number of elements in ebunch and weights should be equal".to_string());
        }

        for (i, (u, v)) in ebunch.iter().enumerate() {
            self.add_edge(u.clone(), v.clone(), Some(weights[i]), directed)?;
        }
        Ok(())
    }

    /// Returns all neighbors (via directed or undirected edges) of a node.
    pub fn all_neighbors(&self, node: &str) -> Result<HashSet<String>, String> {
        let node_idx = self.node_map.get(node)
            .ok_or_else(|| format!("Node {} not found", node))?;

        let successors: HashSet<String> = self.graph
            .neighbors_directed(*node_idx, Direction::Outgoing)
            .map(|idx| self.reverse_node_map[&idx].clone())
            .collect();

        let predecessors: HashSet<String> = self.graph
            .neighbors_directed(*node_idx, Direction::Incoming)
            .map(|idx| self.reverse_node_map[&idx].clone())
            .collect();

        Ok(successors.union(&predecessors).cloned().collect())
    }

    /// Returns children of a node via directed edges (node -> child).
    pub fn directed_children(&self, node: &str) -> Result<HashSet<String>, String> {
        let node_idx = self.node_map.get(node)
            .ok_or_else(|| format!("Node {} not found", node))?;

        let children: HashSet<String> = self.graph
            .neighbors_directed(*node_idx, Direction::Outgoing)
            .filter(|&idx| {
                let child = &self.reverse_node_map[&idx];
                self.directed_edges.contains(&(node.to_string(), child.to_string()))
            })
            .map(|idx| self.reverse_node_map[&idx].clone())
            .collect();

        Ok(children)
    }

    /// Returns parents of a node via directed edges (parent -> node).
    pub fn directed_parents(&self, node: &str) -> Result<HashSet<String>, String> {
        let node_idx = self.node_map.get(node)
            .ok_or_else(|| format!("Node {} not found", node))?;

        let parents: HashSet<String> = self.graph
            .neighbors_directed(*node_idx, Direction::Incoming)
            .filter(|&idx| {
                let parent = &self.reverse_node_map[&idx];
                self.directed_edges.contains(&(parent.to_string(), node.to_string()))
            })
            .map(|idx| self.reverse_node_map[&idx].clone())
            .collect();

        Ok(parents)
    }

    /// Checks if there is a directed edge u -> v.
    pub fn has_directed_edge(&self, u: &str, v: &str) -> bool {
        self.directed_edges.contains(&(u.to_string(), v.to_string()))
    }

    /// Checks if there is an undirected edge u - v.
    pub fn has_undirected_edge(&self, u: &str, v: &str) -> bool {
        self.undirected_edges.contains(&(u.to_string(), v.to_string())) ||
        self.undirected_edges.contains(&(v.to_string(), u.to_string()))
    }

    /// Returns neighbors connected via undirected edges.
    pub fn undirected_neighbors(&self, node: &str) -> Result<HashSet<String>, String> {
        let node_idx = self.node_map.get(node)
            .ok_or_else(|| format!("Node {} not found", node))?;

        let neighbors: HashSet<String> = self.graph
            .neighbors_directed(*node_idx, Direction::Outgoing)
            .filter(|&idx| {
                let neighbor = &self.reverse_node_map[&idx];
                self.has_undirected_edge(node, neighbor)
            })
            .map(|idx| self.reverse_node_map[&idx].clone())
            .collect();

        Ok(neighbors)
    }

    /// Checks if two nodes are adjacent (via any edge: directed or undirected).
    pub fn is_adjacent(&self, u: &str, v: &str) -> bool {
        self.has_directed_edge(u, v) || self.has_directed_edge(v, u) || self.has_undirected_edge(u, v)
    }

    /// Returns a copy of the PDAG.
    pub fn copy(&self) -> RustPDAG {
        RustPDAG {
            graph: self.graph.clone(),
            node_map: self.node_map.clone(),
            reverse_node_map: self.reverse_node_map.clone(),
            directed_edges: self.directed_edges.clone(),
            undirected_edges: self.undirected_edges.clone(),
            latents: self.latents.clone(),
        }
    }

    /// Returns a subgraph containing only directed edges as a RustDAG.
    pub fn directed_graph(&self) -> RustDAG {
        let mut dag = RustDAG::new();

        // Add all nodes with their latent status
        for node in self.node_map.keys() {
            let is_latent = self.latents.contains(node);
            dag.add_node(node.clone(), is_latent).unwrap();
        }

        // Add only directed edges
        for (u, v) in &self.directed_edges {
            dag.add_edge(u.clone(), v.clone(), None).unwrap();
        }

        dag
    }

}
