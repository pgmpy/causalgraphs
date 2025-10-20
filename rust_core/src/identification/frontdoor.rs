use crate::identification::base::BaseIdentification;
use crate::dag::RustDAG;
use crate::graph::Graph;
use crate::graph_role::{GraphError, GraphRoles};
use std::collections::{HashMap, HashSet};
use itertools::Itertools; // For powerset

/// Adjustment class to validate backdoor adjustment sets.
pub struct Adjustment {
    variant: String,
}

impl Adjustment {
    pub fn new(variant: &str) -> Self {
        Adjustment {
            variant: variant.to_string(),
        }
    }

    /// Validate if the adjustment set blocks all backdoor paths from exposure to outcome.
    pub fn validate<T: Graph + GraphRoles>(
        &self,
        causal_graph: &T,
    ) -> Result<bool, GraphError> {
        let exposure = causal_graph.get_role("exposure");
        let outcome = causal_graph.get_role("outcome");
        let adjustment = causal_graph.get_role("adjustment");

        if exposure.is_empty() || outcome.is_empty() {
            return Err(GraphError::InvalidOperation(
                "Exposure and outcome roles must be defined".to_string(),
            ));
        }

        if exposure.len() > 1 || outcome.len() > 1 {
            return Err(GraphError::InvalidOperation(
                "Adjustment validation supports only single exposure and outcome".to_string(),
            ));
        }

        let exposure_str = exposure.first().unwrap();
        let outcome_str = outcome.first().unwrap();

        // Remove all outgoing edges from exposure to check only backdoor paths
        let edges_to_remove: Vec<(String, String)> = causal_graph
            .nodes()
            .into_iter()
            .filter_map(|node| {
                if causal_graph.parents(&node).ok()?.contains(&exposure_str.to_string()) {
                    Some((exposure_str.clone(), node))
                } else {
                    None
                }
            })
            .collect();
        
        let graph_without_forward_edges = causal_graph.remove_edges_from(edges_to_remove)?;

        // Check if there's any unblocked backdoor path
        // include_latents=true is critical - we need to check paths through latent confounders
        let has_unblocked_backdoor = graph_without_forward_edges.is_dconnected(
            exposure_str,
            outcome_str,
            Some(adjustment.clone()),
            true  // MUST be true to include latent variables
        )?;

        // Valid if no unblocked backdoor paths exist
        Ok(!has_unblocked_backdoor)
    }
}

/// Frontdoor identification for causal graphs.
pub struct Frontdoor {
    variant: Option<String>, // None or "all"
}

impl Frontdoor {
    /// Create a new Frontdoor instance.
    pub fn new(variant: Option<String>) -> Self {
        Frontdoor { variant }
    }

    /// Validate a frontdoor set in a causal graph.
    pub fn validate<T: Graph + GraphRoles>(
        &self,
        causal_graph: &T,
    ) -> Result<bool, GraphError> {
        let exposure = causal_graph.get_role("exposure");
        let outcome = causal_graph.get_role("outcome");
        let frontdoor = causal_graph.get_role("frontdoor");

        if exposure.is_empty() || outcome.is_empty() {
            return Err(GraphError::InvalidOperation(
                "Exposure and outcome roles must be defined".to_string(),
            ));
        }

        if exposure.len() > 1 || outcome.len() > 1 {
            return Err(GraphError::InvalidOperation(
                "Frontdoor identification supports only single exposure and outcome".to_string(),
            ));
        }

        let exposure = exposure.first().unwrap();
        let outcome = outcome.first().unwrap();

        println!("Validating frontdoor: exposure={}, outcome={}, frontdoor={:?}", exposure, outcome, frontdoor);

        // 0. Check for directed paths from X to Y
        let directed_paths = causal_graph.all_simple_edge_paths(exposure, outcome)?;
        println!("Step 0: directed_paths count = {}", directed_paths.len());
        if directed_paths.is_empty() {
            return Ok(false);
        }

        // 1. Z intercepts all directed paths from X to Y
        let unblocked_paths: Vec<_> = directed_paths
            .into_iter()
            .filter(|path| !path.iter().any(|(_, v)| frontdoor.contains(v)))
            .collect();
        println!("Step 1: unblocked_paths count = {}", unblocked_paths.len());
        if !unblocked_paths.is_empty() {
            return Ok(false);
        }

        // 2. No backdoor path from X to Z
        let adjustment = Adjustment::new("minimal");
        // In Frontdoor::validate, step 2:
        for z in &frontdoor {
            let mut graph_copy = causal_graph.clone();
            graph_copy = graph_copy.without_role_copy("exposure", None);
            graph_copy = graph_copy.without_role_copy("outcome", None);
            graph_copy = graph_copy.without_role_copy("adjustment", None);
                        
            graph_copy = graph_copy.with_role_copy("exposure".to_string(), vec![exposure.clone()])?;
            graph_copy = graph_copy.with_role_copy("outcome".to_string(), vec![z.clone()])?;
            graph_copy = graph_copy.with_role_copy("adjustment".to_string(), vec![])?;
                        
            let is_valid = adjustment.validate(&graph_copy)?;
            if !is_valid {
                return Ok(false);
            }
        }

        // 3. All backdoor paths from Z to Y are blocked by X
        for z in &frontdoor {
            let mut graph_copy = causal_graph.clone();
            graph_copy = graph_copy.without_role_copy("exposure", None);
            graph_copy = graph_copy.without_role_copy("outcome", None);
            graph_copy = graph_copy.without_role_copy("adjustment", None);
            graph_copy = graph_copy.with_role_copy("exposure".to_string(), vec![z.clone()])?;
            graph_copy = graph_copy.with_role_copy("outcome".to_string(), vec![outcome.clone()])?;
            graph_copy = graph_copy.with_role_copy("adjustment".to_string(), vec![exposure.clone()])?;
            
            let is_valid = adjustment.validate(&graph_copy)?;
            if !is_valid {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

impl BaseIdentification for Frontdoor {
    fn _identify<T: Graph + GraphRoles>(
        &self,
        causal_graph: &T,
    ) -> Result<(T, bool), GraphError> {
        let exposure = causal_graph.get_role("exposure");
        let outcome = causal_graph.get_role("outcome");

        if exposure.is_empty() || outcome.is_empty() {
            return Err(GraphError::InvalidOperation(
                "Exposure and outcome roles must be defined".to_string(),
            ));
        }

        // Get possible frontdoor variables: observed nodes excluding exposure and outcome
        let possible_frontdoor: HashSet<String> = causal_graph
            .nodes()
            .into_iter()
            .filter(|n| !causal_graph.get_role("exposure").contains(n))
            .filter(|n| !causal_graph.get_role("outcome").contains(n))
            .filter(|n| !causal_graph.get_role("latents").contains(n))
            .collect();

        // Generate powerset of possible frontdoor variables
        let mut valid_frontdoor_graphs = Vec::new();
        for s in possible_frontdoor.into_iter().powerset() {
            let s_vec: Vec<String> = s.into_iter().collect();
            let updated_graph = causal_graph.with_role_copy("frontdoor".to_string(), s_vec.clone())?;
            if self.validate(&updated_graph)? {
                if self.variant.is_none() {
                    return Ok((updated_graph, true));
                } else if self.variant.as_deref() == Some("all") {
                    valid_frontdoor_graphs.push(updated_graph);
                }
            }
        }

        if valid_frontdoor_graphs.is_empty() {
            Ok((causal_graph.clone(), false))
        } else {
            Ok((valid_frontdoor_graphs[0].clone(), true))
        }
    }
}