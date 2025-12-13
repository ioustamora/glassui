//! GlassUI Tree View Widget
//!
//! Hierarchical tree for file browsers, navigation:
//! - Expandable/collapsible nodes
//! - Selection
//! - Icons and indentation

use glam::{Vec2, Vec4};
use crate::renderer::GlassRenderer;
use crate::widget_id::WidgetId;
use crate::widgets::core::{Widget, get_theme};

// =============================================================================
// TREE NODE
// =============================================================================

/// A node in the file tree
#[derive(Clone, Debug)]
pub struct FileNode {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub children: Vec<FileNode>,
    pub expanded: bool,
    pub selectable: bool,
}

impl FileNode {
    pub fn new(id: &str, label: &str) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            icon: None,
            children: Vec::new(),
            expanded: false,
            selectable: true,
        }
    }
    
    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }
    
    pub fn with_children(mut self, children: Vec<FileNode>) -> Self {
        self.children = children;
        self
    }
    
    pub fn expanded(mut self) -> Self {
        self.expanded = true;
        self
    }
    
    pub fn add_child(&mut self, child: FileNode) {
        self.children.push(child);
    }
    
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
    
    /// Create a folder node
    pub fn folder(id: &str, label: &str) -> Self {
        Self::new(id, label).with_icon("📁")
    }
    
    /// Create a file node
    pub fn file(id: &str, label: &str) -> Self {
        Self::new(id, label).with_icon("📄")
    }
}

// =============================================================================
// TREE VIEW
// =============================================================================

/// Tree view widget
pub struct FileTree {
    pub id: WidgetId,
    pub position: Vec2,
    pub size: Vec2,
    pub nodes: Vec<FileNode>,
    pub selected_id: Option<String>,
    pub hovered_id: Option<String>,
    pub indent_width: f32,
    pub row_height: f32,
    pub scroll_offset: f32,
    pub on_select: Option<Box<dyn FnMut(&str)>>,
}

impl FileTree {
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::new(250.0, 400.0),
            nodes: Vec::new(),
            selected_id: None,
            hovered_id: None,
            indent_width: 20.0,
            row_height: 28.0,
            scroll_offset: 0.0,
            on_select: None,
        }
    }
    
    /// Add a root node
    pub fn add_node(&mut self, node: FileNode) {
        self.nodes.push(node);
    }
    
    /// Select by ID
    pub fn select(&mut self, id: &str) {
        self.selected_id = Some(id.to_string());
    }
    
    /// Toggle expand/collapse by ID
    pub fn toggle(&mut self, id: &str) {
        fn toggle_in(nodes: &mut [FileNode], id: &str) -> bool {
            for node in nodes {
                if node.id == id {
                    node.expanded = !node.expanded;
                    return true;
                }
                if toggle_in(&mut node.children, id) {
                    return true;
                }
            }
            false
        }
        toggle_in(&mut self.nodes, id);
    }
    
    /// Count visible rows
    fn visible_rows(&self) -> usize {
        fn count_visible(nodes: &[FileNode]) -> usize {
            let mut n = nodes.len();
            for node in nodes {
                if node.expanded {
                    n += count_visible(&node.children);
                }
            }
            n
        }
        count_visible(&self.nodes)
    }
    
    /// Get node at y position
    fn node_at_y(&self, y: f32) -> Option<(String, usize)> {
        let row = ((y - self.position.y + self.scroll_offset) / self.row_height) as usize;
        let mut current = 0;
        
        fn find(nodes: &[FileNode], target: usize, current: &mut usize, depth: usize) -> Option<(String, usize)> {
            for node in nodes {
                if *current == target {
                    return Some((node.id.clone(), depth));
                }
                *current += 1;
                if node.expanded {
                    if let Some(result) = find(&node.children, target, current, depth + 1) {
                        return Some(result);
                    }
                }
            }
            None
        }
        
        find(&self.nodes, row, &mut current, 0)
    }
    
    /// Create sample file tree
    pub fn sample_file_tree() -> Self {
        let mut tree = Self::new();
        tree.add_node(
            FileNode::folder("src", "src").expanded().with_children(vec![
                FileNode::folder("widgets", "widgets").with_children(vec![
                    FileNode::file("mod", "mod.rs"),
                    FileNode::file("panel", "panel.rs"),
                    FileNode::file("button", "button.rs"),
                ]),
                FileNode::file("lib", "lib.rs"),
                FileNode::file("main", "main.rs"),
            ])
        );
        tree.add_node(FileNode::file("cargo", "Cargo.toml"));
        tree.add_node(FileNode::file("readme", "README.md"));
        tree
    }
}

impl Default for FileTree {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for FileTree {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = max_size;
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        // Update hover
        self.hovered_id = None;
        if mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.size.x &&
           mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.size.y {
            if let Some((id, _)) = self.node_at_y(mouse_pos.y) {
                self.hovered_id = Some(id);
            }
        }
        
