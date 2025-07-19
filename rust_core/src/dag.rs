use petgraph::Direction;
use rustworkx_core::petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, HashSet, VecDeque};


#[derive(Debug, Clone)] // Add Debug for easier printing in Rust tests
pub struct RustDAG {
    pub graph: DiGraph<String, f64>, // Make fields public if bindings need direct access,
    pub node_map: HashMap<String, NodeIndex>, // or provide internal methods.
    pub reverse_node_map: HashMap<NodeIndex, String>,
    pub latents: HashSet<String>,
}

// All methods here should be public, but not necessarily #[pymethods]
// They are the *internal* implementations that the bindings will call.
impl RustDAG {
    pub fn new() -> Self {
        RustDAG {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
            reverse_node_map: HashMap::new(),
            latents: HashSet::new(),
        }
    }

    /// Add a single node to the graph
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

    /// Add multiple nodes to the graph
    pub fn add_nodes_from(&mut self, nodes: Vec<String>, latent: Option<Vec<bool>>) -> Result<(), String> {
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

    /// Add an edge between two nodes
    pub fn add_edge(&mut self, u: String, v: String, weight: Option<f64>) -> Result<(), String> {
        // Add nodes if they don't exist. Pass false for latent by default.
        self.add_node(u.clone(), false)?;
        self.add_node(v.clone(), false)?;

        let u_idx: NodeIndex = self.node_map[&u];
        let v_idx: NodeIndex = self.node_map[&v];

        self.graph.add_edge(u_idx, v_idx, weight.unwrap_or(1.0));
        Ok(())
    }

    pub fn add_edges_from(
        &mut self,
        ebunch: Vec<(String, String)>,
        weights: Option<Vec<f64>>,
    ) -> Result<(), String> {
        if let Some(ws) = &weights {
            if ebunch.len() != ws.len() {
                return Err("The number of elements in ebunch and weights should be equal".to_string());
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

    /// Get parents of a node
    pub fn get_parents(&self, node: &str) -> Result<Vec<String>, String> {
        let node_idx = self.node_map.get(node)
            .ok_or_else(|| format!("Node {} not found", node))?;

        let parents: Vec<String> = self.graph
            .neighbors_directed(*node_idx, Direction::Incoming)
            .map(|idx| self.reverse_node_map[&idx].clone())
            .collect();

        Ok(parents)
    }

    /// Get children of a node
    pub fn get_children(&self, node: &str) -> Result<Vec<String>, String> {
        let node_idx = self.node_map.get(node)
            .ok_or_else(|| format!("Node {} not found", node))?;

        let children: Vec<String> = self.graph
            .neighbors_directed(*node_idx, Direction::Outgoing)
            .map(|idx: NodeIndex| self.reverse_node_map[&idx].clone())
            .collect();

        Ok(children)
    }

    /// Get all ancestors of given nodes (optimized Rust implementation)
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
            for parent_idx in self.graph.neighbors_directed(current_idx, Direction::Incoming) {
                if let Some(parent_name) = self.reverse_node_map.get(&parent_idx) {
                    if ancestors.insert(parent_name.clone()) {
                        queue.push_back(parent_idx);
                    }
                } else {
                    return Err(format!("Node index {:?} not found in reverse map", parent_idx));
                }
            }
        }

        Ok(ancestors)
    }


    pub fn active_trail_nodes(&self, variables: Vec<String>, observed: Option<Vec<String>>, include_latents: bool) -> Result<HashMap<String, HashSet<String>>, String> {
        let observed_list: HashSet<String> = observed.unwrap_or_default().into_iter().collect();
        let ancestors_list: HashSet<String> = self.get_ancestors_of(observed_list.iter().cloned().collect())?;

        let mut active_trails: HashMap<String, HashSet<String>> = HashMap::new();
        for start in variables {
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
                    if !observed_list.contains(&node) {
                        active_nodes.insert(node.clone());
                    }
                    traversed_list.insert((node.clone(), direction));

                    if direction == "up" && !observed_list.contains(&node) {
                        for parent in self.get_parents(&node)? {
                            visit_list.insert((parent, "up"));
                        }
                        for child in self.get_children(&node)? {
                            visit_list.insert((child, "down"));
                        }
                    } else if direction == "down" {
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

    pub fn is_dconnected(&self, start: &str, end: &str, observed: Option<Vec<String>>, include_latents: bool) -> Result<bool, String> {
        let trails = self.active_trail_nodes(vec![start.to_string()], observed, include_latents)?;
        Ok(trails.get(start).map(|nodes| nodes.contains(end)).unwrap_or(false))
    }

    /// Check if two nodes are neighbors (directly connected in either direction)
    pub fn are_neighbors(&self, start: &str, end: &str) -> Result<bool, String> {
        let start_idx = self.node_map.get(start)
            .ok_or_else(|| format!("Node {} not found", start))?;
        let end_idx = self.node_map.get(end)
            .ok_or_else(|| format!("Node {} not found", end))?;

        // Check for edge in either direction
        let has_edge = self.graph.find_edge(*start_idx, *end_idx).is_some() ||
                      self.graph.find_edge(*end_idx, *start_idx).is_some();

        Ok(has_edge)
    }

    /// Get ancestral graph containing only ancestors of the given nodes
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



    /// Returns a list of leaves (nodes with out-degree 0)
    pub fn get_leaves(&self) -> Vec<String> {
        self.graph
            .node_indices()
            .filter(|&idx| self.graph.neighbors_directed(idx, Direction::Outgoing).next().is_none())
            .map(|idx| self.reverse_node_map[&idx].clone())
            .collect()
    }

    /// Returns a list of roots (nodes with in-degree 0)
    pub fn get_roots(&self) -> Vec<String> {
        self.graph
            .node_indices()
            .filter(|&idx| self.graph.neighbors_directed(idx, Direction::Incoming).next().is_none())
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