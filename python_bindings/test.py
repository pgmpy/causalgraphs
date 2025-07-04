import causalgraphs

# Instantiate your RustDAG object
dag = causalgraphs.RustDAG()

# Test adding nodes
dag.add_node("A")
dag.add_node("B", latent=True)
dag.add_nodes_from(["C", "D"], latent=[False, True])
print(f"Nodes: {dag.nodes()}")
# Expected output: Nodes: ['A', 'B', 'C', 'D'] (order may vary)

# Test adding edges
dag.add_edge("A", "B")
dag.add_edge("B", "C", weight=0.5)
print(f"Edges: {dag.edges()}")
# Expected output: Edges: [('A', 'B'), ('B', 'C')] (order may vary)

# Test graph properties
print(f"Node count: {dag.node_count()}") # Expected: 4
print(f"Edge count: {dag.edge_count()}") # Expected: 2

# Test methods
print(f"Parents of C: {dag.get_parents('C')}") # Expected: ['B']
print(f"Children of B: {dag.get_children('B')}") # Expected: ['C']

# Test ancestors (Rust-backed logic)
ancestors_of_C = dag.get_ancestors_of(["C"])
print(f"Ancestors of C: {ancestors_of_C}") # Expected: {'A', 'B', 'C'} (order may vary, depends on your ancestor definition)


#  create dag 2
dag2 = causalgraphs.RustDAG()
dag2.add_nodes_from(["V", "W", "X", "Y", "Z"])
dag2.add_edge("V", "X")
dag2.add_edge("X", "Y")
dag2.add_edge("X", "W")
dag2.add_edge("W", "Z")
dag2.add_edge("Y", "Z")
ancestorsOfZ = dag2.get_ancestors_of(['Z'])
print(f"Ancestors of Z: {ancestorsOfZ}")  # Expected: ['V', 'W', 'X', 'Y', 'Z'] (order may vary)