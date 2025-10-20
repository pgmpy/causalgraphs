use std::collections::{HashMap, HashSet};

/// Custom error type for graph operations.
#[derive(Debug)]
pub enum GraphError {
    NodeNotFound(String),
    InvalidOperation(String),
}

impl std::fmt::Display for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GraphError::NodeNotFound(node) => write!(f, "Node '{}' not found in the graph", node),
            GraphError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
        }
    }
}

impl std::error::Error for GraphError {}

/// Trait for handling roles in graphs (similar to Python mixin).
pub trait GraphRoles: Clone {
    /// Check if a node exists in the graph.
    fn has_node(&self, node: &str) -> bool;

    /// Get immutable reference to the roles map.
    fn get_roles_map(&self) -> &HashMap<String, HashSet<String>>;

    /// Get mutable reference to the roles map.
    fn get_roles_map_mut(&mut self) -> &mut HashMap<String, HashSet<String>>;

    /// Get nodes with a specific role.
    fn get_role(&self, role: &str) -> Vec<String> {
        self.get_roles_map()
            .get(role)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .collect()
    }

    /// Get list of all roles.
    fn get_roles(&self) -> Vec<String> {
        self.get_roles_map().keys().cloned().collect()
    }

    /// Get dict of roles to nodes.
    fn get_role_dict(&self) -> HashMap<String, Vec<String>> {
        self.get_roles_map()
            .iter()
            .map(|(k, v)| (k.clone(), v.iter().cloned().collect()))
            .collect()
    }

    /// Check if a role exists and has nodes.
    fn has_role(&self, role: &str) -> bool {
        self.get_roles_map()
            .get(role)
            .map(|set| !set.is_empty())
            .unwrap_or(false)
    }

    /// Assign role to variables in-place, modifying the graph.
    fn with_role(&mut self, role: String, variables: Vec<String>) -> Result<(), GraphError> {
        for var in &variables {
            if !self.has_node(var) {
                return Err(GraphError::NodeNotFound(var.clone()));
            }
        }
        let roles_map = self.get_roles_map_mut();
        let entry = roles_map.entry(role).or_insert(HashSet::new());
        for var in variables {
            entry.insert(var);
        }
        Ok(())
    }

    /// Assign role to variables, returning a new graph without modifying the original.
    fn with_role_copy(&self, role: String, variables: Vec<String>) -> Result<Self, GraphError> {
        let mut new_graph = self.clone();
        new_graph.with_role(role, variables)?;
        Ok(new_graph)
    }

    /// Remove role from variables (or all if None) in-place.
    fn without_role(&mut self, role: &str, variables: Option<Vec<String>>) -> () {
        if let Some(set) = self.get_roles_map_mut().get_mut(role) {
            if let Some(vars) = variables {
                for var in vars {
                    set.remove(&var);
                }
            } else {
                set.clear();
            }
        }
    }

    /// Remove role from variables (or all if None), returning a new graph.
    fn without_role_copy(&self, role: &str, variables: Option<Vec<String>>) -> Self {
        let mut new_graph = self.clone();
        new_graph.without_role(role, variables);
        new_graph
    }

    /// Validate causal structure (has exposure and outcome).
    fn is_valid_causal_structure(&self) -> Result<bool, GraphError> {
        let has_exposure = self.has_role("exposure");
        let has_outcome = self.has_role("outcome");
        if !has_exposure || !has_outcome {
            let mut problems = Vec::new();
            if !has_exposure {
                problems.push("no 'exposure' role was defined");
            }
            if !has_outcome {
                problems.push("no 'outcome' role was defined");
            }
            return Err(GraphError::InvalidOperation(problems.join(", and ")));
        }
        Ok(true)
    }
}