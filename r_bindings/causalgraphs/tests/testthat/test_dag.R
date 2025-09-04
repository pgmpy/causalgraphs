library(testthat)
library(causalgraphs)

test_that("basic DAG operations", {
  dag <- RDAG$new()
  dag$add_node("A", FALSE)
  dag$add_node("B", FALSE)
  dag$add_edge("A", "B", 20)

  expect_setequal(dag$nodes(), c("A", "B"))
  expect_equal(dag$node_count(), 2L)
  expect_equal(dag$edge_count(), 1L)

  expect_equal(dag$get_parents("B"), "A")
  expect_equal(dag$get_children("A"), "B")

  e <- dag$edges()
  expect_true(is.list(e) && all(c("from", "to") %in% names(e)))
  expect_equal(length(e$from), 1L)
  expect_equal(paste0(e$from, "->", e$to), "A->B")
})

test_that("add_nodes_from with and without latent mask", {
  # With latent mask
  dag1 <- RDAG$new()
  dag1$add_nodes_from(c("X", "Y", "Z"), c(TRUE, FALSE, TRUE))
  expect_setequal(dag1$nodes(), c("X", "Y", "Z"))
  expect_setequal(dag1$latents(), c("X", "Z"))

  # Without latent mask (all observed). Pass NULL explicitly.
  dag2 <- RDAG$new()
  dag2$add_nodes_from(c("A", "B", "C"), NULL)
  expect_setequal(dag2$nodes(), c("A", "B", "C"))
  expect_length(dag2$latents(), 0L)
})

test_that("add_node defaults latent=FALSE and duplicate adds are no-ops", {
  dag <- RDAG$new()
  dag$add_node("L", FALSE)
  dag$add_node("L", TRUE)
  expect_setequal(dag$nodes(), "L")
  expect_length(dag$latents(), 0L)
  expect_equal(dag$node_count(), 1L)
})

test_that("add_edge auto-adds missing nodes; optional weight works", {
  dag <- RDAG$new()

  dag$add_edge("S", "T", NULL)
  expect_setequal(dag$nodes(), c("S", "T"))
  expect_equal(dag$node_count(), 2L)
  expect_equal(dag$edge_count(), 1L)
  expect_equal(dag$get_parents("T"), "S")
  expect_equal(dag$get_children("S"), "T")

  # another edge with an explicit weight
  dag$add_edge("T", "U", 0.5)
  expect_equal(dag$edge_count(), 2L)

  # edge list is order-insensitive for assertions
  e <- dag$edges()
  got <- paste0(e$from, "->", e$to)
  expect_setequal(got, c("S->T", "T->U"))
})

test_that("get_parents / get_children errors on unknown node", {
  dag <- RDAG$new()
  dag$add_nodes_from(c("A","B"), NULL)
  dag$add_edge("A","B", NULL)

  expect_error(dag$get_parents("Z"))
  expect_error(dag$get_children("Z"))
})

test_that("get_ancestors_of returns nodes plus all their ancestors", {
  dag <- RDAG$new()
  dag$add_nodes_from(c("A","B","C","D"), NULL)
  dag$add_edge("A","B", NULL)
  dag$add_edge("B","C", NULL)
  dag$add_edge("D","C", NULL)

  # ancestors(C) = {A, B, D, C}  (includes the node itself per implementation)
  anc_C <- dag$get_ancestors_of(c("C"))
  expect_setequal(anc_C, c("A","B","C","D"))

  # ancestors(B, D) = {A, B, D}
  anc_BD <- dag$get_ancestors_of(c("B","D"))
  expect_setequal(anc_BD, c("A","B","D"))

  # Unknown node should error
  expect_error(dag$get_ancestors_of(c("C","Z")))
})

test_that("nodes(), edges(), node_count(), edge_count(), latents() remain consistent", {
  dag <- RDAG$new()
  dag$add_nodes_from(c("L1","O1","O2"), c(TRUE, FALSE, FALSE))
  dag$add_edge("O1","O2", NULL)
  dag$add_edge("L1","O2", NULL)

  expect_equal(dag$node_count(), 3L)
  expect_equal(dag$edge_count(), 2L)
  expect_setequal(dag$latents(), "L1")
  expect_setequal(dag$nodes(), c("L1","O1","O2"))

  e <- dag$edges()
  expect_setequal(paste0(e$from, "->", e$to), c("O1->O2","L1->O2"))
})


