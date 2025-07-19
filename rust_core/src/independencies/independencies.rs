use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndependenceAssertion {
    pub event1: HashSet<String>,
    pub event2: HashSet<String>,
    pub event3: HashSet<String>,
}

impl IndependenceAssertion {
    pub fn new(event1: HashSet<String>, event2: HashSet<String>, event3: HashSet<String>) -> Self {
        IndependenceAssertion { event1, event2, event3 }
    }

    pub fn latex_string(&self) -> String {
        if self.event3.is_empty() {
            format!(
                "{} \\perp {}",
                self.event1.iter().join(", "),
                self.event2.iter().join(", ")
            )
        } else {
            format!(
                "{} \\perp {} \\mid {}",
                self.event1.iter().join(", "),
                self.event2.iter().join(", "),
                self.event3.iter().join(", ")
            )
        }
    }
}

#[derive(Debug, Clone)]
pub struct Independencies {
    pub assertions: Vec<IndependenceAssertion>,
}

impl Independencies {
    pub fn new() -> Self {
        Independencies { assertions: Vec::new() }
    }

    pub fn add_assertion(&mut self, assertion: IndependenceAssertion) {
        self.assertions.push(assertion);
    }

    pub fn reduce(&mut self) -> Independencies {
        let mut unique_assertions: HashSet<IndependenceAssertion> = self.assertions.iter().cloned().collect();
        let mut reduced_assertions = Vec::new();

        for assertion in unique_assertions {
            let mut temp_independencies = Independencies { assertions: reduced_assertions.clone() };
            let assertion_temp = Independencies { assertions: vec![assertion.clone()] };

            let entails = temp_independencies.assertions.iter().any(|existing| {
                existing.event1 == assertion.event1 &&
                existing.event2 == assertion.event2 &&
                existing.event3 == assertion.event3
            });

            if !entails {
                let mut removed_any = true;
                while removed_any {
                    removed_any = false;
                    let current_reduced = reduced_assertions.clone();
                    for existing in current_reduced.iter() {
                        let existing_temp = Independencies { assertions: vec![existing.clone()] };
                        if existing != &assertion {
                            // Placeholder for entailment logic
                            let remove_old = false; // Replace with actual entailment check if needed
                            if remove_old {
                                reduced_assertions.retain(|x| x != existing);
                                removed_any = true;
                                break;
                            }
                        }
                    }
                }
                reduced_assertions.push(assertion);
            }
        }

        Independencies { assertions: reduced_assertions }
    }

    pub fn latex_string(&self) -> Vec<String> {
        self.assertions.iter().map(|assertion| assertion.latex_string()).collect()
    }
}
