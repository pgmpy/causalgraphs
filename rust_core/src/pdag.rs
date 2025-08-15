use petgraph::Direction;
use rustworkx_core::petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, HashSet};
use petgraph::visit::Dfs;

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

    /// Get all edges in the graph - DETERMINISTIC
    pub fn edges(&self) -> Vec<(String, String)> {
        let mut edges: Vec<(String, String)> = self.graph
            .edge_indices()
            .map(|edge_idx| {
                let (source, target) = self.graph.edge_endpoints(edge_idx).unwrap();
                (
                    self.reverse_node_map[&source].clone(),
                    self.reverse_node_map[&target].clone(),
                )
            })
            .collect();
        edges.sort();
        edges
    }

    /// Get all nodes in the graph
    pub fn nodes(&self) -> Vec<String> {
        let mut nodes: Vec<String> = self.node_map.keys().cloned().collect();
        nodes.sort();
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
            .filter(|&idx| self.reverse_node_map.contains_key(&idx))
            .map(|idx| self.reverse_node_map[&idx].clone())
            .collect();

        let predecessors: HashSet<String> = self.graph
            .neighbors_directed(*node_idx, Direction::Incoming)
            .filter(|&idx| self.reverse_node_map.contains_key(&idx))
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
                if let Some(child) = self.reverse_node_map.get(&idx) {
                    self.directed_edges.contains(&(node.to_string(), child.to_string()))
                } else {
                    false // Skip invalid indices
                }
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
                if let Some(parent) = self.reverse_node_map.get(&idx) {
                    self.directed_edges.contains(&(parent.to_string(), node.to_string()))
                } else {
                    false // Skip invalid indices
                }
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
                if let Some(neighbor) = self.reverse_node_map.get(&idx) {
                    self.has_undirected_edge(node, neighbor)
                } else {
                    false // Skip invalid indices
                }
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

    /// Returns a subgraph containing only directed edges as a RustDAG - DETERMINISTIC
    pub fn directed_graph(&self) -> RustDAG {
        let mut dag = RustDAG::new();

        // Add all nodes with their latent status - DETERMINISTIC ORDER
        let mut nodes: Vec<String> = self.node_map.keys().cloned().collect();
        nodes.sort();
        for node in nodes {
            let is_latent = self.latents.contains(&node);
            dag.add_node(node.clone(), is_latent).unwrap();
        }

        // Add only directed edges
        let mut directed_edges: Vec<(String, String)> = self.directed_edges.iter().cloned().collect();
        directed_edges.sort();
        for (u, v) in directed_edges {
            dag.add_edge(u, v, None).unwrap();
        }

        dag
    }

    /// Orient an undirected edge u - v as u -> v
    pub fn orient_undirected_edge(&mut self, u: &str, v: &str, inplace: bool) -> Result<Option<RustPDAG>, String> {
        let mut pdag = if inplace { 
            self
        } else { 
            &mut self.copy()
        };

        // Check if undirected edge exists
        let edge_exists = if pdag.undirected_edges.contains(&(u.to_string(), v.to_string())) {
            pdag.undirected_edges.remove(&(u.to_string(), v.to_string()));
            true
        } else if pdag.undirected_edges.contains(&(v.to_string(), u.to_string())) {
            pdag.undirected_edges.remove(&(v.to_string(), u.to_string()));
            true
        } else {
            false
        };

        if !edge_exists {
            return Err(format!("Undirected Edge {} - {} not present in the PDAG", u, v));
        }

        // Remove the reverse edge from the graph
        let u_idx = pdag.node_map[u];
        let v_idx = pdag.node_map[v];
        
        // Find and remove the edge v -> u
        if let Some(edge_idx) = pdag.graph.find_edge(v_idx, u_idx) {
            pdag.graph.remove_edge(edge_idx);
        }

        // Add to directed edges
        pdag.directed_edges.insert((u.to_string(), v.to_string()));

        if inplace {
            Ok(None)
        } else {
            Ok(Some(pdag.clone()))
        }
    }

    /// Check if orienting u -> v would create a new unshielded collider
    fn check_new_unshielded_collider(&self, u: &str, v: &str) -> Result<bool, String> {
        let parents = self.directed_parents(v)?;
        
        for parent in parents {
            if parent != u && !self.is_adjacent(u, &parent) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Check if there's a path from source to target in the directed subgraph
    pub fn has_directed_path(&self, source: &str, target: &str) -> Result<bool, String> {
        let source_idx = self.node_map.get(source)
            .ok_or_else(|| format!("Node {} not found", source))?;
        let target_idx = self.node_map.get(target)
            .ok_or_else(|| format!("Node {} not found", target))?;

        let directed_graph = self.directed_graph();
        let mut dfs = Dfs::new(&directed_graph.graph, *source_idx);
        
        while let Some(nx) = dfs.next(&directed_graph.graph) {
            if nx == *target_idx {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Apply Meek's rules to orient undirected edges
    pub fn apply_meeks_rules(&mut self, apply_r4: bool, inplace: bool) -> Result<Option<RustPDAG>, String> {
        if inplace {
            // Work directly on self
            self.apply_meeks_rules_internal(apply_r4)?;
            Ok(None)
        } else {
            // Work on a copy
            let mut pdag_copy = self.copy();
            pdag_copy.apply_meeks_rules_internal(apply_r4)?;
            Ok(Some(pdag_copy))
        }
    }

    /// Internal method that applies Meek's rules to the current instance
    fn apply_meeks_rules_internal(&mut self, apply_r4: bool) -> Result<(), String> {
        let mut changed = true;
        while changed {
            changed = false;
            let nodes: Vec<String> = self.nodes();

            // Rule 1: If X -> Y - Z and
            //            (X not adj Z) and
            //            (adding Y -> Z doesn't create cycle) and
            //            (adding Y -> Z doesn't create an unshielded collider) =>  Y → Z
            for y in &nodes {
                if !self.node_map.contains_key(y) {
                    continue;
                }
                // Convert HashSets to sorted vectors for deterministic iteration
                let mut directed_parents: Vec<String> = self.directed_parents(y)?.into_iter().collect();
                directed_parents.sort();
                let mut undirected_neighbors: Vec<String> = self.undirected_neighbors(y)?.into_iter().collect();
                undirected_neighbors.sort();

                for x in &directed_parents {
                    for z in &undirected_neighbors {
                        if !self.is_adjacent(x, z) && !self.check_new_unshielded_collider(y, z)? {
                            if self.orient_undirected_edge(y, z, true).is_ok() {
                                changed = true;
                                break;
                            }
                        }
                    }
                    if changed { break; }
                }
                if changed { break; }
            }
            if changed { continue; }

            // Rule 2: If X -> Z -> Y  and X - Y =>  X -> Y
            for z in &nodes {
                if !self.node_map.contains_key(z) {
                    continue;
                }
                // Convert HashSets to sorted vectors for deterministic iteration
                let mut parents: Vec<String> = self.directed_parents(z)?.into_iter().collect();
                parents.sort();
                let mut children: Vec<String> = self.directed_children(z)?.into_iter().collect();
                children.sort();

                for x in &parents {
                    for y in &children {
                        if self.has_undirected_edge(x, y) {
                            // Ensure x -> z and z -> y exist
                            if self.has_directed_edge(x, z) && self.has_directed_edge(z, y) {
                                if self.orient_undirected_edge(x, y, true).is_ok() {
                                    changed = true;
                                    break;
                                }
                            }
                        }
                    }
                    if changed { break; }
                }
                if changed { break; }
            }
            if changed { continue; }

            // Rule 3: If X - Y, X - Z, X - W and Y -> W, Z -> W => X -> W
            for x in &nodes {
                if !self.node_map.contains_key(x) {
                    continue;
                }
                let mut undirected_nbs: Vec<String> = self.undirected_neighbors(x)?.into_iter().collect();
                undirected_nbs.sort();

                if undirected_nbs.len() < 3 {
                    continue;
                }

                for i in 0..undirected_nbs.len() {
                    for j in (i + 1)..undirected_nbs.len() {
                        let y = &undirected_nbs[i];
                        let z = &undirected_nbs[j];

                        if self.is_adjacent(y, z) {
                            continue;
                        }

                        let y_children = self.directed_children(y)?;
                        let z_children = self.directed_children(z)?;

                        let common_children: HashSet<_> = y_children.intersection(&z_children).collect();

                        for w in common_children {
                            if self.has_undirected_edge(x, w) {
                                if self.orient_undirected_edge(x, w, true).is_ok() {
                                    changed = true;
                                    break;
                                }
                            }
                        }
                        if changed { break; }
                    }
                    if changed { break; }
                }
                if changed { break; }
            }
            if changed { continue; }

            // Rule 4
            if apply_r4 {
                for c in &nodes {
                    if !self.node_map.contains_key(c) {
                        continue;
                    }

                    let mut children: Vec<String> = self.directed_children(c)?.into_iter().collect();
                    children.sort();
                    let mut parents: Vec<String> = self.directed_parents(c)?.into_iter().collect();
                    parents.sort();

                    for b in &children {
                        for d in &parents {
                            if b == d || self.is_adjacent(b, d) {
                                continue;
                            }

                            let b_undirected_set = self.undirected_neighbors(b)?;
                            let c_neighbors_set = self.all_neighbors(c)?;
                            let d_undirected_set = self.undirected_neighbors(d)?;

                            let candidates: HashSet<_> = b_undirected_set.intersection(&c_neighbors_set).cloned().collect();
                            let final_candidates: HashSet<_> = candidates.intersection(&d_undirected_set).cloned().collect();
                            
                            let mut sorted_candidates: Vec<String> = final_candidates.into_iter().collect();
                            sorted_candidates.sort();

                            for a in sorted_candidates {
                                if self.orient_undirected_edge(&a, b, true).is_ok() {
                                    changed = true;
                                    break;
                                }
                            }
                            if changed { break; }
                        }
                        if changed { break; }
                    }
                    if changed { break; }
                }
            }
        }

        Ok(())
    }

    pub fn to_dag(&self) -> Result<RustDAG, String> {
        let mut dag = RustDAG::new();
        
        // Add all nodes with latent status
        for node in self.nodes() {
            let is_latent = self.latents.contains(&node);
            dag.add_node(node.clone(), is_latent)?;
        }
        
        // Add all directed edg
        let mut directed_edges_sorted: Vec<(String, String)> = self.directed_edges.iter().cloned().collect();
        directed_edges_sorted.sort();
        for (u, v) in directed_edges_sorted {
            dag.add_edge(u, v, None)?;
        }

        let mut pdag_copy = self.copy();
        
        // Add undirected edges to dag before node removal
        let mut undirected_edges_sorted: Vec<(String, String)> = self.undirected_edges.iter().cloned().collect();
        undirected_edges_sorted.sort();
        for (u, v) in undirected_edges_sorted {
            if !dag.has_edge(&u, &v) && !dag.has_edge(&v, &u) {
                // Try adding u -> v, if it creates cycle, add v -> u
                if dag.add_edge(u.clone(), v.clone(), None).is_err() {
                    dag.add_edge(v, u, None)?;
                }
            }
        }
        
        while !pdag_copy.nodes().is_empty() {
            let nodes: Vec<String> = pdag_copy.nodes();
            let mut found = false;
            
            for x in &nodes {
                // Check if node still exists
                if !pdag_copy.node_map.contains_key(x) {
                    continue;
                }
                
                // Find nodes with no directed outgoing edges
                let directed_children = pdag_copy.directed_children(x)?;
                let mut undirected_neighbors: Vec<String> = pdag_copy.undirected_neighbors(x)?.into_iter().collect();
                undirected_neighbors.sort();
                let mut directed_parents: Vec<String> = pdag_copy.directed_parents(x)?.into_iter().collect();
                directed_parents.sort();

                // Check if undirected neighbors + parents form a clique
                let mut neighbors_are_clique = true;
                for y in &undirected_neighbors {
                    for z in &directed_parents {
                        if y != z && !pdag_copy.is_adjacent(y, z) {
                            neighbors_are_clique = false;
                            break;
                        }
                    }
                    if !neighbors_are_clique { break; }
                }

                if directed_children.is_empty() && (undirected_neighbors.is_empty() || neighbors_are_clique) {
                    found = true;
                    
                    // Add all incoming edges to DAG
                    let mut all_predecessors: Vec<String> = pdag_copy.all_neighbors(x)?.into_iter().collect();
                    all_predecessors.sort();
                    for y in &all_predecessors {
                        if pdag_copy.is_adjacent(y, x) && !dag.has_edge(y, x) {
                            dag.add_edge(y.clone(), x.clone(), None)?;
                        }
                    }
                    
                    // Remove node from pdag_copy
                    pdag_copy.remove_node(x)?;
                    break; // Break to refresh node list
                }
            }

            if !found {
                // Handle remaining edges arbitrarily, ensuring no cycles
                let mut remaining_edges: Vec<(String, String)> = pdag_copy.undirected_edges.iter().cloned().collect();
                remaining_edges.sort(); // Deterministic order
                for (u, v) in remaining_edges {
                    if pdag_copy.node_map.contains_key(&u) && pdag_copy.node_map.contains_key(&v) && !dag.has_edge(&v, &u) {
                        if let Ok(()) = dag.add_edge(u.clone(), v.clone(), None) {
                            pdag_copy.orient_undirected_edge(&u, &v, true)?;
                        } else {
                            // Try reverse direction if adding u -> v creates a cycle
                            if !dag.has_edge(&u, &v) {
                                if let Ok(()) = dag.add_edge(v.clone(), u.clone(), None) {
                                    pdag_copy.orient_undirected_edge(&v, &u, true)?;
                                }
                            }
                        }
                    }
                }
                break;
            }
        }

        Ok(dag)
    }

    /// Remove a node from the PDAG
    fn remove_node(&mut self, node: &str) -> Result<(), String> {
        let node_idx = self.node_map.get(node)
            .ok_or_else(|| format!("Node {} not found", node))?;

        // Remove from edge sets
        self.directed_edges.retain(|(u, v)| u != node && v != node);
        self.undirected_edges.retain(|(u, v)| u != node && v != node);
        
        // Remove from latents
        self.latents.remove(node);
        
        // Remove from graph
        self.graph.remove_node(*node_idx);
        
        // Remove from mappings
        self.reverse_node_map.remove(node_idx);
        self.node_map.remove(node);

        Ok(())
    }
}