//! Editor Manager Interface
//! 
//! Interface for editor functionality to enable pluggable
//! editor implementations and loose coupling.

/// Interface for editor management
pub trait EditorManager {
    /// Create a new editor instance
    fn create_editor(&self, config: &EditorConfig) -> Result<EditorInstance, String>;
    
    /// Get an existing editor instance
    fn get_editor(&self, editor_id: &str) -> Option<&EditorInstance>;
    
    /// Close an editor instance
    fn close_editor(&self, editor_id: &str) -> Result<(), String>;
    
    /// Apply an edit operation
    fn apply_edit(&self, editor_id: &str, edit: &EditOperation) -> Result<(), String>;
    
    /// Get editor content
    fn get_content(&self, editor_id: &str) -> Result<String, String>;
}

/// Editor configuration
#[derive(Debug, Clone)]
pub struct EditorConfig {
    pub file_path: Option<String>,
    pub language: Option<String>,
    pub read_only: bool,
    pub tab_size: usize,
    pub insert_spaces: bool,
}

/// Editor instance information
#[derive(Debug, Clone)]
pub struct EditorInstance {
    pub id: String,
    pub file_path: Option<String>,
    pub is_dirty: bool,
    pub cursor_position: Position,
    pub selection: Option<Range>,
}

/// Text position
#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

/// Text range
#[derive(Debug, Clone)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

/// Edit operation
#[derive(Debug, Clone)]
pub enum EditOperation {
    Insert { position: Position, text: String },
    Delete { range: Range },
    Replace { range: Range, text: String },
}

