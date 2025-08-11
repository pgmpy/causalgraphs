use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = RustDAG)]
#[derive(Clone)]
pub struct RustDAG {
    inner: rust_core::RustDAG,
}

#[wasm_bindgen]
impl RustDAG {
    #[wasm_bindgen(constructor)]
    pub fn new() -> RustDAG {
        RustDAG {
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
