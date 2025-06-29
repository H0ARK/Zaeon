//! UI State Management
//! 
//! UI-specific state management, separate from business logic state.

/// UI State for editor components
#[derive(Debug, Clone)]
pub struct EditorUiState {
    pub scroll_position: (f64, f64),
    pub selection_range: Option<(usize, usize)>,
    pub cursor_position: (usize, usize),
    pub is_focused: bool,
}

impl Default for EditorUiState {
    fn default() -> Self {
        Self {
            scroll_position: (0.0, 0.0),
            selection_range: None,
            cursor_position: (0, 0),
            is_focused: false,
        }
    }
}

/// UI State for window components
#[derive(Debug, Clone)]
pub struct WindowUiState {
    pub size: (u32, u32),
    pub is_maximized: bool,
    pub active_tab_id: Option<String>,
}

impl Default for WindowUiState {
    fn default() -> Self {
        Self {
            size: (800, 600),
            is_maximized: false,
            active_tab_id: None,
        }
    }
}

/// UI State for panel components
#[derive(Debug, Clone)]
pub struct PanelUiState {
    pub is_visible: bool,
    pub size: u32,
    pub position: PanelPosition,
}

#[derive(Debug, Clone)]
pub enum PanelPosition {
    Left,
    Right,
    Bottom,
}

impl Default for PanelUiState {
    fn default() -> Self {
        Self {
            is_visible: false,
            size: 200,
            position: PanelPosition::Left,
        }
    }
}

