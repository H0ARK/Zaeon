//! Core Interfaces
//! 
//! Trait definitions for dependency injection and loose coupling.
//! These interfaces define contracts between layers.

pub mod command_handler;
pub mod completion_provider;
pub mod debug_manager;
pub mod search_provider;
pub mod editor_manager;

// Re-export interfaces
pub use command_handler::*;
pub use completion_provider::*;
pub use debug_manager::*;
pub use search_provider::*;
pub use editor_manager::*;

