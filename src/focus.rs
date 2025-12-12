//! GlassUI Focus Management
//!
//! Provides keyboard focus navigation and management:
//! - Tab/Shift+Tab traversal
//! - Focus scoping for modals/dialogs
//! - Programmatic focus control

use std::collections::HashMap;

// =============================================================================
// FOCUS ID
// =============================================================================

/// Unique identifier for a focusable widget
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FocusId(u64);

impl FocusId {
    /// Generate a new unique focus ID
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        FocusId(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for FocusId {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// FOCUS NODE
// =============================================================================

/// Information about a focusable widget
#[derive(Clone, Debug)]
pub struct FocusNode {
    /// Unique identifier
    pub id: FocusId,
    /// Tab order index (lower = earlier in tab order)
    pub tab_index: i32,
    /// Whether the widget can currently receive focus
    pub can_focus: bool,
    /// Whether this node creates a focus scope (like a modal)
    pub is_scope: bool,
}

impl FocusNode {
    pub fn new(id: FocusId) -> Self {
        Self {
            id,
            tab_index: 0,
            can_focus: true,
            is_scope: false,
        }
    }
    
    pub fn with_tab_index(mut self, index: i32) -> Self {
        self.tab_index = index;
        self
    }
    
    pub fn as_scope(mut self) -> Self {
        self.is_scope = true;
        self
    }
}

// =============================================================================
// FOCUS MANAGER
// =============================================================================

/// Manages focus state for a widget tree
/// 
/// # Usage
/// 
/// ```rust
/// let mut focus = FocusManager::new();
/// 
/// // Register focusable widgets
/// focus.register(FocusNode::new(button_id));
/// focus.register(FocusNode::new(input_id));
/// 
/// // Handle tab key
/// if key == Tab {
///     if shift_held {
///         focus.focus_previous();
///     } else {
///         focus.focus_next();
///     }
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct FocusManager {
    /// Currently focused widget
    focused: Option<FocusId>,
    /// All registered focusable nodes
    nodes: HashMap<FocusId, FocusNode>,
    /// Ordered list for tab traversal
    tab_order: Vec<FocusId>,
    /// Focus scope stack (for modals)
    scope_stack: Vec<FocusId>,
}

impl FocusManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Register a focusable widget
    pub fn register(&mut self, node: FocusNode) {
        let id = node.id;
        self.nodes.insert(id, node);
        self.rebuild_tab_order();
    }
    
    /// Unregister a widget (e.g., when removed from tree)
    pub fn unregister(&mut self, id: FocusId) {
        self.nodes.remove(&id);
        if self.focused == Some(id) {
            self.focused = None;
        }
        self.rebuild_tab_order();
    }
    
    /// Get currently focused widget
    pub fn focused(&self) -> Option<FocusId> {
        self.focused
    }
    
    /// Check if a specific widget has focus
    pub fn has_focus(&self, id: FocusId) -> bool {
        self.focused == Some(id)
    }
    
    /// Request focus for a specific widget
    pub fn request_focus(&mut self, id: FocusId) -> bool {
        if let Some(node) = self.nodes.get(&id) {
            if node.can_focus {
                self.focused = Some(id);
                return true;
            }
        }
        false
    }
    
    /// Clear focus
    pub fn clear_focus(&mut self) {
        self.focused = None;
    }
    
    /// Move focus to next widget in tab order
    pub fn focus_next(&mut self) -> bool {
        if self.tab_order.is_empty() {
            return false;
        }
        
        let current_idx = self.focused
            .and_then(|id| self.tab_order.iter().position(|&i| i == id));
        
        let next_idx = match current_idx {
            Some(idx) => (idx + 1) % self.tab_order.len(),
            None => 0,
        };
        
        // Find next focusable widget
        for i in 0..self.tab_order.len() {
            let idx = (next_idx + i) % self.tab_order.len();
            let id = self.tab_order[idx];
            if let Some(node) = self.nodes.get(&id) {
                if node.can_focus {
                    self.focused = Some(id);
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Move focus to previous widget in tab order
    pub fn focus_previous(&mut self) -> bool {
        if self.tab_order.is_empty() {
            return false;
        }
        
        let current_idx = self.focused
            .and_then(|id| self.tab_order.iter().position(|&i| i == id));
        
        let prev_idx = match current_idx {
            Some(idx) if idx > 0 => idx - 1,
            Some(_) => self.tab_order.len() - 1,
            None => self.tab_order.len() - 1,
        };
        
        // Find previous focusable widget
        for i in 0..self.tab_order.len() {
            let idx = if prev_idx >= i { prev_idx - i } else { self.tab_order.len() - 1 - (i - prev_idx - 1) };
            let id = self.tab_order[idx];
            if let Some(node) = self.nodes.get(&id) {
                if node.can_focus {
                    self.focused = Some(id);
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Enter a focus scope (e.g., modal opened)
    pub fn push_scope(&mut self, scope_id: FocusId) {
        self.scope_stack.push(scope_id);
    }
    
    /// Exit a focus scope (e.g., modal closed)
    pub fn pop_scope(&mut self) -> Option<FocusId> {
        self.scope_stack.pop()
    }
    
    /// Rebuild tab order from registered nodes
    fn rebuild_tab_order(&mut self) {
        self.tab_order.clear();
        
        let mut nodes: Vec<_> = self.nodes.iter()
            .filter(|(_, n)| n.can_focus)
            .collect();
        
        // Sort by tab_index, then by insertion order (id)
        nodes.sort_by(|a, b| {
            a.1.tab_index.cmp(&b.1.tab_index)
                .then_with(|| a.0.0.cmp(&b.0.0))
        });
        
        self.tab_order = nodes.into_iter().map(|(id, _)| *id).collect();
    }
}

// =============================================================================
// FOCUSABLE TRAIT
// =============================================================================

/// Trait for widgets that can receive keyboard focus
pub trait Focusable {
    /// Get the focus ID for this widget
    fn focus_id(&self) -> FocusId;
    
    /// Whether this widget can currently receive focus
    fn can_focus(&self) -> bool { true }
    
    /// Called when focus is gained
    fn on_focus(&mut self) {}
    
    /// Called when focus is lost
    fn on_blur(&mut self) {}
    
    /// Get the tab index (-1 = not tabbable, 0+ = explicit order)
    fn tab_index(&self) -> i32 { 0 }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_focus_navigation() {
        let mut focus = FocusManager::new();
        
        let id1 = FocusId::new();
        let id2 = FocusId::new();
        let id3 = FocusId::new();
        
        focus.register(FocusNode::new(id1).with_tab_index(0));
        focus.register(FocusNode::new(id2).with_tab_index(1));
        focus.register(FocusNode::new(id3).with_tab_index(2));
        
        // Start with no focus
        assert_eq!(focus.focused(), None);
        
        // Tab to first
        assert!(focus.focus_next());
        assert_eq!(focus.focused(), Some(id1));
        
        // Tab to second
        assert!(focus.focus_next());
        assert_eq!(focus.focused(), Some(id2));
        
        // Tab to third
        assert!(focus.focus_next());
        assert_eq!(focus.focused(), Some(id3));
        
        // Tab wraps to first
        assert!(focus.focus_next());
        assert_eq!(focus.focused(), Some(id1));
        
        // Shift+Tab back to third
        assert!(focus.focus_previous());
        assert_eq!(focus.focused(), Some(id3));
    }
    
    #[test]
    fn test_request_focus() {
        let mut focus = FocusManager::new();
        let id = FocusId::new();
        
        focus.register(FocusNode::new(id));
        
        assert!(focus.request_focus(id));
        assert!(focus.has_focus(id));
        
        focus.clear_focus();
        assert!(!focus.has_focus(id));
    }
}
