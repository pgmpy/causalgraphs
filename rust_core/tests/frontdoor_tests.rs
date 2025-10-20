use rust_core::identification::base::BaseIdentification;
use rust_core::dag::RustDAG;
use rust_core::graph::Graph;
use rust_core::graph_role::{GraphError, GraphRoles};
use std::collections::{HashMap, HashSet};
use itertools::Itertools; // For powerset


#[cfg(test)]
mod tests {
    use rust_core::identification::Frontdoor;

    use super::*;

    fn create_frontdoor_model() -> RustDAG {
        let mut dag = RustDAG::new();
        dag.add_nodes_from(vec!["X".to_string(), "M".to_string(), "Y".to_string()], None)
            .unwrap();
        dag.add_edges_from(vec![("X".to_string(), "M".to_string()), ("M".to_string(), "Y".to_string())], None)
            .unwrap();
        dag.with_role("exposure".to_string(), vec!["X".to_string()])
            .unwrap();
        dag.with_role("outcome".to_string(), vec!["Y".to_string()])
            .unwrap();
        dag
    }

    fn create_frontdoor_model_latent() -> RustDAG {
        let mut dag = RustDAG::new();
        dag.add_nodes_from(
            vec!["X".to_string(), "M".to_string(), "Y".to_string(), "U".to_string()],
            Some(vec![false, false, false, true]),
        )
        .unwrap();
        dag.add_edges_from(
            vec![
                ("X".to_string(), "M".to_string()),
                ("M".to_string(), "Y".to_string()),
                ("U".to_string(), "X".to_string()),
                ("U".to_string(), "Y".to_string()),
            ],
            None,
        )
        .unwrap();
        dag.with_role("exposure".to_string(), vec!["X".to_string()])
            .unwrap();
        dag.with_role("outcome".to_string(), vec!["Y".to_string()])
            .unwrap();
        dag
    }

    fn create_frontdoor_model_noniden() -> RustDAG {
        let mut dag = RustDAG::new();
        dag.add_nodes_from(
            vec!["X".to_string(), "M".to_string(), "Y".to_string(), "U".to_string()],
            Some(vec![false, false, false, true]),
        )
        .unwrap();
        dag.add_edges_from(
            vec![
                ("X".to_string(), "M".to_string()),
                ("M".to_string(), "Y".to_string()),
                ("U".to_string(), "X".to_string()),
                ("U".to_string(), "Y".to_string()),
                ("U".to_string(), "M".to_string()),
            ],
            None,
        )
        .unwrap();
        dag.with_role("exposure".to_string(), vec!["X".to_string()])
            .unwrap();
        dag.with_role("outcome".to_string(), vec!["Y".to_string()])
            .unwrap();
        dag
    }

    #[test]
    fn test_frontdoor() {
        let dag = create_frontdoor_model();
        let frontdoor = Frontdoor::new(None);
        let (identified_dag, is_identified) = frontdoor.identify(&dag).unwrap();

        assert!(is_identified);
        assert_eq!(identified_dag.get_role("exposure"), vec!["X"]);
        assert_eq!(identified_dag.get_role("outcome"), vec!["Y"]);
        assert_eq!(identified_dag.get_role("frontdoor"), vec!["M"]);
        assert_eq!(identified_dag.latents, HashSet::new());
    }

    #[test]
    fn test_frontdoor_latent() {
        let dag = create_frontdoor_model_latent();
        let frontdoor = Frontdoor::new(None);
        let (identified_dag, is_identified) = frontdoor.identify(&dag).unwrap();

        assert!(is_identified);
        assert_eq!(identified_dag.get_role("exposure"), vec!["X"]);
        assert_eq!(identified_dag.get_role("outcome"), vec!["Y"]);
        assert_eq!(identified_dag.get_role("frontdoor"), vec!["M"]);
        assert_eq!(identified_dag.latents, HashSet::from_iter(vec!["U".to_string()]));
    }

    #[test]
    fn test_frontdoor_noniden() {
        let dag = create_frontdoor_model_noniden();
        let frontdoor = Frontdoor::new(None);
        let (identified_dag, is_identified) = frontdoor.identify(&dag).unwrap();

        assert!(!is_identified);
        assert_eq!(identified_dag.get_role("exposure"), vec!["X"]);
        assert_eq!(identified_dag.get_role("outcome"), vec!["Y"]);
        assert_eq!(identified_dag.get_role("frontdoor"), Vec::<String>::new());
        assert_eq!(identified_dag.latents, HashSet::from_iter(vec!["U".to_string()]));
    }
}