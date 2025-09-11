use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use rust_core::{IndependenceAssertion, Independencies};
use js_sys::{Object, Array};

#[wasm_bindgen(js_name = DAG)]
#[derive(Clone)]
pub struct DAG {
    inner: rust_core::RustDAG,
}

#[wasm_bindgen]
impl DAG {
    #[wasm_bindgen(constructor)]
    pub fn new() -> DAG {
        DAG {
            inner: rust_core::RustDAG::new(),
        }
    }

    #[wasm_bindgen(js_name = addNode, catch)]
    pub fn add_node(&mut self, node: String, latent: Option<bool>) -> Result<(), JsValue> {
        self.inner
            .add_node(node, latent.unwrap_or(false))
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = addNodesFrom)]
    pub fn add_nodes_from(
        &mut self,
        nodes: Vec<String>,
        latent: Option<Vec<u8>>,
    ) -> Result<(), JsValue> {
        let latent_bools = latent.map(|v| v.into_iter().map(|x| x != 0).collect());
        self.inner
            .add_nodes_from(nodes, latent_bools)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = addEdge, catch)]
    pub fn add_edge(&mut self, u: String, v: String, weight: Option<f64>) -> Result<(), JsValue> {
        self.inner
            .add_edge(u, v, weight)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = getParents, catch)]
    pub fn get_parents(&self, node: String) -> Result<Vec<String>, JsValue> {
        self.inner
            .get_parents(&node)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = getChildren, catch)]
    pub fn get_children(&self, node: String) -> Result<Vec<String>, JsValue> {
        self.inner
            .get_children(&node)
            .map_err(|e| JsValue::from_str(&e))
    }

    // For `HashSet<String>` return, WASM-bindgen prefers `Vec<String>` or serializable.
    #[wasm_bindgen(js_name = getAncestorsOf, catch)]
    pub fn get_ancestors_of(&self, nodes: Vec<String>) -> Result<Vec<String>, JsValue> {
        self.inner
            .get_ancestors_of(nodes)
            .map(|set| set.into_iter().collect()) // Convert HashSet to Vec
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = nodes)]
    pub fn nodes(&self) -> Vec<String> {
        self.inner.nodes()
    }

    #[wasm_bindgen(js_name = edges)]
    pub fn edges(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.edges())
            .unwrap_or_else(|_| JsValue::from_str("Failed to serialize edges"))
    }

    #[wasm_bindgen(js_name = nodeCount, getter)]
    pub fn node_count(&self) -> usize {
        self.inner.node_count()
    }

    #[wasm_bindgen(js_name = edgeCount, getter)]
    pub fn edge_count(&self) -> usize {
        self.inner.edge_count()
    }

    // Expose latents
    #[wasm_bindgen(js_name = latents, getter)]
    pub fn latents(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.latents)
            .unwrap_or_else(|_| JsValue::from_str("Failed to serialize latents"))
    }

    // In RustDAG impl
    #[wasm_bindgen(js_name = minimalDseparator, catch)]
    pub fn minimal_dseparator(&self, starts: Vec<String>, ends: Vec<String>, include_latents: Option<bool>) -> Result<JsValue, JsValue> {
        let result = self.inner.minimal_dseparator(starts, ends, include_latents.unwrap_or(false))
            .map_err(|e| JsValue::from_str(&e))?;
        
        match result {
            Some(mut set) => {
                let mut vec: Vec<String> = set.drain().collect();
                vec.sort();
                let js_array = Array::new();
                for item in vec {
                    js_array.push(&JsValue::from_str(&item));
                }
                Ok(js_array.into())  // Return JS Array
            }
            None => Ok(JsValue::NULL),
        }
    }

    #[wasm_bindgen(js_name = activeTrailNodes, catch)]
    pub fn active_trail_nodes(&self, variables: Vec<String>, observed: Option<Vec<String>>, include_latents: Option<bool>) -> Result<JsValue, JsValue> {
        let result = self.inner.active_trail_nodes(variables, observed, include_latents.unwrap_or(false))
            .map_err(|e| JsValue::from_str(&e))?;
        
        // Create a plain JS Object
        let js_object = Object::new();
        
        for (key, mut set) in result {
            let mut vec: Vec<String> = set.drain().collect();
            vec.sort();
            
            let js_array = Array::new();
            for item in vec {
                js_array.push(&JsValue::from_str(&item));
            }
            
            // Set property on object (key: array)
            js_sys::Reflect::set(&js_object, &JsValue::from_str(&key), &js_array.into())
                .map_err(|_| JsValue::from_str("Failed to set property"))?;
        }
        
        Ok(js_object.into())
    }

    #[wasm_bindgen(js_name = isDconnected, catch)]
    pub fn is_dconnected(
        &self,
        start: String,
        end: String,
        observed: Option<Vec<String>>,
        include_latents: Option<bool>,
    ) -> Result<bool, JsValue> {
        self.inner.is_dconnected(&start, &end, observed, include_latents.unwrap_or(false))
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = areNeighbors, catch)]
    pub fn are_neighbors(&self, start: String, end: String) -> Result<bool, JsValue> {
        self.inner.are_neighbors(&start, &end)
            .map_err(|e| JsValue::from_str(&e))
    }
}


