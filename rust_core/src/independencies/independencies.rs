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
            // Do Collect pairs for SG3 before modifying all_independencies
            let mut new_pairs: HashSet<(&IndependenceAssertion, &IndependenceAssertion)> = HashSet::new();
            let all_current: Vec<IndependenceAssertion> = all_independencies.iter().cloned().collect();
            let new_current: Vec<IndependenceAssertion> = new_inds.iter().cloned().collect();
            let mut next_round = HashSet::new();
            
            // Apply unary axioms (SG1, SG2) to each new assertion
            for ind in &new_inds {
                next_round.extend(self.sg1_decomposition(ind));
                next_round.extend(self.sg2_weak_union(ind));
            }

            // Apply binary axiom (SG3) to all pairs
            // Example: {A ⊥ B | D} + {A ⊥ C | D} → {A ⊥ B,C | D} via contraction
            // We need to check new × new, new × all
            
            // new × new pairs
            for i in 0..new_current.len() {
                for j in (i+1)..new_current.len() {
                    new_pairs.insert((&new_current[i], &new_current[j]));
                    new_pairs.insert((&new_current[j], &new_current[i]));
                }
            }

            // new × all pairs  
            for new_ind in &new_current {
                for all_ind in &all_current {
                    new_pairs.insert((new_ind, all_ind));
                    new_pairs.insert((all_ind, new_ind));
                }
            }
            
            // Add current new independencies to the complete set
            all_independencies.extend(new_inds.iter().cloned());
            
            // Apply the Binary axiom
            for (ind1, ind2) in new_pairs {
                next_round.extend(self.sg3_contraction(ind1, ind2));
            }
            
            // Remove already known assertions
            // After applying axioms
            next_round.retain(|ind| !ind.event1.is_empty() && !ind.event2.is_empty() && !all_independencies.contains(ind));
            new_inds = next_round;
        }

        Self::from_assertions(all_independencies.into_iter().collect())
    }


    fn sg0(&self, ind: &IndependenceAssertion) -> IndependenceAssertion {
        IndependenceAssertion::new(
            ind.event2.clone(),
            ind.event1.clone(),
            if ind.event3.is_empty() { None } else { Some(ind.event3.clone()) },
        ).unwrap()
    }
    
    /// Decomposition rule: 'X ⊥ Y,W | Z' -> 'X ⊥ Y | Z', 'X ⊥ W | Z'
    fn sg1_decomposition(&self, ind: &IndependenceAssertion) -> Vec<IndependenceAssertion> {
        let mut results = Vec::new();
        
        // Decompose event2 if it has multiple elements
        if ind.event2.len() > 1 {
            for elem in &ind.event2 {
                // Create single-element set for this variable
                let single_var: HashSet<String> = [elem.clone()].into_iter().collect();
                
                if let Ok(assertion) = IndependenceAssertion::new(
                    ind.event1.clone(),
                    single_var,
                    Some(ind.event3.clone()),
                ) {
                    results.push(assertion);
                }
            }
        }
        
        // Decompose event1 if it has multiple elements (symmetry)
        if ind.event1.len() > 1 {
            for elem in &ind.event1 {
                // Create single-element set for this variable
                let single_var: HashSet<String> = [elem.clone()].into_iter().collect();
                
                if let Ok(assertion) = IndependenceAssertion::new(
                    single_var,
                    ind.event2.clone(),
                    Some(ind.event3.clone()),
                ) {
                    results.push(assertion);
                }
            }
        }
        
        results
    }
    
    /// Weak Union rule: 'X ⊥ Y,W | Z' -> 'X ⊥ Y | W,Z', 'X ⊥ W | Y,Z'
    fn sg2_weak_union(&self, ind: &IndependenceAssertion) -> Vec<IndependenceAssertion> {
        let mut results = Vec::new();
        
        // For each variable in event2, move it to the conditioning set
        if ind.event2.len() > 1 {
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
        }
        
        // For each variable in event1, move it to the conditioning set (symmetry)
        if ind.event1.len() > 1 {
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
        }
        
        results
    }
    
    /// Contraction rule: 'X ⊥ W | Y,Z' & 'X ⊥ Y | Z' -> 'X ⊥ W,Y | Z'
    fn sg3_contraction(&self, ind1: &IndependenceAssertion, ind2: &IndependenceAssertion) -> Vec<IndependenceAssertion> {
        let mut results = Vec::new();

        // Original pair
        results.extend(self.try_contraction_all_sym(ind1, ind2));

        // ind1 original, ind2 symmetric
        let ind2_sym = self.sg0(ind2);
        results.extend(self.try_contraction_all_sym(ind1, &ind2_sym));

        // ind1 symmetric, ind2 original
        let ind1_sym = self.sg0(ind1);
        results.extend(self.try_contraction_all_sym(&ind1_sym, ind2));

        // Both symmetric
        results.extend(self.try_contraction_all_sym(&ind1_sym, &ind2_sym));

        let unique_results: HashSet<IndependenceAssertion> = results.into_iter().collect();
        unique_results.into_iter().collect()
    }

    fn try_contraction_all_sym(&self, larger: &IndependenceAssertion, smaller: &IndependenceAssertion) -> Vec<IndependenceAssertion> {
        let mut res = Vec::new();
        if let Some(contracted) = self.try_contraction(larger, smaller) {
            res.push(contracted);
        }
        if let Some(contracted) = self.try_contraction(smaller, larger) {  // Try both orders
            res.push(contracted);
        }
        res
    }

    fn try_contraction(&self, larger: &IndependenceAssertion, smaller: &IndependenceAssertion) -> Option<IndependenceAssertion> {
        if larger.event1 != smaller.event1 {
            return None;
        }
        
        let y: &HashSet<String> = &smaller.event2;
        let z: &HashSet<String> = &smaller.event3;
        let y_z: &HashSet<String> = &larger.event3;
        
        // Use proper subset: subset and not equal
        if  y.is_subset(y_z) && y != y_z &&
            z.is_subset(y_z) && z != y_z &&
            y.is_disjoint(z) {
            // Create new assertion: X ⊥ W,Y | Z
            let mut new_event2 = larger.event2.clone();
            new_event2.extend(y.iter().cloned());
            
            if let Ok(assertion) = IndependenceAssertion::new(
                larger.event1.clone(),
                new_event2,
                if z.is_empty() { None } else { Some(z.clone()) },
            ) {
                return Some(assertion);
            }
        }
        
        None
    }



    pub fn reduce(&self) -> Self {
        let mut unique_assertions: Vec<IndependenceAssertion> = 
            self.assertions.iter().cloned().collect::<HashSet<_>>()
            .into_iter().collect();
        
        // Sort by event2 size (descending) to process more general assertions first
        unique_assertions.sort_by(|a, b| b.event2.len().cmp(&a.event2.len()));
        
        let mut reduced_assertions: Vec<IndependenceAssertion> = Vec::new();

        for assertion in unique_assertions {
            let temp_independencies: Independencies = Self::from_assertions(reduced_assertions.clone());
            let new_assertion: Independencies = Self::from_assertions(vec![assertion.clone()]);

            // Only add if not entailed by current reduced set
            if !temp_independencies.entails(&new_assertion) {
                // Remove any existing assertions that are entailed by the new assertion
                reduced_assertions.retain(|existing: &IndependenceAssertion| {
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