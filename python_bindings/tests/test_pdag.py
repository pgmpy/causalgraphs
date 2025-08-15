import unittest
from causalgraphs import PDAG, DAG

class TestPDAG(unittest.TestCase):
    def setUp(self):
        # PDAG with mixed directed and undirected edges
        self.pdag_mix = PDAG()
        self.pdag_mix.add_edges_from([("A", "C"), ("D", "C")], directed=True)
        self.pdag_mix.add_edges_from([("B", "A"), ("B", "D")], directed=False)

        # PDAG with only directed edges
        self.pdag_dir = PDAG()
        self.pdag_dir.add_edges_from([("A", "B"), ("D", "B"), ("A", "C"), ("D", "C")], directed=True)

        # PDAG with only undirected edges
        self.pdag_undir = PDAG()
        self.pdag_undir.add_edges_from([("A", "C"), ("D", "C"), ("B", "A"), ("B", "D")], directed=False)

        # PDAG with latents
        self.pdag_latent = PDAG()
        self.pdag_latent.add_nodes_from(["A", "B", "C", "D"], latent=[True, False, False, True])
        self.pdag_latent.add_edges_from([("A", "C"), ("D", "C")], directed=True)
        self.pdag_latent.add_edges_from([("B", "A"), ("B", "D")], directed=False)

    def test_init_normal(self):
        # Mix directed and undirected
        pdag = PDAG()
        directed_edges = [("A", "C"), ("D", "C")]
        undirected_edges = [("B", "A"), ("B", "D")]
        pdag.add_edges_from(directed_edges, directed=True)
        pdag.add_edges_from(undirected_edges, directed=False)
        expected_edges = {("A", "C"), ("D", "C"), ("A", "B"), ("B", "A"), ("B", "D"), ("D", "B")}
        self.assertEqual(set(pdag.edges()), expected_edges)
        self.assertEqual(set(pdag.nodes()), {"A", "B", "C", "D"})
        self.assertEqual(set(pdag.directed_edges()), set(directed_edges))
        self.assertEqual(set(pdag.undirected_edges()), set(undirected_edges))

        # Mix with latents
        pdag = PDAG()
        pdag.add_nodes_from(["A", "B", "C", "D"], latent=[True, False, True, False])
        pdag.add_edges_from(directed_edges, directed=True)
        pdag.add_edges_from(undirected_edges, directed=False)
        self.assertEqual(set(pdag.edges()), expected_edges)
        self.assertEqual(set(pdag.nodes()), {"A", "B", "C", "D"})
        self.assertEqual(set(pdag.directed_edges()), set(directed_edges))
        self.assertEqual(set(pdag.undirected_edges()), set(undirected_edges))
        self.assertEqual(set(pdag.latents), {"A", "C"})

        # Only undirected
        pdag = PDAG()
        undirected_edges = [("A", "C"), ("D", "C"), ("B", "A"), ("B", "D")]
        pdag.add_edges_from(undirected_edges, directed=False)
        expected_edges = {("A", "C"), ("C", "A"), ("D", "C"), ("C", "D"), ("B", "A"), ("A", "B"), ("B", "D"), ("D", "B")}
        self.assertEqual(set(pdag.edges()), expected_edges)
        self.assertEqual(set(pdag.nodes()), {"A", "B", "C", "D"})
        self.assertEqual(set(pdag.directed_edges()), set())
        self.assertEqual(set(pdag.undirected_edges()), set(undirected_edges))

        # Only undirected with latents
        pdag = PDAG()
        pdag.add_nodes_from(["A", "B", "C", "D"], latent=[True, False, False, True])
        pdag.add_edges_from(undirected_edges, directed=False)
        self.assertEqual(set(pdag.edges()), expected_edges)
        self.assertEqual(set(pdag.nodes()), {"A", "B", "C", "D"})
        self.assertEqual(set(pdag.directed_edges()), set())
        self.assertEqual(set(pdag.undirected_edges()), set(undirected_edges))
        self.assertEqual(set(pdag.latents), {"A", "D"})

        # Only directed
        pdag = PDAG()
        directed_edges = [("A", "B"), ("D", "B"), ("A", "C"), ("D", "C")]
        pdag.add_edges_from(directed_edges, directed=True)
        self.assertEqual(set(pdag.edges()), set(directed_edges))
        self.assertEqual(set(pdag.nodes()), {"A", "B", "C", "D"})
        self.assertEqual(set(pdag.directed_edges()), set(directed_edges))
        self.assertEqual(set(pdag.undirected_edges()), set())

        # Only directed with latents
        pdag = PDAG()
        pdag.add_nodes_from(["A", "B", "C", "D"], latent=[False, False, False, True])
        pdag.add_edges_from(directed_edges, directed=True)
        self.assertEqual(set(pdag.edges()), set(directed_edges))
        self.assertEqual(set(pdag.nodes()), {"A", "B", "C", "D"})
        self.assertEqual(set(pdag.directed_edges()), set(directed_edges))
        self.assertEqual(set(pdag.undirected_edges()), set())
        self.assertEqual(set(pdag.latents), {"D"})

    def test_all_neighbors(self):
        pdag = self.pdag_mix
        self.assertEqual(set(pdag.all_neighbors("A")), {"B", "C"})
        self.assertEqual(set(pdag.all_neighbors("B")), {"A", "D"})
        self.assertEqual(set(pdag.all_neighbors("C")), {"A", "D"})
        self.assertEqual(set(pdag.all_neighbors("D")), {"B", "C"})

    def test_directed_children(self):
        pdag = self.pdag_mix
        self.assertEqual(set(pdag.directed_children("A")), {"C"})
        self.assertEqual(set(pdag.directed_children("B")), set())
        self.assertEqual(set(pdag.directed_children("C")), set())
        self.assertEqual(set(pdag.directed_children("D")), {"C"})

    def test_directed_parents(self):
        pdag = self.pdag_mix
        self.assertEqual(set(pdag.directed_parents("A")), set())
        self.assertEqual(set(pdag.directed_parents("B")), set())
        self.assertEqual(set(pdag.directed_parents("C")), {"A", "D"})
        self.assertEqual(set(pdag.directed_parents("D")), set())

    def test_has_directed_edge(self):
        pdag = self.pdag_mix
        self.assertTrue(pdag.has_directed_edge("A", "C"))
        self.assertTrue(pdag.has_directed_edge("D", "C"))
        self.assertFalse(pdag.has_directed_edge("A", "B"))
        self.assertFalse(pdag.has_directed_edge("B", "A"))

    def test_has_undirected_edge(self):
        pdag = self.pdag_mix
        self.assertFalse(pdag.has_undirected_edge("A", "C"))
        self.assertFalse(pdag.has_undirected_edge("D", "C"))
        self.assertTrue(pdag.has_undirected_edge("A", "B"))
        self.assertTrue(pdag.has_undirected_edge("B", "A"))
        self.assertTrue(pdag.has_undirected_edge("B", "D"))

    def test_undirected_neighbors(self):
        pdag = self.pdag_mix
        self.assertEqual(set(pdag.undirected_neighbors("A")), {"B"})
        self.assertEqual(set(pdag.undirected_neighbors("B")), {"A", "D"})
        self.assertEqual(set(pdag.undirected_neighbors("C")), set())
        self.assertEqual(set(pdag.undirected_neighbors("D")), {"B"})

    def test_orient_undirected_edge(self):
        pdag = self.pdag_mix.copy()
        mod_pdag = pdag.orient_undirected_edge("B", "A", inplace=False)
        self.assertEqual(
            set(mod_pdag.edges()),
            {("A", "C"), ("D", "C"), ("B", "A"), ("B", "D"), ("D", "B")}
        )
        self.assertEqual(set(mod_pdag.undirected_edges()), {("B", "D")})
        self.assertEqual(set(mod_pdag.directed_edges()), {("A", "C"), ("D", "C"), ("B", "A")})

        pdag.orient_undirected_edge("B", "A", inplace=True)
        self.assertEqual(
            set(pdag.edges()),
            {("A", "C"), ("D", "C"), ("B", "A"), ("B", "D"), ("D", "B")}
        )
        self.assertEqual(set(pdag.undirected_edges()), {("B", "D")})
        self.assertEqual(set(pdag.directed_edges()), {("A", "C"), ("D", "C"), ("B", "A")})

        with self.assertRaises(ValueError):
            pdag.orient_undirected_edge("B", "A", inplace=True)

    def test_copy(self):
        pdag_copy = self.pdag_mix.copy()
        expected_edges = {("A", "C"), ("D", "C"), ("A", "B"), ("B", "A"), ("B", "D"), ("D", "B")}
        expected_dir = [("A", "C"), ("D", "C")]
        expected_undir = [("B", "A"), ("B", "D")]
        self.assertEqual(set(pdag_copy.edges()), expected_edges)
        self.assertEqual(set(pdag_copy.nodes()), {"A", "B", "C", "D"})
        self.assertEqual(set(pdag_copy.directed_edges()), set(expected_dir))
        self.assertEqual(set(pdag_copy.undirected_edges()), set(expected_undir))
        self.assertEqual(set(pdag_copy.latents), set())

        pdag_copy = self.pdag_latent.copy()
        self.assertEqual(set(pdag_copy.edges()), expected_edges)
        self.assertEqual(set(pdag_copy.nodes()), {"A", "B", "C", "D"})
        self.assertEqual(set(pdag_copy.directed_edges()), set(expected_dir))
        self.assertEqual(set(pdag_copy.undirected_edges()), set(expected_undir))
        self.assertEqual(set(pdag_copy.latents), {"A", "D"})

    def test_pdag_to_dag(self):
        # PDAG no: 1 - Possibility of creating a v-structure
        pdag = PDAG()
        pdag.add_edges_from([("A", "B"), ("C", "B")], directed=True)
        pdag.add_edges_from([("C", "D"), ("D", "A")], directed=False)
        dag = pdag.to_dag()
        self.assertTrue(("A", "B") in dag.edges())
        self.assertTrue(("C", "B") in dag.edges())
        self.assertFalse(("A", "D") in dag.edges() and ("C", "D") in dag.edges())
        self.assertEqual(len(dag.edges()), 4)

        # With latents
        pdag = PDAG()
        pdag.add_nodes_from(["A", "B", "C", "D"], latent=[True, False, False, False])
        pdag.add_edges_from([("A", "B"), ("C", "B")], directed=True)
        pdag.add_edges_from([("C", "D"), ("D", "A")], directed=False)
        dag = pdag.to_dag()
        self.assertTrue(("A", "B") in dag.edges())
        self.assertTrue(("C", "B") in dag.edges())
        self.assertFalse(("A", "D") in dag.edges() and ("C", "D") in dag.edges())
        self.assertEqual(set(dag.latents), {"A"})
        self.assertEqual(len(dag.edges()), 4)

        # PDAG no: 2 - No possibility of creating a v-structure
        pdag = PDAG()
        pdag.add_edges_from([("B", "C"), ("A", "C")], directed=True)
        pdag.add_edges_from([("A", "D")], directed=False)
        dag = pdag.to_dag()
        self.assertTrue(("B", "C") in dag.edges())
        self.assertTrue(("A", "C") in dag.edges())
        self.assertTrue(("A", "D") in dag.edges() or ("D", "A") in dag.edges())

        # With latents
        pdag = PDAG()
        pdag.add_nodes_from(["A", "B", "C", "D"], latent=[True, False, False, False])
        pdag.add_edges_from([("B", "C"), ("A", "C")], directed=True)
        pdag.add_edges_from([("A", "D")], directed=False)
        dag = pdag.to_dag()
        self.assertTrue(("B", "C") in dag.edges())
        self.assertTrue(("A", "C") in dag.edges())
        self.assertTrue(("A", "D") in dag.edges() or ("D", "A") in dag.edges())
        self.assertEqual(set(dag.latents), {"A"})

        # PDAG no: 3 - Already existing v-structure, possibility to add another
        pdag = PDAG()
        pdag.add_edges_from([("B", "C"), ("A", "C")], directed=True)
        pdag.add_edges_from([("C", "D")], directed=False)
        dag = pdag.to_dag()
        expected_edges = {("B", "C"), ("C", "D"), ("A", "C")}
        self.assertEqual(set(dag.edges()), expected_edges)

        # With latents
        pdag = PDAG()
        pdag.add_nodes_from(["A", "B", "C", "D"], latent=[True, False, False, False])
        pdag.add_edges_from([("B", "C"), ("A", "C")], directed=True)
        pdag.add_edges_from([("C", "D")], directed=False)
        dag = pdag.to_dag()
        self.assertEqual(set(dag.edges()), expected_edges)
        self.assertEqual(set(dag.latents), {"A"})

    def test_pdag_to_cpdag(self):
        # Test case 1
        pdag = PDAG()
        pdag.add_edges_from([("A", "B")], directed=True)
        pdag.add_edges_from([("B", "C")], directed=False)
        cpdag = pdag.apply_meeks_rules(apply_r4=True, inplace=False)
        self.assertEqual(set(cpdag.edges()), {("A", "B"), ("B", "C")})

        # Test case 2
        pdag = PDAG()
        pdag.add_edges_from([("A", "B")], directed=True)
        pdag.add_edges_from([("B", "C"), ("C", "D")], directed=False)
        cpdag = pdag.apply_meeks_rules(apply_r4=True, inplace=False)
        self.assertEqual(set(cpdag.edges()), {("A", "B"), ("B", "C"), ("C", "D")})

        # Test case 3
        pdag = PDAG()
        pdag.add_edges_from([("A", "B"), ("D", "C")], directed=True)
        pdag.add_edges_from([("B", "C")], directed=False)
        cpdag = pdag.apply_meeks_rules(apply_r4=True, inplace=False)
        self.assertEqual(set(cpdag.edges()), {("A", "B"), ("D", "C"), ("B", "C"), ("C", "B")})

        # Test case 4
        pdag = PDAG()
        pdag.add_edges_from([("A", "B"), ("D", "C"), ("D", "B")], directed=True)
        pdag.add_edges_from([("B", "C")], directed=False)
        cpdag = pdag.apply_meeks_rules(apply_r4=True, inplace=False)
        self.assertEqual(set(cpdag.edges()), {("A", "B"), ("D", "C"), ("D", "B"), ("B", "C")})

        # Test case 5
        pdag = PDAG()
        pdag.add_edges_from([("A", "B"), ("B", "C")], directed=True)
        pdag.add_edges_from([("A", "C")], directed=False)
        cpdag = pdag.apply_meeks_rules(apply_r4=True, inplace=False)
        self.assertEqual(set(cpdag.edges()), {("A", "B"), ("B", "C"), ("A", "C")})

        # Test case 6
        pdag = PDAG()
        pdag.add_edges_from([("A", "B"), ("B", "C"), ("D", "C")], directed=True)
        pdag.add_edges_from([("A", "C")], directed=False)
        cpdag = pdag.apply_meeks_rules(apply_r4=True, inplace=False)
        self.assertEqual(set(cpdag.edges()), {("A", "B"), ("B", "C"), ("A", "C"), ("D", "C")})

        # Perković 2017 example
        pdag = PDAG()
        pdag.add_edges_from([("V1", "X")], directed=True)
        pdag.add_edges_from([("X", "V2"), ("V2", "Y"), ("X", "Y")], directed=False)
        cpdag = pdag.apply_meeks_rules(apply_r4=True, inplace=False)
        self.assertEqual(
            set(cpdag.edges()),
            {("V1", "X"), ("X", "V2"), ("X", "Y"), ("V2", "Y"), ("Y", "V2")}
        )

        # Perković 2017 example with reversed direction
        pdag = PDAG()
        pdag.add_edges_from([("Y", "X")], directed=True)
        pdag.add_edges_from([("V1", "X"), ("X", "V2"), ("V2", "Y")], directed=False)
        cpdag = pdag.apply_meeks_rules(apply_r4=True, inplace=False)
        self.assertEqual(
            set(cpdag.edges()),
            {("X", "V1"), ("Y", "X"), ("X", "V2"), ("V2", "X"), ("V2", "Y"), ("Y", "V2")}
        )

        # Bang 2024 example
        pdag = PDAG()
        pdag.add_edges_from([("B", "D"), ("C", "D")], directed=True)
        pdag.add_edges_from([("A", "D"), ("A", "C")], directed=False)
        cpdag = pdag.apply_meeks_rules(apply_r4=True, inplace=False)
        self.assertEqual(
            set(cpdag.edges()), {("B", "D"), ("D", "A"), ("C", "A"), ("C", "D")}
        )

        # Bang 2024 example with multiple undirected edges
        pdag = PDAG()
        pdag.add_edges_from([("A", "B"), ("C", "B")], directed=True)
        pdag.add_edges_from([("D", "B"), ("D", "A"), ("D", "C")], directed=False)
        cpdag = pdag.apply_meeks_rules(apply_r4=True, inplace=False)
        self.assertEqual(
            set(cpdag.edges()),
            {("A", "B"), ("C", "B"), ("D", "B"), ("D", "A"), ("A", "D"), ("D", "C"), ("C", "D")}
        )

        # Test with inplace=True and apply_r4=False
        undirected_edges = [("A", "C"), ("B", "C"), ("D", "C")]
        directed_edges = [("B", "D"), ("D", "A")]
        pdag = PDAG()
        harnessing = pdag.add_edges_from(directed_edges, directed=True)
        pdag.add_edges_from(undirected_edges, directed=False)
        pdag_inp = pdag.copy()
        pdag_inp.apply_meeks_rules(apply_r4=False, inplace=True)
        self.assertEqual(
            set(pdag_inp.edges()),
            {("A", "C"), ("C", "A"), ("C", "B"), ("B", "C"), ("B", "D"), ("D", "A"), ("D", "C"), ("C", "D")}
        )