//! String Utilities
//! 
//! Common string manipulation utilities used throughout the application.

// TODO: Move string utility functions here from various files

/// Common string manipulation utilities
pub struct StringUtils;

impl StringUtils {
    /// Truncate a string to a maximum length
    pub fn truncate(s: &str, max_len: usize) -> String {
        if s.len() <= max_len {
            s.to_string()
        } else {
            format!("{}...", &s[..max_len.saturating_sub(3)])
        }
    }
    
    /// Check if a string is empty or whitespace only
    pub fn is_blank(s: &str) -> bool {
        s.trim().is_empty()
    }
    
    /// Convert camelCase to snake_case
    pub fn camel_to_snake(s: &str) -> String {
        // TODO: Implement camelCase to snake_case conversion
        s.to_lowercase()
    }
}

