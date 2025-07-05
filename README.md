# CausalGraphs

A cross-language library for working with causal graphs (DAGs) in Rust, Python, and WebAssembly.

## Structure

- **rust_core/**: Core Rust implementation of DAGs and causal graph logic.
- **python_bindings/**: Python bindings using [PyO3](https://github.com/PyO3/pyo3) and [maturin](https://github.com/PyO3/maturin).
- **wasm_bindings/**: WebAssembly bindings for use in JavaScript/Node.js.

## Quick Start

### Rust

```sh
cd rust_core
cargo test
```

### Python

```sh
cd python_bindings
maturin develop
python -c "import causalgraphs; print(dir(causalgraphs))"
```

### WebAssembly (Node.js)

```sh
cd wasm_bindings
npm install
npm run build
npm run test
```

## License

MIT