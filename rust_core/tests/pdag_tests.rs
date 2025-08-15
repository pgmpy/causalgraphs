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

#[test]
fn test_is_adjacent() {
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(
        Some(vec![("A".to_string(), "C".to_string()), ("D".to_string(), "C".to_string())]), 
        None, 
        true
    ).unwrap();
    pdag.add_edges_from(
        Some(vec![("B".to_string(), "A".to_string()), ("B".to_string(), "D".to_string())]), 
        None, 
        false
    ).unwrap();
    
    assert!(pdag.is_adjacent("A", "B"));
    assert!(pdag.is_adjacent("B", "A"));
    assert!(pdag.is_adjacent("A", "C"));
    assert!(pdag.is_adjacent("C", "A"));
    assert!(pdag.is_adjacent("D", "C"));
    assert!(pdag.is_adjacent("C", "D"));
    assert!(!pdag.is_adjacent("A", "D"));
    assert!(!pdag.is_adjacent("B", "C"));
}


#[test]
fn test_orient_undirected_edge() {
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(
        Some(vec![("A".to_string(), "C".to_string()), ("D".to_string(), "C".to_string())]), 
        None, 
        true
    ).unwrap();
    pdag.add_edges_from(
        Some(vec![("B".to_string(), "A".to_string()), ("B".to_string(), "D".to_string())]), 
        None, 
        false
    ).unwrap();

    let mod_pdag = pdag.orient_undirected_edge("B", "A", false).unwrap().unwrap();
    let expected_edges: HashSet<(String, String)> = vec![
        ("A".to_string(), "C".to_string()),
        ("D".to_string(), "C".to_string()),
        ("B".to_string(), "A".to_string()),
        ("B".to_string(), "D".to_string()),
        ("D".to_string(), "B".to_string()),
    ].into_iter().collect();
    
    let actual_edges: HashSet<(String, String)> = mod_pdag.edges().into_iter().collect();
    assert_eq!(actual_edges, expected_edges);
    assert_eq!(mod_pdag.undirected_edges, HashSet::from_iter(vec![("B".to_string(), "D".to_string())]));
    assert_eq!(mod_pdag.directed_edges, HashSet::from_iter(vec![
        ("A".to_string(), "C".to_string()),
        ("D".to_string(), "C".to_string()),
        ("B".to_string(), "A".to_string())
    ]));
    // Test inplace modification
    pdag.orient_undirected_edge("B", "A", true).unwrap();
    let expected_edges_inplace: HashSet<(String, String)> = vec![
        ("A".to_string(), "C".to_string()),
        ("D".to_string(), "C".to_string()),
        ("B".to_string(), "A".to_string()),
        ("B".to_string(), "D".to_string()),
        ("D".to_string(), "B".to_string()),
    ].into_iter().collect();
    
    let actual_edges_inplace: HashSet<(String, String)> = pdag.edges().into_iter().collect();
    assert_eq!(actual_edges_inplace, expected_edges_inplace);
    assert_eq!(pdag.undirected_edges, HashSet::from_iter(vec![("B".to_string(), "D".to_string())]));
    assert_eq!(pdag.directed_edges, HashSet::from_iter(vec![
        ("A".to_string(), "C".to_string()),
        ("D".to_string(), "C".to_string()),
        ("B".to_string(), "A".to_string())
    ]));
    // Test error case - edge doesn't exist
    assert!(pdag.orient_undirected_edge("A", "C", true).is_err());
}


