//! UI Layer - Pure UI components with event-driven communication
//! 
//! This module contains all UI components separated from business logic.
//! Components communicate through events and dependency injection.

pub mod components;
pub mod windows;
pub mod panels;
pub mod editor;
pub mod common;

// Re-export commonly used UI types
pub use common::*;

