#[cfg(test)]
mod independence_tests {
    use rust_core::IndependenceAssertion;

    use super::*;
    use std::collections::HashSet;

    // Helper function to create HashSet from vector
    fn set_from_vec(vec: Vec<&str>) -> HashSet<String> {
        vec.into_iter().map(|s| s.to_string()).collect()
    }

    // Helper function to create IndependenceAssertion from strings
    fn create_assertion(e1: Vec<&str>, e2: Vec<&str>, e3: Option<Vec<&str>>) -> IndependenceAssertion {
        IndependenceAssertion::new(
            set_from_vec(e1),
            set_from_vec(e2),
            e3.map(set_from_vec),
        ).unwrap()
    }

    #[cfg(test)]
    mod test_independence_assertion {
        use rust_core::IndependenceAssertion;

        use super::*;

        #[test]
        fn test_new_assertion() {
            let assertion = IndependenceAssertion::new(
                set_from_vec(vec!["U"]),
                set_from_vec(vec!["V"]),
                Some(set_from_vec(vec!["Z"])),
            ).unwrap();
            
            assert_eq!(assertion.event1, set_from_vec(vec!["U"]));
            assert_eq!(assertion.event2, set_from_vec(vec!["V"]));
            assert_eq!(assertion.event3, set_from_vec(vec!["Z"]));
        }

        #[test]
        fn test_assertion_with_multiple_variables() {
            let assertion = IndependenceAssertion::new(
                set_from_vec(vec!["U", "V"]),
                set_from_vec(vec!["Y", "Z"]),
                Some(set_from_vec(vec!["A", "B"])),
            ).unwrap();
            
            assert_eq!(assertion.event1, set_from_vec(vec!["U", "V"]));
            assert_eq!(assertion.event2, set_from_vec(vec!["Y", "Z"]));
            assert_eq!(assertion.event3, set_from_vec(vec!["A", "B"]));
        }

        #[test]
        fn test_assertion_without_conditioning() {
            let assertion = IndependenceAssertion::new(
                set_from_vec(vec!["U"]),
                set_from_vec(vec!["V"]),
                None
            ).unwrap();
            
            assert_eq!(assertion.event1, set_from_vec(vec!["U"]));
            assert_eq!(assertion.event2, set_from_vec(vec!["V"]));
            assert!(assertion.event3.is_empty());
            assert!(assertion.is_unconditional());
        }

        #[test]
        fn test_assertion_validation_errors() {
            // event1 empty should fail
            let result = IndependenceAssertion::new(
                HashSet::new(),
                set_from_vec(vec!["V"]),
                None,
            );
            assert!(result.is_err());

            // event2 empty should fail
            let result = IndependenceAssertion::new(
                set_from_vec(vec!["U"]),
                HashSet::new(),
                None,
            );
            assert!(result.is_err());
        }

        #[test]
        fn test_all_variables() {
            let assertion = create_assertion(
                vec!["A", "B"],
                vec!["C"],
                Some(vec!["D", "E"]),
            );
            
            let expected_vars = set_from_vec(vec!["A", "B", "C", "D", "E"]);
            assert_eq!(assertion.all_vars, expected_vars);
        }

        #[test]
        fn test_display_formatting() {
            let assertion1 = create_assertion(vec!["X"], vec!["Y"], None);
            assert_eq!(format!("{}", assertion1), "(X ⊥ Y)");

            let assertion2 = create_assertion(vec!["X"], vec!["Y"], Some(vec!["Z"]));
            assert!(format!("{}", assertion2).contains("⊥") && format!("{}", assertion2).contains("|"));
        }

        #[test]
        fn test_latex_formatting() {
            let assertion1 = create_assertion(vec!["X"], vec!["Y"], None);
            assert_eq!(assertion1.to_latex(), "X \\perp Y");

            let assertion2 = create_assertion(vec!["X"], vec!["Y"], Some(vec!["Z"]));
            assert_eq!(assertion2.to_latex(), "X \\perp Y \\mid Z");
        }
    }

    #[cfg(test)]
    mod test_independence_assertion_equality {
        use super::*;

