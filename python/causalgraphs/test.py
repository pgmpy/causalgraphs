from causalgraphs import RustDAG

# Drop-in replacement for performance-critical operations
dag = RustDAG([('A', 'B'), ('B', 'C'), ('D', 'C')])

print("dag:", dag.nodes())
print("edges: ", dag.edges())


# test get_children and parents
dag.add_node('E')
dag.add_edge('E', 'A')
print("Children of A:", dag.get_children('A'))
print("Parents of C:", dag.get_parents('C'))
print("Ancestors of C:", dag._get_ancestors_of('C'))