//! GlassUI Accessibility Support
//!
//! Provides accessibility infrastructure for screen readers and assistive technology.
//! This module defines semantic roles, accessible properties, and the accessibility tree.
//!
//! # Architecture
//!
//! Widgets implement the `Accessible` trait to provide accessibility information.
//! The accessibility tree is built during layout and can be consumed by platform
//! accessibility APIs (Windows UI Automation, macOS Accessibility, Linux ATK).

use std::collections::HashMap;

// =============================================================================
// ROLES
// =============================================================================

/// Semantic role of an accessible element
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Role {
    // Structural
    Window,
    Dialog,
    Alert,
    Group,
    
    // Actions
    Button,
    Link,
    MenuItem,
    MenuBar,
    
    // Selection
    Checkbox,
    RadioButton,
    Switch,
    Tab,
    TabList,
    
    // Input
    TextField,
    TextArea,
    Slider,
    SpinButton,
    ComboBox,
    ListBox,
    
    // Display
    Label,
    Image,
    ProgressBar,
    Tooltip,
    
    // Containers
    List,
    ListItem,
    Table,
    Row,
    Cell,
    Tree,
    TreeItem,
    
    // Layout
    ScrollView,
    Panel,
    Separator,
    
    // Other
    Unknown,
}

// =============================================================================
// STATES
// =============================================================================

/// Accessibility states for an element (bit flags)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct AccessibleState(pub u32);

impl AccessibleState {
    pub const DISABLED: Self = Self(1 << 0);
    pub const SELECTED: Self = Self(1 << 1);
    pub const CHECKED: Self = Self(1 << 2);
    pub const EXPANDED: Self = Self(1 << 3);
    pub const FOCUSED: Self = Self(1 << 4);
    pub const PRESSED: Self = Self(1 << 5);
    pub const READONLY: Self = Self(1 << 6);
    pub const REQUIRED: Self = Self(1 << 7);
    pub const BUSY: Self = Self(1 << 8);
    pub const INVALID: Self = Self(1 << 9);
    pub const HIDDEN: Self = Self(1 << 10);
    
    pub const fn empty() -> Self { Self(0) }
    pub const fn all() -> Self { Self(0x7FF) }
    
    pub const fn contains(&self, other: Self) -> bool { 
        (self.0 & other.0) == other.0 
    }
    
    pub fn insert(&mut self, other: Self) { 
        self.0 |= other.0; 
    }
    
    pub fn remove(&mut self, other: Self) { 
        self.0 &= !other.0; 
    }
    