        #[test]
        fn test_equality_basic() {
            let i1 = create_assertion(vec!["a"], vec!["b"], Some(vec!["c"]));
            let i2 = create_assertion(vec!["a"], vec!["b"], None);
            let i3 = create_assertion(vec!["a"], vec!["b", "c", "d"], None);
            
            assert_ne!(i1, i2);
            assert_ne!(i1, i3);
            assert_ne!(i2, i3);
        }

        #[test]
        fn test_equality_symmetry() {
            let i4 = create_assertion(vec!["a"], vec!["b", "c", "d"], Some(vec!["e"]));
            let i5 = create_assertion(vec!["a"], vec!["d", "c", "b"], Some(vec!["e"]));
            
            // Order shouldn't matter for sets
            assert_eq!(i4, i5);
        }

        #[test]
        fn test_equality_with_swapped_events() {
            // Test symmetry: X ⊥ Y | Z should equal Y ⊥ X | Z
            let i9 = create_assertion(vec!["a"], vec!["d", "k", "b"], Some(vec!["e"]));
            let i10 = create_assertion(vec!["k", "b", "d"], vec!["a"], Some(vec!["e"]));
            
            assert_eq!(i9, i10); // Should be equal due to symmetry
        }

        #[test]
        fn test_inequality_different_conditioning() {
            let i6 = create_assertion(vec!["a"], vec!["d", "c"], Some(vec!["e", "b"]));
            let i7 = create_assertion(vec!["a"], vec!["c", "d"], Some(vec!["b", "e"]));
            let i8 = create_assertion(vec!["a"], vec!["f", "d"], Some(vec!["b", "e"]));
            
            assert_eq!(i6, i7); // Same conditioning set, different order
            assert_ne!(i7, i8); // Different variables
        }
    }

    #[cfg(test)]
    mod test_independencies_collection {
        use rust_core::Independencies;

        use super::*;

        #[test]
        fn test_empty_independencies() {
            let independencies = Independencies::new();
            assert_eq!(independencies.get_assertions().len(), 0);
            assert_eq!(independencies.get_all_variables().len(), 0);
        }

        #[test]
        fn test_add_assertion() {
            let mut independencies = Independencies::new();
            let assertion = create_assertion(vec!["X"], vec!["Y"], Some(vec!["Z"]));
            
            independencies.add_assertion(assertion.clone());
            assert_eq!(independencies.get_assertions().len(), 1);
            assert!(independencies.contains(&assertion));
        }

        #[test]
        fn test_get_all_variables() {
            let mut independencies = Independencies::new();
            independencies.add_assertion(create_assertion(
                vec!["a"], vec!["b", "c", "d"], Some(vec!["e", "f", "g"])
            ));
            independencies.add_assertion(create_assertion(
                vec!["c"], vec!["d", "e", "f"], Some(vec!["g", "h"])
            ));

            let expected_vars = set_from_vec(vec!["a", "b", "c", "d", "e", "f", "g", "h"]);
            assert_eq!(independencies.get_all_variables(), expected_vars);
        }

        #[test]
        fn test_independencies_equality() {
            let mut ind1 = Independencies::new();
            ind1.add_assertion(create_assertion(
                vec!["a"], vec!["b", "c", "d"], Some(vec!["e", "f", "g"])
            ));

            let mut ind2 = Independencies::new();
            ind2.add_assertion(create_assertion(
                vec!["a"], vec!["b", "c", "d"], Some(vec!["e", "f", "g"])
            ));

            assert_eq!(ind1, ind2);
        }
    }

    #[cfg(test)]
    mod test_closure_and_entailment {
        use rust_core::Independencies;

        use super::*;

        #[test]
        fn test_simple_closure() {
            let mut ind = Independencies::new();
            ind.add_assertion(create_assertion(
                vec!["A"], vec!["B", "C"], Some(vec!["D"])
            ));

            let closure = ind.closure();
            let closure_assertions = closure.get_assertions();
            
            // Should contain original assertion
            assert!(closure_assertions.len() >= 1);
            
            // Should contain decompositions: A ⊥ B | D and A ⊥ C | D
            let decomp1 = create_assertion(vec!["A"], vec!["B"], Some(vec!["D"]));
            let decomp2 = create_assertion(vec!["A"], vec!["C"], Some(vec!["D"]));
            
            assert!(closure.contains(&decomp1) || 
                   closure_assertions.iter().any(|a| a.event1 == decomp1.event1 && 
                                                   a.event2 == decomp1.event2 && 
                                                   a.event3 == decomp1.event3));
        }

