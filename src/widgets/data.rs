//! GlassUI Data Display Widgets
//!
//! Widgets for displaying structured data: Table, Tree, List

use glam::{Vec2, Vec4};
use winit::event::{ElementState, MouseButton};
use crate::renderer::GlassRenderer;
use super::core::{Widget, get_theme};

// =============================================================================
// TABLE
// =============================================================================

/// Column definition for Table
pub struct TableColumn {
    pub header: String,
    pub width: f32,
    pub resizable: bool,
}

impl TableColumn {
    pub fn new(header: &str, width: f32) -> Self {
        Self {
            header: header.to_string(),
            width,
            resizable: true,
        }
    }
    
    pub fn fixed(header: &str, width: f32) -> Self {
        Self {
            header: header.to_string(),
            width,
            resizable: false,
        }
    }
}

/// Row data for Table
pub struct TableRow {
    pub cells: Vec<String>,
    pub selected: bool,
}

impl TableRow {
    pub fn new(cells: Vec<&str>) -> Self {
        Self {
            cells: cells.into_iter().map(|s| s.to_string()).collect(),
            selected: false,
        }
    }
}

/// Data table with headers, sortable columns, and row selection
pub struct Table {
    pub position: Vec2,
    pub size: Vec2,
    pub columns: Vec<TableColumn>,
    pub rows: Vec<TableRow>,
    pub row_height: f32,
    pub header_height: f32,
    pub scroll_offset: f32,
    pub selected_row: Option<usize>,
    pub hovered_row: Option<usize>,
    pub corner_radius: f32,
    
    // Scrollbar state
    scrollbar_hovered: bool,
    scrollbar_dragging: bool,
    drag_start_y: f32,
    drag_start_offset: f32,
}

impl Table {
    pub fn new(columns: Vec<TableColumn>) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            columns,
            rows: Vec::new(),
            row_height: 32.0,
            header_height: 36.0,
            scroll_offset: 0.0,
            selected_row: None,
            hovered_row: None,
            corner_radius: 8.0,
            scrollbar_hovered: false,
            scrollbar_dragging: false,
            drag_start_y: 0.0,
            drag_start_offset: 0.0,
        }
    }
    
    pub fn with_rows(mut self, rows: Vec<TableRow>) -> Self {
        self.rows = rows;
        self
    }
    
    pub fn add_row(&mut self, row: TableRow) {
        self.rows.push(row);
    }
    
    fn total_width(&self) -> f32 {
        self.columns.iter().map(|c| c.width).sum()
    }
    
    fn content_height(&self) -> f32 {
        self.rows.len() as f32 * self.row_height
    }
    
    fn visible_height(&self) -> f32 {
        self.size.y - self.header_height
    }
    
    fn max_scroll(&self) -> f32 {
        (self.content_height() - self.visible_height()).max(0.0)
    }
}

impl Widget for Table {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(self.total_width().min(max_size.x), max_size.y.min(400.0));
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let inside = mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.size.x &&
                     mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.size.y;
        
        if !inside && !self.scrollbar_dragging {
            self.hovered_row = None;
            return false;
        }
        
        // Calculate hovered row
        let body_y = self.position.y + self.header_height;
        if mouse_pos.y > body_y {
            let relative_y = mouse_pos.y - body_y + self.scroll_offset;
            let row_idx = (relative_y / self.row_height) as usize;
            if row_idx < self.rows.len() {
                self.hovered_row = Some(row_idx);
            } else {
                self.hovered_row = None;
            }
        } else {
            self.hovered_row = None;
        }
        
