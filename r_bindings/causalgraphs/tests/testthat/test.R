library(causalgraphs)
library(testthat)

test_that("basic DAG operations", {
  dag <- RDAG$new()
  dag$add_node("A", FALSE)
  dag$add_node("B", FALSE)
  dag$add_edge("A", "B", 20)
  expect_setequal(dag$nodes(), c("A", "B"))
  expect_equal(dag$node_count(), 2)
  expect_equal(dag$edge_count(), 1)
  expect_equal(dag$get_parents("B"), "A")
  expect_equal(dag$get_children("A"), "B")
})