#[test]
fn test_copy() {
    // Test copy with mixed edges
    let mut pdag_mix = RustPDAG::new();
    pdag_mix.add_edges_from(
        Some(vec![("A".to_string(), "C".to_string()), ("D".to_string(), "C".to_string())]), 
        None, 
        true
    ).unwrap();
    pdag_mix.add_edges_from(
        Some(vec![("B".to_string(), "A".to_string()), ("B".to_string(), "D".to_string())]), 
        None, 
        false
    ).unwrap();
    let pdag_copy = pdag_mix.copy();
    let expected_edges: HashSet<(String, String)> = vec![
        ("A".to_string(), "C".to_string()),
        ("D".to_string(), "C".to_string()),
        ("A".to_string(), "B".to_string()),
        ("B".to_string(), "A".to_string()),
        ("B".to_string(), "D".to_string()),
        ("D".to_string(), "B".to_string()),
    ].into_iter().collect();
    
    let actual_edges: HashSet<(String, String)> = pdag_copy.edges().into_iter().collect();
    assert_eq!(actual_edges, expected_edges);
    assert_eq!(pdag_copy.nodes().into_iter().collect::<HashSet<_>>(), 
               HashSet::from_iter(vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()]));
    assert_eq!(pdag_copy.directed_edges, HashSet::from_iter(vec![("A".to_string(), "C".to_string()), ("D".to_string(), "C".to_string())]));
    assert_eq!(pdag_copy.undirected_edges, HashSet::from_iter(vec![("B".to_string(), "A".to_string()), ("B".to_string(), "D".to_string())]));
    assert_eq!(pdag_copy.latents, HashSet::new());
    // Test copy with latents
    let mut pdag_latent = RustPDAG::new();
    pdag_latent.add_edges_from(
        Some(vec![("A".to_string(), "C".to_string()), ("D".to_string(), "C".to_string())]), 
        None, 
        true
    ).unwrap();
    pdag_latent.add_edges_from(
        Some(vec![("B".to_string(), "A".to_string()), ("B".to_string(), "D".to_string())]), 
        None, 
        false
    ).unwrap();
    pdag_latent.latents.insert("A".to_string());
    pdag_latent.latents.insert("D".to_string());
    let pdag_copy_latent = pdag_latent.copy();
    assert_eq!(pdag_copy_latent.latents, HashSet::from_iter(vec!["A".to_string(), "D".to_string()]));
}

#[test]
fn test_apply_meeks_rules_basic() {
    // Test Rule 1: A -> B - C and A not adjacent to C => B -> C
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(Some(vec![("A".to_string(), "B".to_string())]), None, true).unwrap();
    pdag.add_edges_from(Some(vec![("B".to_string(), "C".to_string())]), None, false).unwrap();
    
    let cpdag = pdag.apply_meeks_rules(true, false).unwrap().unwrap();
    assert!(cpdag.has_directed_edge("A", "B"));
    assert!(cpdag.has_directed_edge("B", "C"));
    assert_eq!(cpdag.edges().into_iter().collect::<HashSet<_>>(), 
               HashSet::from_iter(vec![("A".to_string(), "B".to_string()), ("B".to_string(), "C".to_string())]));
}

#[test]
fn test_apply_meeks_rules_rule2() {
    // Test Rule 2: A -> B -> C and A - C => A -> C
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(Some(vec![("A".to_string(), "B".to_string())]), None, true).unwrap();
    pdag.add_edges_from(Some(vec![("B".to_string(), "C".to_string()), ("C".to_string(), "D".to_string())]), None, false).unwrap();
    
    let cpdag = pdag.apply_meeks_rules(true, false).unwrap().unwrap();

    let expected_edges: HashSet<(String, String)> = vec![
        ("A".to_string(), "B".to_string()),
        ("B".to_string(), "C".to_string()),
        ("C".to_string(), "D".to_string()),
    ].into_iter().collect();
    
    assert_eq!(cpdag.edges().into_iter().collect::<HashSet<_>>(), expected_edges);
}

#[test]
fn test_apply_meeks_rules_no_change() {
    // Test case where no rules apply
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(Some(vec![("A".to_string(), "B".to_string()), ("D".to_string(), "C".to_string())]), None, true).unwrap();
    pdag.add_edges_from(Some(vec![("B".to_string(), "C".to_string())]), None, false).unwrap();
    
    let cpdag = pdag.apply_meeks_rules(true, false).unwrap().unwrap();
    let expected_edges: HashSet<(String, String)> = vec![
        ("A".to_string(), "B".to_string()),
        ("D".to_string(), "C".to_string()),
        ("B".to_string(), "C".to_string()),
        ("C".to_string(), "B".to_string()),
    ].into_iter().collect();
    
    assert_eq!(cpdag.edges().into_iter().collect::<HashSet<_>>(), expected_edges);
}


