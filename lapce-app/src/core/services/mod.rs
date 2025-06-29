//! Business Logic Services
//! 
//! Core business logic extracted from UI and data layers.
//! Services use dependency injection for loose coupling.

pub mod command_service;
pub mod completion_service;
pub mod debug_service;
pub mod search_service;
pub mod editor_service;

// Re-export services
pub use command_service::*;
pub use completion_service::*;
pub use debug_service::*;
pub use search_service::*;
pub use editor_service::*;

