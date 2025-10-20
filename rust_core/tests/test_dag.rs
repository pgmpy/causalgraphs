use std::collections::HashSet;
use rust_core::graph_role::GraphError;
use std::hash::{Hash, Hasher};
use rust_core::{graph_role::GraphRoles, RustDAG};

#[test]
fn test_add_nodes_and_edges() {
    let mut dag = RustDAG::new();
    dag.add_node("A".to_string(), false).unwrap();
    dag.add_node("B".to_string(), false).unwrap();
    dag.add_edge("A".to_string(), "B".to_string(), None)
        .unwrap();
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
    )
    .unwrap();

    let result = dag
        .active_trail_nodes(vec!["diff".to_string()], None, false)
        .unwrap();

    let expected: HashSet<String> = vec!["diff".to_string(), "grades".to_string()]
        .into_iter()
        .collect();
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
    )
    .unwrap();

    let result = dag
        .active_trail_nodes(
            vec!["diff".to_string(), "intel".to_string()],
            Some(vec!["grades".to_string()]),
            false,
        )
        .unwrap();
    // With grades observed, diff and intel should be in each other's active trail
    let expected_diff: HashSet<String> = vec!["diff".to_string(), "intel".to_string()]
        .into_iter()
        .collect();
    let expected_intel: HashSet<String> = vec!["diff".to_string(), "intel".to_string()]
        .into_iter()
        .collect();

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
    )
    .unwrap();
    // diff and intel are not d-connected (blocked by collider at grades)
    assert_eq!(
        dag.is_dconnected("diff", "intel", None, false).unwrap(),
        false
    );

    // grades and sat are d-connected through intel
    assert_eq!(
        dag.is_dconnected("grades", "sat", None, false).unwrap(),
        true
    );

    // diff and intel become d-connected when grades is observed
    assert_eq!(
        dag.is_dconnected("diff", "intel", Some(vec!["grades".to_string()]), false)
            .unwrap(),
        true
    );
}

#[test]
fn test_are_neighbors() {
    let mut dag = RustDAG::new();
    dag.add_edges_from(
        vec![
            ("A".to_string(), "B".to_string()),
            ("B".to_string(), "C".to_string()),
        ],
        None,
    )
    .unwrap();
    assert_eq!(dag.are_neighbors("A", "B").unwrap(), true);
    assert_eq!(dag.are_neighbors("B", "A").unwrap(), true); // Should work both ways
    assert_eq!(dag.are_neighbors("A", "C").unwrap(), false);
}

#[test]
fn test_minimal_dseparator_simple() {
    let mut dag = RustDAG::new();
    dag.add_edges_from(
        vec![
            ("A".to_string(), "B".to_string()),
            ("B".to_string(), "C".to_string()),
        ],
        None,
    )
    .unwrap();
    let result = dag.minimal_dseparator(vec!["A".to_string()], vec!["C".to_string()], false).unwrap();
    let expected: HashSet<String> = vec!["B".to_string()].into_iter().collect();
    assert_eq!(result, Some(expected));
}

#[test]
fn test_minimal_dseparator_complex() {
    let mut dag = RustDAG::new();
    dag.add_edges_from(
        vec![
            ("A".to_string(), "B".to_string()),
            ("B".to_string(), "C".to_string()),
            ("C".to_string(), "D".to_string()),
            ("A".to_string(), "E".to_string()),
            ("E".to_string(), "D".to_string()),
        ],
        None,
    )
    .unwrap();
    let result = dag.minimal_dseparator(vec!["A".to_string()], vec!["D".to_string()], false).unwrap();
    let expected: HashSet<String> = vec!["C".to_string(), "E".to_string()].into_iter().collect();
    assert_eq!(result, Some(expected));
}

#[test]
fn test_minimal_dseparator_latent_case_1() {
    let mut dag = RustDAG::new();
    dag.add_node("A".to_string(), false).unwrap();
    dag.add_node("B".to_string(), true).unwrap(); // latent
    dag.add_node("C".to_string(), false).unwrap();
    dag.add_edges_from(
        vec![
            ("A".to_string(), "B".to_string()),
            ("B".to_string(), "C".to_string()),
        ],
        None,
    )
    .unwrap();
    // No d-separator should exist because B is latent
    let result = dag.minimal_dseparator(vec!["A".to_string()], vec!["C".to_string()], false).unwrap();
    assert_eq!(result, None);
}

#[test]
fn test_minimal_dseparator_latent_case_2() {
    let mut dag = RustDAG::new();
    dag.add_node("A".to_string(), false).unwrap();
    dag.add_node("D".to_string(), false).unwrap();
    dag.add_node("B".to_string(), true).unwrap(); // B is latent
    dag.add_node("C".to_string(), false).unwrap();
    dag.add_edges_from(
        vec![
            ("A".to_string(), "D".to_string()),
            ("D".to_string(), "B".to_string()),
            ("B".to_string(), "C".to_string()),
        ],
        None,
    )
    .unwrap();

    let result = dag.minimal_dseparator(vec!["A".to_string()], vec!["C".to_string()], false).unwrap();
    let expected: HashSet<String> = vec!["D".to_string()].into_iter().collect();
    assert_eq!(
        result,
        Some(expected),
        "Expected D to d-separate A and C when B is latent"
    );
}