#[test]
fn test_apply_meeks_rules_no_change_2() {
    // Test case where no rules apply
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(Some(vec![("A".to_string(), "B".to_string()), ("D".to_string(), "C".to_string()), ("D".to_string(), "B".to_string())]), None, true).unwrap();
    pdag.add_edges_from(Some(vec![("B".to_string(), "C".to_string())]), None, false).unwrap();
    
    let cpdag = pdag.apply_meeks_rules(true, false).unwrap().unwrap();
    let expected_edges: HashSet<(String, String)> = vec![
        ("A".to_string(), "B".to_string()),
        ("D".to_string(), "C".to_string()),
        ("D".to_string(), "B".to_string()),
        ("B".to_string(), "C".to_string()),
    ].into_iter().collect();
    
    assert_eq!(cpdag.edges().into_iter().collect::<HashSet<_>>(), expected_edges);
}




#[test]
fn test_apply_meeks_rules_inplace() {
    // Test inplace modification
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(Some(vec![("A".to_string(), "B".to_string())]), None, true).unwrap();
    pdag.add_edges_from(Some(vec![("B".to_string(), "C".to_string())]), None, false).unwrap();
    
    pdag.apply_meeks_rules(true, true).unwrap();
    assert!(pdag.has_directed_edge("A", "B"));
    assert!(pdag.has_directed_edge("B", "C"));
    assert_eq!(pdag.edges().into_iter().collect::<HashSet<_>>(), 
               HashSet::from_iter(vec![("A".to_string(), "B".to_string()), ("B".to_string(), "C".to_string())]));
}


#[test]
fn test_meeks_rules_perkovic_2017() {
    // Test case from Perkoviċ et al., 2017
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(Some(vec![("V1".to_string(), "X".to_string())]), None, true).unwrap();
    pdag.add_edges_from(Some(vec![("X".to_string(), "V2".to_string()), ("V2".to_string(), "Y".to_string()), ("X".to_string(), "Y".to_string())]), None, false).unwrap();
    
    let cpdag = pdag.apply_meeks_rules(true, false).unwrap().unwrap();
    let expected_edges: HashSet<(String, String)> = vec![
        ("V1".to_string(), "X".to_string()), 
        ("X".to_string(), "V2".to_string()), 
        ("X".to_string(), "Y".to_string()), 
        ("V2".to_string(), "Y".to_string()), 
        ("Y".to_string(), "V2".to_string())
    ].into_iter().collect();
    assert_eq!(cpdag.edges().into_iter().collect::<HashSet<_>>(), expected_edges);

    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(Some(vec![("Y".to_string(), "X".to_string())]), None, true).unwrap();
    pdag.add_edges_from(Some(vec![("V1".to_string(), "X".to_string()), ("X".to_string(), "V2".to_string()), ("V2".to_string(), "Y".to_string())]), None, false).unwrap();

    let cpdag = pdag.apply_meeks_rules(true, false).unwrap().unwrap();
    let expected_edges: HashSet<(String, String)> = vec![
        ("X".to_string(), "V1".to_string()),
        ("Y".to_string(), "X".to_string()),
        ("X".to_string(), "V2".to_string()),
        ("V2".to_string(), "X".to_string()),
        ("V2".to_string(), "Y".to_string()),
        ("Y".to_string(), "V2".to_string()),
    ].into_iter().collect();
    assert_eq!(cpdag.edges().into_iter().collect::<HashSet<_>>(), expected_edges);
}


