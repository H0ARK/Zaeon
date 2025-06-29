//! Core Business Logic Layer
//! 
//! Contains all business logic services, interfaces, and domain models.
//! This layer is independent of UI and data access implementations.

pub mod services;
pub mod interfaces;
pub mod models;

// Re-export commonly used core types
pub use interfaces::*;
pub use models::*;

