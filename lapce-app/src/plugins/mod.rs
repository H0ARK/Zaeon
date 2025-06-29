//! Plugin System Layer
//! 
//! Consolidated plugin system extracted from plugin.rs and proxy/ directory.
//! Organizes plugin functionality into focused modules.

pub mod core;
pub mod app;
pub mod proxy;
pub mod communication;

// Re-export commonly used plugin types
pub use core::*;
pub use app::*;

