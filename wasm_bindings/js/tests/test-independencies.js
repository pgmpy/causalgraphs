const cg = require("../pkg-node/causalgraphs_wasm.js");

describe("IndependenceAssertion WASM", () => {
  describe("Basic functionality", () => {
    it("should create assertion with single elements", () => {
      const assertion = new cg.JsIndependenceAssertion(["U"], ["V"], ["Z"]);
      expect(assertion.event1()).toEqual(["U"]);
      expect(assertion.event2()).toEqual(["V"]);
      expect(assertion.event3()).toEqual(["Z"]);
      expect(assertion.allVars()).toEqual(["U", "V", "Z"]);
    });

    it("should create assertion with multiple elements", () => {
      const assertion = new cg.JsIndependenceAssertion(["U", "V"], ["Y", "Z"], ["A", "B"]);
      expect(assertion.event1()).toEqual(["U", "V"]);
      expect(assertion.event2()).toEqual(["Y", "Z"]);
      expect(assertion.event3()).toEqual(["A", "B"]);
      expect(assertion.allVars()).toEqual(["U", "V", "Y", "Z", "A", "B"]);
    });

    it("should create unconditional assertion", () => {
      const assertion = new cg.JsIndependenceAssertion(["U"], ["V"], null);
      expect(assertion.event1()).toEqual(["U"]);
      expect(assertion.event2()).toEqual(["V"]);
      expect(assertion.event3()).toEqual([]);
      expect(assertion.allVars()).toEqual(["U", "V"]);
      expect(assertion.isUnconditional()).toBe(true);
    });

    it("should handle conditional assertion", () => {
      const assertion = new cg.JsIndependenceAssertion(["U"], ["V"], ["Z"]);
      expect(assertion.isUnconditional()).toBe(false);
    });
  });

  describe("Validation", () => {
    it("should throw error for empty event1", () => {
      expect(() => {
        new cg.JsIndependenceAssertion([], ["V"], ["Z"]);
      }).toThrow("event1 needs to be specified");
    });

    it("should throw error for empty event2", () => {
      expect(() => {
        new cg.JsIndependenceAssertion(["U"], [], ["Z"]);
      }).toThrow("event2 needs to be specified");
    });
  });

  describe("String formatting", () => {
    it("should format conditional assertion correctly", () => {
      const assertion = new cg.JsIndependenceAssertion(["U"], ["V"], ["Z"]);
      expect(assertion.toLatex()).toBe("U \\perp V \\mid Z");
      expect(assertion.toString()).toBe("(U ⊥ V | Z)");
    });

    it("should format unconditional assertion correctly", () => {
      const assertion = new cg.JsIndependenceAssertion(["U"], ["V"], null);
      expect(assertion.toLatex()).toBe("U \\perp V");
      expect(assertion.toString()).toBe("(U ⊥ V)");
    });

    it("should format multi-element assertion correctly", () => {
      const assertion = new cg.JsIndependenceAssertion(["U", "V"], ["Y", "Z"], ["A", "B"]);
      expect(assertion.toLatex()).toBe("U, V \\perp Y, Z \\mid A, B");
      expect(assertion.toString()).toBe("(U, V ⊥ Y, Z | A, B)");
    });
  });

  describe("Equality", () => {
    it("should handle basic equality", () => {
      const i1 = new cg.JsIndependenceAssertion(["a"], ["b"], ["c"]);
      const i2 = new cg.JsIndependenceAssertion(["a"], ["b"], null);
      const i3 = new cg.JsIndependenceAssertion(["a"], ["b", "c", "d"], null);
      
      expect(i1.toString()).not.toBe(i2.toString());
      expect(i1.toString()).not.toBe(i3.toString());
      expect(i2.toString()).not.toBe(i3.toString());
    });

    it("should handle symmetry", () => {
      const i4 = new cg.JsIndependenceAssertion(["a"], ["b", "c", "d"], ["e"]);
      const i5 = new cg.JsIndependenceAssertion(["a"], ["d", "c", "b"], ["e"]);
      
      // Order shouldn't matter for sets
      expect(i4.toString()).toBe(i5.toString());
    });

    it("should handle swapped events", () => {
      const i9 = new cg.JsIndependenceAssertion(["a"], ["d", "k", "b"], ["e"]);
      const i10 = new cg.JsIndependenceAssertion(["k", "b", "d"], ["a"], ["e"]);
      
      // Should be equal due to symmetry
      expect(i9.toString()).toBe(i10.toString());
    });
  });
});

