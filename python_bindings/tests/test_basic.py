import pytest
from causalgraphs import RustDAG

@pytest.fixture
def dag():
    d = RustDAG()
    d.add_node("X")
    d.add_node("Y")
    d.add_edge("X", "Y")
    return d

def test_nodes_and_edges(dag):
    assert set(dag.nodes()) == {"X", "Y"}
    assert dag.node_count() == 2
    assert dag.edge_count() == 1
    assert dag.edges() == [("X", "Y")]

def test_parents_children(dag):
    assert dag.get_parents("Y") == ["X"]
    assert dag.get_children("X") == ["Y"]
