use extendr_api::prelude::*;
use rust_core::RustDAG;
use rust_core::RustPDAG;

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
        RDAG {
            inner: RustDAG::new(),
        }
    }

    /// Add a single node to the DAG
    /// @param node The node name
    /// @param latent Whether the node is latent (default: FALSE)
    /// @export
    fn add_node(&mut self, node: String, latent: Option<bool>) -> extendr_api::Result<()> {
        self.inner
            .add_node(node, latent.unwrap_or(false))
            .map_err(Error::from)
    }

    /// Add multiple nodes to the DAG
    /// @param nodes Vector of node names
    /// @param latent Optional vector of latent flags
    /// @export
    fn add_nodes_from(
        &mut self,
        nodes: Strings,
        latent: Nullable<Logicals>,
    ) -> extendr_api::Result<()> {
        let node_vec: Vec<String> = nodes.iter().map(|s| s.to_string()).collect();
        let latent_opt: Option<Vec<bool>> = latent
            .into_option()
            .map(|v| v.iter().map(|x| x.is_true()).collect());

        self.inner
            .add_nodes_from(node_vec, latent_opt)
            .map_err(|e| Error::Other(e))
    }

    /// Add an edge between two nodes
    /// @param u Source node
    /// @param v Target node  
    /// @param weight Optional edge weight
    /// @export
    fn add_edge(&mut self, u: String, v: String, weight: Nullable<f64>) -> extendr_api::Result<()> {
        let w = weight.into_option();
        self.inner.add_edge(u, v, w).map_err(|e| Error::Other(e))
    }

    /// Get parents of a node
    /// @param node The node name
    /// @export
    fn get_parents(&self, node: String) -> extendr_api::Result<Strings> {
        let parents = self.inner.get_parents(&node).map_err(|e| Error::Other(e))?;
        Ok(parents.iter().map(|s| s.as_str()).collect::<Strings>())
    }

    /// Get children of a node
    /// @param node The node name
    /// @export
    fn get_children(&self, node: String) -> extendr_api::Result<Strings> {
        let children = self
            .inner
            .get_children(&node)
            .map_err(|e| Error::Other(e))?;
        Ok(children.iter().map(|s| s.as_str()).collect::<Strings>())
    }

    /// Get ancestors of given nodes
    /// @param nodes Vector of node names
    /// @export
    fn get_ancestors_of(&self, nodes: Strings) -> extendr_api::Result<Strings> {
        let node_vec: Vec<String> = nodes.iter().map(|s| s.to_string()).collect();
        let ancestors = self
            .inner
            .get_ancestors_of(node_vec)
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
        self.inner
            .latents
            .iter()
            .map(|s| s.as_str())
            .collect::<Strings>()
    }
}


#[extendr]
#[derive(Debug, Clone)]
pub struct PDAG {
    inner: RustPDAG,
}


#[extendr]
impl PDAG {
    /// Create a new PDAG
    /// @export
    fn new() -> Self {
        PDAG { inner: RustPDAG::new() }
    }

    /// Add a single node
    /// @param node Node name
    /// @param latent Whether latent (default FALSE)
    /// @export
    fn add_node(&mut self, node: String, latent: Option<bool>) -> extendr_api::Result<()> {
        self.inner.add_node(node, latent.unwrap_or(false))
            .map_err(|e| Error::Other(e.to_string()))
    }

    /// Add nodes from vector with optional latent mask (NULL means all false)
    /// @param nodes character vector
    /// @param latent NULL or logical vector
    /// @export
    fn add_nodes_from(&mut self, nodes: Strings, latent: Nullable<Logicals>) -> extendr_api::Result<()> {
        let node_vec: Vec<String> = nodes.iter().map(|s| s.to_string()).collect();
        let latent_opt: Option<Vec<bool>> = latent.into_option().map(|v| v.iter().map(|x| x.is_true()).collect());
        self.inner.add_nodes_from(node_vec, latent_opt).map_err(|e| Error::Other(e.to_string()))
    }

