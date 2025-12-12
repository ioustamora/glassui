//! GlassUI Widget Module (Legacy Re-exports)
//!
//! This file provides backward compatibility by re-exporting from the new modular widgets/ directory.
//! All widget implementations have been moved to dedicated modules in src/widgets/.
//!
//! # New Structure
//! - `widgets::core` - Theme, Widget trait, easing
//! - `widgets::layout` - Column, Row, Stack, Grid, Flex, Spacer, Align
//! - `widgets::controls` - Button, Label, Slider, Checkbox, Panel
//! - `widgets::premium` - ProgressBar, Toggle, RadioGroup, NumberInput
//! - `widgets::input` - TextInput, Dropdown
//! - `widgets::containers` - ScrollArea, TabBar
//! - `widgets::overlays` - Tooltip, ContextMenu, Modal
//! - `widgets::advanced` - Draggable, Resizable
//! - `widgets::data` - Table, ListView, TreeView

// Re-export everything from the new modular widgets system
pub use crate::widgets::*;

// Additional legacy exports that may be expected
pub use crate::widgets::{
    // Core
    Theme, Widget, set_theme, get_theme, easing,
    
    // Layout
    Column, Row, Stack, Spacer, Align, Alignment,
    Grid, Flex, FlexDirection, FlexJustify, FlexAlign,
    
    // Controls
    Button, Label, Slider, Checkbox, Panel,
    
    // Premium
    ProgressBar, Toggle, RadioGroup, NumberInput,
    
    // Input
    TextInput, Dropdown,
    
    // Containers
    ScrollArea, TabBar,
    
    // Overlays
    Tooltip, MenuItem, ContextMenu, ContextMenuTrigger, Modal,
    
    // Advanced
    Draggable, Resizable,
    
    // Data
    Table, TableColumn, TableRow,
    ListView, ListItem,
    TreeView, TreeNode,
};

// Legacy type aliases for full backward compatibility
// (In case any code uses the old paths)

/// Legacy: Accessible widget wrapper (placeholder for future implementation)
pub struct Accessible {
    pub position: glam::Vec2,
    pub size: glam::Vec2,
    pub child: Box<dyn Widget>,
    pub label: String,
    pub role: String,
}

impl Accessible {
    pub fn new(child: Box<dyn Widget>, label: &str) -> Self {
        Self {
            position: glam::Vec2::ZERO,
            size: glam::Vec2::ZERO,
            child,
            label: label.to_string(),
            role: "widget".to_string(),
        }
    }
    
    pub fn with_role(mut self, role: &str) -> Self {
        self.role = role.to_string();
        self
    }
}

impl Widget for Accessible {
    fn layout(&mut self, origin: glam::Vec2, max_size: glam::Vec2) -> glam::Vec2 {
        self.position = origin;
        self.size = self.child.layout(origin, max_size);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: glam::Vec2) -> bool {
        self.child.handle_event(event, mouse_pos)
    }

    fn update(&mut self, dt: f32) {
        self.child.update(dt);
    }

    fn render(&self, renderer: &mut crate::renderer::GlassRenderer) {
        self.child.render(renderer);
    }
}

/// Legacy: DragSource for drag and drop (placeholder)
pub struct DragSource {
    pub position: glam::Vec2,
    pub size: glam::Vec2,
    pub child: Box<dyn Widget>,
    pub data: String,
}

impl DragSource {
    pub fn new(child: Box<dyn Widget>, data: &str) -> Self {
        Self {
            position: glam::Vec2::ZERO,
            size: glam::Vec2::ZERO,
            child,
            data: data.to_string(),
        }
    }
}

impl Widget for DragSource {
    fn layout(&mut self, origin: glam::Vec2, max_size: glam::Vec2) -> glam::Vec2 {
        self.position = origin;
        self.size = self.child.layout(origin, max_size);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: glam::Vec2) -> bool {
        self.child.handle_event(event, mouse_pos)
    }

    fn update(&mut self, dt: f32) {
        self.child.update(dt);
    }

    fn render(&self, renderer: &mut crate::renderer::GlassRenderer) {
        self.child.render(renderer);
    }
}

/// Legacy: DropTarget for drag and drop (placeholder)
pub struct DropTarget {
    pub position: glam::Vec2,
    pub size: glam::Vec2,
    pub child: Box<dyn Widget>,
}

impl DropTarget {
    pub fn new(child: Box<dyn Widget>) -> Self {
        Self {
            position: glam::Vec2::ZERO,
            size: glam::Vec2::ZERO,
            child,
        }
    }
}

impl Widget for DropTarget {
    fn layout(&mut self, origin: glam::Vec2, max_size: glam::Vec2) -> glam::Vec2 {
        self.position = origin;
        self.size = self.child.layout(origin, max_size);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: glam::Vec2) -> bool {
        self.child.handle_event(event, mouse_pos)
    }

    fn update(&mut self, dt: f32) {
        self.child.update(dt);
    }

    fn render(&self, renderer: &mut crate::renderer::GlassRenderer) {
        self.child.render(renderer);
    }
}