        // Handle click
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. }, .. } = event {
            if let Some(idx) = self.hovered_row {
                // Toggle selection
                if self.selected_row == Some(idx) {
                    self.selected_row = None;
                } else {
                    self.selected_row = Some(idx);
                }
                return true;
            }
        }
        
        // Mouse wheel scrolling
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseWheel { delta, .. }, .. } = event {
            let scroll_amount = match delta {
                winit::event::MouseScrollDelta::LineDelta(_, y) => y * 40.0,
                winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
            };
            
            let old = self.scroll_offset;
            self.scroll_offset = (self.scroll_offset - scroll_amount).clamp(0.0, self.max_scroll());
            if (self.scroll_offset - old).abs() > 0.1 {
                return true;
            }
        }
        
        false
    }

    fn update(&mut self, _dt: f32) {}

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Background
        renderer.draw_rounded_rect(
            self.position,
            self.size,
            Vec4::new(0.06, 0.06, 0.08, 0.9),
            self.corner_radius
        );
        
        // Header background
        renderer.draw_rounded_rect(
            self.position,
            Vec2::new(self.size.x, self.header_height),
            Vec4::new(0.08, 0.08, 0.1, 0.95),
            self.corner_radius
        );
        
        // Header text
        let mut x = self.position.x;
        for col in &self.columns {
            renderer.draw_text(&col.header, Vec2::new(x + 12.0, self.position.y + 10.0), 14.0, theme.text);
            // Column separator
            renderer.draw_rounded_rect(
                Vec2::new(x + col.width - 1.0, self.position.y + 6.0),
                Vec2::new(1.0, self.header_height - 12.0),
                Vec4::new(0.3, 0.3, 0.35, 0.5),
                0.5
            );
            x += col.width;
        }
        
        // Set scissor for rows
        let body_y = self.position.y + self.header_height;
        renderer.set_scissor([
            self.position.x as u32,
            body_y as u32,
            self.size.x as u32,
            self.visible_height() as u32,
        ]);
        
        // Rows
        for (i, row) in self.rows.iter().enumerate() {
            let row_y = body_y + i as f32 * self.row_height - self.scroll_offset;
            
            // Skip rows outside visible area
            if row_y + self.row_height < body_y || row_y > self.position.y + self.size.y {
                continue;
            }
            
            // Row background
            let row_bg = if self.selected_row == Some(i) {
                Vec4::new(theme.primary.x, theme.primary.y, theme.primary.z, 0.3)
            } else if self.hovered_row == Some(i) {
                Vec4::new(0.15, 0.15, 0.18, 0.8)
            } else if i % 2 == 1 {
                Vec4::new(0.07, 0.07, 0.09, 0.5)
            } else {
                Vec4::ZERO
            };
            
            if row_bg.w > 0.0 {
                renderer.draw_rounded_rect(
                    Vec2::new(self.position.x + 2.0, row_y),
                    Vec2::new(self.size.x - 4.0, self.row_height),
                    row_bg,
                    4.0
                );
            }
            
            // Cell text
            let mut cell_x = self.position.x;
            for (col_idx, col) in self.columns.iter().enumerate() {
                if let Some(text) = row.cells.get(col_idx) {
                    let text_color = if self.selected_row == Some(i) { theme.text } else { theme.text };
                    renderer.draw_text(text, Vec2::new(cell_x + 12.0, row_y + 8.0), 14.0, text_color);
                }
                cell_x += col.width;
            }
        }
        
        renderer.clear_scissor();
        
        // Border
        renderer.draw_rounded_rect(
            self.position - Vec2::splat(1.0),
            self.size + Vec2::splat(2.0),
            Vec4::new(theme.border.x, theme.border.y, theme.border.z, 0.4),
            self.corner_radius + 1.0
        );
        
        // Scrollbar (if needed)
        if self.content_height() > self.visible_height() {
            let scrollbar_x = self.position.x + self.size.x - 8.0;
            let visible_ratio = (self.visible_height() / self.content_height()).min(1.0);
            let thumb_height = (self.visible_height() * visible_ratio).max(30.0);
            let scroll_ratio = if self.max_scroll() > 0.0 { self.scroll_offset / self.max_scroll() } else { 0.0 };
            let thumb_y = body_y + scroll_ratio * (self.visible_height() - thumb_height);
            
            // Track
            renderer.draw_rounded_rect(
                Vec2::new(scrollbar_x, body_y + 4.0),
                Vec2::new(6.0, self.visible_height() - 8.0),
                Vec4::new(0.1, 0.1, 0.1, 0.3),
                3.0
            );
            
            // Thumb
            renderer.draw_rounded_rect(
                Vec2::new(scrollbar_x, thumb_y),
                Vec2::new(6.0, thumb_height),
                Vec4::new(0.4, 0.4, 0.4, 0.6),
                3.0
            );
        }
    }
}

// =============================================================================
// LIST
// =============================================================================

/// Simple list item
pub struct ListItem {
    pub text: String,
    pub icon: Option<String>,
}

impl ListItem {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            icon: None,
        }
    }
    
    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }
}