    /// Add single edge (directed or undirected)
    /// @param u source
    /// @param v target
    /// @param weight optional numeric (NULL)
    /// @param directed bool (TRUE: directed, FALSE: undirected)
    /// @export
    fn add_edge(&mut self, u: String, v: String, weight: Nullable<f64>, directed: Option<bool>) -> extendr_api::Result<()> {
        let w = weight.into_option();
        let d = directed.unwrap_or(true);
        self.inner.add_edge(u, v, w, d).map_err(|e| Error::Other(e.to_string()))
    }

    /// Add multiple edges from an R list of pairs: list(c("A","B"), c("C","D"))
    /// @param ebunch list of character vectors length 2
    /// @param weights NULL or numeric vector
    /// @param directed bool
    /// @export
    fn add_edges_from(&mut self, ebunch: List, weights: Nullable<Doubles>, directed: Option<bool>) -> extendr_api::Result<()> {
        // convert ebunch (List) -> Vec<(String,String)>
        let mut edges: Vec<(String,String)> = Vec::with_capacity(ebunch.len());
        for (i, item) in ebunch.values().enumerate() {
            // Each item must be a character vector of length 2
            let pair: Strings = item.try_into().map_err(|_| Error::Other(format!("ebunch[{}] must be a character vector of length 2", i)))?;
            if pair.len() != 2 {
                return Err(Error::Other(format!("ebunch[{}] must have exactly 2 elements", i)));
            }
            edges.push((pair[0].to_string(), pair[1].to_string()));
        }
        let weight_opt: Option<Vec<f64>> = weights.into_option().map(|v| v.iter().map(|d| d.inner()).collect());
        let directed = directed.unwrap_or(true);
        self.inner.add_edges_from(Some(edges), weight_opt, directed).map_err(|e| Error::Other(e.to_string()))
    }

    /// Return all edges. For PDAG this includes both directed and undirected (both directions placed into graph).
    /// Return as list(from = ..., to = ...) same as RDAG$edges()
    /// @export
    fn edges(&self) -> List {
        let edges = self.inner.edges();
        let (from, to): (Vec<_>, Vec<_>) = edges.into_iter().unzip();
        list!(from = from, to = to)
    }

    /// Return nodes
    /// @export
    fn nodes(&self) -> Strings {
        self.inner.nodes().iter().map(|s| s.as_str()).collect::<Strings>()
    }

    /// Number of nodes
    /// @export
    fn node_count(&self) -> i32 {
        self.inner.node_map.len() as i32
    }

    /// Number of edges (count unique graph edges)
    /// @export
    fn edge_count(&self) -> i32 {
        self.inner.edges().len() as i32
    }

    /// Latent nodes
    /// @export
    fn latents(&self) -> Strings {
        let mut v: Vec<String> = self.inner.latents.iter().cloned().collect();
        v.sort();
        v.iter().map(|s| s.as_str()).collect::<Strings>()
    }

    /// Directed edges as a list of 2-element character vectors
    /// @export
    fn directed_edges(&self) -> List {
        let mut vec = self.inner.directed_edges.iter().cloned().collect::<Vec<_>>();
        vec.sort();
        let mut out = List::new(vec.len());
        for (i, (u, v)) in vec.into_iter().enumerate() {
            let pair = vec![u.as_str(), v.as_str()].iter().map(|s| *s).collect::<Strings>();
            out.set_elt(i, Into::<Robj>::into(pair)).unwrap();
        }
        out
    }

    /// Undirected edges reported as stored (u, v) for each undirected pair (original insertion)
    /// @export
    fn undirected_edges(&self) -> List {
        let mut vec = self.inner.undirected_edges.iter().cloned().collect::<Vec<_>>();
        vec.sort();
        let mut out = List::new(vec.len());
        for (i, (u, v)) in vec.into_iter().enumerate() {
            let pair = vec![u.as_str(), v.as_str()].iter().map(|s| *s).collect::<Strings>();
            out.set_elt(i, Into::<Robj>::into(pair)).unwrap();
        }
        out
    }

