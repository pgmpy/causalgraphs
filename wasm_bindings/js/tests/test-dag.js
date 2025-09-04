const cg = require("../pkg-node/causalgraphs_wasm.js");

describe("DAG wasm (CJS)", () => {
  it("should add nodes & edges", () => {
    const dag = new cg.DAG();
    dag.addNode("U");
    dag.addNode("V");
    dag.addEdge("U","V");
    expect(dag.nodes()).toEqual(["U","V"]);
    expect(dag.nodeCount).toBe(2);
    expect(dag.edges()).toEqual([["U","V"]]);
    expect(dag.edgeCount).toBe(1);
  });

  it("should check if nodes are d-connected (basic, connected)", () => {
    const dag = new cg.DAG();
    dag.addEdge("A", "B");
    dag.addEdge("B", "C");
    const connected = dag.isDconnected("A", "C");
    expect(connected).toBe(true);  // A -> B -> C is d-connected
  });
  
  it("should check if nodes are d-connected (with observed, disconnected)", () => {
    const dag = new cg.DAG();
    dag.addEdge("A", "B");
    dag.addEdge("B", "C");
    const connected = dag.isDconnected("A", "C", ["B"]);  // Observed B blocks the path
    expect(connected).toBe(false);
  });
  
  it("should check if nodes are neighbors (adjacent)", () => {
    const dag = new cg.DAG();
    dag.addEdge("A", "B");
    const areNeighbors = dag.areNeighbors("A", "B");
    expect(areNeighbors).toBe(true);
  });
  
  it("should check if nodes are neighbors (non-adjacent)", () => {
    const dag = new cg.DAG();
    dag.addEdge("A", "B");
    dag.addEdge("B", "C");
    const areNeighbors = dag.areNeighbors("A", "C");
    expect(areNeighbors).toBe(false);
  });

  it("should compute minimal d-separator (simple)", () => {
    const dag = new cg.DAG();
    dag.addEdge("A", "B");
    dag.addEdge("B", "C");
    const sep = dag.minimalDseparator("A", "C");
    expect(sep.sort()).toEqual(["B"]);
  });

  it("should compute minimal d-separator (complex)", () => {
    const dag = new cg.DAG();
    dag.addEdge("A", "B");
    dag.addEdge("B", "C");
    dag.addEdge("C", "D");
    dag.addEdge("A", "E");
    dag.addEdge("E", "D");
    const sep = dag.minimalDseparator("A", "D");
    expect(sep.sort()).toEqual(["C", "E"]);
  });

  it("should return null for minimal d-separator if none exists (latent)", () => {
    const dag = new cg.DAG();
    dag.addNode("A", false);
    dag.addNode("B", true); // latent
    dag.addNode("C", false);
    dag.addEdge("A", "B");
    dag.addEdge("B", "C");
    const sep = dag.minimalDseparator("A", "C");
    expect(sep).toBeNull();
  });

  it("should compute active trail nodes (basic)", () => {
    const dag = new cg.DAG();
    dag.addEdge("diff", "grades");
    dag.addEdge("intel", "grades");
    const result = dag.activeTrailNodes(["diff"]);
    expect(result["diff"]).toEqual(["diff", "grades"].sort());
  });

  it("should compute active trail nodes with observed", () => {
    const dag = new cg.DAG();
    dag.addEdge("diff", "grades");
    dag.addEdge("intel", "grades");
    const result = dag.activeTrailNodes(["diff", "intel"], ["grades"]);
    expect(result["diff"].sort()).toEqual(["diff", "intel"].sort());
    expect(result["intel"].sort()).toEqual(["diff", "intel"].sort());
  });
});