        #[test]
        fn test_complex_closure() {
            let mut ind = Independencies::new();
            ind.add_assertion(create_assertion(
                vec!["A"], vec!["B", "C", "D"], Some(vec!["E"])
            ));

            let closure = ind.closure();
            let closure_assertions= closure.get_assertions();

            assert!(closure_assertions.len() == 19);
        }

                

        #[test]
        fn test_entailment() {
            let mut ind1 = Independencies::new();
            ind1.add_assertion(create_assertion(
                vec!["W"], vec!["X", "Y", "Z"], None
            ));

            let mut ind2 = Independencies::new();
            ind2.add_assertion(create_assertion(vec!["W"], vec!["X"], None));

            // W ⊥ X,Y,Z should entail W ⊥ X
            assert!(ind1.entails(&ind2));
            assert!(!ind2.entails(&ind1));
        }
    }

    #[cfg(test)]
    mod test_reduce_method {
        use rust_core::Independencies;

        use super::*;

        #[test]
        fn test_reduce_duplicates() {
            let mut ind = Independencies::new();
            let assertion = create_assertion(vec!["X"], vec!["Y"], Some(vec!["Z"]));
            
            // Add the same assertion twice
            ind.add_assertion(assertion.clone());
            ind.add_assertion(assertion.clone());
            
            let reduced = ind.reduce();
            assert_eq!(reduced.get_assertions().len(), 1);
        }

        #[test]
        fn test_reduce_entailment() {
            let mut ind = Independencies::new();
            
            // More general assertion
            ind.add_assertion(create_assertion(
                vec!["W"], vec!["X", "Y", "Z"], None
            ));
            // More specific assertion (should be removed)
            ind.add_assertion(create_assertion(vec!["W"], vec!["X"], None));

            let reduced = ind.reduce();
            assert_eq!(reduced.get_assertions().len(), 1);
            
            // Should keep the more general assertion
            let general = create_assertion(vec!["W"], vec!["X", "Y", "Z"], None);
            assert!(reduced.contains(&general));
        }

        #[test]
        fn test_reduce_independent_assertions() {
            let mut ind: Independencies = Independencies::new();
            ind.add_assertion(create_assertion(vec!["A"], vec!["B"], Some(vec!["C"])));
            ind.add_assertion(create_assertion(vec!["D"], vec!["E"], Some(vec!["F"])));

            let reduced: Independencies = ind.reduce();
            assert_eq!(reduced.get_assertions().len(), 2);
        }

        #[test]
        fn test_reduce_inplace() {
            let mut ind = Independencies::new();
            let assertion = create_assertion(vec!["X"], vec!["Y"], Some(vec!["Z"]));
            
            ind.add_assertion(assertion.clone());
            ind.add_assertion(assertion.clone());
            ind.add_assertion(create_assertion(vec!["A"], vec!["B"], Some(vec!["C"])));
            
            let original_len = ind.get_assertions().len();
            ind.reduce_inplace();
            
            assert_ne!(original_len, ind.get_assertions().len());
            assert_eq!(ind.get_assertions().len(), 2); // Should have 2 unique assertions
        }

        #[test]
        fn test_reduce_empty() {
            let ind = Independencies::new();
            let reduced = ind.reduce();
            assert_eq!(reduced.get_assertions().len(), 0);
        }

        #[test]
        fn test_reduce_complex_case() {
            let mut ind = Independencies::new();
            
            // General assertion that entails the specific ones
            ind.add_assertion(create_assertion(
                vec!["A"], vec!["B", "C"], Some(vec!["D"])
            ));
            // Specific assertions that should be removed
            ind.add_assertion(create_assertion(vec!["A"], vec!["B"], Some(vec!["D"])));
            ind.add_assertion(create_assertion(vec!["A"], vec!["C"], Some(vec!["D"])));
            // Independent assertion
            ind.add_assertion(create_assertion(vec!["E"], vec!["F"], Some(vec!["G"])));

            let reduced = ind.reduce();
            assert_eq!(reduced.get_assertions().len(), 2);
            
            let general = create_assertion(vec!["A"], vec!["B", "C"], Some(vec!["D"]));
            let independent = create_assertion(vec!["E"], vec!["F"], Some(vec!["G"]));
            
            assert!(reduced.contains(&general));
            assert!(reduced.contains(&independent));
        }
    }

