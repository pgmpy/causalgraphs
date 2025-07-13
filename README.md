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

## Supported Platforms

This library is actively developed and tested on:
- **Windows** (via WSL, or native MSVC toolchain for Rust + Rtools for R)
- **Linux** (Ubuntu, other distros)
- **macOS** 

## Quick Start

We provide a top‑level `Makefile` to save you typing:

- **Build everything** (Rust core + Python + WASM + R):

  ```sh
  make all
  ```

- **Run all tests**:

  ```sh
  make test
  ```

| Target        | What it does                                     |
| ------------- | ------------------------------------------------ |
| `make core`   | Builds only the `rust_core` crate.               |
| `make python` | Builds & installs Python bindings.               |
| `make wasm`   | Builds WASM modules for JS/Node via wasm-pack    |
| `make r`      | Generates R wrappers via `rextendr::document()`. |


## License

MIT
