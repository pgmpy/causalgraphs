use extendr_api::prelude::*;
use rust_core::{RustDAG, IndependenceAssertion, Independencies};
use std::collections::HashSet;
use std::panic;


#[extendr]
fn on_load() {
    panic::set_hook(Box::new(|info| {
        eprintln!("Panic: {:?}", info);
    }));
}



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
        self.inner.add_node(node, latent.unwrap_or(false)).map_err(|e| Error::Other(e.to_string()))
    }


    /// Add multiple nodes to the DAG
    /// @param nodes Vector of node names
    /// @param latent Optional vector of latent flags
    /// @export
    fn add_nodes_from(&mut self, nodes: Strings, latent: Nullable<Logicals>) -> extendr_api::Result<()> {
        let node_vec: Vec<String> = nodes.iter().map(|s| s.to_string()).collect();
        let latent_opt: Option<Vec<bool>> = latent.into_option().map(|v| v.iter().map(|x| x.is_true()).collect());
        self.inner.add_nodes_from(node_vec, latent_opt).map_err(|e| Error::Other(e.to_string()))
    }

    /// Add an edge between two nodes
    /// @param u Source node
    /// @param v Target node
    /// @param weight Optional edge weight (default: NULL)
    /// @export
    fn add_edge(&mut self, u: String, v: String, weight: Nullable<f64>) -> extendr_api::Result<()> {
        let w = weight.into_option();
        self.inner.add_edge(u, v, w).map_err(|e| Error::Other(e.to_string()))
    }

    /// Get parents of a node
    /// @param node The node name
    /// @export
    fn get_parents(&self, node: String) -> extendr_api::Result<Strings> {
        let parents = self.inner.get_parents(&node).map_err(|e| Error::Other(e.to_string()))?;
        Ok(parents.iter().map(|s| s.as_str()).collect::<Strings>())
    }
    /// Get children of a node
    /// @param node The node name
    /// @export
    fn get_children(&self, node: String) -> extendr_api::Result<Strings> {
        let children = self.inner.get_children(&node).map_err(|e| Error::Other(e.to_string()))?;
        Ok(children.iter().map(|s| s.as_str()).collect::<Strings>())
    }

    /// Get ancestors of given nodes
    /// @param nodes Vector of node names
    /// @export
    fn get_ancestors_of(&self, nodes: Strings) -> extendr_api::Result<Strings> {
        let node_vec: Vec<String> = nodes.iter().map(|s| s.to_string()).collect();
        let ancestors = self.inner.get_ancestors_of(node_vec).map_err(|e| Error::Other(e.to_string()))?;
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

    /// Add multiple edges to the DAG
    /// @param ebunch List of (u, v) pairs (each pair as a character vector of length 2)
    /// @param weights Optional vector of weights (must match ebunch length)
    /// @export
    fn add_edges_from(&mut self, ebunch: List, weights: Nullable<Doubles>) -> extendr_api::Result<()> {
        let mut edge_vec: Vec<(String, String)> = Vec::with_capacity(ebunch.len());
        let weight_opt: Option<Vec<f64>> = weights.into_option().map(|v| v.iter().map(|x| x.inner()).collect());
        
        if let Some(ref w) = weight_opt {
            if w.len() != ebunch.len() {
                return Err(Error::Other("Weights length must match ebunch".to_string()));
            }
        }
        
        for (i, pair) in ebunch.values().enumerate() {
            let pair_vec: Strings = pair.try_into()
                .map_err(|_| Error::Other(format!("tuples[{}] must be a list", i)))?; // Changed error message
            if pair_vec.len() != 2 {
                return Err(Error::Other(format!("ebunch[{}] must have exactly 2 elements", i))); // Removed "(u, v)" part
            }
            edge_vec.push((pair_vec[0].to_string(), pair_vec[1].to_string()));
        }
        
        self.inner.add_edges_from(edge_vec, weight_opt).map_err(|e| Error::Other(e.to_string()))
    }

    /// Get active trail nodes
    /// @param variables Vector of starting variables
    /// @param observed Optional vector of observed nodes
    /// @param include_latents Whether to include latents (default: FALSE)
    /// @export
    fn active_trail_nodes(&self, variables: Strings, observed: Nullable<Strings>, include_latents: Option<bool>) -> extendr_api::Result<List> {
        let var_vec: Vec<String> = variables.iter().map(|s| s.to_string()).collect();
        if var_vec.is_empty() {
            return Err(Error::Other("variables cannot be empty".to_string()));
        }
        let obs_opt: Option<Vec<String>> = observed.into_option().map(|v| v.iter().map(|s| s.to_string()).collect());
        
        let result = self.inner.active_trail_nodes(var_vec, obs_opt, include_latents.unwrap_or(false))
            .map_err(|e| Error::Other(e.to_string()))?;
        
        let result_clone = result.clone();
        
        let r_list = List::from_names_and_values(
            result.keys().map(|k| k.as_str()),
            result_clone.into_values().map(|set| {
                let vec: Vec<String> = set.into_iter().collect();
                let strings: Strings = vec.iter().map(|s| s.as_str()).collect();
                Into::<Robj>::into(strings)
            })
        )?;
        Ok(r_list)
    }


    /// Check if two nodes are d-connected
    /// @param start Starting node
    /// @param end Ending node
    /// @param observed Optional vector of observed nodes
    /// @param include_latents Whether to include latents (default: FALSE)
    /// @export
    fn is_dconnected(&self, start: String, end: String, observed: Nullable<Strings>, include_latents: Option<bool>) -> extendr_api::Result<bool> {
        let obs_opt: Option<Vec<String>> = observed.into_option().map(|v| v.iter().map(|s| s.to_string()).collect());
        self.inner.is_dconnected(&start, &end, obs_opt, include_latents.unwrap_or(false))
            .map_err(|e| Error::Other(e.to_string()))
    }


    /// Check if two nodes are neighbors
    /// @param start First node
    /// @param end Second node
    /// @export
    fn are_neighbors(&self, start: String, end: String) -> extendr_api::Result<bool> {
        self.inner.are_neighbors(&start, &end).map_err(|e| Error::Other(e.to_string()))
    }

    /// Get ancestral graph for given nodes
    /// @param nodes Vector of nodes
    /// @export
    fn get_ancestral_graph(&self, nodes: Strings) -> extendr_api::Result<RDAG> {
        let node_vec: Vec<String> = nodes.iter().map(|s| s.to_string()).collect();
        self.inner.get_ancestral_graph(node_vec)
            .map(|dag| RDAG { inner: dag })
            .map_err(|e| Error::Other(e.to_string()))
    }

    /// Get minimal d-separator between two nodes
    /// @param start Starting node
    /// @param end Ending node
    /// @param include_latents Whether to include latents (default: FALSE)
    /// @export
    fn minimal_dseparator(&self, start: String, end: String, include_latents: Option<bool>) -> extendr_api::Result<Nullable<Strings>> {
        let result = self.inner.minimal_dseparator(&start, &end, include_latents.unwrap_or(false))
            .map_err(|e| Error::Other(e.to_string()))?;
        match result {
            Some(set) => {
                let vec: Vec<String> = set.into_iter().collect();
                Ok(Nullable::NotNull(vec.iter().map(|s| s.as_str()).collect::<Strings>()))
            }
            None => Ok(Nullable::Null),
        }
    }
}

