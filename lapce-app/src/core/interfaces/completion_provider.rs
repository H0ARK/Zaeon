//! Completion Provider Interface
//! 
//! Interface for text completion providers to enable pluggable
//! completion systems and loose coupling.

/// Interface for providing text completions
pub trait CompletionProvider {
    /// Get completions for the given context
    fn get_completions(&self, context: &CompletionContext) -> Vec<CompletionItem>;
    
    /// Check if this provider can handle the given context
    fn can_provide(&self, context: &CompletionContext) -> bool;
}

/// Context information for completion requests
#[derive(Debug, Clone)]
pub struct CompletionContext {
    pub text: String,
    pub position: usize,
    pub file_type: Option<String>,
    pub trigger_character: Option<char>,
}

/// A completion item
#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: String,
    pub kind: CompletionKind,
}

/// Types of completions
#[derive(Debug, Clone)]
pub enum CompletionKind {
    Text,
    Method,
    Function,
    Constructor,
    Field,
    Variable,
    Class,
    Interface,
    Module,
    Property,
    Unit,
    Value,
    Enum,
    Keyword,
    Snippet,
}

