# MLFlowChart

Interactive ML workflow visualization tool with drag-and-drop interface, real-time tracking, and automated documentation for machine learning pipelines.

## Features

- 🎯 **Drag-and-drop workflow builder** - Intuitive visual pipeline construction
- 📊 **Real-time ML pipeline visualization** - Live updates and monitoring
- 🔄 **Model iteration tracking** - Version control for ML experiments
- 📈 **Data flow monitoring** - Track data through your pipeline with throughput & latency metrics
- 🛡️ **Data quality gates** - Schema validation, null-rate checks, and skew detection per edge
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

## Pipeline Configuration Schema

Pipelines are defined as directed acyclic graphs (DAGs). Each node declares its type, inputs, and parameters:

```json
{
  "pipeline_id": "train-v2",
  "nodes": [
    {
      "id": "src_1",
      "type": "data_source",
      "config": { "format": "parquet", "path": "s3://bucket/features/" }
    },
    {
      "id": "validate_1",
      "type": "schema_validator",
      "config": { "null_threshold": 0.01, "expect_columns": ["user_id", "feature_vec"] }
    },
    {
      "id": "train_1",
      "type": "model_train",
      "config": { "framework": "pytorch", "epochs": 10, "batch_size": 256 }
    }
  ],
  "edges": [
    { "from": "src_1", "to": "validate_1" },
    { "from": "validate_1", "to": "train_1" }
  ]
}
```

## Data Flow Monitoring & Metrics

Each edge in the pipeline tracks real-time throughput metrics:

| Metric | Description | Unit |
|--------|-------------|------|
| `records_per_sec` | Row throughput at each edge | rows/s |
| `bytes_per_sec` | Data volume throughput | bytes/s |
| `p50_latency_ms` | Median per-record processing latency | ms |
| `p99_latency_ms` | Tail latency (99th percentile) | ms |
| `backpressure_ratio` | Consumer lag / producer rate | 0.0–1.0 |
| `null_rate` | Fraction of null values per column | 0.0–1.0 |

Metrics are sampled every 500ms and exposed via the dashboard. When `backpressure_ratio > 0.8` or `null_rate` exceeds the configured threshold, the node border turns red to flag potential data quality issues or pipeline bottlenecks.

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

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests — run `cargo test` and `wasm-pack test --headless --firefox`
5. Ensure `cargo clippy` passes with no warnings
6. Submit a pull request with a clear description of the change

### Contribution Ideas

- **Data connectors** — Add nodes for Kafka, BigQuery, Delta Lake, or other data sources
- **Schema inference** — Auto-detect column types from upstream data
- **Pipeline benchmarks** — Measure WASM rendering FPS with 50 / 200 / 1000 nodes
- **Export formats** — Airflow DAG, Kubeflow Pipeline, or Argo Workflow YAML exports for new functionality
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
