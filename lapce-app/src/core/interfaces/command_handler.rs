//! Command Handler Interface
//! 
//! Interface for command handling to enable dependency injection
//! and loose coupling between command sources and handlers.

/// Interface for handling commands
pub trait CommandHandler {
    /// Handle a command with the given parameters
    fn handle_command(&self, command: &str, params: &[String]) -> Result<(), String>;
    
    /// Get list of supported commands
    fn supported_commands(&self) -> Vec<String>;
}

/// Interface for command registration
pub trait CommandRegistry {
    /// Register a command handler
    fn register_handler(&mut self, handler: Box<dyn CommandHandler>);
    
    /// Unregister a command handler
    fn unregister_handler(&mut self, handler_id: &str);
}

