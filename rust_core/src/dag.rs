use petgraph::Direction;
use rustworkx_core::petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, HashSet, VecDeque};

/// Directed Acyclic Graph (DAG) with optional latent variables.
///
/// Each node represents a random variable (or a cluster of variables). Directed
/// edges represent dependencies. A subset of nodes can be marked **latent**
/// to represent unobserved variables (e.g., unobserved confounding).
///
/// # Examples
/// Create an empty DAG and add nodes/edges:
/// ```rust
/// # use std::collections::HashSet;
/// let mut g = RustDAG::new();
/// g.add_node("A".into(), false).unwrap();
/// g.add_node("B".into(), false).unwrap();
/// g.add_edge("A".into(), "B".into(), None).unwrap();
/// assert!(g.has_edge("A", "B"));
/// ```
#[derive(Debug, Clone)]
pub struct RustDAG {
    pub graph: DiGraph<String, f64>,
    pub node_map: HashMap<String, NodeIndex>,
    pub reverse_node_map: HashMap<NodeIndex, String>,
    pub latents: HashSet<String>,
}

impl RustDAG {  
    /// Create an empty DAG with no nodes and edges.
    ///
    /// # Returns
    /// A new empty `RustDAG`.
    pub fn new() -> Self {
        RustDAG {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
            reverse_node_map: HashMap::new(),
            latents: HashSet::new(),
        }
    }

    /// Add a single node to the graph.
    ///
    /// Nodes are identified by their string name. If the node already exists,
    /// the call is a no-op.
    ///
    /// # Parameters
    /// - `node`: Node name.
    /// - `latent`: Mark the node as latent (unobserved).
    ///
    /// # Returns
    /// `Ok(())` on success.
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

    /// Add multiple nodes to the graph.
    ///
    /// # Parameters
    /// - `nodes`: List of node names.
    /// - `latent`: Optional list of latent flags parallel to `nodes`. If not
    ///   provided, all nodes are assumed observed.
    ///
    /// # Errors
    /// Returns an error if `latent` is provided and its length differs from `nodes`.
    pub fn add_nodes_from(
        &mut self,
        nodes: Vec<String>,
        latent: Option<Vec<bool>>,
    ) -> Result<(), String> {
        let latent_flags: Vec<bool> = latent.unwrap_or_else(|| vec![false; nodes.len()]);

        if nodes.len() != latent_flags.len() {
            return Err("Length of nodes and latent flags must match".to_string());
        }

        for (node, is_latent) in nodes.iter().zip(latent_flags.iter()) {
            // Note: Call self.add_node directly now, not self.add_node_internal
            self.add_node(node.clone(), *is_latent)?;
        }
        Ok(())
    }


    /// Add a directed edge `u -> v`.
    ///
    /// If either endpoint is missing, it is added automatically (as observed by default).
    ///
    /// # Parameters
    /// - `u`, `v`: Endpoint node names.
    /// - `weight`: Optional edge weight (defaults to `1.0`).
    pub fn add_edge(&mut self, u: String, v: String, weight: Option<f64>) -> Result<(), String> {
        // Add nodes if they don't exist. Pass false for latent by default.
        self.add_node(u.clone(), false)?;
        self.add_node(v.clone(), false)?;

        let u_idx: NodeIndex = self.node_map[&u];
        let v_idx: NodeIndex = self.node_map[&v];

        self.graph.add_edge(u_idx, v_idx, weight.unwrap_or(1.0));
        Ok(())
    }


    /// Add multiple directed edges.
    ///
    /// # Parameters
    /// - `ebunch`: List of `(u, v)` edges to add.
    /// - `weights`: Optional list of weights parallel to `ebunch`.
    ///
    /// # Errors
    /// Returns an error if `weights` is given and its length differs from `ebunch`.
    pub fn add_edges_from(
        &mut self,
        ebunch: Vec<(String, String)>,
        weights: Option<Vec<f64>>,
    ) -> Result<(), String> {
        if let Some(ws) = &weights {
            if ebunch.len() != ws.len() {
                return Err(
                    "The number of elements in ebunch and weights should be equal".to_string(),
                );
            }
            for (i, (u, v)) in ebunch.iter().enumerate() {
                self.add_edge(u.clone(), v.clone(), Some(ws[i]))?;
            }
        } else {
            for (u, v) in ebunch {
                self.add_edge(u, v, None)?;
            }
        }
        Ok(())
    }

