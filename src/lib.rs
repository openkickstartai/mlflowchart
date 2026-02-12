//! MLFlowChart - Interactive ML workflow visualization tool
//! 
//! This library provides WebAssembly bindings for rendering ML pipelines
//! with drag-and-drop functionality and real-time tracking.

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Node {
    pub id: String,
    pub node_type: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Edge {
    pub id: String,
    pub source: String,
    pub target: String,
}

#[wasm_bindgen]
pub struct Pipeline {
    nodes: HashMap<String, Node>,
    edges: Vec<Edge>,
    metrics: HashMap<String, f64>,
}

#[wasm_bindgen]
impl Pipeline {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Pipeline {
        console_log!("Initializing new ML pipeline");
        Pipeline {
            nodes: HashMap::new(),
            edges: Vec::new(),
            metrics: HashMap::new(),
        }
    }

    #[wasm_bindgen]
    pub fn add_node(&mut self, id: &str, node_type: &str, x: f64, y: f64) {
        let node = Node {
            id: id.to_string(),
            node_type: node_type.to_string(),
            x,
            y,
            width: 120.0,
            height: 60.0,
        };
        self.nodes.insert(id.to_string(), node);
        console_log!("Added node: {} of type {}", id, node_type);
    }

    #[wasm_bindgen]
    pub fn connect_nodes(&mut self, source: &str, target: &str) {
        let edge = Edge {
            id: format!("{}-{}", source, target),
            source: source.to_string(),
            target: target.to_string(),
        };
        self.edges.push(edge);
        console_log!("Connected {} -> {}", source, target);
    }

    #[wasm_bindgen]
    pub fn update_metrics(&mut self, node_id: &str, accuracy: f64, loss: f64) {
        self.metrics.insert(format!("{}_accuracy", node_id), accuracy);
        self.metrics.insert(format!("{}_loss", node_id), loss);
        console_log!("Updated metrics for {}: acc={}, loss={}", node_id, accuracy, loss);
    }

    #[wasm_bindgen]
    pub fn get_node_count(&self) -> usize {
        self.nodes.len()
    }

    #[wasm_bindgen]
    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log!("MLFlowChart WebAssembly module loaded");
}