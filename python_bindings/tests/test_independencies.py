import pytest
from causalgraphs import IndependenceAssertion, Independencies, RustDAG
from typing import List, Optional, Set, Dict
from collections import defaultdict


class TestIndependenceAssertion:
    def setup_method(self):
        self.assertion = IndependenceAssertion(["U"], ["V"], ["Z"])

    def test_init(self):
        # Test basic initialization with single elements
        assertion = IndependenceAssertion(["U"], ["V"], ["Z"])
        print(assertion.event1, assertion.event2, assertion.event3, assertion.all_vars)
        assert set(assertion.event1) == {"U"}
        assert set(assertion.event2) == {"V"}
        assert set(assertion.event3) == {"Z"}
        assert set(assertion.all_vars) == {"U", "V", "Z"}

        # Test initialization with multiple elements
        assertion = IndependenceAssertion(["U", "V"], ["Y", "Z"], ["A", "B"])
        assert set(assertion.event1) == {"U", "V"}
        assert set(assertion.event2) == {"Y", "Z"}
        assert set(assertion.event3) == {"A", "B"}
        assert set(assertion.all_vars) == {"U", "V", "Y", "Z", "A", "B"}

        # Test unconditional assertion
        assertion = IndependenceAssertion(["U"], ["V"], None)
        assert set(assertion.event1) == {"U"}
        assert set(assertion.event2) == {"V"}
        assert set(assertion.event3) == set()
        assert set(assertion.all_vars) == {"U", "V"}

    def test_init_exceptions(self):
        # Test missing event1
        with pytest.raises(ValueError, match="event1 needs to be specified"):
            IndependenceAssertion([], ["U"], ["V"])

        # Test missing event2
        with pytest.raises(ValueError, match="event2 needs to be specified"):
            IndependenceAssertion(["U"], [], ["V"])

    def test_is_unconditional(self):
        assert not IndependenceAssertion(["U"], ["V"], ["Z"]).is_unconditional()
        assert IndependenceAssertion(["U"], ["V"], None).is_unconditional()

    def test_to_latex(self):
        assert IndependenceAssertion(["U"], ["V"], ["Z"]).to_latex() == "U \\perp V \\mid Z"
        assert IndependenceAssertion(["U"], ["V"], None).to_latex() == "U \\perp V"
        assert (
            IndependenceAssertion(["U", "V"], ["Y", "Z"], ["A", "B"]).to_latex()
            == "U, V \\perp Y, Z \\mid A, B"
        )

    def test_to_string(self):
        assert str(IndependenceAssertion(["U"], ["V"], ["Z"])) == "(U ⊥ V | Z)"
        assert str(IndependenceAssertion(["U"], ["V"], None)) == "(U ⊥ V)"
        assert (
            str(IndependenceAssertion(["U", "V"], ["Y", "Z"], ["A", "B"]))
            == "(U, V ⊥ Y, Z | A, B)"
        )

    def test_eq(self):
        i1 = IndependenceAssertion(["a"], ["b"], ["c"])
        i2 = IndependenceAssertion(["a"], ["b"], None)
        i3 = IndependenceAssertion(["a"], ["b", "c", "d"])
        i4 = IndependenceAssertion(["a"], ["b", "c", "d"], ["e"])
        i5 = IndependenceAssertion(["a"], ["d", "c", "b"], ["e"])
        i6 = IndependenceAssertion(["a"], ["c", "d"], ["e", "b"])
        i7 = IndependenceAssertion(["a"], ["d", "c"], ["b", "e"])
        i8 = IndependenceAssertion(["a"], ["f", "d"], ["b", "e"])
        i9 = IndependenceAssertion(["a"], ["d", "k", "b"], ["e"])
        i10 = IndependenceAssertion(["k", "b", "d"], ["a"], ["e"])

        # Test inequality with non-assertion types
        assert i1 != "a"
        assert i2 != 1
        assert i4 != [2, "a"]
        assert i6 != "c"

        # Test inequality between different assertions
        assert i1 != i2
        assert i1 != i3
        assert i2 != i4
        assert i3 != i6

        # Test equality with symmetric and reordered assertions
        assert i4 == i5
        assert i6 == i7
        assert i7 != i8
        assert i4 != i9
        assert i5 != i9
        assert i10 == i9
        assert i10 != i8