    /// Return the list of **parents** of `node` (in-neighbors).
    ///
    /// # Parameters
    /// - `node`: Node name.
    ///
    /// # Returns
    /// Vector of parent names.
    ///
    /// # Errors
    /// Returns an error if `node` is not in the graph.
    pub fn get_parents(&self, node: &str) -> Result<Vec<String>, String> {
        let node_idx = self
            .node_map
            .get(node)
            .ok_or_else(|| format!("Node {} not found", node))?;

        let parents: Vec<String> = self
            .graph
            .neighbors_directed(*node_idx, Direction::Incoming)
            .map(|idx| self.reverse_node_map[&idx].clone())
            .collect();

        Ok(parents)
    }

    /// Return the list of **children** of `node` (out-neighbors).
    ///
    /// # Parameters
    /// - `node`: Node name.
    ///
    /// # Returns
    /// Vector of child names.
    ///
    /// # Errors
    /// Returns an error if `node` is not in the graph.
    pub fn get_children(&self, node: &str) -> Result<Vec<String>, String> {
        let node_idx = self
            .node_map
            .get(node)
            .ok_or_else(|| format!("Node {} not found", node))?;

        let children: Vec<String> = self
            .graph
            .neighbors_directed(*node_idx, Direction::Outgoing)
            .map(|idx: NodeIndex| self.reverse_node_map[&idx].clone())
            .collect();

        Ok(children)
    }

    /// Return the set of **ancestors** of the given `nodes` (including the nodes themselves).
    ///
    /// # Parameters
    /// - `nodes`: Node names.
    ///
    /// # Returns
    /// Set of ancestor names.
    ///
    /// # Errors
    /// Returns an error if any node is missing.
    ///
    /// # Examples
    /// ```rust
    /// let mut g = RustDAG::new();
    /// g.add_edges_from(vec![("D".into(), "G".into()), ("I".into(), "G".into())], None).unwrap();
    /// let a = g.get_ancestors_of(vec!["G".into()]).unwrap();
    /// ```
    pub fn get_ancestors_of(&self, nodes: Vec<String>) -> Result<HashSet<String>, String> {
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
            for parent_idx in self
                .graph
                .neighbors_directed(current_idx, Direction::Incoming)
            {
                if let Some(parent_name) = self.reverse_node_map.get(&parent_idx) {
                    if ancestors.insert(parent_name.clone()) {
                        queue.push_back(parent_idx);
                    }
                } else {
                    return Err(format!(
                        "Node index {:?} not found in reverse map",
                        parent_idx
                    ));
                }
            }
        }