    #[cfg(test)]
    mod test_complex_pgmpy_scenarios {
        use super::*;
        use rust_core::Independencies;

        // Helper to create complex Independencies matching PGMPY patterns
        fn create_independencies_3() -> Independencies {
            let mut ind = Independencies::new();
            ind.add_assertions_from_tuples(vec![
                (vec!["a".to_string()], vec!["b".to_string(), "c".to_string(), "d".to_string()], Some(vec!["e".to_string(), "f".to_string(), "g".to_string()])),
                (vec!["c".to_string()], vec!["d".to_string(), "e".to_string(), "f".to_string()], Some(vec!["g".to_string(), "h".to_string()]))
            ]).unwrap();
            ind
        }

        fn create_independencies_4() -> Independencies {
            let mut ind = Independencies::new();
            ind.add_assertions_from_tuples(vec![
                (vec!["f".to_string(), "d".to_string(), "e".to_string()], vec!["c".to_string()], Some(vec!["h".to_string(), "g".to_string()])),
                (vec!["b".to_string(), "c".to_string(), "d".to_string()], vec!["a".to_string()], Some(vec!["f".to_string(), "g".to_string(), "e".to_string()]))
            ]).unwrap();
            ind
        }

        fn create_independencies_5() -> Independencies {
            let mut ind = Independencies::new();
            ind.add_assertions_from_tuples(vec![
                (vec!["a".to_string()], vec!["b".to_string(), "c".to_string(), "d".to_string()], Some(vec!["e".to_string(), "f".to_string(), "g".to_string()])),
                (vec!["c".to_string()], vec!["d".to_string(), "e".to_string(), "f".to_string()], Some(vec!["g".to_string()]))
            ]).unwrap();
            ind
        }

        #[test]
        fn test_complex_multi_assertion_equality() {
            // This tests the complex scenario from PGMPY setUp method
            let ind3 = create_independencies_3();
            let ind4 = create_independencies_4();
            let ind5 = create_independencies_5();

            // These should be equal due to symmetric equivalence
            assert_eq!(ind3, ind4, "Independencies3 and Independencies4 should be equal");
            
            // These should not be equal
            assert_ne!(ind3, ind5, "Independencies3 and Independencies5 should not be equal");
            assert_ne!(ind4, ind5, "Independencies4 and Independencies5 should not be equal");
        }

        #[test]
        fn test_pgmpy_complex_equivalence_scenarios() {
            // Test case 1: ind1 vs ind2 (should NOT be equivalent)
            let mut ind1 = Independencies::new();
            ind1.add_assertions_from_tuples(vec![
                (vec!["X".to_string()], vec!["Y".to_string(), "W".to_string()], Some(vec!["Z".to_string()]))
            ]).unwrap();

            let mut ind2 = Independencies::new();
            ind2.add_assertions_from_tuples(vec![
                (vec!["X".to_string()], vec!["Y".to_string()], Some(vec!["Z".to_string()])),
                (vec!["X".to_string()], vec!["W".to_string()], Some(vec!["Z".to_string()]))
            ]).unwrap();

            // This should be FALSE - ind1 should NOT be equivalent to ind2
            assert!(!ind1.is_equivalent(&ind2), "ind1 should NOT be equivalent to ind2");

            // Test case 2: ind1 vs ind3 (should be equivalent)
            let mut ind3 = Independencies::new();
            ind3.add_assertions_from_tuples(vec![
                (vec!["X".to_string()], vec!["Y".to_string()], Some(vec!["Z".to_string()])),
                (vec!["X".to_string()], vec!["W".to_string()], Some(vec!["Z".to_string()])),
                (vec!["X".to_string()], vec!["Y".to_string()], Some(vec!["W".to_string(), "Z".to_string()]))
            ]).unwrap();

            // This should be TRUE - ind1 should be equivalent to ind3
            assert!(ind1.is_equivalent(&ind3), "ind1 should be equivalent to ind3");
        }

