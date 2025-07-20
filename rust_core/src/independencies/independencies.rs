use std::collections::{BTreeSet, HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Eq)]
pub struct IndependenceAssertion {
    pub event1: HashSet<String>,
    pub event2: HashSet<String>, 
    pub event3: HashSet<String>,
    pub all_vars: HashSet<String>,
}

impl IndependenceAssertion {
    pub fn new(
        event1: HashSet<String>,
        event2: HashSet<String>,
        event3: Option<HashSet<String>>,
    ) -> Result<Self, String> {
        if event1.is_empty() {
            return Err("event1 needs to be specified".to_string());
        }
        if event2.is_empty() {
            return Err("event2 needs to be specified".to_string());
        }
        
        let e3 = event3.unwrap_or_default();
        
        let mut all_vars = HashSet::new();
        all_vars.extend(event1.iter().cloned());
        all_vars.extend(event2.iter().cloned());
        all_vars.extend(e3.iter().cloned());
        
        Ok(Self {
            event1,
            event2,
            event3: e3,
            all_vars,
        })
    }
    
    pub fn is_unconditional(&self) -> bool {
        self.event3.is_empty()
    }
    
    pub fn to_latex(&self) -> String {
        let e1_str = self.event1.iter().cloned().collect::<Vec<_>>().join(", ");
        let e2_str = self.event2.iter().cloned().collect::<Vec<_>>().join(", ");
        
        if self.event3.is_empty() {
            format!("{} \\perp {}", e1_str, e2_str)
        } else {
            let e3_str = self.event3.iter().cloned().collect::<Vec<_>>().join(", ");
            format!("{} \\perp {} \\mid {}", e1_str, e2_str, e3_str)
        }
    }
}

impl std::fmt::Display for IndependenceAssertion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let e1_str = self.event1.iter().cloned().collect::<Vec<_>>().join(", ");
        let e2_str = self.event2.iter().cloned().collect::<Vec<_>>().join(", ");
        
        if self.event3.is_empty() {
            write!(f, "({} ⊥ {})", e1_str, e2_str)
        } else {
            let e3_str = self.event3.iter().cloned().collect::<Vec<_>>().join(", ");
            write!(f, "({} ⊥ {} | {})", e1_str, e2_str, e3_str)
        }
    }
}

impl Hash for IndependenceAssertion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Convert HashSets to BTreeSets for deterministic, sortable representation
        let event1_ordered: BTreeSet<_> = self.event1.iter().collect();
        let event2_ordered: BTreeSet<_> = self.event2.iter().collect();
        let event3_ordered: BTreeSet<_> = self.event3.iter().collect();
        
        // Create symmetric hash: X ⊥ Y | Z should equal Y ⊥ X | Z
        let mut symmetric_pair: Vec<BTreeSet<&String>> = vec![event1_ordered, event2_ordered];
        symmetric_pair.sort(); // BTreeSet implements Ord, so this works
        
        // Hash the components
        symmetric_pair.hash(state);    // Symmetric part
        event3_ordered.hash(state);    // Conditioning set
    }
}

impl PartialEq for IndependenceAssertion {
    fn eq(&self, other: &Self) -> bool {
        // Check if event3 (conditioning set) is the same
        if self.event3 != other.event3 {
            return false;
        }
        
        // Check both orientations for event1 and event2 (symmetry)
        (self.event1 == other.event1 && self.event2 == other.event2) ||
        (self.event1 == other.event2 && self.event2 == other.event1)
    }
}


#[derive(Debug, Clone)]
pub struct Independencies {
    assertions: Vec<IndependenceAssertion>,
}

impl Independencies {
    pub fn new() -> Self {
        Self {
            assertions: Vec::new(),
        }
    }
    
    pub fn from_assertions(assertions: Vec<IndependenceAssertion>) -> Self {
        Self { assertions }
    }
    
    pub fn add_assertion(&mut self, assertion: IndependenceAssertion) {
        self.assertions.push(assertion);
    }
    
    pub fn add_assertions_from_tuples(&mut self, tuples: Vec<(Vec<String>, Vec<String>, Option<Vec<String>>)>) -> Result<(), String> {
        for (e1, e2, e3) in tuples {
            let assertion = IndependenceAssertion::new(
                e1.into_iter().collect::<HashSet<_>>(),
                e2.into_iter().collect::<HashSet<_>>(),
                e3.map(|x| x.into_iter().collect::<HashSet<_>>()),
            )?;
            self.add_assertion(assertion);
        }
        Ok(())
    }
    
    pub fn get_assertions(&self) -> &Vec<IndependenceAssertion> {
        &self.assertions
    }
    
