//! Common Utilities
//! 
//! Miscellaneous utility functions used throughout the application.

/// Common utility functions
pub struct CommonUtils;

impl CommonUtils {
    /// Clamp a value between min and max
    pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }
    
    /// Generate a unique ID
    pub fn generate_id() -> String {
        // TODO: Implement proper ID generation
        format!("id_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos())
    }
    
    /// Debounce function calls
    pub fn debounce<F>(_func: F, _delay_ms: u64) -> impl Fn()
    where
        F: Fn() + 'static,
    {
        // TODO: Implement debouncing
        || {}
    }
}

