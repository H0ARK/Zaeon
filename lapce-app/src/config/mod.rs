//! Configuration Management Layer
//! 
//! Centralized configuration management extracted from config.rs
//! and scattered configuration logic throughout the codebase.

pub mod management;

// Re-export configuration types
pub use management::*;

