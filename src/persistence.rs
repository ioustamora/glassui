//! GlassUI Persistence
//!
//! Save and load workspace layouts:
//! - Workspace serialization
//! - Panel positions and sizes
//! - User preferences

use glam::Vec2;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

use crate::panel_style::PanelPreset;
use crate::workspace::WorkspaceLayout;

// =============================================================================
// SERIALIZABLE TYPES
// =============================================================================

/// Serializable panel state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PanelState {
    pub id: u64,
    pub title: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub preset: String,
    pub minimized: bool,
    pub z_index: i32,
}

impl PanelState {
    pub fn new(id: u64, title: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            x: 0.0,
            y: 0.0,
            width: 300.0,
            height: 200.0,
            preset: "default".to_string(),
            minimized: false,
            z_index: 0,
        }
    }
    
    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }
    
    pub fn with_size(mut self, w: f32, h: f32) -> Self {
        self.width = w;
        self.height = h;
        self
    }
    
    pub fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
    
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }
}

/// Serializable workspace state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkspaceState {
    pub name: String,
    pub panels: Vec<PanelState>,
    pub layout: String,
    pub width: f32,
    pub height: f32,
}

impl WorkspaceState {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            panels: Vec::new(),
            layout: "free".to_string(),
            width: 1920.0,
            height: 1080.0,
        }
    }
    
    pub fn add_panel(&mut self, panel: PanelState) {
        self.panels.push(panel);
    }
}

impl Default for WorkspaceState {
    fn default() -> Self {
        Self::new("Default")
    }
}

/// Serializable app state (all workspaces)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppState {
    pub version: String,
    pub workspaces: Vec<WorkspaceState>,
    pub active_workspace: usize,
    pub theme: String,
    pub sound_enabled: bool,
    pub master_volume: f32,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            workspaces: vec![WorkspaceState::default()],
            active_workspace: 0,
            theme: "cyberpunk".to_string(),
            sound_enabled: true,
            master_volume: 0.7,
        }
    }
}

// =============================================================================
// PERSISTENCE MANAGER
// =============================================================================

/// Manages saving and loading of app state
pub struct PersistenceManager {
    state: AppState,
    config_path: Option<std::path::PathBuf>,
    dirty: bool,
}

impl PersistenceManager {
    pub fn new() -> Self {
        Self {
            state: AppState::default(),
            config_path: None,
            dirty: false,
        }
    }
    
    /// Set the config file path
    pub fn with_path(mut self, path: impl AsRef<Path>) -> Self {
        self.config_path = Some(path.as_ref().to_path_buf());
        self
    }
    
    /// Get current state
    pub fn state(&self) -> &AppState {
        &self.state
    }
    
    /// Get mutable state (marks as dirty)
    pub fn state_mut(&mut self) -> &mut AppState {
        self.dirty = true;
        &mut self.state
    }
    
    /// Check if state needs saving
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    
    /// Load state from file
    pub fn load(&mut self) -> Result<(), PersistenceError> {
        let path = self.config_path.as_ref()
            .ok_or(PersistenceError::NoPath)?;
        
        if !path.exists() {
            return Ok(());  // No file yet, use defaults
        }
        
        let content = fs::read_to_string(path)
            .map_err(|e| PersistenceError::IoError(e.to_string()))?;
        
        self.state = serde_json::from_str(&content)
            .map_err(|e| PersistenceError::ParseError(e.to_string()))?;
        
        self.dirty = false;
        Ok(())
    }
    
    /// Save state to file
    pub fn save(&mut self) -> Result<(), PersistenceError> {
        let path = self.config_path.as_ref()
            .ok_or(PersistenceError::NoPath)?;
        
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| PersistenceError::IoError(e.to_string()))?;
        }
        
        let content = serde_json::to_string_pretty(&self.state)
            .map_err(|e| PersistenceError::SerializeError(e.to_string()))?;
        
        fs::write(path, content)
            .map_err(|e| PersistenceError::IoError(e.to_string()))?;
        
        self.dirty = false;
        Ok(())
    }
    
    /// Auto-save if dirty (call periodically)
    pub fn auto_save(&mut self) -> Result<bool, PersistenceError> {
        if self.dirty && self.config_path.is_some() {
            self.save()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    /// Get active workspace
    pub fn active_workspace(&self) -> &WorkspaceState {
        &self.state.workspaces[self.state.active_workspace]
    }
    
    /// Get active workspace (mutable)
    pub fn active_workspace_mut(&mut self) -> &mut WorkspaceState {
        self.dirty = true;
        &mut self.state.workspaces[self.state.active_workspace]
    }
    
    /// Add a new workspace
    pub fn add_workspace(&mut self, name: &str) -> usize {
        self.dirty = true;
        self.state.workspaces.push(WorkspaceState::new(name));
        self.state.workspaces.len() - 1
    }
    
    /// Switch to workspace by index
    pub fn switch_workspace(&mut self, index: usize) {
        if index < self.state.workspaces.len() {
            self.dirty = true;
            self.state.active_workspace = index;
        }
    }
}

impl Default for PersistenceManager {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// ERROR TYPE
// =============================================================================

/// Persistence errors
#[derive(Clone, Debug)]
pub enum PersistenceError {
    NoPath,
    IoError(String),
    ParseError(String),
    SerializeError(String),
}

impl std::fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersistenceError::NoPath => write!(f, "No config path set"),
            PersistenceError::IoError(e) => write!(f, "IO error: {}", e),
            PersistenceError::ParseError(e) => write!(f, "Parse error: {}", e),
            PersistenceError::SerializeError(e) => write!(f, "Serialize error: {}", e),
        }
    }
}

impl std::error::Error for PersistenceError {}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_panel_state() {
        let panel = PanelState::new(1, "Test Panel")
            .with_position(100.0, 200.0)
            .with_size(300.0, 250.0);
        
        assert_eq!(panel.position(), Vec2::new(100.0, 200.0));
        assert_eq!(panel.size(), Vec2::new(300.0, 250.0));
    }
    
    #[test]
    fn test_workspace_state() {
        let mut ws = WorkspaceState::new("Test");
        ws.add_panel(PanelState::new(1, "Panel 1"));
        ws.add_panel(PanelState::new(2, "Panel 2"));
        
        assert_eq!(ws.panels.len(), 2);
    }
    
    #[test]
    fn test_serialization() {
        let state = AppState::default();
        let json = serde_json::to_string(&state).unwrap();
        
        let loaded: AppState = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.version, "1.0");
    }
}