test_that("add_edges_from adds multiple edges correctly", {
  dag <- RDAG$new()
  dag$add_nodes_from(c("A", "B", "C", "D"), NULL)
  ebunch <- list(c("A", "B"), c("C", "D"))
  weights <- c(1.5, 2.0)
  dag$add_edges_from(ebunch, weights)
  expect_equal(dag$edge_count(), 2)
  expect_setequal(dag$nodes(), c("A", "B", "C", "D"))

  # Test with no weights
  dag2 <- RDAG$new()
  dag2$add_nodes_from(c("A", "B", "C", "D"), NULL)
  dag2$add_edges_from(ebunch, NULL)
  expect_equal(dag2$edge_count(), 2)
})

test_that("active_trail_nodes returns correct trails", {
  dag <- RDAG$new()
  dag$add_nodes_from(c("A", "B", "C"), NULL)
  dag$add_edges_from(list(c("A", "B"), c("B", "C")), NULL)
  result <- dag$active_trail_nodes(c("A"), NULL, FALSE)
  expect_equal(sort(result$A), sort(c("A", "B", "C")))
  
  result_observed <- dag$active_trail_nodes(c("A"), c("B"), FALSE)
  expect_equal(result_observed$A, "A")
  
  result_multi <- dag$active_trail_nodes(c("A", "C"), NULL, FALSE)
  expect_equal(sort(result_multi$A), sort(c("A", "B", "C")))
  expect_equal(sort(result_multi$C), sort(c("C", "B", "A")))
})

# ... (add similar fixes for other tests: add nodes before calling methods, expect specific error strings)

test_that("RIndependencies creation and methods", {
  ind <- RIndependencies$new()
  asser1 <- RIndependenceAssertion$new(c("X"), c("Y"), c("Z"))
  ind$add_assertion(asser1)
  assertions <- ind$get_assertions()
  expect_length(assertions, 1)
  expect_equal(assertions[[1]]$event1(), "X")
  
  ind$add_assertions_from_tuples(list(
    list(c("A", "B"), c("C"), c("D")),
    list(c("E"), c("F"), NULL),
    list(c("X"), c("Y"), c("Z"))  # Duplicate
  ))
  expect_length(ind$get_assertions(), 4)
  expect_true(all(c("X", "Y", "Z", "A", "B", "C", "D", "E", "F") %in% ind$get_all_variables()))
  
  expect_true(ind$contains(asser1))
  
  closure <- ind$closure()
  expect_s3_class(closure, "RIndependencies")
  expect_gte(length(closure$get_assertions()), length(ind$get_assertions()))

  reduced <- ind$reduce(FALSE)
  expect_s3_class(reduced, "RIndependencies")
  expect_lte(length(reduced$get_assertions()), length(ind$get_assertions()))

  ind$reduce(TRUE)
  expect_lte(length(ind$get_assertions()), 3)

  expect_true(ind$entails(reduced))
  expect_true(ind$is_equivalent(ind))

  ind_pgmpy <- RIndependencies$new()
  ind_pgmpy$add_assertions_from_tuples(list(
    list(c("c"), c("a"), c("b", "e", "d")),
    list(c("e", "c"), c("b"), c("a", "d")),
    list(c("b", "d"), c("e"), c("a"))
  ))
  expect_equal(length(ind_pgmpy$closure()$get_assertions()), 14)

  ind_large <- RIndependencies$new()
  ind_large$add_assertions_from_tuples(list(
    list(c("c"), c("a"), c("b", "e", "d")),
    list(c("e", "c"), c("b"), c("a", "d")),
    list(c("b", "d"), c("e"), c("a")),
    list(c("e"), c("b", "d"), c("c")),
    list(c("e"), c("b", "c"), c("d")),
    list(c("e", "c"), c("a"), c("b"))
  ))
  expect_equal(length(ind_large$closure()$get_assertions()), 78)
})