        Ok(ancestors)
    }

    /// Compute **active trail nodes** (d-connection reachability) from each start variable.
    ///
    /// Returns a map `{start_variable -> reachable_nodes}` under d-separation rules,
    /// optionally conditioning on `observed`.
    ///
    /// Follows Koller & Friedman (PGM) Algorithm 3.1 (message-passing with up/down directions).
    ///
    /// # Parameters
    /// - `variables`: Start variables.
    /// - `observed`: Optional list of observed nodes (conditioning set).
    /// - `include_latents`: If `false`, latent variables are excluded from the result.
    ///
    /// # Returns
    /// Map from start variable to the set of reachable nodes via active trails.
    ///
    /// # Errors
    /// Returns an error if any start variable is missing.
    pub fn active_trail_nodes(
        &self,
        variables: Vec<String>,
        observed: Option<Vec<String>>,
        include_latents: bool,
    ) -> Result<HashMap<String, HashSet<String>>, String> {
        let observed_list: HashSet<String> = observed.unwrap_or_default().into_iter().collect();
        // Precompute ancestors of observed nodes (needed for collider rule)
        // Example: If C is observed in A→B←C→D, ancestors_list = {A, B, C}
        let ancestors_list: HashSet<String> =
            self.get_ancestors_of(observed_list.iter().cloned().collect())?;

        let mut active_trails: HashMap<String, HashSet<String>> = HashMap::new();
        // For each starting variable, find all nodes reachable via active trails
        for start in variables {
            // BFS with direction tracking: (node, direction_of_arrival)
            // "up" = coming from child toward parents, "down" = coming from parent toward children
            let mut visit_list: HashSet<(String, &str)> = HashSet::new();
            let mut traversed_list: HashSet<(String, &str)> = HashSet::new();
            let mut active_nodes: HashSet<String> = HashSet::new();

            if !self.node_map.contains_key(&start) {
                return Err(format!("Node {} not in graph", start));
            }

            visit_list.insert((start.clone(), "up"));
            while let Some((node, direction)) = visit_list.iter().next().map(|x| x.clone()) {
                visit_list.remove(&(node.clone(), direction));
                if !traversed_list.contains(&(node.clone(), direction)) {
                    // Add to active trail if not observed (observed nodes block but aren't "reachable")
                    if !observed_list.contains(&node) {
                        active_nodes.insert(node.clone());
                    }
                    traversed_list.insert((node.clone(), direction));

                    // If arriving "up" at unobserved B, can continue to parents and switch to children
                    if direction == "up" && !observed_list.contains(&node) {
                        for parent in self.get_parents(&node)? {
                            visit_list.insert((parent, "up")); // Continue up the chain
                        }
                        for child in self.get_children(&node)? {
                            visit_list.insert((child, "down")); // Switch direction
                        }
                    }
                    // If arriving "down", can continue down if unobserved, or go up if it's a collider
                    else if direction == "down" {
                        if !observed_list.contains(&node) {
                            for child in self.get_children(&node)? {
                                visit_list.insert((child, "down"));
                            }
                        }
                        if ancestors_list.contains(&node) {
                            for parent in self.get_parents(&node)? {
                                visit_list.insert((parent, "up"));
                            }
                        }
                    }
                }
            }

            let final_nodes: HashSet<String> = if include_latents {
                active_nodes
            } else {
                active_nodes.difference(&self.latents).cloned().collect()
            };
            active_trails.insert(start, final_nodes);
        }

        Ok(active_trails)
    }


    /// Check whether `start` and `end` are **d-connected** given `observed`.
    ///
    /// Returns `true` if and only if there exists an active trail between `start` and `end`
    /// under the given conditioning set.
    ///
    /// # Parameters
    /// - `start`, `end`: Node names.
    /// - `observed`: Optional conditioning set.
    /// - `include_latents`: If `false`, latent variables are excluded from reachable sets.
    pub fn is_dconnected(
        &self,
        start: &str,
        end: &str,
        observed: Option<Vec<String>>,
        include_latents: bool,
    ) -> Result<bool, String> {
        let trails = self.active_trail_nodes(vec![start.to_string()], observed, include_latents)?;
        Ok(trails
            .get(start)
            .map(|nodes| nodes.contains(end))
            .unwrap_or(false))
    }

    /// Find a **minimal d-separating set** for `start` and `end`, if one exists.
    ///
    /// Implements the classic approach:
    /// 1) Work in the **ancestral graph** of `{start, end}`.
    /// 2) Start from `parents(start) U parents(end)` (replacing latent parents by their observable parents if requested).
    /// 3) Greedily remove redundant variables while preserving d-separation.
    ///
    /// # Parameters
    /// - `start`, `end`: Node names.
    /// - `include_latents`: If `true`, latent variables may appear in the separator; otherwise they are replaced by observable parents.
    ///
    /// # Returns
    /// - `Ok(Some(S))` if a minimal separator `S` exists.
    /// - `Ok(None)` if no separator exists (i.e., still d-connected after step 2).
    ///
    /// # Errors
    /// Returns an error if `start` and `end` are adjacent (no separator possible).
    ///
    /// # References
    /// Tian, Paz, Pearl (1998), *Finding Minimal d-Separators*.
    pub fn minimal_dseparator(
        &self,
        starts: Vec<String>,
        ends: Vec<String>,
        include_latents: bool,
    ) -> Result<Option<HashSet<String>>, String> {
        // Validate inputs
        if starts.is_empty() || ends.is_empty() {
            return Ok(Some(HashSet::new()));
        }

        // Check for adjacent pairs - if any start-end pair is adjacent, no separator exists
        for start in &starts {
            for end in &ends {
                if self.has_edge(start, end) || self.has_edge(end, start) {
                    return Err(format!(
                        "No possible separators because {} and {} are adjacent",
                        start, end
                    ));
                }
            }
        }

        
        // Create ancestral graph containing only ancestors of all starts and ends
        let mut all_nodes = starts.clone();
        all_nodes.extend(ends.clone());
        let ancestral_graph = self.get_ancestral_graph(all_nodes)?;

        // Initial separator: all parents of all start and end nodes
        let mut separator: HashSet<String> = HashSet::new();
        
        for start in &starts {
            separator.extend(self.get_parents(start)?);
        }
        for end in &ends {
            separator.extend(self.get_parents(end)?);
        }

        // Replace latent variables with their observable parents
        if !include_latents {
            let mut changed = true;
            while changed {
                changed = false;
                let mut new_separator: HashSet<String> = HashSet::new();

                for node in &separator {
                    if self.latents.contains(node) {
                        new_separator.extend(self.get_parents(node)?);
                        changed = true;
                    } else {
                        new_separator.insert(node.clone());
                    }
                }
                separator = new_separator;
            }
        }

        // Remove starts and ends from separator (can't separate a node from itself)
        for start in &starts {
            separator.remove(start);
        }
        for end in &ends {
            separator.remove(end);
        }

        // Helper function to check if all start-end pairs are d-separated
        let check_all_separated = |sep: &[String]| -> Result<bool, String> {
            for start in &starts {
                for end in &ends {
                    if ancestral_graph.is_dconnected(start, end, Some(sep.to_vec()), include_latents)? {
                        return Ok(false); // Found a connected pair
                    }
                }
            }
            Ok(true) // All pairs are separated
        };

        // Sanity check: if our "guaranteed" separator doesn't work, no separator exists
        if !check_all_separated(&separator.iter().cloned().collect::<Vec<_>>())? {
            return Ok(None);
        }

        // Greedy minimization: remove each node if separation still holds without it
        let mut minimal_separator = separator.clone();
        for u in separator {
            let test_separator: Vec<String> = minimal_separator
                .iter()
                .cloned()
                .filter(|x| x != &u)
                .collect();

            // If all pairs are still d-separated WITHOUT this node, we can remove it
            if check_all_separated(&test_separator)? {
                minimal_separator.remove(&u);
            }
        }

        Ok(Some(minimal_separator))
    }

    /// Check whether two nodes are **neighbors** (adjacent in either direction).
    ///
    /// # Returns
    /// `true` if `start -> end` or `end -> start` exists.
    ///
    /// # Errors
    /// Returns an error if either node is missing.
    pub fn are_neighbors(&self, start: &str, end: &str) -> Result<bool, String> {
        let start_idx = self
            .node_map
            .get(start)
            .ok_or_else(|| format!("Node {} not found", start))?;
        let end_idx = self
            .node_map
            .get(end)
            .ok_or_else(|| format!("Node {} not found", end))?;

        // Check for edge in either direction
        let has_edge = self.graph.find_edge(*start_idx, *end_idx).is_some()
            || self.graph.find_edge(*end_idx, *start_idx).is_some();

        Ok(has_edge)
    }

    /// Return the **ancestral graph** induced by the ancestors of `nodes`.
    ///
    /// The returned DAG contains exactly the ancestors (including the nodes
    /// themselves) and all edges among them, preserving latent-status.
    ///
    /// # Parameters
    /// - `nodes`: Node names.
    ///
    /// # Returns
    /// A new `RustDAG` containing only the ancestors and their edges.
    ///
    /// # Errors
    /// Propagates errors from ancestor computation or edge insertion.
    pub fn get_ancestral_graph(&self, nodes: Vec<String>) -> Result<RustDAG, String> {
        let ancestors = self.get_ancestors_of(nodes)?;
        let mut ancestral_graph = RustDAG::new();

        // Add all ancestor nodes with their latent status
        for node in &ancestors {
            let is_latent = self.latents.contains(node);
            ancestral_graph.add_node(node.clone(), is_latent)?;
        }

        // Add edges between ancestors only
        for (source, target) in self.edges() {
            if ancestors.contains(&source) && ancestors.contains(&target) {
                ancestral_graph.add_edge(source, target, None)?;
            }
        }

        Ok(ancestral_graph)
    }

    /// Return the list of **leaves** (out-degree = 0).
    ///
    /// # Examples
    /// ```rust
    /// let mut g = RustDAG::new();
    /// g.add_edges_from(vec![("A".into(),"B".into()), ("B".into(),"C".into()), ("B".into(),"D".into())], None).unwrap();
    /// let mut leaves = g.get_leaves();
    /// ```
    pub fn get_leaves(&self) -> Vec<String> {
        self.graph
            .node_indices()
            .filter(|&idx| {
                self.graph
                    .neighbors_directed(idx, Direction::Outgoing)
                    .next()
                    .is_none()
            })
            .map(|idx| self.reverse_node_map[&idx].clone())
            .collect()
    }

    /// Return the list of **roots** (in-degree = 0).
    ///
    /// # Examples
    /// ```rust
    /// let mut g = RustDAG::new();
    /// g.add_edges_from(vec![
    ///   ("A".into(),"B".into()),
    ///   ("B".into(),"C".into()),
    ///   ("B".into(),"D".into()),
    ///   ("E".into(),"B".into())
    /// ], None).unwrap();
    /// let mut roots = g.get_roots();
    /// ```
    pub fn get_roots(&self) -> Vec<String> {
        self.graph
            .node_indices()
            .filter(|&idx| {
                self.graph
                    .neighbors_directed(idx, Direction::Incoming)
                    .next()
                    .is_none()
            })
            .map(|idx| self.reverse_node_map[&idx].clone())
            .collect()
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

    pub fn has_edge(&self, u: &str, v: &str) -> bool {
        match (self.node_map.get(u), self.node_map.get(v)) {
            (Some(u_idx), Some(v_idx)) => self.graph.find_edge(*u_idx, *v_idx).is_some(),
            _ => false,
        }
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