#[extendr]
#[derive(Debug, Clone)]
pub struct RIndependenceAssertion {
    inner: IndependenceAssertion,
}

#[extendr]
impl RIndependenceAssertion {
    /// Create a new IndependenceAssertion
    /// @param event1 Vector of event1 variables
    /// @param event2 Vector of event2 variables
    /// @param event3 Optional vector of event3 variables
    /// @export
    fn new(event1: Strings, event2: Strings, event3: Nullable<Strings>) -> extendr_api::Result<Self> {
        let e1: HashSet<String> = event1.iter().map(|s| s.to_string()).collect();
        let e2: HashSet<String> = event2.iter().map(|s| s.to_string()).collect();
        let e3_opt: Option<HashSet<String>> = event3.into_option().map(|v| v.iter().map(|s| s.to_string()).collect());
        let inner = IndependenceAssertion::new(e1, e2, e3_opt)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(RIndependenceAssertion { inner })
    }

    /// Get event1 variables
    /// @export
    fn event1(&self) -> Strings {
        let mut result: Vec<String> = self.inner.event1.iter().cloned().collect();
        result.sort();
        result.iter().map(|s| s.as_str()).collect::<Strings>()
    }

    /// Get event2 variables
    /// @export
    fn event2(&self) -> Strings {
        let mut result: Vec<String> = self.inner.event2.iter().cloned().collect();
        result.sort();
        result.iter().map(|s| s.as_str()).collect::<Strings>()
    }

    /// Get event3 variables
    /// @export
    fn event3(&self) -> Strings {
        let mut result: Vec<String> = self.inner.event3.iter().cloned().collect();
        result.sort();
        result.iter().map(|s| s.as_str()).collect::<Strings>()
    }

    /// Get all variables
    /// @export
    fn all_vars(&self) -> Strings {
        let mut result: Vec<String> = self.inner.all_vars.iter().cloned().collect();
        result.sort();
        result.iter().map(|s| s.as_str()).collect::<Strings>()
    }

    /// Check if unconditional
    /// @export
    fn is_unconditional(&self) -> bool {
        self.inner.is_unconditional()
    }

