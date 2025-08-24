library(causalgraphs)
library(testthat)


test_that("basic PDAG operations and properties", {
  pdag <- PDAG$new()
  pdag$add_edges_from(list(c("A", "C"), c("D", "C")), weights = NULL, directed = TRUE)
  pdag$add_edges_from(list(c("B", "A"), c("B", "D")), weights = NULL, directed = FALSE)

  expect_setequal(pdag$nodes(), c("A", "B", "C", "D"))
  expect_equal(pdag$node_count(), 4L)
  expect_equal(pdag$edge_count(), 6L) 

  # Check directed edges
  dir_edges <- pdag$directed_edges()
  expect_length(dir_edges, 2)
  expect_setequal(sapply(dir_edges, paste, collapse="->"), c("A->C", "D->C"))

  # Check undirected edges
  undir_edges <- pdag$undirected_edges()
  expect_length(undir_edges, 2)
  # Sorting to ensure consistent comparison
  undir_pairs <- sapply(undir_edges, function(x) paste(sort(x), collapse="-"))
  expect_setequal(undir_pairs, c("A-B", "B-D")) 

  # Check all edges in the representation
  all_edges <- pdag$edges()
  all_edges_str <- paste0(all_edges$from, "->", all_edges$to)
  expect_setequal(all_edges_str, c("A->C", "D->C", "A->B", "B->A", "B->D", "D->B"))
})

test_that("PDAG neighbor and parent/child queries work correctly", {
  pdag <- PDAG$new()
  pdag$add_edges_from(list(c("A", "C"), c("D", "C")), weights = NULL, directed = TRUE)
  pdag$add_edges_from(list(c("B", "A"), c("B", "D")), weights = NULL, directed = FALSE)

  expect_setequal(pdag$all_neighbors("A"), c("B", "C"))
  expect_setequal(pdag$all_neighbors("B"), c("A", "D"))
  expect_setequal(pdag$all_neighbors("C"), c("A", "D"))
  expect_setequal(pdag$all_neighbors("D"), c("B", "C"))

  expect_setequal(pdag$directed_children("A"), "C")
  expect_length(pdag$directed_children("B"), 0)
  expect_setequal(pdag$directed_parents("C"), c("A", "D"))

  expect_setequal(pdag$undirected_neighbors("A"), "B")
  expect_setequal(pdag$undirected_neighbors("B"), c("A", "D"))
  expect_length(pdag$undirected_neighbors("C"), 0)
})


test_that("PDAG edge existence checks work", {
  pdag <- PDAG$new()
  pdag$add_edges_from(list(c("A", "C"), c("D", "C")), weights = NULL, directed = TRUE)
  pdag$add_edges_from(list(c("B", "A"), c("B", "D")), weights = NULL, directed = FALSE)

  expect_true(pdag$has_directed_edge("A", "C"))
  expect_false(pdag$has_directed_edge("C", "A"))
  expect_false(pdag$has_directed_edge("A", "B"))

  expect_true(pdag$has_undirected_edge("A", "B"))
  expect_true(pdag$has_undirected_edge("B", "A"))
  expect_false(pdag$has_undirected_edge("A", "C"))

  expect_true(pdag$is_adjacent("A", "B"))
  expect_true(pdag$is_adjacent("A", "C"))
  expect_false(pdag$is_adjacent("A", "D"))
})


test_that("PDAG copy and orient_undirected_edge work", {
  pdag <- PDAG$new()
  pdag$add_edges_from(list(c("A", "C"), c("D", "C")), weights = NULL, directed = TRUE)
  pdag$add_edges_from(list(c("B", "A"), c("B", "D")), weights = NULL, directed = FALSE)

  # Test copy
  pdag_copy <- pdag$copy()
  expect_equal(pdag$nodes(), pdag_copy$nodes())
  expect_equal(pdag$directed_edges(), pdag_copy$directed_edges())
  expect_equal(pdag$undirected_edges(), pdag_copy$undirected_edges())

  # Test orient_undirected_edge (not in-place)
  mod_pdag <- pdag$orient_undirected_edge("B", "A", inplace = FALSE)
  expect_false(is.null(mod_pdag))
  expect_setequal(sapply(mod_pdag$directed_edges(), paste, collapse="->"), c("A->C", "D->C", "B->A"))
  expect_setequal(sapply(mod_pdag$undirected_edges(), function(x) paste(sort(x), collapse="-")), "B-D")

  # Test orient_undirected_edge (in-place)
  pdag$orient_undirected_edge("B", "A", inplace = TRUE)
  expect_setequal(sapply(pdag$directed_edges(), paste, collapse="->"), c("A->C", "D->C", "B->A"))
  expect_setequal(sapply(pdag$undirected_edges(), function(x) paste(sort(x), collapse="-")), "B-D")

  # Orienting an already directed edge should fail
  expect_error(pdag$orient_undirected_edge("B", "A", inplace = TRUE))
})

test_that("PDAG to_dag conversion works", {
  pdag <- PDAG$new()
  pdag$add_edges_from(list(c("A", "B"), c("C", "B")), weights = NULL, directed = TRUE)
  pdag$add_edges_from(list(c("C", "D"), c("D", "A")), weights = NULL, directed = FALSE)

  dag <- pdag$to_dag()
  expect_s3_class(dag, "RDAG")
  expect_equal(dag$edge_count(), 4L)
  
  e <- dag$edges()
  edges_str <- paste0(e$from, "->", e$to)
  expect_true("A->B" %in% edges_str)
  expect_true("C->B" %in% edges_str)
  # Should not create a v-structure at D
  expect_false(all(c("A->D", "C->D") %in% edges_str))
})


test_that("PDAG apply_meeks_rules works", {
  # Test case 1: A -> B - C  =>  A -> B -> C
  pdag <- PDAG$new()
  pdag$add_edge("A", "B", weight = NULL, directed = TRUE)
  pdag$add_edge("B", "C", weight = NULL, directed = FALSE)
  cpdag <- pdag$apply_meeks_rules(apply_r4 = TRUE, inplace = FALSE)
  
  e <- cpdag$edges()
  edges_str <- paste0(e$from, "->", e$to)
  expect_setequal(edges_str, c("A->B", "B->C"))

  # Test case 2: A -> B, D -> C, B - C => No change (potential v-structure)
  pdag2 <- PDAG$new()
  pdag2$add_edges_from(list(c("A", "B"), c("D", "C")), weights = NULL, directed = TRUE)
  pdag2$add_edge("B", "C", weight = NULL, directed = FALSE)
  cpdag2 <- pdag2$apply_meeks_rules(apply_r4 = TRUE, inplace = FALSE)
  
  e2 <- cpdag2$edges()
  edges_str2 <- paste0(e2$from, "->", e2$to)
  expect_setequal(edges_str2, c("A->B", "D->C", "B->C", "C->B"))
})