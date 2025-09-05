const cg = require("../pkg-node/causalgraphs_wasm.js");


describe('cg.PDAG', () => {
    it('can be instantiated', () => {
        const pdag = new cg.PDAG();
        expect(pdag.nodeCount).toBe(0);
        expect(pdag.edgeCount).toBe(0);
    });

    it('can add nodes and edges', () => {
        const pdag = new cg.PDAG();
        pdag.addNode('A');
        pdag.addNode('B');
        pdag.addNode('C');
        pdag.addEdge('A', 'B', null, true); // A -> B
        pdag.addEdge('B', 'C', null, false); // B - C

        expect(pdag.nodeCount).toBe(3);
        expect(pdag.edgeCount).toBe(2);

        const directedEdges = new Set(pdag.directedEdges().map(e => e.join(',')));
        expect(directedEdges).toEqual(new Set(['A,B']));

        const undirectedEdges = new Set(pdag.undirectedEdges().map(e => e.sort().join(',')));
        expect(undirectedEdges).toEqual(new Set(['B,C']));
    });

    it('can add multiple edges from a list', () => {
        const pdag = new cg.PDAG();
        const directed = [['A', 'B'], ['D', 'C']];
        const undirected = [['B', 'C']];
        pdag.addEdgesFrom(directed, null, true);
        pdag.addEdgesFrom(undirected, null, false);

        expect(pdag.nodeCount).toBe(4);
        expect(pdag.edgeCount).toBe(3);
        expect(pdag.nodes().sort()).toEqual(['A', 'B', 'C', 'D']);
    });

    it("applies Meek's rules correctly (basic case)", () => {
        const pdag = new cg.PDAG();
        pdag.addEdge('A', 'B', null, true); // A -> B
        pdag.addEdge('B', 'C', null, false); // B - C

        const cpdag = pdag.applyMeeksRules(true, false);
        const expectedEdges = new Set(['A,B', 'B,C']);
        const actualEdges = new Set(cpdag.edges().map(e => e.join(',')));

        expect(actualEdges).toEqual(expectedEdges);
    });

    it("applies Meek's rules correctly (no change)", () => {
        const pdag = new cg.PDAG();
        pdag.addEdgesFrom([['A', 'B'], ['D', 'C']], null, true);
        pdag.addEdgesFrom([['B', 'C']], null, false);

        const cpdag = pdag.applyMeeksRules(true, false);
        
        // Expect B-C to remain undirected
        const directed = new Set(cpdag.directedEdges().map(e => e.join(',')));
        const undirected = new Set(cpdag.undirectedEdges().map(e => e.sort().join(',')));

        expect(directed).toEqual(new Set(['A,B', 'D,C']));
        expect(undirected).toEqual(new Set(['B,C']));
    });

    it('converts to a DAG', () => {
        const pdag = new cg.PDAG();
        pdag.addEdge('A', 'B', null, true);
        pdag.addEdge('B', 'C', null, false);

        const dag = pdag.toDag();
        expect(dag.constructor.name).toBe('DAG');
        
        const dagEdges = new Set(dag.edges().map(e => e.join(',')));
        // to_dag is consistent, so B-C will be oriented B->C in this case
        const expectedEdges = new Set(['A,B', 'B,C']);
        expect(dagEdges).toEqual(expectedEdges);
    });
});