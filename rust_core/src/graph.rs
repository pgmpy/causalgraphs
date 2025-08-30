use crate::graph_role::GraphError;
use std::collections::HashSet;

/// Trait for core graph operations required by causal graphs.
pub trait Graph {
    /// Get all nodes in the graph.
    fn nodes(&self) -> Vec<String>;

    /// Get the parents of a node.
    fn parents(&self, node: &str) -> Result<Vec<String>, GraphError>;

    /// Get the ancestors of a set of nodes (including the nodes themselves).
    fn ancestors(&self, nodes: Vec<String>) -> Result<HashSet<String>, GraphError>;
}