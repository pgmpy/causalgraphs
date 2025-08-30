use std::collections::{HashMap, HashSet};
use rust_core::{graph::Graph, graph_role::{GraphError, GraphRoles}, identification::base, RustDAG};

use base::BaseIdentification;


/// A simple identification method that assigns the "adjustment" role to either
/// the first or last non-exposure, non-outcome node (alphabetically sorted),
/// based on the variant parameter.
#[derive(Debug, Clone)]
struct DummyIdentification {
    variant: Option<String>,
}
impl DummyIdentification {
    fn new(variant: Option<&str>) -> Self {
        DummyIdentification {
            variant: variant.map(|s| s.to_string()),
        }
    }
}
impl BaseIdentification for DummyIdentification {
    fn _identify<T:Graph + GraphRoles>(&self, causal_graph: &T) -> Result<(T, bool), GraphError> {
        let mut mutable_graph = causal_graph.clone();
        match self.variant.as_deref() {
            Some("first") => {
                let non_role_nodes: HashSet<String> = causal_graph
                    .nodes()
                    .into_iter()
                    .collect::<HashSet<String>>()
                    .difference(
                        &causal_graph
                            .get_role("exposure")
                            .into_iter()
                            .chain(causal_graph.get_role("outcome").into_iter())
                            .collect::<HashSet<String>>(),
                    )
                    .cloned()
                    .collect();
                let mut sorted_nodes: Vec<String> = non_role_nodes.into_iter().collect();
                sorted_nodes.sort();
                if let Some(adjustment_node) = sorted_nodes.first() {
                    let identified_cg = mutable_graph.with_role(
                        "adjustment".to_string(),
                        vec![adjustment_node.clone()],
                        false,
                    )?;
                    Ok((identified_cg, true))
                } else {
                    Ok((causal_graph.clone(), false))
                }
            }
            Some("last") => {
                let non_role_nodes: HashSet<String> = causal_graph
                    .nodes()
                    .into_iter()
                    .collect::<HashSet<String>>()
                    .difference(
                        &causal_graph
                            .get_role("exposure")
                            .into_iter()
                            .chain(causal_graph.get_role("outcome").into_iter())
                            .collect::<HashSet<String>>(),
                    )
                    .cloned()
                    .collect();
                let mut sorted_nodes: Vec<String> = non_role_nodes.into_iter().collect();
                sorted_nodes.sort();
                if let Some(adjustment_node) = sorted_nodes.last() {
                    let identified_cg = mutable_graph.with_role(
                        "adjustment".to_string(),
                        vec![adjustment_node.clone()],
                        false,
                    )?;
                    Ok((identified_cg, true))
                } else {
                    Ok((causal_graph.clone(), false))
                }
            }
            _ => Ok((causal_graph.clone(), false)),
        }
    }
}


#[test]
fn test_base_identification_first() {
    let mut cg = RustDAG::new();
    cg.add_edges_from(
        vec![
            ("U".to_string(), "X".to_string()),
            ("X".to_string(), "M".to_string()),
            ("M".to_string(), "Y".to_string()),
            ("U".to_string(), "Y".to_string()),
        ],
        None,
    )
    .unwrap();

    cg.with_role("exposure".to_string(), vec!["X".to_string()], true).unwrap();
    cg.with_role("outcome".to_string(), vec!["Y".to_string()], true).unwrap();
    let identifier = DummyIdentification::new(Some("first"));
    let (identified_cg, is_identified) = identifier.identify(&cg).unwrap();
    assert!(is_identified);
    let expected_roles: HashMap<String, Vec<String>> = [
        ("exposure".to_string(), vec!["X".to_string()]),
        ("outcome".to_string(), vec!["Y".to_string()]),
        ("adjustment".to_string(), vec!["M".to_string()]),
    ]
    .into_iter()
    .collect();
    assert_eq!(identified_cg.get_role_dict(), expected_roles);
}


#[test]
fn test_base_identification_last() {
    let mut cg = RustDAG::new();
    cg.add_edges_from(
        vec![
            ("U".to_string(), "X".to_string()),
            ("X".to_string(), "M".to_string()),
            ("M".to_string(), "Y".to_string()),
            ("U".to_string(), "Y".to_string()),
        ],
        None,
    )
    .unwrap();
    cg.with_role("exposure".to_string(), vec!["X".to_string()], true).unwrap();
    cg.with_role("outcome".to_string(), vec!["Y".to_string()], true).unwrap();
    let identifier = DummyIdentification::new(Some("last"));
    let (identified_cg, is_identified) = identifier.identify(&cg).unwrap();
    assert!(is_identified);
    let expected_roles: HashMap<String, Vec<String>> = [
        ("exposure".to_string(), vec!["X".to_string()]),
        ("outcome".to_string(), vec!["Y".to_string()]),
        ("adjustment".to_string(), vec!["U".to_string()]),
    ]
    .into_iter()
    .collect();
    assert_eq!(identified_cg.get_role_dict(), expected_roles);
}

#[test]
fn test_base_identification_gibberish() {
    let mut cg = RustDAG::new();
    cg.add_edges_from(
        vec![
            ("U".to_string(), "X".to_string()),
            ("X".to_string(), "M".to_string()),
            ("M".to_string(), "Y".to_string()),
            ("U".to_string(), "Y".to_string()),
        ],
        None,
    )
    .unwrap();
    cg.with_role("exposure".to_string(), vec!["X".to_string()], true).unwrap();
    cg.with_role("outcome".to_string(), vec!["Y".to_string()], true).unwrap();
    let identifier = DummyIdentification::new(Some("gibberish"));
    let (identified_cg, is_identified) = identifier.identify(&cg).unwrap();
    assert!(!is_identified);
    let expected_roles: HashMap<String, Vec<String>> = [
        ("exposure".to_string(), vec!["X".to_string()]),
        ("outcome".to_string(), vec!["Y".to_string()]),
    ]
    .into_iter()
    .collect();
    assert_eq!(identified_cg.get_role_dict(), expected_roles);
}