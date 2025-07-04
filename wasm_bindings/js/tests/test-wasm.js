/**
 * Node.js test for CausalGraphs WASM
 */

import * as causalgraphs from '../pkg-node/causalgraphs_wasm.js';

async function testWasm() {
    console.log('🚀 Testing CausalGraphs WASM...\n');

    try {
        // No need to call init for node target!
        // Create a new DAG
        const dag = new causalgraphs.RustDAG();
        console.log('✅ DAG created successfully!\n');

        // Add some nodes
        dag.addNode('A');
        dag.addNode('B');
        dag.addNode('C');
        console.log('Added nodes: A, B, C');

        // Add edges
        dag.addEdge('A', 'B');
        dag.addEdge('B', 'C');
        console.log('Added edges: A→B, B→C\n');

        // Get graph information
        const nodes = dag.nodes();
        const edges = dag.edges();
        const nodeCount = dag.nodeCount;
        const edgeCount = dag.edgeCount;

        console.log(`Nodes: ${nodes.join(', ')}`);
        console.log(`Edges: ${JSON.stringify(edges)}`);
        console.log(`Node count: ${nodeCount}`);
        console.log(`Edge count: ${edgeCount}\n`);

        // Test graph traversal
        const parentsOfC = dag.getParents('C');
        const childrenOfA = dag.getChildren('A');
        const ancestorsOfC = dag.getAncestorsOf(['C']);

        console.log(`Parents of C: ${parentsOfC.join(', ')}`);
        console.log(`Children of A: ${childrenOfA.join(', ')}`);
        console.log(`Ancestors of C: ${ancestorsOfC.join(', ')}\n`);

        // Test with latent variables
        dag.addNode('L', true); // Add latent node
        dag.addEdge('L', 'A');
        console.log('Added latent node L → A');

        // Test more complex graph
        const dag2 = new causalgraphs.RustDAG();
        const nodeNames = ['X', 'Y', 'Z', 'W', 'V'];
        dag2.addNodesFrom(nodeNames);

        dag2.addEdge('X', 'Y');
        dag2.addEdge('Y', 'Z');
        dag2.addEdge('X', 'W');
        dag2.addEdge('W', 'Z');
        dag2.addEdge('V', 'X');

        console.log('\nCreated complex graph:');
        console.log('V → X → Y → Z');
        console.log('    ↓   ↑');
        console.log('    W → Z\n');

        const ancestorsOfZ = dag2.getAncestorsOf(['Z']);
        const parentsOfZ = dag2.getParents('Z');
        const childrenOfX = dag2.getChildren('X');

        console.log(`Ancestors of Z: ${ancestorsOfZ.join(', ')}`);
        console.log(`Parents of Z: ${parentsOfZ.join(', ')}`);
        console.log(`Children of X: ${childrenOfX.join(', ')}\n`);

        console.log('🎉 All tests passed!');

    } catch (error) {
        console.error('❌ Error:', error);
        process.exit(1);
    }
}

// Run the test
testWasm();