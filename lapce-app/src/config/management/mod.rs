//! Configuration Management
//! 
//! Centralized configuration management system.
//! Consolidates configuration logic from config.rs and settings.rs.

pub mod config_manager;
pub mod settings_manager;

// Re-export managers
pub use config_manager::*;
pub use settings_manager::*;