class TestIndependencies:
    def setup_method(self):
        self.independencies = Independencies()
        self.ind3 = Independencies()
        self.ind3.add_assertions_from_tuples(
            [
                (["a"], ["b", "c", "d"], ["e", "f", "g"]),
                (["c"], ["d", "e", "f"], ["g", "h"]),
            ]
        )
        self.ind4 = Independencies()
        self.ind4.add_assertions_from_tuples(
            [
                (["f", "d", "e"], ["c"], ["h", "g"]),
                (["b", "c", "d"], ["a"], ["f", "g", "e"]),
            ]
        )
        self.ind5 = Independencies()
        self.ind5.add_assertions_from_tuples(
            [
                (["a"], ["b", "c", "d"], ["e", "f", "g"]),
                (["c"], ["d", "e", "f"], ["g"]),
            ]
        )

    def test_init(self):
        ind1 = Independencies()
        ind1.add_assertions_from_tuples([(["X"], ["Y"], ["Z"])])

        ind2 = Independencies()
        ind2.add_assertions_from_tuples([(["X"], ["Y"], ["Z"])])
        
        assert ind1 == ind2  # Compare two equivalent objects

        ind3 = Independencies()
        assert ind3 == Independencies()

    def test_get_assertions(self):
        ind1 = Independencies()
        ind1.add_assertions_from_tuples([(["X"], ["Y"], ["Z"])])
        assertions = ind1.get_assertions()
        assert len(assertions) == 1
        assert set(assertions[0].event1) == {"X"}
        assert set(assertions[0].event2) == {"Y"}
        assert set(assertions[0].event3) == {"Z"}

        ind2 = Independencies()
        ind2.add_assertions_from_tuples([(["A"], ["B"], ["C"]), (["D"], ["E"], ["F"])])
        assertions = ind2.get_assertions()
        assert len(assertions) == 2
        assert set(assertions[0].event1) == {"A"}
        assert set(assertions[0].event2) == {"B"}
        assert set(assertions[0].event3) == {"C"}
        assert set(assertions[1].event1) == {"D"}
        assert set(assertions[1].event2) == {"E"}
        assert set(assertions[1].event3) == {"F"}

    def test_get_all_variables(self):
        assert set(self.ind3.get_all_variables()) == {"a", "b", "c", "d", "e", "f", "g", "h"}
        assert set(self.ind4.get_all_variables()) == {"a", "b", "c", "d", "e", "f", "g", "h"}
        assert set(self.ind5.get_all_variables()) == {"a", "b", "c", "d", "e", "f", "g"}

    def test_closure(self):
        ind1 = Independencies()
        ind1.add_assertions_from_tuples([(["A"], ["B", "C"], ["D"])])
        closure = ind1.closure()
        expected = Independencies()
        expected.add_assertions_from_tuples(
            [
                (["A"], ["B", "C"], ["D"]),
                (["A"], ["B"], ["C", "D"]),
                (["A"], ["C"], ["B", "D"]),
                (["A"], ["B"], ["D"]),
                (["A"], ["C"], ["D"]),
            ]
        )
        assert closure == expected

        ind2 = Independencies()
        ind2.add_assertions_from_tuples([(["W"], ["X", "Y", "Z"], None)])
        closure = ind2.closure()
        expected = Independencies()
        expected.add_assertions_from_tuples(
            [
                (["W"], ["Y"], None),
                (["W"], ["Y"], ["X"]),
                (["W"], ["Y"], ["Z"]),
                (["W"], ["Y"], ["X", "Z"]),
                (["W"], ["X", "Y"], None),
                (["W"], ["X"], ["Y", "Z"]),
                (["W"], ["X", "Z"], ["Y"]),
                (["W"], ["X"], None),
                (["W"], ["X", "Z"], None),
                (["W"], ["Y", "Z"], ["X"]),
                (["W"], ["X", "Y", "Z"], None),
                (["W"], ["X"], ["Z"]),
                (["W"], ["Y", "Z"], None),
                (["W"], ["Z"], ["X"]),
                (["W"], ["Z"], None),
                (["W"], ["Y", "X"], ["Z"]),
                (["W"], ["X"], ["Y"]),
                (["W"], ["Z"], ["Y", "X"]),
                (["W"], ["Z"], ["Y"]),
            ]
        )
        assert closure == expected

        ind3 = Independencies()
        ind3.add_assertions_from_tuples(
            [
                (["c"], ["a"], ["b", "e", "d"]),
                (["e", "c"], ["b"], ["a", "d"]),
                (["b", "d"], ["e"], ["a"]),
                (["e"], ["b", "d"], ["c"]),
                (["e"], ["b", "c"], ["d"]),
                (["e", "c"], ["a"], ["b"]),
            ]
        )
        assert len(ind3.closure().get_assertions()) == 78

    def test_entails(self):
        ind1 = Independencies()
        ind1.add_assertions_from_tuples([(["A", "B"], ["C", "D"], ["E"])])
        ind2 = Independencies()
        ind2.add_assertions_from_tuples([(["A"], ["C"], ["E"])])
        assert ind1.entails(ind2)
        assert not ind2.entails(ind1)

        ind3 = Independencies()
        ind3.add_assertions_from_tuples([(["W"], ["X", "Y", "Z"], None)])
        assert ind3.entails(ind3.closure())
        assert ind3.closure().entails(ind3)

    def test_is_equivalent(self):
        ind1 = Independencies()
        ind1.add_assertions_from_tuples([(["X"], ["Y", "W"], ["Z"])])
        ind2 = Independencies()
        ind2.add_assertions_from_tuples([(["X"], ["Y"], ["Z"]), (["X"], ["W"], ["Z"])])
        ind3 = Independencies()
        ind3.add_assertions_from_tuples(
            [(["X"], ["Y"], ["Z"]), (["X"], ["W"], ["Z"]), (["X"], ["Y"], ["W", "Z"])]
        )
        assert not ind1.is_equivalent(ind2)
        assert ind1.is_equivalent(ind3)

    def test_eq(self):
        assert self.ind3 == self.ind4
        assert not (self.ind3 != self.ind4)
        assert self.ind3 != self.ind5
        assert self.ind4 != self.ind5
        assert Independencies() != Independencies().add_assertions_from_tuples(
            [(["A"], ["B"], ["C"])]
        )
        assert Independencies().add_assertions_from_tuples([(["A"], ["B"], ["C"])]) != Independencies()
        assert Independencies() == Independencies()

    def test_reduce(self):
        ind1 = Independencies()
        ind1.add_assertions_from_tuples([(["X"], ["Y"], ["Z"]), (["X"], ["Y"], ["Z"])])
        reduced = ind1.reduce()
        assert len(reduced.get_assertions()) == 1
        assert reduced.get_assertions() == reduced.independencies 
        assert set(reduced.get_assertions()[0].event1) == {"X"}
        assert set(reduced.get_assertions()[0].event2) == {"Y"}
        assert set(reduced.get_assertions()[0].event3) == {"Z"}

        ind2 = Independencies()
        ind2.add_assertions_from_tuples([(["A"], ["B"], ["C"]), (["D"], ["E"], ["F"])])
        reduced = ind2.reduce()
        assertions = reduced.get_assertions()
        assert len(assertions) == 2
        assert IndependenceAssertion(["A"], ["B"], ["C"]) in assertions
        assert IndependenceAssertion(["D"], ["E"], ["F"]) in assertions

        ind3 = Independencies()
        ind3.add_assertions_from_tuples([(["W"], ["X", "Y", "Z"], None), (["W"], ["X"], ["Y"])])
        reduced = ind3.reduce()
        assertions = reduced.get_assertions()
        assert len(assertions) == 1
        assert IndependenceAssertion(["W"], ["X", "Y", "Z"], None) in assertions

        ind4 = Independencies()
        ind4.add_assertions_from_tuples(
            [
                (["A"], ["B", "C"], ["D"]),
                (["A"], ["B"], ["D"]),
                (["A"], ["C"], ["D"]),
                (["E"], ["F"], ["G"]),
            ]
        )
        reduced = ind4.reduce()
        assert len(ind4.get_assertions()) == 4
        assertions = reduced.get_assertions()
        assert len(assertions) == 2
        assert IndependenceAssertion(["A"], ["B", "C"], ["D"]) in assertions
        assert IndependenceAssertion(["E"], ["F"], ["G"]) in assertions

        ind5 = Independencies()
        ind5.add_assertions_from_tuples([(["X"], ["Y"], ["Z"]), (["X"], ["Y"], ["Z"]), (["A"], ["B"], ["C"])])
        original_len = len(ind5.get_assertions())
        ind5.reduce(inplace=True)
        assert len(ind5.get_assertions()) == 2
        assert original_len != len(ind5.get_assertions())
        assert IndependenceAssertion(["X"], ["Y"], ["Z"]) in ind5.get_assertions()
        assert IndependenceAssertion(["A"], ["B"], ["C"]) in ind5.get_assertions()

        ind6 = Independencies()
        reduced = ind6.reduce()
        assert len(reduced.get_assertions()) == 0

        ind7 = Independencies()
        ind7.add_assertions_from_tuples([(["X"], ["Y"], ["Z"])])
        reduced = ind7.reduce()
        assertions = reduced.get_assertions()
        assert len(assertions) == 1
        assert IndependenceAssertion(["X"], ["Y"], ["Z"]) in assertions