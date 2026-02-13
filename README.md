# MLFlowChart

Interactive ML workflow visualization tool with drag-and-drop interface, real-time tracking, and automated documentation for machine learning pipelines.

## Features

- 🎯 **Drag-and-drop workflow builder** - Intuitive visual pipeline construction
- 📊 **Real-time ML pipeline visualization** - Live updates and monitoring
- 🔄 **Model iteration tracking** - Version control for ML experiments
- 📈 **Data flow monitoring** - Track data through your pipeline
- 📝 **Automated documentation generation** - Self-documenting workflows
- 🤖 **Integration with TensorFlow/PyTorch** - Connect to popular ML frameworks
- 📋 **Performance metrics dashboard** - Real-time performance insights
- ⚡ **WebAssembly-powered rendering engine** - High-performance visualization

## Tech Stack

- **Rust** - Core logic and WebAssembly module
- **TypeScript** - Frontend interface and interactions
- **WebAssembly** - High-performance rendering engine

## Quick Start

### Prerequisites

- Rust (latest stable)
- wasm-pack
- Node.js (for development server)

### Build WebAssembly Module

```bash
# Install wasm-pack if you haven't already
cargo install wasm-pack

# Build the WebAssembly module
wasm-pack build --target web --out-dir web/pkg
```

### Run Development Server

```bash
# Serve the web directory
cd web
python -m http.server 8000
# or use any static file server
```

Open http://localhost:8000 in your browser.

## Architecture

### Core Components

- **Pipeline** - Main data structure for ML workflows
- **Node** - Individual components (data sources, models, training steps)
- **Edge** - Connections between nodes representing data flow
- **Renderer** - WebAssembly-powered canvas rendering engine

### WebAssembly Integration

The Rust core is compiled to WebAssembly for high-performance rendering and pipeline management. The TypeScript frontend provides the user interface and handles DOM interactions.

## Usage

1. **Add Components** - Drag nodes from the sidebar or use toolbar buttons
2. **Connect Nodes** - Click and drag to create connections between components
3. **Configure Parameters** - Click nodes to set training parameters and data sources
4. **Run Pipeline** - Execute your ML workflow and monitor real-time metrics
5. **Export Results** - Save pipeline configurations and results

## Development

### Project Structure

```
mlflowchart/
├── src/
│   ├── lib.rs          # Main WebAssembly module
│   └── renderer.rs     # Canvas rendering engine
├── web/
│   ├── index.html      # Frontend interface
│   └── pkg/           # Generated WebAssembly bindings
├── Cargo.toml         # Rust dependencies
└── README.md
```

### Building

```bash
# Development build
wasm-pack build --dev --target web

# Production build
wasm-pack build --release --target web
```

### Testing

```bash
# Run Rust tests
cargo test

# Run WebAssembly tests
wasm-pack test --headless --firefox
```

## Security Considerations

### Input Validation

- **Node labels and parameters** must be sanitized before rendering to the DOM to prevent stored XSS — a malicious pipeline JSON with `<script>` in a node name will execute in the viewer's browser.
- **Pipeline configuration files** (JSON/YAML imports) should be validated against a strict schema before deserialization. Use `serde`'s `#[serde(deny_unknown_fields)]` on all config structs.
- **Edge connections** must be validated: reject self-loops, duplicate edges, and references to non-existent node IDs. Failing to do so can cause panics or undefined behavior in graph traversal.

### Pipeline Safety

- **Cycle detection** is mandatory before pipeline execution. A cyclic ML pipeline graph will cause infinite loops or stack overflows inside the WASM linear memory, crashing the entire tab.
- **Resource limits** — enforce maximum node count, edge count, and nesting depth to prevent denial-of-service via memory exhaustion in the WASM heap.

### WebAssembly Hardening

- Always enable `console_error_panic_hook` so panics produce actionable stack traces instead of opaque `unreachable` traps. Unhandled panics in WASM silently corrupt state.
- Use `#[wasm_bindgen]` with explicit typing; avoid passing raw pointers across the JS↔WASM boundary.
- Integer overflow in node/edge ID generation can cause ID collisions — use checked arithmetic or `u64` counters.

### Deployment

- Do **not** use `python -m http.server` in production — it lacks TLS, CORS controls, and security headers.
- Serve over HTTPS with these response headers:
  - `Content-Security-Policy: default-src 'self'; script-src 'self' 'wasm-unsafe-eval'`
  - `X-Content-Type-Options: nosniff`
  - `X-Frame-Options: DENY`

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests — include both happy-path and adversarial/malformed inputs
5. Run `cargo test` and `wasm-pack test --headless --firefox`
6. Submit a pull request with a clear description of your changess for new functionality
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Roadmap

- [ ] TensorFlow.js integration
- [ ] PyTorch model loading
- [ ] Advanced metrics visualization
- [ ] Collaborative editing
- [ ] Cloud deployment support
- [ ] Plugin system for custom nodes
