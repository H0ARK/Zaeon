//! Data Access Layer
//! 
//! Data access and persistence logic separated from business logic.
//! Contains repositories and data access objects.

pub mod repositories;

// Re-export commonly used data types
pub use repositories::*;

