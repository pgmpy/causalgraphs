# WASM Bindings for CausalGraphs

WebAssembly (WASM) bindings for the Rust `causalgraphs` library, usable from JavaScript and Node.js.

## Build

```sh
npm run build
```

## Test

```sh
npm run test
```

## Usage Example

```js
import * as causalgraphs from './pkg-node/causalgraphs_wasm.js';

const dag = new causalgraphs.RustDAG();
dag.addNode('A');
dag.addNode('B');
dag.addEdge('A', 'B');
console.log(dag.nodes());
console.log(dag.edges());
```

## Development

- Rust source: `../rust_core`
- WASM bindings: `src/`
- JS tests: `js/tests/`