    /// All neighbors (directed or undirected) as character vector
    /// @export
    fn all_neighbors(&self, node: String) -> extendr_api::Result<Strings> {
        let s = self.inner.all_neighbors(&node).map_err(|e| Error::Other(e))?;
        let mut v: Vec<String> = s.into_iter().collect();
        v.sort();
        Ok(v.iter().map(|x| x.as_str()).collect::<Strings>())
    }

    /// Directed children
    /// @export
    fn directed_children(&self, node: String) -> extendr_api::Result<Strings> {
        let s = self.inner.directed_children(&node).map_err(|e| Error::Other(e))?;
        let mut v: Vec<String> = s.into_iter().collect();
        v.sort();
        Ok(v.iter().map(|x| x.as_str()).collect::<Strings>())
    }

    /// Directed parents
    /// @export
    fn directed_parents(&self, node: String) -> extendr_api::Result<Strings> {
        let s = self.inner.directed_parents(&node).map_err(|e| Error::Other(e))?;
        let mut v: Vec<String> = s.into_iter().collect();
        v.sort();
        Ok(v.iter().map(|x| x.as_str()).collect::<Strings>())
    }

    /// has_directed_edge
    /// @export
    fn has_directed_edge(&self, u: String, v: String) -> bool {
        self.inner.has_directed_edge(&u, &v)
    }

    /// has_undirected_edge
    /// @export
    fn has_undirected_edge(&self, u: String, v: String) -> bool {
        self.inner.has_undirected_edge(&u, &v)
    }

    /// undirected_neighbors
    /// @export
    fn undirected_neighbors(&self, node: String) -> extendr_api::Result<Strings> {
        let s = self.inner.undirected_neighbors(&node).map_err(|e| Error::Other(e))?;
        let mut v: Vec<String> = s.into_iter().collect();
        v.sort();
        Ok(v.iter().map(|x| x.as_str()).collect::<Strings>())
    }

    /// is_adjacent
    /// @export
    fn is_adjacent(&self, u: String, v: String) -> bool {
        self.inner.is_adjacent(&u, &v)
    }

    /// copy
    /// @export
    fn copy(&self) -> PDAG {
        PDAG { inner: self.inner.copy() }
    }

    /// orient_undirected_edge (returns NULL if inplace = TRUE, otherwise returns new PDAG)
    /// @param u
    /// @param v
    /// @param inplace default TRUE
    /// @export
    fn orient_undirected_edge(&mut self, u: String, v: String, inplace: Option<bool>) -> extendr_api::Result<Nullable<PDAG>> {
        let in_place = inplace.unwrap_or(true);
        match self.inner.orient_undirected_edge(&u, &v, in_place) {
            Ok(None) => Ok(Nullable::Null),
            Ok(Some(pdag)) => Ok(Nullable::NotNull(PDAG { inner: pdag })),
            Err(e) => Err(Error::Other(e)),
        }
    }

    /// apply_meeks_rules (apply_r4 bool, inplace bool)
    /// @export
    fn apply_meeks_rules(&mut self, apply_r4: Option<bool>, inplace: Option<bool>) -> extendr_api::Result<Nullable<PDAG>> {
        let apply_r4 = apply_r4.unwrap_or(true);
        let inplace = inplace.unwrap_or(false);
        match self.inner.apply_meeks_rules(apply_r4, inplace) {
            Ok(None) => Ok(Nullable::Null),
            Ok(Some(pdag)) => Ok(Nullable::NotNull(PDAG { inner: pdag })),
            Err(e) => Err(Error::Other(e)),
        }
    }

    /// to_dag -> RDAG
    /// @export
    fn to_dag(&self) -> extendr_api::Result<RDAG> {
        let dag = self.inner.to_dag().map_err(|e| Error::Other(e))?;
        Ok(RDAG { inner: dag })
    }
}



// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`
extendr_module! {
    mod causalgraphs;
    impl RDAG;
    impl PDAG;
}
