use std::collections::HashMap;
use floem::peniko::Color;
use floem::text::FamilyOwned;
use lapce_rpc::plugin::VoltID;

use crate::config::{
    color::LapceColor,
    color_theme::{ColorThemeConfig, ThemeColor},
    core::CoreConfig,
    editor::{EditorConfig, WrapStyle},
    icon::LapceIcons,
    icon_theme::IconThemeConfig,
    terminal::TerminalConfig,
    ui::{UIConfig, TabCloseButton, TabSeparatorHeight},
    DropdownInfo,
};

/// Core configuration provider trait for application-wide settings
pub trait CoreConfigProvider {
    /// Get modal editing setting
    fn modal(&self) -> bool;
    
    /// Get color theme name
    fn color_theme(&self) -> &str;
    
    /// Get icon theme name  
    fn icon_theme(&self) -> &str;
    
    /// Get custom titlebar setting
    fn custom_titlebar(&self) -> bool;
    
    /// Get file explorer double-click setting
    fn file_explorer_double_click(&self) -> bool;
    
    /// Get auto-reload plugin setting
    fn auto_reload_plugin(&self) -> bool;
    
    /// Get the full core configuration
    fn core_config(&self) -> &CoreConfig;
}

/// Editor configuration provider trait for editor-specific settings
pub trait EditorConfigProvider {
    /// Get line height for editor
    fn line_height(&self) -> usize;
    
    /// Get font family for editor
    fn font_family(&self) -> Vec<FamilyOwned>;
    
    /// Get font size for editor
    fn font_size(&self) -> usize;
    
    /// Get wrap style setting
    fn wrap_style(&self) -> WrapStyle;
    
    /// Get tab width setting
    fn tab_width(&self) -> usize;
    
    /// Get whether to show line numbers
    fn show_line_numbers(&self) -> bool;
    
    /// Get whether to show relative line numbers
    fn show_relative_line_numbers(&self) -> bool;
    
    /// Get whether to show indent guides
    fn show_indent_guides(&self) -> bool;
    
    /// Get the full editor configuration
    fn editor_config(&self) -> &EditorConfig;
}

/// UI configuration provider trait for user interface settings
pub trait UIConfigProvider {
    /// Get UI scale factor
    fn scale(&self) -> f64;
    
    /// Get UI font family
    fn font_family(&self) -> Vec<FamilyOwned>;
    
    /// Get UI font size
    fn font_size(&self) -> usize;
    
    /// Get icon size
    fn icon_size(&self) -> usize;
    
    /// Get header height
    fn header_height(&self) -> usize;
    
    /// Get status bar height
    fn status_height(&self) -> usize;
    
    /// Get tab minimum width
    fn tab_min_width(&self) -> usize;
    
    /// Get tab separator height setting
    fn tab_separator_height(&self) -> TabSeparatorHeight;
    
    /// Get scroll bar width
    fn scroll_width(&self) -> usize;
    
    /// Get drop shadow width
    fn drop_shadow_width(&self) -> usize;
    
    /// Get command palette width
    fn palette_width(&self) -> usize;
    
    /// Get tab close button position
    fn tab_close_button(&self) -> TabCloseButton;
    
    /// Get whether to show open editors in explorer
    fn open_editors_visible(&self) -> bool;
    
    /// Get whether to trim whitespace from search results
    fn trim_search_results_whitespace(&self) -> bool;
    
    /// Get list line height
    fn list_line_height(&self) -> usize;
    
    /// Get the full UI configuration
    fn ui_config(&self) -> &UIConfig;
}

/// Theme configuration provider trait for color and icon themes
pub trait ThemeConfigProvider {
    /// Get a color by its string identifier
    fn color(&self, color: &str) -> Color;
    
    /// Get theme color with alpha
    fn color_with_alpha(&self, color: &str, alpha: f32) -> Color;
    
    /// Get an icon by its identifier
    fn icon(&self, icon: LapceIcons) -> Option<&str>;
    
    /// Get the current color theme configuration
    fn color_theme_config(&self) -> &ColorThemeConfig;
    
    /// Get the current icon theme configuration
    fn icon_theme_config(&self) -> &IconThemeConfig;
    
    /// Get the resolved theme colors
    fn theme_color(&self) -> &ThemeColor;
    
