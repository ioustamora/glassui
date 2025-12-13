//! GlassUI Dashboard Framework
//!
//! High-level API for creating AI-powered management dashboards:
//! - Dashboard builder with fluent API
//! - Pre-built templates
//! - Responsive layouts
//! - Quick configuration

use glam::Vec2;
use crate::widget_id::WidgetId;
use crate::panel_style::{PanelPreset, PanelStyle};
use crate::widgets::Theme;

// =============================================================================
// DASHBOARD
// =============================================================================

/// Main dashboard builder
/// 
/// # Example
/// ```rust
/// let dashboard = Dashboard::new("AI Control Center")
///     .add_panel(AgentStatusGrid::new(&agents))
///     .add_panel(KpiRow::new(&metrics))
///     .with_theme(Theme::cyberpunk())
///     .with_layout(DashboardLayout::Responsive);
/// ```
pub struct Dashboard {
    pub id: WidgetId,
    pub title: String,
    pub panels: Vec<DashboardPanel>,
    pub layout: DashboardLayout,
    pub theme: Theme,
    pub size: Vec2,
}

impl Dashboard {
    /// Create a new dashboard with a title
    pub fn new(title: &str) -> Self {
        Self {
            id: WidgetId::new(),
            title: title.to_string(),
            panels: Vec::new(),
            layout: DashboardLayout::Responsive,
            theme: Theme::cyberpunk(),
            size: Vec2::ZERO,
        }
    }
    
    /// Add a panel to the dashboard
    pub fn add_panel(mut self, panel: DashboardPanel) -> Self {
        self.panels.push(panel);
        self
    }
    
    /// Set the layout mode
    pub fn with_layout(mut self, layout: DashboardLayout) -> Self {
        self.layout = layout;
        self
    }
    
    /// Set the theme
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    
    /// Get panel count
    pub fn panel_count(&self) -> usize {
        self.panels.len()
    }
}

// =============================================================================
// DASHBOARD PANEL
// =============================================================================

/// A panel within a dashboard
pub struct DashboardPanel {
    pub id: WidgetId,
    pub title: Option<String>,
    pub preset: PanelPreset,
    pub style: PanelStyle,
    pub size_hint: SizeHint,
    pub position_hint: PositionHint,
}

impl DashboardPanel {
    /// Create a new dashboard panel
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(),
            title: None,
            preset: PanelPreset::Default,
            style: PanelStyle::default(),
            size_hint: SizeHint::Auto,
            position_hint: PositionHint::Auto,
        }
    }
    
    /// Set the title
    pub fn titled(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self.style.title_bar = true;
        self
    }
    
    /// Set the preset style
    pub fn preset(mut self, preset: PanelPreset) -> Self {
        self.preset = preset;
        self.style = PanelStyle::from_preset(preset);
        self
    }
    
    /// Set size hint
    pub fn size(mut self, hint: SizeHint) -> Self {
        self.size_hint = hint;
        self
    }
    
    /// Set position hint
    pub fn position(mut self, hint: PositionHint) -> Self {
        self.position_hint = hint;
        self
    }
}

