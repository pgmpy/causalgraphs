import pytest
from causalgraphs import DAG

class TestDAG:
    @pytest.fixture
    def dag(self):
        d = DAG()
        d.add_node("X")
        d.add_node("Y")
        d.add_edge("X", "Y")
        return d

    def test_nodes_and_edges(self, dag):
        assert set(dag.nodes()) == {"X", "Y"}
        assert dag.node_count() == 2
        assert dag.edge_count() == 1
        assert dag.edges() == [("X", "Y")]

    def test_parents_children(self, dag):
        assert dag.get_parents("Y") == ["X"]
        assert dag.get_children("X") == ["Y"]

    def test_minimal_dseparator(self):
        # Test case: A → B → C
        dag1 = DAG()
        dag1.add_edges_from([("A", "B"), ("B", "C")])
        assert dag1.minimal_dseparator(["A"], ["C"]) == {"B"}

        # Test case: A → B → C, C → D, A → E, E → D
        dag2 = DAG()
        dag2.add_edges_from([("A", "B"), ("B", "C"), ("C", "D"), ("A", "E"), ("E", "D")])
        assert dag2.minimal_dseparator(["A"], ["D"]) == {"C", "E"}

        # Test case: B → A, B → C, A → D, D → C, A → E, C → E
        dag3 = DAG()
        dag3.add_edges_from([("B", "A"), ("B", "C"), ("A", "D"), ("D", "C"), ("A", "E"), ("C", "E")])
        assert dag3.minimal_dseparator(["A"], ["C"]) == {"B", "D"}

        # Test with latents
        dag_lat1 = DAG()
        dag_lat1.add_nodes_from(["A", "B", "C"], latent=[False, True, False])
        dag_lat1.add_edges_from([("A", "B"), ("B", "C")])
        assert dag_lat1.minimal_dseparator(["A"], ["C"]) is None
        # assert dag_lat1.minimal_dseparator(["A"], ["C"], include_latents=True) == {"B"}

        dag_lat2 = DAG()
        dag_lat2.add_nodes_from(["A", "B", "C", "D"], latent=[False, True, False, False])
        dag_lat2.add_edges_from([("A", "D"), ("D", "B"), ("B", "C")])
        assert dag_lat2.minimal_dseparator(["A"], ["C"]) == {"D"}

        dag_lat3 = DAG()
        dag_lat3.add_nodes_from(["A", "B", "C", "D"], latent=[False, True, False, False])
        dag_lat3.add_edges_from([("A", "B"), ("B", "D"), ("D", "C")])
        assert dag_lat3.minimal_dseparator(["A"], ["C"]) == {"D"}

        dag_lat4 = DAG()
        dag_lat4.add_nodes_from(["A", "B", "C", "D"], latent=[False, False, False, True])
        dag_lat4.add_edges_from([("A", "B"), ("B", "C"), ("A", "D"), ("D", "C")])
        assert dag_lat4.minimal_dseparator(["A"], ["C"]) is None

        # Test adjacent nodes (should raise error)
        dag5 = DAG()
        dag5.add_edges_from([("A", "B")])
        with pytest.raises(ValueError, match="No possible separators because A and B are adjacent"):
            dag5.minimal_dseparator(["A"], ["B"])