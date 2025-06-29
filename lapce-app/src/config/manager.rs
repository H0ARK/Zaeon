use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use floem::peniko::Color;
use floem::text::FamilyOwned;
use lapce_rpc::plugin::VoltID;

use crate::config::{
    color_theme::{ColorThemeConfig, ThemeColor},
    core::CoreConfig,
    editor::{EditorConfig, WrapStyle},
    icon::LapceIcons,
    icon_theme::IconThemeConfig,
    terminal::TerminalConfig,
    ui::{UIConfig, TabCloseButton, TabSeparatorHeight},
    traits::*,
    DropdownInfo, LapceConfig,
};
use crate::workspace::LapceWorkspace;

/// Implementation of ConfigManager that wraps the existing LapceConfig
/// This allows gradual migration while maintaining backward compatibility
pub struct LapceConfigManager {
    config: Arc<LapceConfig>,
}

impl LapceConfigManager {
    /// Create a new config manager from a LapceConfig
    pub fn new(config: Arc<LapceConfig>) -> Self {
        Self { config }
    }

    /// Create a new config manager by loading configuration
    pub fn load(
        workspace: &LapceWorkspace,
        disabled_volts: &[VoltID],
        extra_plugin_paths: &[PathBuf],
    ) -> Self {
        let config = LapceConfig::load(workspace, disabled_volts, extra_plugin_paths);
        Self::new(Arc::new(config))
    }

    /// Get the underlying LapceConfig (for backward compatibility during migration)
    pub fn inner(&self) -> &Arc<LapceConfig> {
        &self.config
    }

    /// Update the underlying config
    pub fn update_config(&mut self, config: Arc<LapceConfig>) {
        self.config = config;
    }

    /// Create lightweight config views for performance-critical components
    pub fn editor_config_view(&self) -> EditorConfigView {
        EditorConfigView {
            line_height: self.config.editor.line_height(),
            font_family: floem::text::FamilyOwned::parse_list(&self.config.editor.font_family).collect(),
            font_size: self.config.editor.font_size(),
            wrap_style: self.config.editor.wrap_style,
            tab_width: self.config.editor.tab_width,
            show_line_numbers: true, // TODO: This should come from UI config or be computed
            show_relative_line_numbers: self.config.editor.modal_mode_relative_line_numbers,
            show_indent_guides: self.config.editor.show_indent_guide,
        }
    }

    pub fn ui_config_view(&self) -> UIConfigView {
        UIConfigView {
            scale: self.config.ui.scale(),
            font_family: self.config.ui.font_family(),
            font_size: self.config.ui.font_size(),
            icon_size: self.config.ui.icon_size(),
            header_height: self.config.ui.header_height(),
            status_height: self.config.ui.status_height(),
            tab_min_width: 100, // TODO: Add accessor method to UIConfig
            tab_separator_height: self.config.ui.tab_separator_height,
            scroll_width: 10, // TODO: Add accessor method to UIConfig
            drop_shadow_width: 5, // TODO: Add accessor method to UIConfig
            palette_width: self.config.ui.palette_width(),
            tab_close_button: self.config.ui.tab_close_button,
            open_editors_visible: self.config.ui.open_editors_visible,
            trim_search_results_whitespace: self.config.ui.trim_search_results_whitespace,
            list_line_height: 20, // TODO: Add accessor method to UIConfig
        }
    }

    pub fn theme_config_view(&self) -> ThemeConfigView {
        ThemeConfigView {
            color_theme_name: self.config.color_theme.name.clone(),
            icon_theme_name: self.config.icon_theme.name.clone(),
        }
    }

    pub fn core_config_view(&self) -> CoreConfigView {
        CoreConfigView {
            modal: self.config.core.modal,
            color_theme: self.config.core.color_theme.clone(),
            icon_theme: self.config.core.icon_theme.clone(),
            custom_titlebar: self.config.core.custom_titlebar,
            file_explorer_double_click: self.config.core.file_explorer_double_click,
            auto_reload_plugin: self.config.core.auto_reload_plugin,
        }
    }
}

impl CoreConfigProvider for LapceConfigManager {
    fn modal(&self) -> bool {
        self.config.core.modal
    }

    fn color_theme(&self) -> &str {
        &self.config.core.color_theme
    }

    fn icon_theme(&self) -> &str {
        &self.config.core.icon_theme
    }

    fn custom_titlebar(&self) -> bool {
        self.config.core.custom_titlebar
    }

    fn file_explorer_double_click(&self) -> bool {
        self.config.core.file_explorer_double_click
    }

    fn auto_reload_plugin(&self) -> bool {
        self.config.core.auto_reload_plugin
    }

    fn core_config(&self) -> &CoreConfig {
        &self.config.core
    }
}

impl EditorConfigProvider for LapceConfigManager {
    fn line_height(&self) -> usize {
        self.config.editor.line_height()
    }

    fn font_family(&self) -> Vec<FamilyOwned> {
        floem::text::FamilyOwned::parse_list(&self.config.editor.font_family).collect()
    }

    fn font_size(&self) -> usize {
        self.config.editor.font_size()
    }

    fn wrap_style(&self) -> WrapStyle {
        self.config.editor.wrap_style
    }

    fn tab_width(&self) -> usize {
        self.config.editor.tab_width
    }

    fn show_line_numbers(&self) -> bool {
        // TODO: This should be determined by UI settings or computed based on context
        true
    }

    fn show_relative_line_numbers(&self) -> bool {
        self.config.editor.modal_mode_relative_line_numbers
    }

