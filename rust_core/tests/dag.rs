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