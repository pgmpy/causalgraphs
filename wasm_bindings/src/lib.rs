use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize}; // For serializing/deserializing data to JS
use std::collections::HashSet; // For returning HashSet from Rust to Vec in JS

// Import RustDAG from your core library
use rust_core::RustDAG;

// Add a public struct to wrap RustDAG for WASM.
// It can directly be `RustDAG` if you make it `#[wasm_bindgen]`
// but sometimes a wrapper is cleaner for WASM for more control over JS API.
// Let's directly expose RustDAG as #[wasm_bindgen] for simplicity, similar to Python.
// However, RustDAG must be Clone, and its internal fields must be serializable if exposed.

// You will likely want to make your RustDAG cloneable and serialize/deserialize for WASM,
// allowing it to be passed between Rust and JS contexts.
// Make sure `RustDAG` in `rust_core/src/dag.rs` has `#[derive(Clone, Serialize, Deserialize)]` if needed.
// IMPORTANT: `DiGraph` from `petgraph` does NOT implement `Serialize` or `Deserialize` directly.
// You'll need to either:
// 1. Manually serialize/deserialize `RustDAG` (complex).
// 2. Expose methods that operate on the graph but don't pass the graph *object* itself.
// 3. Use a different graph library if it provides Serde support.
//
// For simplicity in this example, let's assume `RustDAG` will have methods,
// but the struct itself won't be directly serialized/deserialized to JS object,
// unless you add custom Serde implementations.
// If you only pass primitives and call methods, `#[wasm_bindgen]` can apply directly.

// Applying `#[wasm_bindgen]` to `RustDAG` directly.
// Note: If `RustDAG` in `rust_core` needs to be `Serialize`/`Deserialize`
// for use with `serde_wasm_bindgen`, you'll need to add `#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]`
// to the `RustDAG` struct *in `rust_core/src/dag.rs`*. This requires `serde` to be
// an optional dependency in `rust_core`'s `Cargo.toml` with a `wasm` feature.
// This is getting more complex, so let's stick to methods for now.

#[wasm_bindgen]
#[derive(Clone)] // Make sure RustDAG in rust_core also derives Clone
pub struct WasmDAG { // Use a wrapper struct named WasmDAG for clarity
    inner: RustDAG,
}

#[wasm_bindgen]
impl WasmDAG {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmDAG {
        WasmDAG { inner: RustDAG::new() }
    }

    #[wasm_bindgen(js_name = addNode, catch)]
    pub fn add_node(&mut self, node: String, latent: Option<bool>) -> Result<(), JsValue> {
        self.inner.add_node(node, latent.unwrap_or(false))
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen]
    pub fn add_nodes_from(&mut self, nodes: Vec<String>, latent: Option<Vec<u8>>) -> Result<(), JsValue> {
        let latent_bools = latent.map(|v| v.into_iter().map(|x| x != 0).collect());
        self.inner.add_nodes_from(nodes, latent_bools).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = addEdge, catch)]
    pub fn add_edge(&mut self, u: String, v: String, weight: Option<f64>) -> Result<(), JsValue> {
        self.inner.add_edge(u, v, weight)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = getParents, catch)]
    pub fn get_parents(&self, node: String) -> Result<Vec<String>, JsValue> {
        self.inner.get_parents(&node)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = getChildren, catch)]
    pub fn get_children(&self, node: String) -> Result<Vec<String>, JsValue> {
        self.inner.get_children(&node)
            .map_err(|e| JsValue::from_str(&e))
    }

    // For `HashSet<String>` return, WASM-bindgen prefers `Vec<String>` or serializable.
    #[wasm_bindgen(js_name = getAncestorsOf, catch)]
    pub fn get_ancestors_of(&self, nodes: Vec<String>) -> Result<Vec<String>, JsValue> {
        self.inner.get_ancestors_of(nodes)
            .map(|set| set.into_iter().collect()) // Convert HashSet to Vec
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = nodes)]
    pub fn nodes(&self) -> Vec<String> {
        self.inner.nodes()
    }

    #[wasm_bindgen(js_name = edges)]
    pub fn edges(&self) -> JsValue { // Return JsValue for complex types like Vec<(String, String)>
        serde_wasm_bindgen::to_value(&self.inner.edges()).unwrap_or_else(|_| JsValue::from_str("Failed to serialize edges"))
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
        serde_wasm_bindgen::to_value(&self.inner.latents).unwrap_or_else(|_| JsValue::from_str("Failed to serialize latents"))
    }
}

// Optional: Add a start function for debugging or initialization
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This is optional, but useful for setting up panic hooks in browser
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // You can do some initialization here if needed
    web_sys::console::log_1(&"WasmDAG loaded!".into());

    Ok(())
}