/// Scrollable list view
pub struct ListView {
    pub position: Vec2,
    pub size: Vec2,
    pub items: Vec<ListItem>,
    pub item_height: f32,
    pub selected_index: Option<usize>,
    pub hovered_index: Option<usize>,
    pub scroll_offset: f32,
    pub corner_radius: f32,
}

impl ListView {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            items: Vec::new(),
            item_height: 36.0,
            selected_index: None,
            hovered_index: None,
            scroll_offset: 0.0,
            corner_radius: 8.0,
        }
    }
    
    pub fn with_items(mut self, items: Vec<ListItem>) -> Self {
        self.items = items;
        self
    }
    
    pub fn add_item(&mut self, item: ListItem) {
        self.items.push(item);
    }
    
    fn content_height(&self) -> f32 {
        self.items.len() as f32 * self.item_height
    }
    
    fn max_scroll(&self) -> f32 {
        (self.content_height() - self.size.y).max(0.0)
    }
}

impl Default for ListView {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for ListView {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(max_size.x.min(300.0), max_size.y.min(400.0));
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let inside = mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.size.x &&
                     mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.size.y;
        
        if !inside {
            self.hovered_index = None;
            return false;
        }
        
        // Calculate hovered item
        let relative_y = mouse_pos.y - self.position.y + self.scroll_offset;
        let idx = (relative_y / self.item_height) as usize;
        self.hovered_index = if idx < self.items.len() { Some(idx) } else { None };
        
        // Handle click
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. }, .. } = event {
            if let Some(idx) = self.hovered_index {
                self.selected_index = Some(idx);
                return true;
            }
        }
        
        // Mouse wheel
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseWheel { delta, .. }, .. } = event {
            let amount = match delta {
                winit::event::MouseScrollDelta::LineDelta(_, y) => y * 40.0,
                winit::event::MouseScrollDelta::PixelDelta(p) => p.y as f32,
            };
            self.scroll_offset = (self.scroll_offset - amount).clamp(0.0, self.max_scroll());
            return true;
        }
        
        false
    }

    fn update(&mut self, _dt: f32) {}

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Background
        renderer.draw_rounded_rect(
            self.position,
            self.size,
            Vec4::new(0.06, 0.06, 0.08, 0.9),
            self.corner_radius
        );
        
        // Set scissor
        renderer.set_scissor([
            self.position.x as u32,
            self.position.y as u32,
            self.size.x as u32,
            self.size.y as u32,
        ]);
        
        // Items
        for (i, item) in self.items.iter().enumerate() {
            let item_y = self.position.y + i as f32 * self.item_height - self.scroll_offset;
            
            if item_y + self.item_height < self.position.y || item_y > self.position.y + self.size.y {
                continue;
            }
            
            // Background
            let bg = if self.selected_index == Some(i) {
                Vec4::new(theme.primary.x, theme.primary.y, theme.primary.z, 0.3)
            } else if self.hovered_index == Some(i) {
                Vec4::new(0.15, 0.15, 0.18, 0.8)
            } else {
                Vec4::ZERO
            };
            
            if bg.w > 0.0 {
                renderer.draw_rounded_rect(
                    Vec2::new(self.position.x + 4.0, item_y + 2.0),
                    Vec2::new(self.size.x - 8.0, self.item_height - 4.0),
                    bg,
                    4.0
                );
            }
            
            // Icon
            let text_x = if let Some(icon) = &item.icon {
                renderer.draw_text(icon, Vec2::new(self.position.x + 12.0, item_y + 10.0), 16.0, theme.text_secondary);
                self.position.x + 36.0
            } else {
                self.position.x + 12.0
            };
            
            // Text
            renderer.draw_text(&item.text, Vec2::new(text_x, item_y + 10.0), 15.0, theme.text);
        }
        
        renderer.clear_scissor();
        
        // Border
        renderer.draw_rounded_rect(
            self.position - Vec2::splat(1.0),
            self.size + Vec2::splat(2.0),
            Vec4::new(theme.border.x, theme.border.y, theme.border.z, 0.4),
            self.corner_radius + 1.0
        );
    }
}

// =============================================================================
// TREE VIEW
// =============================================================================

/// Node in a tree view
pub struct TreeNode {
    pub label: String,
    pub icon: Option<String>,
    pub children: Vec<TreeNode>,
    pub expanded: bool,
    pub selected: bool,
}

