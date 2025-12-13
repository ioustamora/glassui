//! GlassUI Workspace System
//!
//! Provides workspace management for multi-window/panel layouts:
//! - Workspace persistence (save/load)
//! - Auto-layout algorithms
//! - Panel arrangement and snapping
//! - Swarm organization

use std::path::Path;
use glam::Vec2;

use crate::widget_id::{WidgetId, WorkspaceId};
use crate::panel_style::{PanelPreset, PanelStyle};
use crate::dashboard::DashboardLayout;

// =============================================================================
// WORKSPACE
// =============================================================================

/// A workspace containing multiple panels
#[derive(Clone, Debug)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub name: String,
    pub panels: Vec<WorkspacePanel>,
    pub layout: WorkspaceLayout,
    pub size: Vec2,
}

impl Workspace {
    /// Create a new workspace
    pub fn new(name: &str) -> Self {
        Self {
            id: WorkspaceId::new(),
            name: name.to_string(),
            panels: Vec::new(),
            layout: WorkspaceLayout::default(),
            size: Vec2::new(1920.0, 1080.0),
        }
    }
    
    /// Add a panel to the workspace
    pub fn add_panel(&mut self, panel: WorkspacePanel) {
        self.panels.push(panel);
    }
    
    /// Set the layout mode
    pub fn with_layout(mut self, layout: WorkspaceLayout) -> Self {
        self.layout = layout;
        self
    }
    
    /// Get panel count
    pub fn panel_count(&self) -> usize {
        self.panels.len()
    }
    
    /// Find panel by ID
    pub fn find_panel(&self, id: WidgetId) -> Option<&WorkspacePanel> {
        self.panels.iter().find(|p| p.widget_id == id)
    }
    
    /// Find panel by ID (mutable)
    pub fn find_panel_mut(&mut self, id: WidgetId) -> Option<&mut WorkspacePanel> {
        self.panels.iter_mut().find(|p| p.widget_id == id)
    }
}

// =============================================================================
// WORKSPACE PANEL
// =============================================================================

/// A panel within a workspace
#[derive(Clone, Debug)]
pub struct WorkspacePanel {
    pub widget_id: WidgetId,
    pub title: String,
    pub position: Vec2,
    pub size: Vec2,
    pub style: PanelStyle,
    pub z_index: i32,
    pub minimized: bool,
    pub maximized: bool,
}

impl WorkspacePanel {
    pub fn new(title: &str) -> Self {
        Self {
            widget_id: WidgetId::new(),
            title: title.to_string(),
            position: Vec2::ZERO,
            size: Vec2::new(400.0, 300.0),
            style: PanelStyle::default(),
            z_index: 0,
            minimized: false,
            maximized: false,
        }
    }
    
    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.position = Vec2::new(x, y);
        self
    }
    
    pub fn with_size(mut self, w: f32, h: f32) -> Self {
        self.size = Vec2::new(w, h);
        self
    }
    
    pub fn with_preset(mut self, preset: PanelPreset) -> Self {
        self.style = PanelStyle::from_preset(preset);
        self
    }
}

// =============================================================================
// WORKSPACE LAYOUT
// =============================================================================

/// Layout mode for workspace
#[derive(Clone, Debug)]
pub enum WorkspaceLayout {
    /// Free positioning (manual)
    Free,
    /// Auto-tiling
    Tiled(TileMode),
    /// Fixed grid
    Grid { columns: usize, rows: usize },
    /// Stacked/tabbed
    Stacked,
}

impl Default for WorkspaceLayout {
    fn default() -> Self {
        WorkspaceLayout::Free
    }
}

/// Tiling modes
#[derive(Clone, Copy, Debug)]
pub enum TileMode {
    /// Split horizontally
    Horizontal,
    /// Split vertically
    Vertical,
    /// Binary space partitioning
    Bsp,
    /// Master + stack
    MasterStack,
}

// =============================================================================
// SNAP TARGET
// =============================================================================

/// Targets for panel snapping
#[derive(Clone, Copy, Debug)]
pub enum SnapTarget {
    /// Snap to screen edge
    Edge(SnapEdge),
    /// Snap to grid cell
    Grid(usize, usize),
    /// Snap relative to another panel
    Panel(WidgetId, PanelRelation),
    /// Center on screen
    Center,
}

/// Screen edges for snapping
#[derive(Clone, Copy, Debug)]
pub enum SnapEdge {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Relation to another panel
#[derive(Clone, Copy, Debug)]
pub enum PanelRelation {
    Above,
    Below,
    LeftOf,
    RightOf,
}

// =============================================================================
// WORKSPACE MANAGER
// =============================================================================

/// Manages multiple workspaces
#[derive(Default)]
pub struct WorkspaceManager {
    workspaces: Vec<Workspace>,
    active_index: usize,
}

impl WorkspaceManager {
    pub fn new() -> Self {
        Self {
            workspaces: vec![Workspace::new("Default")],
            active_index: 0,
        }
    }
    
    /// Get active workspace
    pub fn active(&self) -> &Workspace {
        &self.workspaces[self.active_index]
    }
    
    /// Get active workspace (mutable)
    pub fn active_mut(&mut self) -> &mut Workspace {
        &mut self.workspaces[self.active_index]
    }
    
    /// Add a new workspace
    pub fn add_workspace(&mut self, workspace: Workspace) {
        self.workspaces.push(workspace);
    }
    
    /// Switch to workspace by index
    pub fn switch_to(&mut self, index: usize) {
        if index < self.workspaces.len() {
            self.active_index = index;
        }
    }
    
