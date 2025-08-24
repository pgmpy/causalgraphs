library(causalgraphs)
library(testthat)

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