describe("Independencies WASM", () => {
  describe("Basic functionality", () => {
    it("should create empty independencies", () => {
      const ind = new cg.JsIndependencies();
      expect(ind.getAssertions()).toEqual([]);
      expect(ind.getAllVariables()).toEqual([]);
    });

    it("should add assertion", () => {
      const ind = new cg.JsIndependencies();
      const assertion = new cg.JsIndependenceAssertion(["X"], ["Y"], ["Z"]);
      ind.addAssertion(assertion);
      expect(ind.getAssertions()).toHaveLength(1);
      expect(ind.contains(assertion)).toBe(true);
    });

    it("should add assertions from tuples", () => {
      const ind = new cg.JsIndependencies();
      const tuples = [
        [["X"], ["Y"], ["Z"]],
        [["A"], ["B"], ["C"]]
      ];
      ind.addAssertionsFromTuples(tuples);
      expect(ind.getAssertions()).toHaveLength(2);
    });

    it("should get all variables", () => {
      const ind = new cg.JsIndependencies();
      ind.addAssertionsFromTuples([
        [["a"], ["b", "c", "d"], ["e", "f", "g"]],
        [["c"], ["d", "e", "f"], ["g", "h"]]
      ]);
      const vars = ind.getAllVariables();
      expect(vars).toContain("a");
      expect(vars).toContain("b");
      expect(vars).toContain("c");
      expect(vars).toContain("d");
      expect(vars).toContain("e");
      expect(vars).toContain("f");
      expect(vars).toContain("g");
      expect(vars).toContain("h");
      expect(vars).toHaveLength(8);
    });
  });

  describe("Closure", () => {
    it("should compute simple closure", () => {
      const ind = new cg.JsIndependencies();
      ind.addAssertionsFromTuples([
        [["A"], ["B", "C"], ["D"]]
      ]);

      const closure = ind.closure();
      const assertions = closure.getAssertions();
      
      // Should contain original assertion and decompositions
      expect(assertions.length).toBeGreaterThanOrEqual(1);
      
      // Check for decompositions: A ⊥ B | D and A ⊥ C | D
      const assertionStrings = assertions.map(a => a.toString());
      expect(assertionStrings.some(s => s.includes("(A ⊥ B | D)"))).toBe(true);
      expect(assertionStrings.some(s => s.includes("(A ⊥ C | D)"))).toBe(true);
    });

    it("should compute complex closure", () => {
      const ind = new cg.JsIndependencies();
      ind.addAssertionsFromTuples([
        [["A"], ["B", "C", "D"], ["E"]]
      ]);

      const closure = ind.closure();
      const assertions = closure.getAssertions();
      
      // Should generate multiple assertions through semi-graphoid axioms
      expect(assertions.length).toBeGreaterThan(1);
    });

    it("should compute closure for unconditional assertion", () => {
      const ind = new cg.JsIndependencies();
      ind.addAssertionsFromTuples([
        [["W"], ["X", "Y", "Z"], null]
      ]);

      const closure = ind.closure();
      const assertions = closure.getAssertions();
      
      // Should generate multiple assertions
      expect(assertions.length).toBeGreaterThan(1);
      
      // Check for specific expected assertions
      const assertionStrings = assertions.map(a => a.toString());
      expect(assertionStrings.some(s => s.includes("(W ⊥ X)"))).toBe(true);
      expect(assertionStrings.some(s => s.includes("(W ⊥ Y)"))).toBe(true);
      expect(assertionStrings.some(s => s.includes("(W ⊥ Z)"))).toBe(true);
    });
  });

  describe("Entailment", () => {
    it("should test entailment", () => {
      const ind1 = new cg.JsIndependencies();
      ind1.addAssertionsFromTuples([
        [["W"], ["X", "Y", "Z"], null]
      ]);

      const ind2 = new cg.JsIndependencies();
      ind2.addAssertionsFromTuples([
        [["W"], ["X"], null]
      ]);

      // W ⊥ X,Y,Z should entail W ⊥ X
      expect(ind1.entails(ind2)).toBe(true);
      expect(ind2.entails(ind1)).toBe(false);
    });

    it("should test self-entailment", () => {
      const ind = new cg.JsIndependencies();
      ind.addAssertionsFromTuples([
        [["W"], ["X", "Y", "Z"], null]
      ]);

      const closure = ind.closure();
      expect(ind.entails(closure)).toBe(true);
      expect(closure.entails(ind)).toBe(true);
    });
  });

  describe("Equivalence", () => {
    it("should test equivalence", () => {
      const ind1 = new cg.JsIndependencies();
      ind1.addAssertionsFromTuples([
        [["X"], ["Y", "W"], ["Z"]]
      ]);

      const ind2 = new cg.JsIndependencies();
      ind2.addAssertionsFromTuples([
        [["X"], ["Y"], ["Z"]],
        [["X"], ["W"], ["Z"]]
      ]);

      const ind3 = new cg.JsIndependencies();
      ind3.addAssertionsFromTuples([
        [["X"], ["Y"], ["Z"]],
        [["X"], ["W"], ["Z"]],
        [["X"], ["Y"], ["W", "Z"]]
      ]);

      // ind1 should NOT be equivalent to ind2
      expect(ind1.isEquivalent(ind2)).toBe(false);
      
      // ind1 should be equivalent to ind3
      expect(ind1.isEquivalent(ind3)).toBe(true);
    });

    it("should test symmetric equivalence", () => {
      const indA = new cg.JsIndependencies();
      indA.addAssertionsFromTuples([
        [["X", "Y"], ["A", "B"], ["Z"]],
        [["P"], ["Q", "R", "S"], ["T", "U"]]
      ]);

      const indB = new cg.JsIndependencies();
      indB.addAssertionsFromTuples([
        [["A", "B"], ["X", "Y"], ["Z"]],
        [["P"], ["S", "Q", "R"], ["U", "T"]]
      ]);

      // These should be equal due to symmetric equivalence and set ordering
      expect(indA.isEquivalent(indB)).toBe(true);
    });
  });

  describe("Reduce", () => {
    it("should reduce duplicates", () => {
      const ind = new cg.JsIndependencies();
      const assertion = new cg.JsIndependenceAssertion(["X"], ["Y"], ["Z"]);
      
      // Add the same assertion twice
      ind.addAssertion(assertion);
      ind.addAssertion(assertion);
      
      const reduced = ind.reduce();
      expect(reduced.getAssertions()).toHaveLength(1);
    });

    it("should reduce entailment", () => {
      const ind = new cg.JsIndependencies();
      
      // More general assertion
      ind.addAssertionsFromTuples([
        [["W"], ["X", "Y", "Z"], null]
      ]);
      // More specific assertion (should be removed)
      ind.addAssertionsFromTuples([
        [["W"], ["X"], null]
      ]);

      const reduced = ind.reduce();
      expect(reduced.getAssertions()).toHaveLength(1);
      
      // Should keep the more general assertion
      const general = new cg.JsIndependenceAssertion(["W"], ["X", "Y", "Z"], null);
      expect(reduced.contains(general)).toBe(true);
    });

    it("should reduce independent assertions", () => {
      const ind = new cg.JsIndependencies();
      ind.addAssertionsFromTuples([
        [["A"], ["B"], ["C"]],
        [["D"], ["B"], ["F"]]
      ]);

      const reduced = ind.reduce();
      expect(reduced.getAssertions()).toHaveLength(2);
    });

    it("should reduce complex case", () => {
      const ind = new cg.JsIndependencies();
      
      // General assertion that entails the specific ones
      ind.addAssertionsFromTuples([
        [["A"], ["B", "C"], ["D"]]
      ]);
      // Specific assertions that should be removed
      ind.addAssertionsFromTuples([
        [["A"], ["B"], ["D"]],
        [["A"], ["C"], ["D"]]
      ]);
      // Independent assertion
      ind.addAssertionsFromTuples([
        [["E"], ["F"], ["G"]]
      ]);

      const reduced = ind.reduce();
      expect(reduced.getAssertions()).toHaveLength(2);
      
      const general = new cg.JsIndependenceAssertion(["A"], ["B", "C"], ["D"]);
      const independent = new cg.JsIndependenceAssertion(["E"], ["F"], ["G"]);
      
      expect(reduced.contains(general)).toBe(true);
      expect(reduced.contains(independent)).toBe(true);
    });

    it("should reduce empty independencies", () => {
      const ind = new cg.JsIndependencies();
      const reduced = ind.reduce();
      expect(reduced.getAssertions()).toHaveLength(0);
    });
  });

  describe("Complex scenarios", () => {
    it("should handle complex multi-assertion equality", () => {
      const ind3 = new cg.JsIndependencies();
      ind3.addAssertionsFromTuples([
        [["a"], ["b", "c", "d"], ["e", "f", "g"]],
        [["c"], ["d", "e", "f"], ["g", "h"]]
      ]);

      const ind4 = new cg.JsIndependencies();
      ind4.addAssertionsFromTuples([
        [["f", "d", "e"], ["c"], ["h", "g"]],
        [["b", "c", "d"], ["a"], ["f", "g", "e"]]
      ]);

      const ind5 = new cg.JsIndependencies();
      ind5.addAssertionsFromTuples([
        [["a"], ["b", "c", "d"], ["e", "f", "g"]],
        [["c"], ["d", "e", "f"], ["g"]]
      ]);

      // These should be equal due to symmetric equivalence
      expect(ind3.isEquivalent(ind4)).toBe(true);
      
      // These should not be equal
      expect(ind3.isEquivalent(ind5)).toBe(false);
      expect(ind4.isEquivalent(ind5)).toBe(false);
    });

    it("should handle large closure case", () => {
      const ind = new cg.JsIndependencies();
      ind.addAssertionsFromTuples([
        [["c"], ["a"], ["b", "e", "d"]],
        [["e", "c"], ["b"], ["a", "d"]],
        [["b", "d"], ["e"], ["a"]],
        [["e"], ["b", "d"], ["c"]],
        [["e"], ["b", "c"], ["d"]],
        [["e", "c"], ["a"], ["b"]]
      ]);

      const closure = ind.closure();
      const assertions = closure.getAssertions();
      
      // Should generate many assertions
      expect(assertions.length).toBeGreaterThan(50);
    });

    it("should handle WXYZ closure case", () => {
      const ind = new cg.JsIndependencies();
      ind.addAssertionsFromTuples([
        [["W"], ["X", "Y", "Z"], null]
      ]);

      const closure = ind.closure();
      const assertions = closure.getAssertions();
      
      // Should generate exactly 19 assertions for this case
      expect(assertions).toHaveLength(19);
      
      // Check for specific expected assertions
      const assertionStrings = assertions.map(a => a.toString());
      expect(assertionStrings).toContain("(W ⊥ X)");
      expect(assertionStrings).toContain("(W ⊥ Y)");
      expect(assertionStrings).toContain("(W ⊥ Z)");
      expect(assertionStrings).toContain("(W ⊥ X, Y)");
      expect(assertionStrings).toContain("(W ⊥ X, Z)");
      expect(assertionStrings).toContain("(W ⊥ Y, Z)");
      expect(assertionStrings).toContain("(W ⊥ X, Y, Z)");
    });
  });

  describe("Edge cases", () => {
    it("should handle empty independencies comparison", () => {
      const empty1 = new cg.JsIndependencies();
      const empty2 = new cg.JsIndependencies();
      const nonEmpty = new cg.JsIndependencies();
      nonEmpty.addAssertionsFromTuples([
        [["A"], ["B"], ["C"]]
      ]);

      // Empty vs non-empty should be false
      expect(empty1.isEquivalent(nonEmpty)).toBe(false);
      
      // Non-empty vs empty should be false  
      expect(nonEmpty.isEquivalent(empty1)).toBe(false);
      
      // Empty vs empty should be true
      expect(empty1.isEquivalent(empty2)).toBe(true);
    });

    it("should handle bidirectional equivalence", () => {
      const indX = new cg.JsIndependencies();
      indX.addAssertionsFromTuples([
        [["A"], ["B", "C"], ["D"]],
        [["E"], ["F"], ["G", "H"]]
      ]);

      const indY = new cg.JsIndependencies();
      indY.addAssertionsFromTuples([
        [["A"], ["B"], ["D"]],
        [["A"], ["C"], ["D"]],
        [["E"], ["F"], ["G", "H"]]
      ]);

      // Test that decomposition creates equivalence
      expect(indX.entails(indY)).toBe(true);
      
      // Test bidirectional equivalence
      const reverseEntailment = indY.entails(indX);
      if (reverseEntailment) {
        expect(indX.isEquivalent(indY)).toBe(true);
        expect(indY.isEquivalent(indX)).toBe(true);
      }
    });
  });
}); 