    /// Get LaTeX representation
    /// @export
    fn to_latex(&self) -> String {
        self.inner.to_latex()
    }

    /// Get string representation
    /// @export
    fn to_string(&self) -> String {
        format!("{}", self.inner)
    }
}

#[extendr]
#[derive(Debug, Clone)]
pub struct RIndependencies {
    inner: Independencies,
}

#[extendr]
impl RIndependencies {
    /// Create a new Independencies
    /// @export
    fn new() -> Self {
        RIndependencies { inner: Independencies::new() }
    }

    /// Add a single assertion
    /// @param assertion An RIndependenceAssertion object
    /// @export
    fn add_assertion(&mut self, assertion: &RIndependenceAssertion) {
        self.inner.add_assertion(assertion.inner.clone());
    }

    /// Add multiple assertions from R tuples
    /// @param tuples A list of 2- or 3-tuples `(event1, event2, event3)`
    /// @export
    fn add_assertions_from_tuples(&mut self, tuples: List) -> extendr_api::Result<()> {
        let mut rust_tuples: Vec<(Vec<String>, Vec<String>, Option<Vec<String>>)> = Vec::with_capacity(tuples.len());
        
        for (i, pair) in tuples.values().enumerate() {
            if pair.is_null() {
                continue;  // Skip NULL items if any
            }
            let inner = pair.as_list().ok_or_else(|| Error::Other(format!("tuples[{}] must be a list", i)))?;
            if inner.len() < 2 || inner.len() > 3 {
                return Err(Error::Other(format!("tuples[{}] must have 2 or 3 elements", i)));
            }
            
            let e1: Strings = inner.elt(0)?.try_into().map_err(|_| Error::Other(format!("tuples[{}][0] must be character vector", i)))?;
            let e1_vec = e1.iter().map(|s| s.to_string()).collect::<Vec<_>>();
            let e2: Strings = inner.elt(1)?.try_into().map_err(|_| Error::Other(format!("tuples[{}][1] must be character vector", i)))?;
            let e2_vec = e2.iter().map(|s| s.to_string()).collect::<Vec<_>>();
            
            let e3_opt = if inner.len() == 3 {
                let e3_robj = inner.elt(2)?;
                if e3_robj.is_null() {
                    None
                } else {
                    let e3: Strings = e3_robj.try_into().map_err(|_| Error::Other(format!("tuples[{}][2] must be character vector", i)))?;
                    Some(e3.iter().map(|s| s.to_string()).collect::<Vec<_>>())
                }
            } else {
                None
            };
            rust_tuples.push((e1_vec, e2_vec, e3_opt));
        }
        
        self.inner.add_assertions_from_tuples(rust_tuples).map_err(|e| Error::Other(e.to_string()))
    }

    /// Get all assertions
    /// @export
    fn get_assertions(&self) -> List {
        let assertions = self.inner.get_assertions();
        let mut r_list = List::new(assertions.len());
        for (i, a) in assertions.iter().enumerate() {
            let r_assertion = RIndependenceAssertion { inner: a.clone() };
            r_list.set_elt(i, r_assertion.into()).unwrap();
        }
        r_list
    }

    /// Get all variables
    /// @export
    fn get_all_variables(&self) -> Strings {
        let mut result: Vec<String> = self.inner.get_all_variables().into_iter().collect();
        result.sort();
        result.iter().map(|s| s.as_str()).collect::<Strings>()
    }

    /// Check if contains assertion
    /// @param assertion An RIndependenceAssertion object
    /// @export
    fn contains(&self, assertion: &RIndependenceAssertion) -> bool {
        self.inner.contains(&assertion.inner)
    }

    /// Compute closure
    /// @export
    fn closure(&self) -> RIndependencies {
        RIndependencies { inner: self.inner.closure() }
    }

    /// Reduce independencies
    /// @param inplace Whether to modify in place (default: FALSE)
    /// @export
    fn reduce(&mut self, inplace: Option<bool>) -> Nullable<RIndependencies> {
        if inplace.unwrap_or(false) {
            self.inner.reduce_inplace();
            Nullable::Null
        } else {
            Nullable::NotNull(RIndependencies { inner: self.inner.reduce() })
        }
    }

    /// Check if entails another set
    /// @param other Another RIndependencies object
    /// @export
    fn entails(&self, other: &RIndependencies) -> bool {
        self.inner.entails(&other.inner)
    }

    /// Check if equivalent to another set
    /// @param other Another RIndependencies object
    /// @export
    fn is_equivalent(&self, other: &RIndependencies) -> bool {
        self.inner.is_equivalent(&other.inner)
    }
}


extendr_module! {
    mod causalgraphs;
    impl RDAG;
    impl RIndependenceAssertion;
    impl RIndependencies;
}