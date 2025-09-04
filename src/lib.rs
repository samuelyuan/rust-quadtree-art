//! # Rust Quadtree Art
//!
//! A high-performance Rust implementation of quadtree-based image art generation.

pub mod quad;

// Re-export main types for convenience
pub use quad::{Quad, QuadConfig, subdivide_nodes, generate_image};
