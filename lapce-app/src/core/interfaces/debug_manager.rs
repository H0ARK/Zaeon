//! Debug Manager Interface
//! 
//! Interface for debug functionality to enable pluggable
//! debug adapters and loose coupling.

/// Interface for debug management
pub trait DebugManager {
    /// Start a debug session
    fn start_session(&self, config: &DebugConfig) -> Result<DebugSession, String>;
    
    /// Stop a debug session
    fn stop_session(&self, session_id: &str) -> Result<(), String>;
    
    /// Set a breakpoint
    fn set_breakpoint(&self, session_id: &str, breakpoint: &Breakpoint) -> Result<(), String>;
    
    /// Remove a breakpoint
    fn remove_breakpoint(&self, session_id: &str, breakpoint_id: &str) -> Result<(), String>;
    
    /// Step through code
    fn step(&self, session_id: &str, step_type: StepType) -> Result<(), String>;
    
    /// Continue execution
    fn continue_execution(&self, session_id: &str) -> Result<(), String>;
}

/// Debug configuration
#[derive(Debug, Clone)]
pub struct DebugConfig {
    pub program: String,
    pub args: Vec<String>,
    pub working_directory: Option<String>,
    pub environment: Vec<(String, String)>,
}

/// Debug session information
#[derive(Debug, Clone)]
pub struct DebugSession {
    pub id: String,
    pub status: DebugStatus,
}

/// Debug session status
#[derive(Debug, Clone)]
pub enum DebugStatus {
    Starting,
    Running,
    Paused,
    Stopped,
    Error(String),
}

/// Breakpoint information
#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub id: String,
    pub file: String,
    pub line: usize,
    pub condition: Option<String>,
}

/// Step types for debugging
#[derive(Debug, Clone)]
pub enum StepType {
    Into,
    Over,
    Out,
}