impl Default for DashboardPanel {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// LAYOUT TYPES
// =============================================================================

/// Dashboard layout mode
#[derive(Clone, Debug)]
pub enum DashboardLayout {
    /// Fixed positions (manual)
    Fixed,
    /// Auto-adapt to screen size
    Responsive,
    /// Sidebar + main area
    TwoColumn { sidebar_width: f32 },
    /// Three column layout
    ThreeColumn { left_width: f32, right_width: f32 },
    /// Grid layout
    Grid { columns: usize, gap: f32 },
    /// Pinterest-style masonry
    Masonry { columns: usize },
}

impl Default for DashboardLayout {
    fn default() -> Self {
        DashboardLayout::Responsive
    }
}

/// Size hint for auto-layout
#[derive(Clone, Copy, Debug)]
pub enum SizeHint {
    /// System decides size
    Auto,
    /// Fixed size in pixels
    Fixed(Vec2),
    /// Fraction of available space
    Fraction(f32, f32),
    /// Fill remaining space
    Fill,
    /// Minimum size
    MinSize(Vec2),
}

impl Default for SizeHint {
    fn default() -> Self {
        SizeHint::Auto
    }
}

/// Position hint for auto-layout
#[derive(Clone, Copy, Debug)]
pub enum PositionHint {
    /// System decides position
    Auto,
    /// Fixed position
    Fixed(Vec2),
    /// Grid cell (row, col)
    GridCell(usize, usize),
    /// Relative to edge
    Edge(Edge),
}

impl Default for PositionHint {
    fn default() -> Self {
        PositionHint::Auto
    }
}

/// Screen edge
#[derive(Clone, Copy, Debug)]
pub enum Edge {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

// =============================================================================
// DASHBOARD TEMPLATES
// =============================================================================

/// Pre-built dashboard templates
#[derive(Clone, Copy, Debug)]
pub enum DashboardTemplate {
    /// AI agent monitoring and control
    AiControlCenter,
    /// Data pipeline and ETL monitoring
    DataOpsBoard,
    /// DevOps CI/CD dashboard
    DevOpsDashboard,
    /// Analytics and reporting
    AnalyticsDash,
    /// Kanban-style task management
    TaskManager,
    /// Conversational AI interface
    ChatInterface,
}

impl DashboardTemplate {
    /// Build a dashboard from template
    pub fn build(self, title: &str) -> Dashboard {
        let mut dash = Dashboard::new(title);
        
        match self {
            DashboardTemplate::AiControlCenter => {
                dash = dash
                    .with_layout(DashboardLayout::TwoColumn { sidebar_width: 300.0 })
                    .add_panel(DashboardPanel::new().titled("Agents").preset(PanelPreset::Technical))
                    .add_panel(DashboardPanel::new().titled("Metrics").preset(PanelPreset::Data))
                    .add_panel(DashboardPanel::new().titled("Chat").preset(PanelPreset::Default));
            },
            DashboardTemplate::DataOpsBoard => {
                dash = dash
                    .with_layout(DashboardLayout::Grid { columns: 3, gap: 16.0 })
                    .add_panel(DashboardPanel::new().titled("Pipelines").preset(PanelPreset::Technical))
                    .add_panel(DashboardPanel::new().titled("Status").preset(PanelPreset::Status))
                    .add_panel(DashboardPanel::new().titled("Errors").preset(PanelPreset::Alert));
            },
            DashboardTemplate::TaskManager => {
                dash = dash
                    .with_layout(DashboardLayout::ThreeColumn { left_width: 280.0, right_width: 280.0 })
                    .add_panel(DashboardPanel::new().titled("To Do").preset(PanelPreset::Default))
                    .add_panel(DashboardPanel::new().titled("In Progress").preset(PanelPreset::Data))
                    .add_panel(DashboardPanel::new().titled("Done").preset(PanelPreset::Status));
            },
            DashboardTemplate::ChatInterface => {
                dash = dash
                    .with_layout(DashboardLayout::TwoColumn { sidebar_width: 250.0 })
                    .add_panel(DashboardPanel::new().titled("Conversations").preset(PanelPreset::Minimal))
                    .add_panel(DashboardPanel::new().titled("Chat").preset(PanelPreset::Default));
            },
            _ => {
                // Default layout for other templates
                dash = dash.with_layout(DashboardLayout::Responsive);
            }
        }
        
        dash
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dashboard_builder() {
        let dash = Dashboard::new("Test")
            .add_panel(DashboardPanel::new().titled("Panel 1"))
            .add_panel(DashboardPanel::new().titled("Panel 2"));
        
        assert_eq!(dash.panel_count(), 2);
    }
    
    #[test]
    fn test_template() {
        let dash = DashboardTemplate::AiControlCenter.build("AI Dashboard");
        assert!(dash.panel_count() >= 2);
    }
}
