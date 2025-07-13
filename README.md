# CausalGraphs

A cross-language library for working with causal graphs (DAGs) in Rust, Python, and WebAssembly.

## Structure

- **rust_core/**: Core Rust implementation of DAGs and causal graph logic.
- **python_bindings/**: Python bindings using [PyO3](https://github.com/PyO3/pyo3) and [maturin](https://github.com/PyO3/maturin).
- **wasm_bindings/**: WebAssembly bindings for use in JavaScript and Node.js environments via [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/) and [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/)
- **r_bindings/**: R bindings using [extendr](https://github.com/extendr/extendr).

## Prerequisites

To build and develop this project locally, you will need:
* [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
* [Python 3.x](https://www.python.org/downloads/) with `pip`
* [Node.js](https://nodejs.org/) (LTS recommended) with `npm`
* [R 4.2+](https://www.r-project.org/)
* [make](https://www.gnu.org/software/make/) (usually pre-installed on Linux/macOS, available via build tools on Windows)

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