#[test]
fn test_minimal_dseparator_latent_case_3() {
    let mut dag = RustDAG::new();
    dag.add_node("A".to_string(), false).unwrap();
    dag.add_node("B".to_string(), false).unwrap();
    dag.add_node("C".to_string(), false).unwrap();
    dag.add_node("D".to_string(), true).unwrap(); // D is latent
    dag.add_edges_from(
        vec![
            ("A".to_string(), "B".to_string()),
            ("B".to_string(), "C".to_string()),
            ("A".to_string(), "D".to_string()),
            ("D".to_string(), "C".to_string()),
        ],
        None,
    )
    .unwrap();
    let result = dag.minimal_dseparator(vec!["A".to_string()], vec!["C".to_string()], false).unwrap();
    assert_eq!(result, None, "Expected no d-separator when D is latent with multiple paths A→B→C and A→D→C (D is unobservable, because of its latent status)");
}

#[test]
fn test_minimal_dseparator_latent_case_5() {
    let mut dag = RustDAG::new();
    dag.add_node("A".to_string(), false).unwrap();
    dag.add_node("B".to_string(), false).unwrap();
    dag.add_node("C".to_string(), false).unwrap();
    dag.add_node("D".to_string(), false).unwrap();
    dag.add_node("E".to_string(), true).unwrap(); // E is latent
    dag.add_edges_from(
        vec![
            ("A".to_string(), "B".to_string()),
            ("B".to_string(), "C".to_string()),
            ("A".to_string(), "D".to_string()),
            ("D".to_string(), "E".to_string()),
            ("E".to_string(), "C".to_string()),
        ],
        None,
    )
    .unwrap();
    let result = dag.minimal_dseparator(vec!["A".to_string()], vec!["C".to_string()], false).unwrap();
    let expected: HashSet<String> = vec!["B".to_string(), "D".to_string()].into_iter().collect();
    assert_eq!(
        result,
        Some(expected),
        "Expected [B, D] to d-separate A and C when E is latent (Observe B & parent of C => D)"
    );
}

#[test]
fn test_minimal_dseparator_adjacent_error() {
    let mut dag = RustDAG::new();
    dag.add_edges_from(vec![("A".to_string(), "B".to_string())], None)
        .unwrap();

    let result = dag.minimal_dseparator(vec!["A".to_string()], vec!["B".to_string()], false);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("adjacent"));
}

#[test]
fn test_role_hash_equality() {
    let mut dag = RustDAG::new();
    dag.add_edges_from(
        vec![
            ("A".to_string(), "B".to_string()),
            ("B".to_string(), "C".to_string()),
            ("C".to_string(), "D".to_string()),
            ("A".to_string(), "E".to_string()),
            ("E".to_string(), "D".to_string()),
        ],
        None,
    )
    .unwrap();

    let dag1 = dag.clone();
    let dag2 = dag.clone();

    assert_eq!(dag1, dag2);
}

// Helper function to calculate hash value as u64
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

#[test]
fn test_hash() {
    // Create two identical M-bias DAGs: A -> B <- C, B -> D, E -> D
    let mut dag1 = RustDAG::new();
    dag1.add_edges_from(
        vec![
            ("A".to_string(), "B".to_string()),
            ("C".to_string(), "B".to_string()),
            ("B".to_string(), "D".to_string()),
            ("E".to_string(), "D".to_string()),
        ],
        None,
    )
    .unwrap();

    let mut dag2 = RustDAG::new();
    dag2.add_edges_from(
        vec![
            ("A".to_string(), "B".to_string()),
            ("C".to_string(), "B".to_string()),
            ("B".to_string(), "D".to_string()),
            ("E".to_string(), "D".to_string()),
        ],
        None,
    )
    .unwrap();

    // Test identical DAGs have same hash
    assert_eq!(dag1, dag2);
    assert_eq!(calculate_hash(&dag1), calculate_hash(&dag2));

    // Add exposure role to dag1
    dag1.with_role("exposure".to_string(), vec!["E".to_string()]).unwrap();
    assert_ne!(dag1, dag2);
    assert_ne!(calculate_hash(&dag1), calculate_hash(&dag2));

    // Add exposure role to dag2
    dag2.with_role("exposure".to_string(), vec!["E".to_string()]).unwrap();
    assert_eq!(dag1, dag2);
    assert_eq!(calculate_hash(&dag1), calculate_hash(&dag2));

    // Add outcome role to dag1
    dag1.with_role("outcome".to_string(), vec!["D".to_string()]).unwrap();
    assert_ne!(dag1, dag2);
    assert_ne!(calculate_hash(&dag1), calculate_hash(&dag2));

    // Add outcome role to dag2
    dag2.with_role("outcome".to_string(), vec!["D".to_string()]).unwrap();
    assert_eq!(dag1, dag2);
    assert_eq!(calculate_hash(&dag1), calculate_hash(&dag2));
}

