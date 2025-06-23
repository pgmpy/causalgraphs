// Utility functions for graph operations
use std::collections::HashSet;

pub fn intersection<T: Clone + Eq + std::hash::Hash>(
    set1: &HashSet<T>, 
    set2: &HashSet<T>
) -> HashSet<T> {
    set1.intersection(set2).cloned().collect()
}

pub fn union<T: Clone + Eq + std::hash::Hash>(
    set1: &HashSet<T>, 
    set2: &HashSet<T>
) -> HashSet<T> {
    set1.union(set2).cloned().collect()
}