#[wasm_bindgen]
#[derive(Clone)]
pub struct JsIndependenceAssertion {
    inner: IndependenceAssertion,
}

#[wasm_bindgen]
impl JsIndependenceAssertion {
    #[wasm_bindgen(constructor)]
    pub fn new(event1: Vec<String>, event2: Vec<String>, event3: Option<Vec<String>>) -> Result<JsIndependenceAssertion, JsValue> {
        let e1: HashSet<String> = event1.into_iter().collect();
        let e2: HashSet<String> = event2.into_iter().collect();
        let e3: Option<HashSet<String>> = event3.map(|v| v.into_iter().collect());
        let assertion = IndependenceAssertion::new(e1, e2, e3)
            .map_err(|e| JsValue::from_str(&e))?;
        Ok(JsIndependenceAssertion { inner: assertion })
    }

    #[wasm_bindgen(js_name = event1)]
    pub fn event1(&self) -> Vec<String> {
        self.inner.event1.iter().cloned().collect()
    }

    #[wasm_bindgen(js_name = event2)]
    pub fn event2(&self) -> Vec<String> {
        self.inner.event2.iter().cloned().collect()
    }

    #[wasm_bindgen(js_name = event3)]
    pub fn event3(&self) -> Vec<String> {
        let mut e3_vec: Vec<String> = self.inner.event3.iter().cloned().collect();
        e3_vec.sort();
        e3_vec
    }

    #[wasm_bindgen(js_name = allVars)]
    pub fn all_vars(&self) -> Vec<String> {
        // Return variables in the order: event1, event2, event3
        // Sort within each set for consistency
        let mut all_vars_vec = Vec::new();
        
        // Add event1 variables (sorted)
        let mut e1_vec: Vec<String> = self.inner.event1.iter().cloned().collect();
        e1_vec.sort();
        all_vars_vec.extend(e1_vec);
        
        // Add event2 variables (sorted)
        let mut e2_vec: Vec<String> = self.inner.event2.iter().cloned().collect();
        e2_vec.sort();
        all_vars_vec.extend(e2_vec);
        
        // Add event3 variables (sorted)
        let mut e3_vec: Vec<String> = self.inner.event3.iter().cloned().collect();
        e3_vec.sort();
        all_vars_vec.extend(e3_vec);
        
        all_vars_vec
    }

    #[wasm_bindgen(js_name = isUnconditional)]
    pub fn is_unconditional(&self) -> bool {
        self.inner.is_unconditional()
    }

    #[wasm_bindgen(js_name = toLatex)]
    pub fn to_latex(&self) -> String {
        self.inner.to_latex()
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        // Create a canonical representation that handles symmetry
        let mut e1_vec: Vec<String> = self.inner.event1.iter().cloned().collect();
        let mut e2_vec: Vec<String> = self.inner.event2.iter().cloned().collect();
        e1_vec.sort();
        e2_vec.sort();
        
        // For symmetry, ensure consistent ordering: put the lexicographically smaller set first
        let (first, second) = if e1_vec < e2_vec {
            (e1_vec, e2_vec)
        } else {
            (e2_vec, e1_vec)
        };
        
        let first_str = first.join(", ");
        let second_str = second.join(", ");
        
        if self.inner.event3.is_empty() {
            format!("({} ⊥ {})", first_str, second_str)
        } else {
            let mut e3_vec: Vec<String> = self.inner.event3.iter().cloned().collect();
            e3_vec.sort();
            let e3_str = e3_vec.join(", ");
            format!("({} ⊥ {} | {})", first_str, second_str, e3_str)
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct JsIndependencies {
    inner: Independencies,
}

#[wasm_bindgen]
impl JsIndependencies {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsIndependencies {
        JsIndependencies { inner: Independencies::new() }
    }

    #[wasm_bindgen(js_name = addAssertion)]
    pub fn add_assertion(&mut self, assertion: &JsIndependenceAssertion) {
        self.inner.add_assertion(assertion.inner.clone());
    }

    #[wasm_bindgen(js_name = addAssertionsFromTuples)]
    pub fn add_assertions_from_tuples(&mut self, tuples: JsValue) -> Result<(), JsValue> {
        let tuples: Vec<(Vec<String>, Vec<String>, Option<Vec<String>>)> =
            serde_wasm_bindgen::from_value(tuples)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.inner.add_assertions_from_tuples(tuples)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = getAssertions)]
    pub fn get_assertions(&self) -> Vec<JsIndependenceAssertion> {
        self.inner.get_assertions()
            .iter()
            .map(|a| JsIndependenceAssertion { inner: a.clone() })
            .collect()
    }

    #[wasm_bindgen(js_name = getAllVariables)]
    pub fn get_all_variables(&self) -> Vec<String> {
        self.inner.get_all_variables().into_iter().collect()
    }

    #[wasm_bindgen(js_name = contains)]
    pub fn contains(&self, assertion: &JsIndependenceAssertion) -> bool {
        self.inner.contains(&assertion.inner)
    }

    #[wasm_bindgen(js_name = closure)]
    pub fn closure(&self) -> JsIndependencies {
        JsIndependencies { inner: self.inner.closure() }
    }

    #[wasm_bindgen(js_name = reduce)]
    pub fn reduce(&self) -> JsIndependencies {
        JsIndependencies { inner: self.inner.reduce() }
    }

    #[wasm_bindgen(js_name = entails)]
    pub fn entails(&self, other: &JsIndependencies) -> bool {
        self.inner.entails(&other.inner)
    }

    #[wasm_bindgen(js_name = isEquivalent)]
    pub fn is_equivalent(&self, other: &JsIndependencies) -> bool {
        self.inner.is_equivalent(&other.inner)
    }
}


#[wasm_bindgen(js_name = PDAG)]
pub struct PDAG {
    inner: rust_core::RustPDAG,
}

#[wasm_bindgen]
impl PDAG {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: rust_core::RustPDAG::new(),
        }
    }

