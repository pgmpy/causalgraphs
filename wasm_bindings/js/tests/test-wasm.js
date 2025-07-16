const cg = require("../pkg-node/causalgraphs_wasm.js");

describe("RustDAG wasm (CJS)", () => {
  it("should add nodes & edges", () => {
    const dag = new cg.RustDAG();
    dag.addNode("U");
    dag.addNode("V");
    dag.addEdge("U","V");
    expect(dag.nodes()).toEqual(["U","V"]);
    expect(dag.nodeCount).toBe(2);
    expect(dag.edges()).toEqual([["U","V"]]);
    expect(dag.edgeCount).toBe(1);
  });
});