    pub fn get_all_variables(&self) -> HashSet<String> {
        self.assertions
            .iter()
            .flat_map(|a| a.all_vars.iter().cloned())
            .collect()
    }
    
    pub fn contains(&self, assertion: &IndependenceAssertion) -> bool {
        self.assertions.contains(assertion)
    }

    /// Generate closure using semi-graphoid axioms
    pub fn closure(&self) -> Self {
        let mut all_independencies: HashSet<IndependenceAssertion> = HashSet::new();
        let mut new_inds: HashSet<IndependenceAssertion> = self.assertions.iter().cloned().collect();

        // Example: Start with {A ⊥ B,C | D}
        // Iteration 1: SG1 generates {A ⊥ B | D, A ⊥ C | D}
        // Iteration 2: SG2 generates {A ⊥ B | C,D, A ⊥ C | B,D} 
        // Iteration 3: SG3 might combine pairs, continues until no new assertions
        while !new_inds.is_empty() {
            // Add current new independencies to the complete set
            all_independencies.extend(new_inds.iter().cloned());
            
            let mut next_round = HashSet::new();
            
            // Apply unary axioms (SG1, SG2) to each new assertion individually
            for ind in &new_inds {
                next_round.extend(self.sg1_decomposition(ind));
                next_round.extend(self.sg2_weak_union(ind));
            }
            
            // Apply binary axiom (SG3) to all pairs - this is the expensive O(n²) part
            // Example: {A ⊥ B | D} + {A ⊥ C | D} → {A ⊥ B,C | D} via contraction
            // We need to check new × new, new × all, and all × new pairs
            let all_current: Vec<IndependenceAssertion> = all_independencies.iter().cloned().collect();
            let new_current: Vec<IndependenceAssertion> = new_inds.iter().cloned().collect();
            
            // new × new pairs
            for i in 0..new_current.len() {
                for j in i..new_current.len() {
                    next_round.extend(self.sg3_contraction(&new_current[i], &new_current[j]));
                    if i != j {
                        next_round.extend(self.sg3_contraction(&new_current[j], &new_current[i]));
                    }
                }
            }
            
            // new × all pairs  
            for new_ind in &new_current {
                for all_ind in &all_current {
                    next_round.extend(self.sg3_contraction(new_ind, all_ind));
                    next_round.extend(self.sg3_contraction(all_ind, new_ind));
                }
            }
            
            // Remove already known assertions
            next_round.retain(|ind| !all_independencies.contains(ind));
            new_inds = next_round;
        }

        Self::from_assertions(all_independencies.into_iter().collect())
    }
    
    /// Decomposition rule: 'X ⊥ Y,W | Z' -> 'X ⊥ Y | Z', 'X ⊥ W | Z'
    fn sg1_decomposition(&self, ind: &IndependenceAssertion) -> Vec<IndependenceAssertion> {
        if ind.event2.len() <= 1 {
            return vec![];
        }
        
        let mut results: Vec<IndependenceAssertion> = Vec::new();
        for elem in &ind.event2 {
            let mut new_event2: HashSet<String> = ind.event2.clone();
            new_event2.remove(elem);
            
            if let Ok(assertion) = IndependenceAssertion::new(
                ind.event1.clone(),
                new_event2,
                Some(ind.event3.clone()),
            ) {
                results.push(assertion);
            }
        }
        
        // Apply symmetry
        for elem in &ind.event1 {
            let mut new_event1 = ind.event1.clone();
            new_event1.remove(elem);
            
            if let Ok(assertion) = IndependenceAssertion::new(
                new_event1,
                ind.event2.clone(),
                Some(ind.event3.clone()),
            ) {
                results.push(assertion);
            }
        }
        
        results
    }
    
    /// Weak Union rule: 'X ⊥ Y,W | Z' -> 'X ⊥ Y | W,Z', 'X ⊥ W | Y,Z'
    fn sg2_weak_union(&self, ind: &IndependenceAssertion) -> Vec<IndependenceAssertion> {
        if ind.event2.len() <= 1 {
            return vec![];
        }
        
        let mut results = Vec::new();
        for elem in &ind.event2 {
            let mut new_event2 = ind.event2.clone();
            new_event2.remove(elem);
            let mut new_event3 = ind.event3.clone();
            new_event3.insert(elem.clone());
            
            if let Ok(assertion) = IndependenceAssertion::new(
                ind.event1.clone(),
                new_event2,
                Some(new_event3),
            ) {
                results.push(assertion);
            }
        }
        
        // Apply symmetry for event1
        for elem in &ind.event1 {
            let mut new_event1 = ind.event1.clone();
            new_event1.remove(elem);
            let mut new_event3 = ind.event3.clone();
            new_event3.insert(elem.clone());
            
            if let Ok(assertion) = IndependenceAssertion::new(
                new_event1,
                ind.event2.clone(),
                Some(new_event3),
            ) {
                results.push(assertion);
            }
        }
        
        results
    }
    
