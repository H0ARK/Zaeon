//! File Utilities
//! 
//! Common file and path manipulation utilities.

use std::path::{Path, PathBuf};

/// File and path utilities
pub struct FileUtils;

impl FileUtils {
    /// Get file extension from path
    pub fn get_extension(path: &Path) -> Option<String> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase())
    }
    
    /// Check if path is a hidden file/directory
    pub fn is_hidden(path: &Path) -> bool {
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.starts_with('.'))
            .unwrap_or(false)
    }
    
    /// Normalize path separators
    pub fn normalize_path(path: &Path) -> PathBuf {
        // TODO: Implement path normalization
        path.to_path_buf()
    }
    
    /// Get relative path from base to target
    pub fn get_relative_path(base: &Path, target: &Path) -> Option<PathBuf> {
        // TODO: Implement relative path calculation
        target.strip_prefix(base).ok().map(|p| p.to_path_buf())
    }
}

