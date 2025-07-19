use std::collections::HashSet;

use rust_core::RustDAG;

#[test]
fn test_add_nodes_and_edges() {
    let mut dag = RustDAG::new();
    dag.add_node("A".to_string(), false).unwrap();
    dag.add_node("B".to_string(), false).unwrap();
    dag.add_edge("A".to_string(), "B".to_string(), None).unwrap();
    assert_eq!(dag.node_count(), 2);
    assert_eq!(dag.edge_count(), 1);
    assert!(dag.get_parents("B").unwrap().contains(&"A".to_string()));
}

#[test]
fn test_active_trail_basic() {
    let mut dag: RustDAG = RustDAG::new();
    dag.add_edges_from(
        vec![
            ("diff".to_string(), "grades".to_string()),
            ("intel".to_string(), "grades".to_string()),
        ],
        None,
    ).unwrap();

    let result = dag.active_trail_nodes(
        vec!["diff".to_string()], 
        None, 
        false
    ).unwrap();
        
    let expected: HashSet<String> = vec!["diff".to_string(), "grades".to_string()]
        .into_iter().collect();
    assert_eq!(result["diff"], expected);
}


#[test]
fn test_active_trail_with_observed() {
    let mut dag = RustDAG::new();
    dag.add_edges_from(
        vec![
            ("diff".to_string(), "grades".to_string()),
            ("intel".to_string(), "grades".to_string()),
        ],
        None,
    ).unwrap();

    let result = dag.active_trail_nodes(
        vec!["diff".to_string(), "intel".to_string()],
        Some(vec!["grades".to_string()]),
        false
    ).unwrap();
    // With grades observed, diff and intel should be in each other's active trail
    let expected_diff: HashSet<String> = vec!["diff".to_string(), "intel".to_string()]
        .into_iter().collect();
    let expected_intel: HashSet<String> = vec!["diff".to_string(), "intel".to_string()]
        .into_iter().collect();
        
    assert_eq!(result["diff"], expected_diff);
    assert_eq!(result["intel"], expected_intel);
}


 #[test]
fn test_is_dconnected() {
    let mut dag = RustDAG::new();
    dag.add_edges_from(
        vec![
            ("diff".to_string(), "grades".to_string()),
            ("intel".to_string(), "grades".to_string()),
            ("grades".to_string(), "letter".to_string()),
            ("intel".to_string(), "sat".to_string()),
        ],
        None,
    ).unwrap();
    // diff and intel are not d-connected (blocked by collider at grades)
    assert_eq!(dag.is_dconnected("diff", "intel", None, false).unwrap(), false);
    
    // grades and sat are d-connected through intel
    assert_eq!(dag.is_dconnected("grades", "sat", None, false).unwrap(), true);
    
    // diff and intel become d-connected when grades is observed
    assert_eq!(
        dag.is_dconnected("diff", "intel", Some(vec!["grades".to_string()]), false).unwrap(), 
        true
    );
}