    /// Get all workspaces
    pub fn all(&self) -> &[Workspace] {
        &self.workspaces
    }
    
    /// Add panel to active workspace
    pub fn add_panel(&mut self, panel: WorkspacePanel) {
        self.active_mut().add_panel(panel);
    }
    
    /// Arrange panels according to layout
    pub fn arrange(&mut self) {
        let workspace = self.active_mut();
        match &workspace.layout {
            WorkspaceLayout::Free => {
                // No auto-arrangement
            },
            WorkspaceLayout::Tiled(mode) => {
                Self::arrange_tiled(&mut workspace.panels, workspace.size, *mode);
            },
            WorkspaceLayout::Grid { columns, rows } => {
                Self::arrange_grid(&mut workspace.panels, workspace.size, *columns, *rows);
            },
            WorkspaceLayout::Stacked => {
                // Stack all panels at same position
                for panel in &mut workspace.panels {
                    panel.position = Vec2::ZERO;
                    panel.size = workspace.size;
                }
            },
        }
    }
    
    /// Tile panels
    fn arrange_tiled(panels: &mut [WorkspacePanel], size: Vec2, mode: TileMode) {
        if panels.is_empty() { return; }
        
        match mode {
            TileMode::Horizontal => {
                let width = size.x / panels.len() as f32;
                for (i, panel) in panels.iter_mut().enumerate() {
                    panel.position = Vec2::new(i as f32 * width, 0.0);
                    panel.size = Vec2::new(width, size.y);
                }
            },
            TileMode::Vertical => {
                let height = size.y / panels.len() as f32;
                for (i, panel) in panels.iter_mut().enumerate() {
                    panel.position = Vec2::new(0.0, i as f32 * height);
                    panel.size = Vec2::new(size.x, height);
                }
            },
            TileMode::MasterStack => {
                if panels.len() == 1 {
                    panels[0].position = Vec2::ZERO;
                    panels[0].size = size;
                } else {
                    // Master takes left half
                    panels[0].position = Vec2::ZERO;
                    panels[0].size = Vec2::new(size.x * 0.5, size.y);
                    
                    // Stack takes right half
                    let stack_height = size.y / (panels.len() - 1) as f32;
                    for (i, panel) in panels.iter_mut().skip(1).enumerate() {
                        panel.position = Vec2::new(size.x * 0.5, i as f32 * stack_height);
                        panel.size = Vec2::new(size.x * 0.5, stack_height);
                    }
                }
            },
            _ => {},
        }
    }
    
    /// Grid layout
    fn arrange_grid(panels: &mut [WorkspacePanel], size: Vec2, cols: usize, rows: usize) {
        let cell_w = size.x / cols as f32;
        let cell_h = size.y / rows as f32;
        
        for (i, panel) in panels.iter_mut().enumerate() {
            let col = i % cols;
            let row = i / cols;
            if row < rows {
                panel.position = Vec2::new(col as f32 * cell_w, row as f32 * cell_h);
                panel.size = Vec2::new(cell_w, cell_h);
            }
        }
    }
    
    /// Snap a panel to a target
    pub fn snap_panel(&mut self, panel_id: WidgetId, target: SnapTarget) {
        let workspace = self.active_mut();
        let size = workspace.size;
        
        if let Some(panel) = workspace.find_panel_mut(panel_id) {
            match target {
                SnapTarget::Edge(edge) => {
                    match edge {
                        SnapEdge::TopLeft => {
                            panel.position = Vec2::ZERO;
                        },
                        SnapEdge::TopRight => {
                            panel.position = Vec2::new(size.x - panel.size.x, 0.0);
                        },
                        SnapEdge::BottomLeft => {
                            panel.position = Vec2::new(0.0, size.y - panel.size.y);
                        },
                        SnapEdge::BottomRight => {
                            panel.position = Vec2::new(size.x - panel.size.x, size.y - panel.size.y);
                        },
                        SnapEdge::Top => {
                            panel.position = Vec2::new((size.x - panel.size.x) / 2.0, 0.0);
                        },
                        SnapEdge::Bottom => {
                            panel.position = Vec2::new((size.x - panel.size.x) / 2.0, size.y - panel.size.y);
                        },
                        SnapEdge::Left => {
                            panel.position = Vec2::new(0.0, (size.y - panel.size.y) / 2.0);
                        },
                        SnapEdge::Right => {
                            panel.position = Vec2::new(size.x - panel.size.x, (size.y - panel.size.y) / 2.0);
                        },
                    }
                },
                SnapTarget::Center => {
                    panel.position = (size - panel.size) / 2.0;
                },
                SnapTarget::Grid(col, row) => {
                    // Assume 4x3 grid for now
                    let cell_w = size.x / 4.0;
                    let cell_h = size.y / 3.0;
                    panel.position = Vec2::new(col as f32 * cell_w, row as f32 * cell_h);
                },
                _ => {},
            }
        }
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_workspace_creation() {
        let mut ws = Workspace::new("Test");
        ws.add_panel(WorkspacePanel::new("Panel 1"));
        ws.add_panel(WorkspacePanel::new("Panel 2"));
        
        assert_eq!(ws.panel_count(), 2);
    }
    
    #[test]
    fn test_tile_layout() {
        let mut manager = WorkspaceManager::new();
        manager.active_mut().layout = WorkspaceLayout::Tiled(TileMode::Horizontal);
        manager.add_panel(WorkspacePanel::new("P1"));
        manager.add_panel(WorkspacePanel::new("P2"));
        manager.arrange();
        
        let panels = &manager.active().panels;
        assert!(panels[0].position.x < panels[1].position.x);
    }
}
