//! GlassUI Widget Identity System
//!
//! Provides unique identifiers and context for widgets, enabling:
//! - Parent-child relationship tracking
//! - Widget lookup by ID
//! - Task/workspace association
//! - Reactive binding targets

use std::sync::atomic::{AtomicU64, Ordering};

// =============================================================================
// WIDGET ID
// =============================================================================

/// Unique identifier for every widget instance
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WidgetId(u64);

static WIDGET_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

impl WidgetId {
    /// Generate a new unique widget ID
    pub fn new() -> Self {
        Self(WIDGET_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
    
    /// Get the numeric value (for debugging)
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl Default for WidgetId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for WidgetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Widget({})", self.0)
    }
}

// =============================================================================
// WORKSPACE ID
// =============================================================================

/// Unique identifier for a workspace
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WorkspaceId(u64);

static WORKSPACE_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

impl WorkspaceId {
    pub fn new() -> Self {
        Self(WORKSPACE_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for WorkspaceId {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// TASK ID
// =============================================================================

/// Unique identifier for a background task
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TaskId(u64);

static TASK_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

impl TaskId {
    pub fn new() -> Self {
        Self(TASK_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for TaskId {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// WIDGET CONTEXT
// =============================================================================

/// Context passed through widget tree during layout
/// 
/// Provides widgets with information about their position in the hierarchy
/// and their association with workspaces/tasks.
#[derive(Clone, Debug)]
pub struct WidgetContext {
    /// This widget's unique ID
    pub id: WidgetId,
    /// Parent widget's ID (None for root)
    pub parent_id: Option<WidgetId>,
    /// Depth in widget tree (0 = root)
    pub depth: usize,
    /// Associated workspace (if any)
    pub workspace_id: Option<WorkspaceId>,
    /// Associated task (if any)
    pub task_id: Option<TaskId>,
}

impl WidgetContext {
    /// Create a root context (no parent)
    pub fn root(id: WidgetId) -> Self {
        Self {
            id,
            parent_id: None,
            depth: 0,
            workspace_id: None,
            task_id: None,
        }
    }
    
    /// Create a child context from parent
    pub fn child_of(&self, child_id: WidgetId) -> Self {
        Self {
            id: child_id,
            parent_id: Some(self.id),
            depth: self.depth + 1,
            workspace_id: self.workspace_id,
            task_id: self.task_id,
        }
    }
    
    /// Associate with a workspace
    pub fn with_workspace(mut self, workspace_id: WorkspaceId) -> Self {
        self.workspace_id = Some(workspace_id);
        self
    }
    
    /// Associate with a task
    pub fn with_task(mut self, task_id: TaskId) -> Self {
        self.task_id = Some(task_id);
        self
    }
    
    /// Check if this is a root widget
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_widget_id_uniqueness() {
        let id1 = WidgetId::new();
        let id2 = WidgetId::new();
        let id3 = WidgetId::new();
        
        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_ne!(id1, id3);
    }
    
    #[test]
    fn test_context_hierarchy() {
        let root_id = WidgetId::new();
        let child_id = WidgetId::new();
        
        let root_ctx = WidgetContext::root(root_id);
        assert!(root_ctx.is_root());
        assert_eq!(root_ctx.depth, 0);
        
        let child_ctx = root_ctx.child_of(child_id);
        assert!(!child_ctx.is_root());
        assert_eq!(child_ctx.depth, 1);
        assert_eq!(child_ctx.parent_id, Some(root_id));
    }
}
