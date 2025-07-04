# Python Bindings for CausalGraphs

This package provides Python bindings for the Rust `causalgraphs` library using [PyO3](https://github.com/PyO3/pyo3).

## Setup (Recommended)

It is recommended to use a virtual environment for Python development:

```sh
python3 -m venv .venv
source .venv/bin/activate
```

If you have a `requirements.txt`, install dependencies:

```sh
pip install -r requirements.txt
```

## Build & Install

Install [maturin](https://github.com/PyO3/maturin) if you haven't already:

```sh
pip install maturin
```

Then build and install the Rust extension in your environment:

```sh
maturin develop
```

## Usage Example

```python
from causalgraphs import RustDAG

dag = RustDAG()
dag.add_node("A")
dag.add_node("B")
dag.add_edge("A", "B")
print(dag.nodes())
print(dag.edges())
```

## Development Notes

- Edit Rust code in `src/`.
- Run `maturin develop` to rebuild the Python extension.
- Run your Python tests as needed.

## License