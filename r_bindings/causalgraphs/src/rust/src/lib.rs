use extendr_api::prelude::*;
use rust_core::RustDAG;
use std::collections::HashSet;

#[extendr]
#[derive(Debug, Clone)]
pub struct RDAG {
    inner: RustDAG,
}

#[extendr]
impl RDAG {
    /// Create a new DAG
    /// @export
    fn new() -> Self {
        RDAG { inner: RustDAG::new() }
    }

    /// Add a single node to the DAG
    /// @param node The node name
    /// @param latent Whether the node is latent (default: FALSE)
    /// @export
    fn add_node(&mut self, node: String, latent: Option<bool>) -> extendr_api::Result<()> {
        self.inner.add_node(node, latent.unwrap_or(false))
            .map_err(Error::from)
    }


    /// Add multiple nodes to the DAG
    /// @param nodes Vector of node names
    /// @param latent Optional vector of latent flags
    /// @export
    fn add_nodes_from(&mut self, nodes: Strings, latent: Nullable<Logicals>) -> extendr_api::Result<()> {
        let node_vec: Vec<String> = nodes.iter().map(|s| s.to_string()).collect();
        let latent_opt: Option<Vec<bool>> = latent.into_option().map(|v| v.iter().map(|x| x.is_true()).collect());
        
        self.inner.add_nodes_from(node_vec, latent_opt)
            .map_err(|e| Error::Other(e))
    }

    /// Add an edge between two nodes
    /// @param u Source node
    /// @param v Target node  
    /// @param weight Optional edge weight
    /// @export
    fn add_edge(&mut self, u: String, v: String, weight: Nullable<f64>) -> extendr_api::Result<()> {
        let w = weight.into_option();
        self.inner.add_edge(u, v, w)
            .map_err(|e| Error::Other(e))
    }

    /// Get parents of a node
    /// @param node The node name
    /// @export
    fn get_parents(&self, node: String) -> extendr_api::Result<Strings> {
        let parents = self.inner.get_parents(&node)
            .map_err(|e| Error::Other(e))?;
        Ok(parents.iter().map(|s| s.as_str()).collect::<Strings>())
    }

    /// Get children of a node
    /// @param node The node name
    /// @export
    fn get_children(&self, node: String) -> extendr_api::Result<Strings> {
        let children = self.inner.get_children(&node)
            .map_err(|e| Error::Other(e))?;
        Ok(children.iter().map(|s| s.as_str()).collect::<Strings>())
    }

    /// Get ancestors of given nodes
    /// @param nodes Vector of node names
    /// @export
    fn get_ancestors_of(&self, nodes: Strings) -> extendr_api::Result<Strings> {
        let node_vec: Vec<String> = nodes.iter().map(|s| s.to_string()).collect();
        let ancestors = self.inner.get_ancestors_of(node_vec)
            .map_err(|e| Error::Other(e))?;
        Ok(ancestors.iter().map(|s| s.as_str()).collect::<Strings>())
    }

    /// Get all nodes in the DAG
    /// @export
    fn nodes(&self) -> Strings {
        let nodes = self.inner.nodes();
        nodes.iter().map(|s| s.as_str()).collect::<Strings>()
    }

    /// Get all edges in the DAG
    /// @export
    fn edges(&self) -> List {
        let edges = self.inner.edges();
        let (from, to): (Vec<_>, Vec<_>) = edges.into_iter().unzip();
        list!(from = from, to = to)
    }

    /// Get number of nodes
    /// @export
    fn node_count(&self) -> i32 {
        self.inner.node_count() as i32
    }

    /// Get number of edges
    /// @export
    fn edge_count(&self) -> i32 {
        self.inner.edge_count() as i32
    }

    /// Get latent nodes
    /// @export
    fn latents(&self) -> Strings {
        self.inner.latents.iter().map(|s| s.as_str()).collect::<Strings>()
    }
}

// Expose the module to R
extendr_module! {
    mod causalgraphs;
    impl RDAG;
}