    /// Contraction rule: 'X ⊥ W | Y,Z' & 'X ⊥ Y | Z' -> 'X ⊥ W,Y | Z'
    fn sg3_contraction(&self, ind1: &IndependenceAssertion, ind2: &IndependenceAssertion) -> Vec<IndependenceAssertion> {
        let mut results = Vec::new();
        
        // Must have same event1
        if ind1.event1 != ind2.event1 {
            return results;
        }
        
        // Simple case: same conditioning set, combine the independence sets
        if ind1.event3 == ind2.event3 {
            let mut combined_event2 = ind1.event2.clone();
            combined_event2.extend(ind2.event2.iter().cloned());
            
            // Only add if it's actually combining something new
            if combined_event2 != ind1.event2 && combined_event2 != ind2.event2 {
                if let Ok(assertion) = IndependenceAssertion::new(
                    ind1.event1.clone(),
                    combined_event2,
                    if ind1.event3.is_empty() { None } else { Some(ind1.event3.clone()) },
                ) {
                    results.push(assertion);
                }
            }
        }
        
        // Standard contraction rule cases
        results.extend(self.try_standard_contraction(ind1, ind2));
        results.extend(self.try_standard_contraction(ind2, ind1));
        
        results
    }

    fn try_standard_contraction(&self, larger: &IndependenceAssertion, smaller: &IndependenceAssertion) -> Vec<IndependenceAssertion> {
        let y = &smaller.event2; // Variables we're independent from in smaller assertion
        let z = &smaller.event3; // What we condition on in smaller assertion  
        let y_z = &larger.event3; // What we condition on in larger assertion
        
        // Check if larger conditions on exactly Y∪Z and Y∩Z = ∅
        // Example: smaller = {A ⊥ C | D}, larger = {A ⊥ B | C,D} 
        // Here: y={C}, z={D}, y_z={C,D}, y∪z={C,D} ✓, y∩z=∅ ✓
        let y_union_z: HashSet<String> = y.union(z).cloned().collect();
        if y_union_z == *y_z && y.is_disjoint(z) {
            let mut new_event2 = larger.event2.clone();
            new_event2.extend(y.iter().cloned());
            
            if let Ok(assertion) = IndependenceAssertion::new(
                larger.event1.clone(),
                new_event2,
                if z.is_empty() { None } else { Some(z.clone()) },
            ) {
                return vec![assertion];
            }
        }
        
        vec![]
    }

    pub fn reduce(&self) -> Self {
        let mut unique_assertions: Vec<IndependenceAssertion> = 
            self.assertions.iter().cloned().collect::<HashSet<_>>()
            .into_iter().collect();
        
        // Sort by event2 size (descending) to process more general assertions first
        // unique_assertions.sort_by(|a, b| b.event2.len().cmp(&a.event2.len()));
        
        let mut reduced_assertions = Vec::new();

        for assertion in unique_assertions {
            let temp_independencies: Independencies = Self::from_assertions(reduced_assertions.clone());
            let new_assertion: Independencies = Self::from_assertions(vec![assertion.clone()]);

            // Only add if not entailed by current reduced set
            if !temp_independencies.entails(&new_assertion) {
                // Remove any existing assertions that are entailed by the new assertion
                reduced_assertions.retain(|existing| {
                    let existing_temp = Self::from_assertions(vec![existing.clone()]);
                    !new_assertion.entails(&existing_temp)
                });
                reduced_assertions.push(assertion);
            }
        }

        Self::from_assertions(reduced_assertions)
    }

    pub fn reduce_inplace(&mut self) {
        let reduced = self.reduce();
        self.assertions = reduced.assertions;
    }
    
    /// Check if this set of independencies entails another set
    pub fn entails(&self, other: &Independencies) -> bool {
        let closure_assertions = self.closure().assertions;
        other.assertions.iter().all(|assertion| closure_assertions.contains(assertion))
    }
    
    /// Check if two sets of independencies are equivalent
    pub fn is_equivalent(&self, other: &Independencies) -> bool {
        self.entails(other) && other.entails(self)
    }
}

impl PartialEq for Independencies {
    fn eq(&self, other: &Self) -> bool {
        // Convert to sets to ignore order
        let self_set: std::collections::HashSet<_> = self.assertions.iter().collect();
        let other_set: std::collections::HashSet<_> = other.assertions.iter().collect();
        self_set == other_set
    }
}
