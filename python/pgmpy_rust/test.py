from pgmpy_rust import RustDAG

# Drop-in replacement for performance-critical operations
dag = RustDAG([('A', 'B'), ('B', 'C'), ('D', 'C')])

# These operations are now much faster
# ancestors = dag._get_ancestors_of(['C'])
# separator = dag.minimal_dseparator('A', 'D')
# is_connected = dag.is_dconnected('A', 'D', ['B'])

print("dag:", dag.nodes())
print("edges: ", dag.edges())