        // Handle click
        if let winit::event::Event::WindowEvent { 
            event: winit::event::WindowEvent::MouseInput { 
                state: winit::event::ElementState::Pressed,
                button: winit::event::MouseButton::Left,
                ..
            }, .. 
        } = event {
            if let Some(id) = &self.hovered_id {
                self.toggle(id);
                self.selected_id = Some(id.clone());
                if let Some(callback) = &mut self.on_select {
                    callback(id);
                }
                return true;
            }
        }
        
        // Handle scroll
        if let winit::event::Event::WindowEvent { 
            event: winit::event::WindowEvent::MouseWheel { delta, .. }, 
            .. 
        } = event {
            let scroll = match delta {
                winit::event::MouseScrollDelta::LineDelta(_, y) => *y * 30.0,
                winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
            };
            let max_scroll = (self.visible_rows() as f32 * self.row_height - self.size.y).max(0.0);
            self.scroll_offset = (self.scroll_offset - scroll).clamp(0.0, max_scroll);
            return true;
        }
        
        false
    }

    fn update(&mut self, _dt: f32) {}

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Background
        renderer.draw_rounded_rect(self.position, self.size, Vec4::new(0.06, 0.06, 0.08, 0.9), 8.0);
        
        // Render nodes
        let mut y = self.position.y - self.scroll_offset;
        
        fn render_nodes(
            nodes: &[FileNode], 
            renderer: &mut GlassRenderer, 
            theme: &crate::widgets::core::Theme,
            pos: Vec2,
            size: Vec2,
            y: &mut f32, 
            depth: usize,
            indent: f32,
            row_h: f32,
            selected: &Option<String>,
            hovered: &Option<String>,
        ) {
            for node in nodes {
                let row_y = *y;
                *y += row_h;
                
                // Skip if outside visible area
                if row_y + row_h < pos.y || row_y > pos.y + size.y {
                    if node.expanded {
                        render_nodes(&node.children, renderer, theme, pos, size, y, depth + 1, indent, row_h, selected, hovered);
                    }
                    continue;
                }
                
                let x = pos.x + depth as f32 * indent;
                
                // Selection/hover background
                let is_selected = selected.as_ref() == Some(&node.id);
                let is_hovered = hovered.as_ref() == Some(&node.id);
                
                if is_selected {
                    renderer.draw_rounded_rect(
                        Vec2::new(pos.x + 4.0, row_y),
                        Vec2::new(size.x - 8.0, row_h - 2.0),
                        Vec4::new(theme.primary.x, theme.primary.y, theme.primary.z, 0.3),
                        4.0
                    );
                } else if is_hovered {
                    renderer.draw_rounded_rect(
                        Vec2::new(pos.x + 4.0, row_y),
                        Vec2::new(size.x - 8.0, row_h - 2.0),
                        Vec4::new(1.0, 1.0, 1.0, 0.1),
                        4.0
                    );
                }
                
                // Expand/collapse arrow
                if node.has_children() {
                    let arrow = if node.expanded { "▼" } else { "▶" };
                    renderer.draw_text(arrow, Vec2::new(x, row_y + 6.0), 10.0, theme.text_secondary);
                }
                
                // Icon
                let mut text_x = x + 16.0;
                if let Some(icon) = &node.icon {
                    renderer.draw_text(icon, Vec2::new(text_x, row_y + 5.0), 14.0, theme.text);
                    text_x += 20.0;
                }
                
                // Label
                let label_color = if is_selected { theme.text } else { theme.text_secondary };
                renderer.draw_text(&node.label, Vec2::new(text_x, row_y + 6.0), 13.0, label_color);
                
                // Render children if expanded
                if node.expanded {
                    render_nodes(&node.children, renderer, theme, pos, size, y, depth + 1, indent, row_h, selected, hovered);
                }
            }
        }
        
        let nodes_ref = &self.nodes;
        let selected_ref = &self.selected_id;
        let hovered_ref = &self.hovered_id;
        render_nodes(nodes_ref, renderer, &theme, self.position, self.size, &mut y, 0, self.indent_width, self.row_height, selected_ref, hovered_ref);
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tree_view() {
        let mut tree = FileTree::new();
        tree.add_node(FileNode::folder("root", "Root").with_children(vec![
            FileNode::file("child", "Child"),
        ]));
        
        assert_eq!(tree.nodes.len(), 1);
        assert!(tree.nodes[0].has_children());
    }
    
    #[test]
    fn test_toggle() {
        let mut tree = FileTree::sample_file_tree();
        let initial = tree.nodes[0].expanded;
        tree.toggle("src");
        assert_ne!(tree.nodes[0].expanded, initial);
    }
}
