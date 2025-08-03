use std::collections::HashSet;
use rust_core::RustPDAG;

#[test]
fn test_init_normal() {
    // Test initialization with mixed edges
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(Some(vec![("A".to_string(), "C".to_string()), ("D".to_string(), "C".to_string())]), None, true).unwrap();
    pdag.add_edges_from(Some(vec![("B".to_string(), "A".to_string()), ("B".to_string(), "D".to_string())]), None, false).unwrap();
    
    let expected_edges: HashSet<(String, String)> = vec![
        ("A".to_string(), "C".to_string()),
        ("D".to_string(), "C".to_string()),
        ("A".to_string(), "B".to_string()),
        ("B".to_string(), "A".to_string()),
        ("B".to_string(), "D".to_string()),
        ("D".to_string(), "B".to_string()),
    ].into_iter().collect();

    // Convert pdag.edges() to HashSet for order-insensitive comparison
    let actual_edges: HashSet<(String, String)> = pdag.edges().into_iter().collect();
    assert_eq!(actual_edges, expected_edges);
    
    let actual_nodes: HashSet<String> = pdag.nodes().into_iter().collect();
    let expected_nodes: HashSet<String> = vec!["A", "B", "C", "D"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    assert_eq!(actual_nodes, expected_nodes);
    
    assert_eq!(pdag.directed_edges, HashSet::from_iter(vec![("A".to_string(), "C".to_string()), ("D".to_string(), "C".to_string())]));
    assert_eq!(pdag.undirected_edges, HashSet::from_iter(vec![("B".to_string(), "A".to_string()), ("B".to_string(), "D".to_string())]));

    // Test with latents
    let mut pdag_latent = RustPDAG::new();
    pdag_latent.add_nodes_from(vec!["A".to_string(), "D".to_string()], Some(vec![true, true])).unwrap();
    pdag_latent.add_edges_from(Some(vec![("A".to_string(), "C".to_string()), ("D".to_string(), "C".to_string())]), None, false).unwrap();
    
    assert_eq!(pdag_latent.latents, HashSet::from_iter(vec!["A".to_string(), "D".to_string()]));
}

#[test]
fn test_all_neighbors() {
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(Some(vec![("A".to_string(), "C".to_string()), ("D".to_string(), "C".to_string())]), None, true).unwrap();
    pdag.add_edges_from(Some(vec![("B".to_string(), "A".to_string()), ("B".to_string(), "D".to_string())]), None, false).unwrap();
    
    assert_eq!(pdag.all_neighbors("A").unwrap(), HashSet::from_iter(vec!["B".to_string(), "C".to_string()]));
    assert_eq!(pdag.all_neighbors("B").unwrap(), HashSet::from_iter(vec!["A".to_string(), "D".to_string()]));
    assert_eq!(pdag.all_neighbors("C").unwrap(), HashSet::from_iter(vec!["A".to_string(), "D".to_string()]));
    assert_eq!(pdag.all_neighbors("D").unwrap(), HashSet::from_iter(vec!["B".to_string(), "C".to_string()]));
}


#[test]
fn test_directed_children() {
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(Some(vec![("A".to_string(), "C".to_string()), ("D".to_string(), "C".to_string())]), None, true).unwrap();
    pdag.add_edges_from(Some(vec![("B".to_string(), "A".to_string()), ("B".to_string(), "D".to_string())]), None, false).unwrap();
    
    assert_eq!(pdag.directed_children("A").unwrap(), HashSet::from_iter(vec!["C".to_string()]));
    assert_eq!(pdag.directed_children("B").unwrap(), HashSet::new());
    assert_eq!(pdag.directed_children("C").unwrap(), HashSet::new());
}

#[test]
fn test_directed_parents() {
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(Some(vec![("A".to_string(), "C".to_string()), ("D".to_string(), "C".to_string())]), None, true).unwrap();
    pdag.add_edges_from(Some(vec![("B".to_string(), "A".to_string()), ("B".to_string(), "D".to_string())]), None, false).unwrap();
    
    assert_eq!(pdag.directed_parents("A").unwrap(), HashSet::new());
    assert_eq!(pdag.directed_parents("B").unwrap(), HashSet::new());


    assert_eq!(pdag.directed_parents("C").unwrap(), HashSet::from_iter(vec!["A".to_string(), "D".to_string()]));
}

#[test]
fn test_has_directed_edge() {
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(Some(vec![("A".to_string(), "C".to_string()), ("D".to_string(), "C".to_string())]), None, true).unwrap();
    pdag.add_edges_from(Some(vec![("B".to_string(), "A".to_string()), ("B".to_string(), "D".to_string())]), None, false).unwrap();
    
    assert!(pdag.has_directed_edge("A", "C"));
    assert!(pdag.has_directed_edge("D", "C"));
    assert!(!pdag.has_directed_edge("A", "B"));
    assert!(!pdag.has_directed_edge("B", "A"));
}


#[test]
fn test_has_undirected_edge() {
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(Some(vec![("A".to_string(), "C".to_string()), ("D".to_string(), "C".to_string())]), None, true).unwrap();
    pdag.add_edges_from(Some(vec![("B".to_string(), "A".to_string()), ("B".to_string(), "D".to_string())]), None, false).unwrap();
    
    assert!(!pdag.has_undirected_edge("A", "C"));
    assert!(!pdag.has_undirected_edge("D", "C"));
    assert!(pdag.has_undirected_edge("A", "B"));
    assert!(pdag.has_undirected_edge("B", "A"));
}


#[test]
fn test_undirected_neighbors() {
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(Some(vec![("A".to_string(), "C".to_string()), ("D".to_string(), "C".to_string())]), None, true).unwrap();
    pdag.add_edges_from(Some(vec![("B".to_string(), "A".to_string()), ("B".to_string(), "D".to_string())]), None, false).unwrap();
    
    assert_eq!(pdag.undirected_neighbors("A").unwrap(), HashSet::from_iter(vec!["B".to_string()]));
    assert_eq!(pdag.undirected_neighbors("B").unwrap(), HashSet::from_iter(vec!["A".to_string(), "D".to_string()]));
    assert_eq!(pdag.undirected_neighbors("C").unwrap(), HashSet::new());
    assert_eq!(pdag.undirected_neighbors("D").unwrap(), HashSet::from_iter(vec!["B".to_string()]));
}