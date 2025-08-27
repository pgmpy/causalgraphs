// Re-export modules/structs from your core logic
pub mod dag;
pub mod independencies;
pub mod pdag; // Add PDAG.rs later if needed
pub mod graph_role;

pub use dag::RustDAG;
pub use pdag::RustPDAG;
pub use independencies::{IndependenceAssertion, Independencies};