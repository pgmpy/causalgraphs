const cg = require("../pkg-node/causalgraphs_wasm.js");

// --- helpers ---
const sortStrings = (arr) => arr.slice().sort();
const sortPairs = (pairs) =>
  pairs
    .map(([a, b]) => [String(a), String(b)])
    .sort((p, q) => (p[0] === q[0] ? p[1].localeCompare(q[1]) : p[0].localeCompare(q[0])));

const latentsList = (dag) => {
  const v = dag.latents; // exposed as a getter property
  if (Array.isArray(v)) return sortStrings(v);
  if (v && typeof v === "object") {
    return sortStrings(Object.keys(v).filter((k) => v[k]));
  }
  return [];
};

describe("DAG wasm (CJS)", () => {
  it("should add nodes & edges (basic)", () => {
    const dag = new cg.DAG();
    dag.addNode("U");
    dag.addNode("V");
    dag.addEdge("U", "V");
    expect(dag.nodes()).toEqual(["U", "V"]);
    expect(dag.nodeCount).toBe(2);
    expect(dag.edges()).toEqual([["U", "V"]]);
    expect(dag.edgeCount).toBe(1);
  });

  it("addNode with optional latent flag; latents getter", () => {
    const dag = new cg.DAG();
    dag.addNode("A");
    dag.addNode("L", true);
    expect(sortStrings(dag.nodes())).toEqual(["A", "L"]);

    const lats = latentsList(dag);
    expect(lats).toContain("L");
    expect(lats).not.toContain("A");
  });

  it("addNodesFrom with optional latent mask (Uint8Array)", () => {
    const dag = new cg.DAG();
    dag.addNodesFrom(["X", "Y", "Z"], [true, false, true]);
    expect(sortStrings(dag.nodes())).toEqual(["X", "Y", "Z"]);

    const lats = latentsList(dag);
    expect(lats).toEqual(["X", "Z"]);
  });

  it("getParents and getChildren", () => {
    const dag = new cg.DAG();
    dag.addNodesFrom(["A", "B", "C", "D"]);
    dag.addEdge("A", "B");
    dag.addEdge("A", "C");
    dag.addEdge("B", "D");
    dag.addEdge("C", "D");

    expect(sortStrings(dag.getParents("D"))).toEqual(["B", "C"]);
    expect(sortStrings(dag.getChildren("A"))).toEqual(["B", "C"]);
  });

  it("getAncestorsOf for a single target", () => {
    const dag = new cg.DAG();
    dag.addNodesFrom(["A", "B", "C", "D"]);
    dag.addEdge("A", "B");
    dag.addEdge("A", "C");
    dag.addEdge("B", "D");
    dag.addEdge("C", "D");

    const ancD = sortStrings(dag.getAncestorsOf(["D"]));
    expect(ancD).toEqual(["A", "B", "C", "D"]);
  });

  it("getAncestorsOf for multiple targets", () => {
    const dag = new cg.DAG();
    dag.addNodesFrom(["A", "B", "C", "D", "E"]);
    dag.addEdge("A", "B");
    dag.addEdge("B", "C");
    dag.addEdge("A", "D");
    dag.addEdge("D", "E");

    const anc = sortStrings(dag.getAncestorsOf(["C", "E"]));
    expect(anc).toEqual(["A", "B", "C", "D", "E"]);
  });

  it("edges reflects added edges (order-insensitive)", () => {
    const dag = new cg.DAG();
    dag.addNodesFrom(["A", "B", "C"]);
    dag.addEdge("A", "B");
    dag.addEdge("B", "C");

    const expected = sortPairs([
      ["A", "B"],
      ["B", "C"],
    ]);
    const got = dag.edges();
    expect(Array.isArray(got)).toBe(true);

    const normalized = sortPairs(got.map((e) => (Array.isArray(e) ? e.slice(0, 2) : e)));
    expect(normalized).toEqual(expected);
  });

  it("addEdge can take an optional weight (graph relations still correct)", () => {
    const dag = new cg.DAG();
    dag.addNodesFrom(["S", "T"]);
    dag.addEdge("S", "T", 0.75);

    expect(dag.getParents("T")).toEqual(["S"]);
    expect(dag.getChildren("S")).toEqual(["T"]);
  });

  it("nodeCount / edgeCount track mutations", () => {
    const dag = new cg.DAG();
    expect(dag.nodeCount).toBe(0);
    expect(dag.edgeCount).toBe(0);

    dag.addNodesFrom(["A", "B", "C"]);
    expect(dag.nodeCount).toBe(3);

    dag.addEdge("A", "B");
    dag.addEdge("B", "C");
    expect(dag.edgeCount).toBe(2);
  });
});