    pub fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

// =============================================================================
// ACCESSIBLE NODE
// =============================================================================

/// Unique identifier for an accessible node
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AccessibleId(u64);

impl AccessibleId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        AccessibleId(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for AccessibleId {
    fn default() -> Self {
        Self::new()
    }
}

/// An accessible element in the accessibility tree
#[derive(Clone, Debug)]
pub struct AccessibleNode {
    /// Unique identifier
    pub id: AccessibleId,
    
    /// Semantic role
    pub role: Role,
    
    /// Human-readable label
    pub label: Option<String>,
    
    /// Description for more context
    pub description: Option<String>,
    
    /// Current value (for sliders, inputs, etc.)
    pub value: Option<String>,
    
    /// Minimum value (for range widgets)
    pub min_value: Option<f64>,
    
    /// Maximum value (for range widgets)
    pub max_value: Option<f64>,
    
    /// Current numeric value (for range widgets)
    pub numeric_value: Option<f64>,
    
    /// Current states
    pub state: AccessibleState,
    
    /// Bounding rectangle (screen coordinates)
    pub bounds: (f32, f32, f32, f32), // x, y, width, height
    
    /// Child node IDs
    pub children: Vec<AccessibleId>,
    
    /// Actions that can be performed
    pub actions: Vec<AccessibleAction>,
}

impl AccessibleNode {
    pub fn new(role: Role) -> Self {
        Self {
            id: AccessibleId::new(),
            role,
            label: None,
            description: None,
            value: None,
            min_value: None,
            max_value: None,
            numeric_value: None,
            state: AccessibleState::empty(),
            bounds: (0.0, 0.0, 0.0, 0.0),
            children: Vec::new(),
            actions: Vec::new(),
        }
    }
    
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
    
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
    
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    
    pub fn with_bounds(mut self, x: f32, y: f32, w: f32, h: f32) -> Self {
        self.bounds = (x, y, w, h);
        self
    }
    
    pub fn with_state(mut self, state: AccessibleState) -> Self {
        self.state = state;
        self
    }
}

// =============================================================================
// ACTIONS
// =============================================================================

/// Actions that can be performed on an accessible element
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccessibleAction {
    Click,
    Focus,
    ScrollIntoView,
    Expand,
    Collapse,
    ShowMenu,
    SetValue(String),
    Increment,
    Decrement,
}

// =============================================================================
// ACCESSIBILITY TREE
// =============================================================================

/// The accessibility tree for a window
#[derive(Default)]
pub struct AccessibilityTree {
    /// All nodes by ID
    nodes: HashMap<AccessibleId, AccessibleNode>,
    
    /// Root node ID
    root: Option<AccessibleId>,
    
    /// Currently focused node
    focused: Option<AccessibleId>,
}

impl AccessibilityTree {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Clear and rebuild the tree
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.root = None;
    }
    
    /// Add a node to the tree
    pub fn add_node(&mut self, node: AccessibleNode) -> AccessibleId {
        let id = node.id;
        self.nodes.insert(id, node);
        id
    }
    
    /// Set the root node
    pub fn set_root(&mut self, id: AccessibleId) {
        self.root = Some(id);
    }
    
    /// Get a node by ID
    pub fn get_node(&self, id: AccessibleId) -> Option<&AccessibleNode> {
        self.nodes.get(&id)
    }
    
    /// Get mutable node by ID
    pub fn get_node_mut(&mut self, id: AccessibleId) -> Option<&mut AccessibleNode> {
        self.nodes.get_mut(&id)
    }
    
    /// Set focused node
    pub fn set_focus(&mut self, id: AccessibleId) {
        // Update old focused node
        if let Some(old_id) = self.focused {
            if let Some(node) = self.nodes.get_mut(&old_id) {
                node.state.remove(AccessibleState::FOCUSED);
            }
        }
        
        // Update new focused node
        if let Some(node) = self.nodes.get_mut(&id) {
            node.state.insert(AccessibleState::FOCUSED);
            self.focused = Some(id);
        }
    }
    
    /// Get the currently focused node
    pub fn get_focused(&self) -> Option<AccessibleId> {
        self.focused
    }
    
    /// Get node count
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

// =============================================================================
// ACCESSIBLE TRAIT
// =============================================================================

/// Trait for widgets that provide accessibility information
pub trait Accessible {
    /// Get the accessibility node for this widget
    fn accessibility_node(&self) -> AccessibleNode;
    
    /// Get the accessible label (may be computed)
    fn accessible_label(&self) -> Option<String> {
        None
    }
    
    /// Handle an accessibility action
    fn perform_action(&mut self, _action: &AccessibleAction) -> bool {
        false
    }
}

// =============================================================================
// ANNOUNCE FUNCTION
// =============================================================================

/// Announce text to screen readers (live region)
/// 
/// This is a placeholder - real implementation would use platform APIs
pub fn announce(_text: &str, _priority: AnnouncePriority) {
    // TODO: Integrate with platform accessibility APIs
    // - Windows: UIA LiveRegion
    // - macOS: NSAccessibilityAnnouncementNotification
    // - Linux: ATK announce
    log::debug!("A11y announce: {}", _text);
}

/// Priority for accessibility announcements
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnnouncePriority {
    /// Polite - wait for current speech to finish
    Polite,
    /// Assertive - interrupt current speech
    Assertive,
}
