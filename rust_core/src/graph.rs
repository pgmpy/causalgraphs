use crate::graph_role::GraphError;
use std::collections::HashSet;

/// Trait for core graph operations required by causal graphs.
pub trait Graph: Clone {
    /// Get all nodes in the graph.
    fn nodes(&self) -> Vec<String>;

    /// Get the parents of a node.
    fn parents(&self, node: &str) -> Result<Vec<String>, GraphError>;

    /// Get the ancestors of a set of nodes (including the nodes themselves).
    fn ancestors(&self, nodes: Vec<String>) -> Result<HashSet<String>, GraphError>;

    /// Check if two nodes are d-connected given an optional set of observed nodes.
    fn is_dconnected(
        &self,
        start: &str,
        end: &str,
        observed: Option<Vec<String>>,
        include_latents: bool,
    ) -> Result<bool, GraphError>;

    fn minimal_dseparator(
        &self,
        start: Vec<String>,
        end: Vec<String>,
        include_latents: bool
    ) -> Result<Option<HashSet<String>>, GraphError>;

    /// Get all simple directed edge paths from source to target.
    fn all_simple_edge_paths(
        &self,
        source: &str,
        target: &str,
    ) -> Result<Vec<Vec<(String, String)>>, GraphError>;

    /// Remove a list of edges from the graph, returning a new graph.
    fn remove_edges_from(&self, edges: Vec<(String, String)>) -> Result<Self, GraphError>;
}