#[test]
fn test_meeks_rules_bang_2024() {
    // Test case from Bang et al., 2024
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(Some(vec![("B".to_string(), "D".to_string()), ("C".to_string(), "D".to_string())]), None, true).unwrap();
    pdag.add_edges_from(Some(vec![("A".to_string(), "D".to_string()), ("A".to_string(), "C".to_string())]), None, false).unwrap();

    let cpdag = pdag.apply_meeks_rules(true, false).unwrap().unwrap();
    let expected_edges: HashSet<(String, String)> = vec![
        ("B".to_string(), "D".to_string()), 
        ("D".to_string(), "A".to_string()), 
        ("C".to_string(), "A".to_string()), 
        ("C".to_string(), "D".to_string())
    ].into_iter().collect();
    assert_eq!(cpdag.edges().into_iter().collect::<HashSet<_>>(), expected_edges);

    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(Some(vec![("A".to_string(), "B".to_string()), ("C".to_string(), "B".to_string())]), None, true).unwrap();
    pdag.add_edges_from(Some(vec![("D".to_string(), "B".to_string()), ("D".to_string(), "A".to_string()), ("D".to_string(), "C".to_string())]), None, false).unwrap();

    let cpdag = pdag.apply_meeks_rules(true, false).unwrap().unwrap();
    let expected_edges: HashSet<(String, String)> = vec![
        ("A".to_string(), "B".to_string()),
        ("C".to_string(), "B".to_string()),
        ("D".to_string(), "B".to_string()),
        ("D".to_string(), "A".to_string()),
        ("A".to_string(), "D".to_string()),
        ("D".to_string(), "C".to_string()),
        ("C".to_string(), "D".to_string()),
    ].into_iter().collect();
    assert_eq!(cpdag.edges().into_iter().collect::<HashSet<_>>(), expected_edges);
}

#[test]
fn test_meeks_rules_complex_cases() {
    let undirected_edges = vec![("A".to_string(), "C".to_string()), ("B".to_string(), "C".to_string()), ("D".to_string(), "C".to_string())];
    let directed_edges = vec![("B".to_string(), "D".to_string()), ("D".to_string(), "A".to_string())];

    // With apply_r4 = true
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(Some(directed_edges.clone()), None, true).unwrap();
    pdag.add_edges_from(Some(undirected_edges.clone()), None, false).unwrap();
    let cpdag = pdag.apply_meeks_rules(true, false).unwrap().unwrap();
    let expected_edges_r4: HashSet<(String, String)> = vec![
        ("C".to_string(), "A".to_string()),
        ("C".to_string(), "B".to_string()),
        ("B".to_string(), "C".to_string()),
        ("B".to_string(), "D".to_string()),
        ("D".to_string(), "A".to_string()),
        ("D".to_string(), "C".to_string()),
        ("C".to_string(), "D".to_string()),
    ].into_iter().collect();
    assert_eq!(cpdag.edges().into_iter().collect::<HashSet<_>>(), expected_edges_r4);

    // With apply_r4 = false
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(Some(directed_edges.clone()), None, true).unwrap();
    pdag.add_edges_from(Some(undirected_edges.clone()), None, false).unwrap();
    let cpdag = pdag.apply_meeks_rules(false, false).unwrap().unwrap();
    let expected_edges_no_r4: HashSet<(String, String)> = vec![
        ("A".to_string(), "C".to_string()),
        ("C".to_string(), "A".to_string()),
        ("C".to_string(), "B".to_string()),
        ("B".to_string(), "C".to_string()),
        ("B".to_string(), "D".to_string()),
        ("D".to_string(), "A".to_string()),
        ("D".to_string(), "C".to_string()),
        ("C".to_string(), "D".to_string()),
    ].into_iter().collect();
    assert_eq!(cpdag.edges().into_iter().collect::<HashSet<_>>(), expected_edges_no_r4);

    // With apply_r4 = false and inplace = true
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(Some(directed_edges.clone()), None, true).unwrap();
    pdag.add_edges_from(Some(undirected_edges.clone()), None, false).unwrap();
    pdag.apply_meeks_rules(false, true).unwrap();
    assert_eq!(pdag.edges().into_iter().collect::<HashSet<_>>(), expected_edges_no_r4);
}


