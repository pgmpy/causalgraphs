use crate::graph::Graph;
use crate::graph_role::{GraphError, GraphRoles};

/// Trait for causal identification algorithms, mirroring Python's BaseIdentification.
pub trait BaseIdentification {
    /// Internal identification method to be implemented by specific algorithms.
    fn _identify<T: Graph + GraphRoles>(
        &self,
        causal_graph: &T,
    ) -> Result<(T, bool), GraphError>;

    /// Run the identification algorithm on a causal graph.
    fn identify<T: Graph + GraphRoles>(
        &self,
        causal_graph: &T,
    ) -> Result<(T, bool), GraphError> {
        causal_graph.is_valid_causal_structure()?;
        self._identify(causal_graph)
    }
}