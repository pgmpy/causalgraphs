use std::collections::HashSet;

use rust_core::RustDAG;

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
    let result = dag.minimal_dseparator("A", "C", false).unwrap();
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
    let result = dag.minimal_dseparator("A", "D", false).unwrap();
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
    let result = dag.minimal_dseparator("A", "C", false).unwrap();
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

    let result = dag.minimal_dseparator("A", "C", false).unwrap();
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
    let result = dag.minimal_dseparator("A", "C", false).unwrap();
    assert_eq!(result, None, "Expected no d-separator when D is latent with multiple paths A→B→C and A→D→C (D is unobservable, because of its latent status)");
}

#[test]
fn test_minimal_dseparator_latent_case_5() {
    // dag_lat5 = DAG([("A", "B"), ("B", "C"), ("A", "D"), ("D", "E"), ("E", "C")], latents={"E"})
    // self.assertEqual(dag_lat5.minimal_dseparator(start="A", end="C"), {"B", "D"})
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
    let result = dag.minimal_dseparator("A", "C", false).unwrap();
    let expected: HashSet<String> = vec!["B".to_string(), "D".to_string()].into_iter().collect();
    assert_eq!(
        result,
        Some(expected),
        "Expected [B, D] to d-separate A and C when E is latent(Observe B & parent of C => D)"
    );
}

#[test]
fn test_minimal_dseparator_adjacent_error() {
    let mut dag = RustDAG::new();
    dag.add_edges_from(vec![("A".to_string(), "B".to_string())], None)
        .unwrap();

    let result = dag.minimal_dseparator("A", "B", false);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("adjacent"));
}
