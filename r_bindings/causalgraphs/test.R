library(causalgraphs)

# Create a new DAG
dag <- RDAG$new()

# Add nodes
dag$add_node("A", latent = FALSE)
dag$add_node("B", latent = FALSE)
dag$add_node("L", latent = TRUE)  # Latent node

# Add edges
dag$add_edge("A", "B", 10)
dag$add_edge("B", "C", 20)

# Inspect graph
cat("Nodes:", dag$nodes(), "\n")
cat("Latents:", dag$latents(), "\n")
cat("Node count:", dag$node_count(), "\n")
cat("Edge count:", dag$edge_count(), "\n")

# Get edges
edges <- dag$edges()
cat("Edges:\n")
print(edges)

# Get parents
cat("Parents of B:", dag$get_parents("B"), "\n")

# Get children
cat("Children of A:", dag$get_children("A"), "\n")

# Get ancestors
cat("Ancestors of C:", dag$get_ancestors_of(c("C")), "\n")