    /// Get available color themes
    fn available_color_themes(&self) -> &HashMap<String, (String, config::Config)>;
    
    /// Get available icon themes
    fn available_icon_themes(&self) -> &HashMap<String, (String, config::Config, Option<std::path::PathBuf>)>;
}

/// Terminal configuration provider trait for terminal settings
pub trait TerminalConfigProvider {
    /// Get terminal font family
    fn font_family(&self) -> Vec<FamilyOwned>;
    
    /// Get terminal font size
    fn font_size(&self) -> usize;
    
    /// Get terminal line height
    fn line_height(&self) -> usize;
    
    /// Get default terminal profile for current OS
    fn default_profile(&self) -> Option<&str>;
    
    /// Get all terminal profiles
    fn profiles(&self) -> &im::HashMap<String, HashMap<String, serde_json::Value>>;
    
    /// Get terminal color for ANSI color
    fn get_terminal_color(&self, color: alacritty_terminal::vte::ansi::NamedColor) -> Color;
    
    /// Get the full terminal configuration
    fn terminal_config(&self) -> &TerminalConfig;
}

/// Plugin configuration provider trait for plugin-specific settings
pub trait PluginConfigProvider {
    /// Get configuration for a specific plugin
    fn plugin_config(&self, plugin_id: &str) -> Option<&HashMap<String, serde_json::Value>>;
    
    /// Get all plugin configurations
    fn all_plugin_configs(&self) -> &HashMap<String, HashMap<String, serde_json::Value>>;
    
    /// Check if a plugin is disabled
    fn is_plugin_disabled(&self, plugin_id: &VoltID) -> bool;
}

/// Main configuration manager trait that combines all configuration providers
pub trait ConfigManager: 
    CoreConfigProvider + 
    EditorConfigProvider + 
    UIConfigProvider + 
    ThemeConfigProvider + 
    TerminalConfigProvider + 
    PluginConfigProvider 
{
    /// Get the configuration ID for change detection
    fn config_id(&self) -> u64;
    
    /// Get dropdown information for settings UI
    fn get_dropdown_info(&self, kind: &str, key: &str) -> Option<DropdownInfo>;
    
    /// Update a configuration setting
    fn update_setting(&self, parent: &str, key: &str, value: toml_edit::Value) -> Option<()>;
    
    /// Reset a configuration setting to default
    fn reset_setting(&self, parent: &str, key: &str) -> Option<()>;
    
    /// Reload configuration from files
    fn reload(&mut self);
    
    /// Get the settings file path
    fn settings_file() -> Option<std::path::PathBuf>;
    
    /// Get the keymaps file path  
    fn keymaps_file() -> Option<std::path::PathBuf>;
}

/// Lightweight configuration structs for specific use cases

/// Editor-focused configuration for performance-critical paths
#[derive(Debug, Clone)]
pub struct EditorConfigView {
    pub line_height: usize,
    pub font_family: Vec<FamilyOwned>,
    pub font_size: usize,
    pub wrap_style: WrapStyle,
    pub tab_width: usize,
    pub show_line_numbers: bool,
    pub show_relative_line_numbers: bool,
    pub show_indent_guides: bool,
}

/// UI-focused configuration for interface components
#[derive(Debug, Clone)]
pub struct UIConfigView {
    pub scale: f64,
    pub font_family: Vec<FamilyOwned>,
    pub font_size: usize,
    pub icon_size: usize,
    pub header_height: usize,
    pub status_height: usize,
    pub tab_min_width: usize,
    pub tab_separator_height: TabSeparatorHeight,
    pub scroll_width: usize,
    pub drop_shadow_width: usize,
    pub palette_width: usize,
    pub tab_close_button: TabCloseButton,
    pub open_editors_visible: bool,
    pub trim_search_results_whitespace: bool,
    pub list_line_height: usize,
}

/// Theme-focused configuration for styling components
#[derive(Debug, Clone)]
pub struct ThemeConfigView {
    pub color_theme_name: String,
    pub icon_theme_name: String,
    // Note: Colors and icons are accessed through methods due to their complexity
}

/// Core application configuration
#[derive(Debug, Clone)]
pub struct CoreConfigView {
    pub modal: bool,
    pub color_theme: String,
    pub icon_theme: String,
    pub custom_titlebar: bool,
    pub file_explorer_double_click: bool,
    pub auto_reload_plugin: bool,
}