    fn show_indent_guides(&self) -> bool {
        self.config.editor.show_indent_guide
    }

    fn editor_config(&self) -> &EditorConfig {
        &self.config.editor
    }
}

impl UIConfigProvider for LapceConfigManager {
    fn scale(&self) -> f64 {
        self.config.ui.scale()
    }

    fn font_family(&self) -> Vec<FamilyOwned> {
        self.config.ui.font_family()
    }

    fn font_size(&self) -> usize {
        self.config.ui.font_size()
    }

    fn icon_size(&self) -> usize {
        self.config.ui.icon_size()
    }

    fn header_height(&self) -> usize {
        self.config.ui.header_height()
    }

    fn status_height(&self) -> usize {
        self.config.ui.status_height()
    }

    fn tab_min_width(&self) -> usize {
        100 // TODO: Add accessor method to UIConfig
    }

    fn tab_separator_height(&self) -> TabSeparatorHeight {
        self.config.ui.tab_separator_height
    }

    fn scroll_width(&self) -> usize {
        10 // TODO: Add accessor method to UIConfig
    }

    fn drop_shadow_width(&self) -> usize {
        5 // TODO: Add accessor method to UIConfig
    }

    fn palette_width(&self) -> usize {
        self.config.ui.palette_width()
    }

    fn tab_close_button(&self) -> TabCloseButton {
        self.config.ui.tab_close_button
    }

    fn open_editors_visible(&self) -> bool {
        self.config.ui.open_editors_visible
    }

    fn trim_search_results_whitespace(&self) -> bool {
        self.config.ui.trim_search_results_whitespace
    }

    fn list_line_height(&self) -> usize {
        20 // TODO: Add accessor method to UIConfig
    }

    fn ui_config(&self) -> &UIConfig {
        &self.config.ui
    }
}

impl ThemeConfigProvider for LapceConfigManager {
    fn color(&self, color: &str) -> Color {
        self.config.color(color)
    }

    fn color_with_alpha(&self, color: &str, alpha: f32) -> Color {
        self.config.color(color).multiply_alpha(alpha)
    }

    fn icon(&self, icon: LapceIcons) -> Option<&str> {
        self.config.icon_svg(icon)
    }

    fn color_theme_config(&self) -> &ColorThemeConfig {
        &self.config.color_theme
    }

    fn icon_theme_config(&self) -> &IconThemeConfig {
        &self.config.icon_theme
    }

    fn theme_color(&self) -> &ThemeColor {
        &self.config.color
    }

    fn available_color_themes(&self) -> &HashMap<String, (String, config::Config)> {
        &self.config.available_color_themes
    }

    fn available_icon_themes(&self) -> &HashMap<String, (String, config::Config, Option<PathBuf>)> {
        &self.config.available_icon_themes
    }
}

impl TerminalConfigProvider for LapceConfigManager {
    fn font_family(&self) -> Vec<FamilyOwned> {
        self.config.terminal.font_family()
    }

    fn font_size(&self) -> usize {
        self.config.terminal.font_size()
    }

    fn line_height(&self) -> usize {
        self.config.terminal.line_height()
    }

    fn default_profile(&self) -> Option<&str> {
        self.config.terminal.default_profile.get(std::env::consts::OS)
            .map(|s| s.as_str())
    }

    fn profiles(&self) -> &im::HashMap<String, HashMap<String, serde_json::Value>> {
        &self.config.terminal.profiles
    }

    fn get_terminal_color(&self, color: alacritty_terminal::vte::ansi::NamedColor) -> Color {
        self.config.get_terminal_color(color)
    }

    fn terminal_config(&self) -> &TerminalConfig {
        &self.config.terminal
    }
}

impl PluginConfigProvider for LapceConfigManager {
    fn plugin_config(&self, plugin_id: &str) -> Option<&HashMap<String, serde_json::Value>> {
        self.config.plugins.get(plugin_id)
    }

    fn all_plugin_configs(&self) -> &HashMap<String, HashMap<String, serde_json::Value>> {
        &self.config.plugins
    }

    fn is_plugin_disabled(&self, _plugin_id: &VoltID) -> bool {
        // TODO: Implement plugin disable logic if needed
        false
    }
}

impl ConfigManager for LapceConfigManager {
    fn config_id(&self) -> u64 {
        self.config.id
    }

    fn get_dropdown_info(&self, kind: &str, key: &str) -> Option<DropdownInfo> {
        self.config.get_dropdown_info(kind, key)
    }

    fn update_setting(&self, parent: &str, key: &str, value: toml_edit::Value) -> Option<()> {
        LapceConfig::update_file(parent, key, value)
    }

    fn reset_setting(&self, parent: &str, key: &str) -> Option<()> {
        LapceConfig::reset_setting(parent, key)
    }

    fn reload(&mut self) {
        // For now, this is a no-op since the config is managed externally
        // In a full implementation, this would reload from files
    }

    fn settings_file() -> Option<PathBuf> {
        LapceConfig::settings_file()
    }

    fn keymaps_file() -> Option<PathBuf> {
        LapceConfig::keymaps_file()
    }
}

/// Utility functions for creating config managers
impl LapceConfigManager {
    /// Create a config manager from reactive signals (for Floem integration)
    pub fn from_signal(config: floem::reactive::ReadSignal<Arc<LapceConfig>>) -> Self {
        Self::new(config.get_untracked())
    }

    /// Update from reactive signal
    pub fn update_from_signal(&mut self, config: floem::reactive::ReadSignal<Arc<LapceConfig>>) {
        self.config = config.get_untracked();
    }
}
