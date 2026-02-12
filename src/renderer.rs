//! WebAssembly-powered rendering engine for ML workflow visualization

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use crate::{Node, Edge, Pipeline};

#[wasm_bindgen]
pub struct Renderer {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    scale: f64,
    offset_x: f64,
    offset_y: f64,
}

#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<Renderer, JsValue> {
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;
        
        Ok(Renderer {
            canvas,
            context,
            scale: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
        })
    }

    #[wasm_bindgen]
    pub fn clear(&self) {
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;
        self.context.clear_rect(0.0, 0.0, width, height);
        
        // Set background
        self.context.set_fill_style(&"#f8f9fa".into());
        self.context.fill_rect(0.0, 0.0, width, height);
    }

    #[wasm_bindgen]
    pub fn draw_node(&self, node: &Node, selected: bool) {
        let x = (node.x + self.offset_x) * self.scale;
        let y = (node.y + self.offset_y) * self.scale;
        let width = node.width * self.scale;
        let height = node.height * self.scale;

        // Node background
        let color = match node.node_type.as_str() {
            "data" => "#e3f2fd",
            "model" => "#f3e5f5",
            "training" => "#e8f5e8",
            "evaluation" => "#fff3e0",
            _ => "#ffffff",
        };
        
        self.context.set_fill_style(&color.into());
        self.context.fill_rect(x, y, width, height);
        
        // Border
        let border_color = if selected { "#2196f3" } else { "#cccccc" };
        let border_width = if selected { 3.0 } else { 1.0 };
        
        self.context.set_stroke_style(&border_color.into());
        self.context.set_line_width(border_width);
        self.context.stroke_rect(x, y, width, height);
        
        // Text
        self.context.set_fill_style(&"#333333".into());
        self.context.set_font("12px Arial");
        let text_x = x + width / 2.0;
        let text_y = y + height / 2.0 + 4.0;
        self.context.set_text_align("center");
        let _ = self.context.fill_text(&node.id, text_x, text_y);
    }

    #[wasm_bindgen]
    pub fn draw_edge(&self, edge: &Edge, source_node: &Node, target_node: &Node) {
        let start_x = (source_node.x + source_node.width + self.offset_x) * self.scale;
        let start_y = (source_node.y + source_node.height / 2.0 + self.offset_y) * self.scale;
        let end_x = (target_node.x + self.offset_x) * self.scale;
        let end_y = (target_node.y + target_node.height / 2.0 + self.offset_y) * self.scale;

        self.context.begin_path();
        self.context.move_to(start_x, start_y);
        
        // Bezier curve for smooth connection
        let control_x1 = start_x + 50.0;
        let control_y1 = start_y;
        let control_x2 = end_x - 50.0;
        let control_y2 = end_y;
        
        self.context.bezier_curve_to(control_x1, control_y1, control_x2, control_y2, end_x, end_y);
        
        self.context.set_stroke_style(&"#666666".into());
        self.context.set_line_width(2.0);
        self.context.stroke();
        
        // Arrow head
        let angle = (end_y - control_y2).atan2(end_x - control_x2);
        let arrow_length = 10.0;
        
        self.context.begin_path();
        self.context.move_to(end_x, end_y);
        self.context.line_to(
            end_x - arrow_length * (angle - 0.5).cos(),
            end_y - arrow_length * (angle - 0.5).sin()
        );
        self.context.line_to(
            end_x - arrow_length * (angle + 0.5).cos(),
            end_y - arrow_length * (angle + 0.5).sin()
        );
        self.context.close_path();
        self.context.set_fill_style(&"#666666".into());
        self.context.fill();
    }

    #[wasm_bindgen]
    pub fn set_zoom(&mut self, scale: f64) {
        self.scale = scale.max(0.1).min(3.0);
    }

    #[wasm_bindgen]
    pub fn set_pan(&mut self, offset_x: f64, offset_y: f64) {
        self.offset_x = offset_x;
        self.offset_y = offset_y;
    }
}