        #[test]
        fn test_comprehensive_equality_edge_cases() {
            let empty_ind = Independencies::new();
            
            let mut non_empty_ind = Independencies::new();
            non_empty_ind.add_assertions_from_tuples(vec![
                (vec!["A".to_string()], vec!["B".to_string()], Some(vec!["C".to_string()]))
            ]).unwrap();

            // Empty vs non-empty should be false
            assert_ne!(empty_ind, non_empty_ind, "Empty and non-empty independencies should not be equal");
            
            // Non-empty vs empty should be false  
            assert_ne!(non_empty_ind, empty_ind, "Non-empty and empty independencies should not be equal");
            
            // Empty vs empty should be true
            let another_empty = Independencies::new();
            assert_eq!(empty_ind, another_empty, "Two empty independencies should be equal");
            
            // Test inequality operator consistency
            assert!(empty_ind != non_empty_ind, "Inequality operator should work");
            assert!(!(empty_ind != another_empty), "Double negative inequality should work");
        }

        #[test] 
        fn test_complex_symmetric_equivalence() {
            // Create complex assertions that test symmetry at multiple levels
            let mut ind_a = Independencies::new();
            ind_a.add_assertions_from_tuples(vec![
                (vec!["X".to_string(), "Y".to_string()], vec!["A".to_string(), "B".to_string()], Some(vec!["Z".to_string()])),
                (vec!["P".to_string()], vec!["Q".to_string(), "R".to_string(), "S".to_string()], Some(vec!["T".to_string(), "U".to_string()]))
            ]).unwrap();

            let mut ind_b = Independencies::new();
            ind_b.add_assertions_from_tuples(vec![
                (vec!["A".to_string(), "B".to_string()], vec!["X".to_string(), "Y".to_string()], Some(vec!["Z".to_string()])),
                (vec!["P".to_string()], vec!["S".to_string(), "Q".to_string(), "R".to_string()], Some(vec!["U".to_string(), "T".to_string()]))
            ]).unwrap();

            // These should be equal due to symmetric equivalence and set ordering
            assert_eq!(ind_a, ind_b, "Symmetric complex independencies should be equal");
        }

        #[test]
        fn test_pgmpy_setup_variable_extraction() {
            // Test the get_all_variables method with complex scenarios
            let ind3 = create_independencies_3();
            let ind4 = create_independencies_4(); 
            let ind5 = create_independencies_5();

            let vars3 = ind3.get_all_variables();
            let vars4 = ind4.get_all_variables();
            let vars5 = ind5.get_all_variables();

            // All should contain the same variables
            let expected_vars: HashSet<String> = set_from_vec(vec!["a", "b", "c", "d", "e", "f", "g", "h"]);
            assert_eq!(vars3, expected_vars, "Independencies3 should have all expected variables");
            assert_eq!(vars4, expected_vars, "Independencies4 should have all expected variables");

            let expected_vars5: HashSet<String> = set_from_vec(vec!["a", "b", "c", "d", "e", "f", "g"]);
            assert_eq!(vars5, expected_vars5, "Independencies5 should have subset of variables");
        }

        #[test]
        fn test_bidirectional_equivalence_complex() {
            // Test that equivalence is truly bidirectional in complex scenarios
            let mut ind_x = Independencies::new();
            ind_x.add_assertions_from_tuples(vec![
                (vec!["A".to_string()], vec!["B".to_string(), "C".to_string()], Some(vec!["D".to_string()])),
                (vec!["E".to_string()], vec!["F".to_string()], Some(vec!["G".to_string(), "H".to_string()]))
            ]).unwrap();

            let mut ind_y = Independencies::new();
            ind_y.add_assertions_from_tuples(vec![
                (vec!["A".to_string()], vec!["B".to_string()], Some(vec!["D".to_string()])),
                (vec!["A".to_string()], vec!["C".to_string()], Some(vec!["D".to_string()])),
                (vec!["E".to_string()], vec!["F".to_string()], Some(vec!["G".to_string(), "H".to_string()]))
            ]).unwrap();

            // Test that decomposition creates equivalence
            assert!(ind_x.entails(&ind_y), "Complex independencies should entail their decomposition");
            
            // But decomposition might not entail the original (depending on axioms)
            let reverse_entailment = ind_y.entails(&ind_x);
            
            // If they entail each other, they should be equivalent
            if reverse_entailment {
                assert!(ind_x.is_equivalent(&ind_y), "Bidirectional entailment should mean equivalence");
                assert!(ind_y.is_equivalent(&ind_x), "Equivalence should be symmetric");
            }
        }
    }

}