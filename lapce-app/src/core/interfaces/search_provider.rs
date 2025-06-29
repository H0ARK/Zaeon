//! Search Provider Interface
//! 
//! Interface for search functionality to enable pluggable
//! search engines and loose coupling.

/// Interface for search providers
pub trait SearchProvider {
    /// Perform a search with the given query
    fn search(&self, query: &SearchQuery) -> Result<SearchResults, String>;
    
    /// Check if this provider can handle the given query
    fn can_search(&self, query: &SearchQuery) -> bool;
}

/// Search query parameters
#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub text: String,
    pub scope: SearchScope,
    pub options: SearchOptions,
}

/// Search scope definition
#[derive(Debug, Clone)]
pub enum SearchScope {
    CurrentFile,
    OpenFiles,
    Workspace,
    Directory(String),
}

/// Search options
#[derive(Debug, Clone)]
pub struct SearchOptions {
    pub case_sensitive: bool,
    pub whole_word: bool,
    pub regex: bool,
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
}

/// Search results
#[derive(Debug, Clone)]
pub struct SearchResults {
    pub matches: Vec<SearchMatch>,
    pub total_count: usize,
    pub has_more: bool,
}

/// Individual search match
#[derive(Debug, Clone)]
pub struct SearchMatch {
    pub file_path: String,
    pub line_number: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub line_content: String,
    pub match_text: String,
}