impl TreeNode {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            icon: None,
            children: Vec::new(),
            expanded: false,
            selected: false,
        }
    }
    
    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }
    
    pub fn with_children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }
    
    pub fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }
    
    fn count_visible(&self) -> usize {
        if self.expanded {
            1 + self.children.iter().map(|c| c.count_visible()).sum::<usize>()
        } else {
            1
        }
    }
}

/// Hierarchical tree view
pub struct TreeView {
    pub position: Vec2,
    pub size: Vec2,
    pub roots: Vec<TreeNode>,
    pub item_height: f32,
    pub indent: f32,
    pub scroll_offset: f32,
    pub corner_radius: f32,
    hovered_path: Option<Vec<usize>>,
    selected_path: Option<Vec<usize>>,
}

impl TreeView {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            roots: Vec::new(),
            item_height: 28.0,
            indent: 20.0,
            scroll_offset: 0.0,
            corner_radius: 8.0,
            hovered_path: None,
            selected_path: None,
        }
    }
    
    pub fn with_roots(mut self, roots: Vec<TreeNode>) -> Self {
        self.roots = roots;
        self
    }
    
    fn count_visible(&self) -> usize {
        self.roots.iter().map(|r| r.count_visible()).sum()
    }
    
    fn content_height(&self) -> f32 {
        self.count_visible() as f32 * self.item_height
    }
    
    fn max_scroll(&self) -> f32 {
        (self.content_height() - self.size.y).max(0.0)
    }
}

impl Default for TreeView {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for TreeView {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(max_size.x.min(300.0), max_size.y.min(400.0));
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let inside = mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.size.x &&
                     mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.size.y;
        
        if !inside {
            self.hovered_path = None;
            return false;
        }
        
        // Mouse wheel
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseWheel { delta, .. }, .. } = event {
            let amount = match delta {
                winit::event::MouseScrollDelta::LineDelta(_, y) => y * 40.0,
                winit::event::MouseScrollDelta::PixelDelta(p) => p.y as f32,
            };
            self.scroll_offset = (self.scroll_offset - amount).clamp(0.0, self.max_scroll());
            return true;
        }
        
        false
    }

    fn update(&mut self, _dt: f32) {}

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Background
        renderer.draw_rounded_rect(
            self.position,
            self.size,
            Vec4::new(0.06, 0.06, 0.08, 0.9),
            self.corner_radius
        );
        
        // Set scissor
        renderer.set_scissor([
            self.position.x as u32,
            self.position.y as u32,
            self.size.x as u32,
            self.size.y as u32,
        ]);
        
        // Render tree recursively
        let mut y = self.position.y - self.scroll_offset;
        for root in &self.roots {
            y = self.render_node(renderer, root, 0, y, &theme);
        }
        
        renderer.clear_scissor();
        
        // Border
        renderer.draw_rounded_rect(
            self.position - Vec2::splat(1.0),
            self.size + Vec2::splat(2.0),
            Vec4::new(theme.border.x, theme.border.y, theme.border.z, 0.4),
            self.corner_radius + 1.0
        );
    }
}

impl TreeView {
    fn render_node(&self, renderer: &mut GlassRenderer, node: &TreeNode, depth: usize, y: f32, theme: &super::core::Theme) -> f32 {
        let mut current_y = y;
        
        // Skip if outside visible area
        if current_y + self.item_height >= self.position.y && current_y <= self.position.y + self.size.y {
            let x = self.position.x + depth as f32 * self.indent + 8.0;
            
            // Expand/collapse arrow
            if !node.children.is_empty() {
                let arrow = if node.expanded { "▼" } else { "▶" };
                renderer.draw_text(arrow, Vec2::new(x, current_y + 6.0), 12.0, theme.text_secondary);
            }
            
            // Icon
            let text_x = if let Some(icon) = &node.icon {
                renderer.draw_text(icon, Vec2::new(x + 18.0, current_y + 6.0), 14.0, theme.text_secondary);
                x + 38.0
            } else {
                x + 18.0
            };
            
            // Label
            renderer.draw_text(&node.label, Vec2::new(text_x, current_y + 6.0), 14.0, theme.text);
        }
        
        current_y += self.item_height;
        
        // Render children if expanded
        if node.expanded {
            for child in &node.children {
                current_y = self.render_node(renderer, child, depth + 1, current_y, theme);
            }
        }
        
        current_y
    }
}
