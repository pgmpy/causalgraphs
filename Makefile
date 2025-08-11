# Top-level Makefile for CausalGraphs project
PROJECT := causalgraphs
RUST_CORE := rust_core
PY_BINDINGS := python_bindings
WASM_BINDINGS := wasm_bindings
R_BINDINGS := r_bindings/causalgraphs

# Build targets
.PHONY: all core python wasm r install test format clean

all: core python wasm r

core:
	@echo "\n=== Building Rust Core ==="
	cd $(RUST_CORE) && cargo build --release

python: core
	@echo "\n=== Building Python Bindings ==="
	cd $(PY_BINDINGS) && \
	pip install -r requirements.txt && \
	maturin build --release --out target/wheels && \
	pip install target/wheels/*.whl

wasm: core
	@echo "\n=== Building WebAssembly Bindings ==="
	# ensure wasm-pack is in PATH (installs it 1st time only)
	@if ! command -v wasm-pack >/dev/null 2>&1; then \
	  echo "→ Installing wasm-pack…"; \
	  cargo install wasm-pack; \
	fi
	cd $(WASM_BINDINGS) && \
		npm install && \
		npm run build && \
		npm run build:node

r: core
	@echo "\n=== Building R Bindings ==="
	cd $(R_BINDINGS) && \
	Rscript -e "if(!require('devtools')) install.packages('devtools', repos='https://cloud.r-project.org')" && \
	Rscript -e "if(!require('rextendr')) install.packages('rextendr', repos='https://cloud.r-project.org')" && \
	Rscript -e "rextendr::document()"

install: python wasm r

test: test-core test-python test-wasm test-r

test-core:
	cd $(RUST_CORE) && cargo test

test-python:
	cd $(PY_BINDINGS) && pytest tests/

test-wasm:
	cd $(WASM_BINDINGS) && npm test

test-r:
	cd $(R_BINDINGS) && Rscript -e 'devtools::test()'

format:
	@echo "\n=== Formatting Code ==="
	cargo fmt --all

clean:
	@echo "\n=== Cleaning All Build Artifacts ==="
	cd $(RUST_CORE) && cargo clean
	cd $(PY_BINDINGS) && rm -rf target/ *.so
	cd $(WASM_BINDINGS) && rm -rf js/pkg-* node_modules
	cd $(R_BINDINGS) && rm -rf src/rust/target src/.cargo