    #[wasm_bindgen(js_name = addNode, catch)]
    pub fn add_node(&mut self, node: String, latent: Option<bool>) -> Result<(), JsValue> {
        self.inner
            .add_node(node, latent.unwrap_or(false))
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = addNodesFrom, catch)]
    pub fn add_nodes_from(
        &mut self,
        nodes: Vec<String>,
        latent: Option<Vec<u8>>,
    ) -> Result<(), JsValue> {
        let latent_bools = latent.map(|v| v.into_iter().map(|x| x != 0).collect());
        self.inner
            .add_nodes_from(nodes, latent_bools)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = addEdge, catch)]
    pub fn add_edge(
        &mut self,
        u: String,
        v: String,
        weight: Option<f64>,
        directed: bool,
    ) -> Result<(), JsValue> {
        self.inner
            .add_edge(u, v, weight, directed)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = addEdgesFrom, catch)]
    pub fn add_edges_from(
        &mut self,
        ebunch: JsValue,
        weights: Option<Vec<f64>>,
        directed: bool,
    ) -> Result<(), JsValue> {
        let ebunch_vec: Vec<(String, String)> = serde_wasm_bindgen::from_value(ebunch)?;
        self.inner
            .add_edges_from(Some(ebunch_vec), weights, directed)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = nodes)]
    pub fn nodes(&self) -> Vec<String> {
        self.inner.nodes()
    }

    #[wasm_bindgen(js_name = edges)]
    pub fn edges(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.edges()).unwrap()
    }

    #[wasm_bindgen(js_name = directedEdges)]
    pub fn directed_edges(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.directed_edges).unwrap()
    }

    #[wasm_bindgen(js_name = undirectedEdges)]
    pub fn undirected_edges(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.undirected_edges).unwrap()
    }

    #[wasm_bindgen(js_name = nodeCount, getter)]
    pub fn node_count(&self) -> usize {
        self.inner.node_map.len()
    }

    #[wasm_bindgen(js_name = edgeCount, getter)]
    pub fn edge_count(&self) -> usize {
        self.inner.directed_edges.len() + self.inner.undirected_edges.len()
    }

    #[wasm_bindgen(js_name = latents, getter)]
    pub fn latents(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.latents).unwrap()
    }

    #[wasm_bindgen(js_name = applyMeeksRules, catch)]
    pub fn apply_meeks_rules(
        &mut self,
        apply_r4: bool,
        inplace: bool,
    ) -> Result<Option<PDAG>, JsValue> {
        self.inner
            .apply_meeks_rules(apply_r4, inplace)
            .map(|opt| opt.map(|pdag| PDAG { inner: pdag }))
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = toDag, catch)]
    pub fn to_dag(&self) -> Result<DAG, JsValue> {
        self.inner
            .to_dag()
            .map(|dag| DAG { inner: dag })
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = copy)]
    pub fn copy(&self) -> PDAG {
        PDAG {
            inner: self.inner.clone(),
        }
    }
}

// Optional: Add a start function for debugging or initialization
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This is optional, but useful for setting up panic hooks in browser
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // logs
    web_sys::console::log_1(&"RustDAG loaded!".into());

    Ok(())
}
