# causalgraphs
Rust implemention of causal graphs


## Building & Installing the Python Extension

First, make sure you have Rust installed on your system. If you haven't already done so, try following the instructions [here](https://www.rust-lang.org/tools/install).


This project uses [PyO3](https://pyo3.rs/v0.25.1/) to create Python bindings for the Rust code.
We use [Maturin](https://github.com/PyO3/maturin) build tool to compile(.so/.dll) our Rust core into a native Python module(.whl) and install it into a local venv. 

1. **Clone the repo**  
   ```bash
   git clone https://github.com/pgmpy/causalgraphs.git
   cd causalgraphs
   ```

2. **Create & activate a venv**  
   ```bash
    cd python
    python3 -m venv .venv
    source .venv/bin/activate
   ```
3. **Install Python requirements**  
   ```bash
    pip install -r requirements.txt
   ```
4. **Build & install the Rust extension**  
   From the project root (where Cargo.toml and pyproject.toml live):
      ```bash
      maturin build --release
      ```
   This will compile the Rust code and generate a .whl file in the target/wheels/ directory. You can then install it:

      ```bash
      pip install target/wheels/causalgraphs-0.1.0-*.whl
      ```

      (Replace your_package_name-0.1.0-cp3x-cp3x-your_platform.whl with the actual filename.)

      ```bash
      Note: The `causalgraphs` package is not yet published on PyPI. You must install it from the locally built wheel as shown above while the project is in active development.
      ```
---

5. **🚀 Usage**

   Once installed, you can import the `DAG` from `causalgraphs` and use it just like a regular Python class.
      ```bash
      >>> from causalgraphs import RustDAG
      >>> dag = RustDAG()
      >>> dag.add_node("A")
      >>> dag.nodes()
      ["A"]
      ```