#[test]
fn test_to_dag_basic() {
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(
        Some(vec![("A".to_string(), "C".to_string()), ("C".to_string(), "B".to_string())]), 
        None, 
        true
    ).unwrap();
    pdag.add_edges_from(
        Some(vec![("C".to_string(), "D".to_string()), ("D".to_string(), "A".to_string())]), 
        None, 
        false
    ).unwrap();
    
    let dag = pdag.to_dag().unwrap();
    let dag_edges: HashSet<(String, String)> = dag.edges().into_iter().collect();
    
    // Expected edges: A -> C, C -> B, and either C -> D, A -> D or D -> C, D -> A
    assert_eq!(dag_edges.len(), 4);
    assert!(dag.has_edge("A", "C"));
    assert!(dag.has_edge("C", "B"));
    assert!(!(dag.has_edge("A", "D") && dag.has_edge("C", "D"))); // No V-structure
    assert!(dag_edges.contains(&("C".to_string(), "D".to_string())) || dag_edges.contains(&("D".to_string(), "C".to_string())));
    assert!(dag_edges.contains(&("D".to_string(), "A".to_string())) || dag_edges.contains(&("A".to_string(), "D".to_string())));
}

#[test]
fn test_pdag_to_dag() {
    // PDAG no: 1 - Possibility of creating a v-structure
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(
        Some(vec![
            ("A".to_string(), "B".to_string()),
            ("C".to_string(), "B".to_string()),
        ]),
        None,
        true,
    )
    .unwrap();
    pdag.add_edges_from(
        Some(vec![
            ("C".to_string(), "D".to_string()),
            ("D".to_string(), "A".to_string()),
        ]),
        None,
        false,
    )
    .unwrap();

    let dag = pdag.to_dag().unwrap();
    let dag_edges: HashSet<(String, String)> = dag.edges().into_iter().collect();

    assert_eq!(dag_edges.len(), 4, "Expected 4 edges in DAG");
    assert!(dag.has_edge("A", "B"), "Expected edge A -> B");
    assert!(dag.has_edge("C", "B"), "Expected edge C -> B");
    assert!(
        !(dag.has_edge("A", "D") && dag.has_edge("C", "D")),
        "Should not have both A -> D and C -> D (v-structure)"
    );
    assert!(
        dag_edges.contains(&("C".to_string(), "D".to_string()))
            || dag_edges.contains(&("D".to_string(), "C".to_string())),
        "Expected either C -> D or D -> C"
    );
    assert!(
        dag_edges.contains(&("D".to_string(), "A".to_string()))
            || dag_edges.contains(&("A".to_string(), "D".to_string())),
        "Expected either D -> A or A -> D"
    );

    // With latents
    let mut pdag = RustPDAG::new();
    pdag.add_nodes_from(
        vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()],
        Some(vec![true, false, false, false]),
    )
    .unwrap();
    pdag.add_edges_from(
        Some(vec![
            ("A".to_string(), "B".to_string()),
            ("C".to_string(), "B".to_string()),
        ]),
        None,
        true,
    )
    .unwrap();
    pdag.add_edges_from(
        Some(vec![
            ("C".to_string(), "D".to_string()),
            ("D".to_string(), "A".to_string()),
        ]),
        None,
        false,
    )
    .unwrap();

    let dag = pdag.to_dag().unwrap();
    let dag_edges: HashSet<(String, String)> = dag.edges().into_iter().collect();
    let dag_latents: HashSet<String> = dag.latents.clone().into_iter().collect();

    assert_eq!(dag_edges.len(), 4, "Expected 4 edges in DAG with latents");
    assert!(dag.has_edge("A", "B"), "Expected edge A -> B with latents");
    assert!(dag.has_edge("C", "B"), "Expected edge C -> B with latents");
    assert!(
        !(dag.has_edge("A", "D") && dag.has_edge("C", "D")),
        "Should not have both A -> D and C -> D with latents (v-structure)"
    );
    assert!(
        dag_edges.contains(&("C".to_string(), "D".to_string()))
            || dag_edges.contains(&("D".to_string(), "C".to_string())),
        "Expected either C -> D or D -> C with latents"
    );
    assert!(
        dag_edges.contains(&("D".to_string(), "A".to_string()))
            || dag_edges.contains(&("A".to_string(), "D".to_string())),
        "Expected either D -> A or A -> D with latents"
    );
    assert_eq!(
        dag_latents,
        HashSet::from_iter(vec!["A".to_string()]),
        "Expected latent node A"
    );

    // PDAG no: 2 - No possibility of creating a v-structure
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(
        Some(vec![
            ("B".to_string(), "C".to_string()),
            ("A".to_string(), "C".to_string()),
        ]),
        None,
        true,
    )
    .unwrap();
    pdag.add_edges_from(Some(vec![("A".to_string(), "D".to_string())]), None, false)
        .unwrap();

    let dag = pdag.to_dag().unwrap();
    let dag_edges: HashSet<(String, String)> = dag.edges().into_iter().collect();

    assert!(dag.has_edge("B", "C"), "Expected edge B -> C");
    assert!(dag.has_edge("A", "C"), "Expected edge A -> C");
    assert!(
        dag_edges.contains(&("A".to_string(), "D".to_string()))
            || dag_edges.contains(&("D".to_string(), "A".to_string())),
        "Expected either A -> D or D -> A"
    );

    // With latents
    let mut pdag = RustPDAG::new();
    pdag.add_nodes_from(
        vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()],
        Some(vec![true, false, false, false]),
    )
    .unwrap();
    pdag.add_edges_from(
        Some(vec![
            ("B".to_string(), "C".to_string()),
            ("A".to_string(), "C".to_string()),
        ]),
        None,
        true,
    )
    .unwrap();
    pdag.add_edges_from(Some(vec![("A".to_string(), "D".to_string())]), None, false)
        .unwrap();

    let dag = pdag.to_dag().unwrap();
    let dag_edges: HashSet<(String, String)> = dag.edges().into_iter().collect();
    let dag_latents: HashSet<String> = dag.latents.clone().into_iter().collect();

    assert!(dag.has_edge("B", "C"), "Expected edge B -> C with latents");
    assert!(dag.has_edge("A", "C"), "Expected edge A -> C with latents");
    assert!(
        dag_edges.contains(&("A".to_string(), "D".to_string()))
            || dag_edges.contains(&("D".to_string(), "A".to_string())),
        "Expected either A -> D or D -> A with latents"
    );
    assert_eq!(
        dag_latents,
        HashSet::from_iter(vec!["A".to_string()]),
        "Expected latent node A"
    );

    // PDAG no: 3 - Already existing v-structure, possibility to add another
    let mut pdag = RustPDAG::new();
    pdag.add_edges_from(
        Some(vec![
            ("B".to_string(), "C".to_string()),
            ("A".to_string(), "C".to_string()),
        ]),
        None,
        true,
    )
    .unwrap();
    pdag.add_edges_from(Some(vec![("C".to_string(), "D".to_string())]), None, false)
        .unwrap();

    let dag = pdag.to_dag().unwrap();
    let expected_edges: HashSet<(String, String)> = vec![
        ("B".to_string(), "C".to_string()),
        ("C".to_string(), "D".to_string()),
        ("A".to_string(), "C".to_string()),
    ]
    .into_iter()
    .collect();
    let dag_edges: HashSet<(String, String)> = dag.edges().into_iter().collect();

    assert_eq!(dag_edges, expected_edges, "Expected edges for PDAG no: 3");

    // With latents
    let mut pdag: RustPDAG = RustPDAG::new();
    pdag.add_nodes_from(
        vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()],
        Some(vec![true, false, false, false]),
    )
    .unwrap();
    pdag.add_edges_from(
        Some(vec![
            ("B".to_string(), "C".to_string()),
            ("A".to_string(), "C".to_string()),
        ]),
        None,
        true,
    )
    .unwrap();
    pdag.add_edges_from(Some(vec![("C".to_string(), "D".to_string())]), None, false)
        .unwrap();

    let dag = pdag.to_dag().unwrap();
    let dag_edges: HashSet<(String, String)> = dag.edges().into_iter().collect();
    let dag_latents: HashSet<String> = dag.latents.into_iter().collect();

    assert_eq!(
        dag_edges, expected_edges,
        "Expected edges for PDAG no: 3 with latents"
    );
    assert_eq!(
        dag_latents,
        HashSet::from_iter(vec!["A".to_string()]),
        "Expected latent node A"
    );
}