#[test]
fn test_roles() {
    let mut dag = RustDAG::new();
    dag.add_edges_from(
        vec![
            ("A".to_string(), "B".to_string()),
            ("B".to_string(), "C".to_string()),
            ("C".to_string(), "D".to_string()),
            ("A".to_string(), "E".to_string()),
            ("E".to_string(), "D".to_string()),
        ],
        None,
    )
    .unwrap();

    // Test assigning role to existing node
    let result = dag.with_role("exposure".to_string(), vec!["A".to_string()]);
    assert!(result.is_ok());
    assert!(dag.has_role("exposure"));
    assert_eq!(dag.get_role("exposure"), vec!["A".to_string()]);

    // Test assigning role to non-existent node (should fail)
    let result = dag.with_role("exposure".to_string(), vec!["Z".to_string()]);
    assert!(matches!(result, Err(GraphError::NodeNotFound(ref s)) if s == "Z"));
    // Verify exposure role still contains only "A"
    assert_eq!(dag.get_role("exposure"), vec!["A".to_string()]);

    // Test assigning outcome role
    let result = dag.with_role("outcome".to_string(), vec!["D".to_string()]);
    assert!(result.is_ok());
    assert!(dag.has_role("outcome"));
    assert_eq!(dag.get_role("outcome"), vec!["D".to_string()]);

    // Test valid causal structure
    assert!(dag.is_valid_causal_structure().is_ok());

    // Test removing role
    dag.without_role("exposure", None);
    assert!(!dag.has_role("exposure"));
    assert!(dag.is_valid_causal_structure().is_err());
}

#[test]
fn test_multiple_chains_separation() {
    // Graph: A1→B1→C1, A2→B2→C2, A3→B3→C3 (three independent chains)
    let mut dag = RustDAG::new();
    dag.add_edges_from(
        vec![
            ("A1".to_string(), "B1".to_string()),
            ("B1".to_string(), "C1".to_string()),
            ("A2".to_string(), "B2".to_string()),
            ("B2".to_string(), "C2".to_string()),
            ("A3".to_string(), "B3".to_string()),
            ("B3".to_string(), "C3".to_string()),
        ],
        None,
    ).unwrap();
    
    // Separate {A1, A2, A3} from {C1, C2, C3}
    let result = dag.minimal_dseparator(
        vec!["A1".to_string(), "A2".to_string(), "A3".to_string()], 
        vec!["C1".to_string(), "C2".to_string(), "C3".to_string()], 
        false
    ).unwrap();
    
    let expected: HashSet<String> = vec![
        "B1".to_string(), "B2".to_string(), "B3".to_string()
    ].into_iter().collect();
    assert_eq!(result, Some(expected));
}

#[test]
fn test_fork_convergence_pattern() {
    // Graph: A→B, A→C, B→D, C→D (fork from A, convergence at D)
    let mut dag = RustDAG::new();
    dag.add_edges_from(
        vec![
            ("A".to_string(), "B".to_string()),
            ("A".to_string(), "C".to_string()),
            ("B".to_string(), "D".to_string()),
            ("C".to_string(), "D".to_string()),
        ],
        None,
    ).unwrap();
    
    // Separate {B, C} from {D} - should need both B and C
    let result = dag.minimal_dseparator(
        vec!["B".to_string(), "C".to_string()], 
        vec!["D".to_string()], 
        false
    );

    // No separator should exist because B→D and C→D are direct edges
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("No possible separators because B and D are adjacent"));
}

#[test]
fn test_diamond_pattern_multiple() {
    // Graph: Multiple diamond patterns
    // A1→B1, A1→C1, B1→D1, C1→D1
    // A2→B2, A2→C2, B2→D2, C2→D2
    let mut dag = RustDAG::new();
    dag.add_edges_from(
        vec![
            ("A1".to_string(), "B1".to_string()),
            ("A1".to_string(), "C1".to_string()),
            ("B1".to_string(), "D1".to_string()),
            ("C1".to_string(), "D1".to_string()),
            ("A2".to_string(), "B2".to_string()),
            ("A2".to_string(), "C2".to_string()),
            ("B2".to_string(), "D2".to_string()),
            ("C2".to_string(), "D2".to_string()),
        ],
        None,
    ).unwrap();
    
    // Separate {A1, A2} from {D1, D2}
    let result = dag.minimal_dseparator(
        vec!["A1".to_string(), "A2".to_string()], 
        vec!["D1".to_string(), "D2".to_string()], 
        false
    ).unwrap();
    
    assert!(result.is_some());
    let separator: HashSet<String> = result.unwrap();
    
    // Should include intermediate nodes from both diamonds
    assert!(!separator.is_empty());
    // Should contain some combination of B1, C1, B2, C2
    let intermediate_nodes: HashSet<String> = vec![
        "B1".to_string(), "C1".to_string(), "B2".to_string(), "C2".to_string()
    ].into_iter().collect();
    
    assert_eq!(separator, intermediate_nodes);
}