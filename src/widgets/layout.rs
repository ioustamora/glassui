//! GlassUI Layout Widgets
//!
//! Container widgets for arranging child widgets: Row, Column, Stack, Grid, Flex, Align, Spacer

use glam::{Vec2, Vec4};
use winit::event::{ElementState, MouseButton};
use crate::renderer::GlassRenderer;
use super::core::{Widget, get_theme};

// =============================================================================
// COLUMN
// =============================================================================

/// Vertical layout container
pub struct Column {
    pub position: Vec2,
    pub size: Vec2,
    pub children: Vec<Box<dyn Widget>>,
    pub spacing: f32,
    pub padding: f32,
}

impl Column {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            children: Vec::new(),
            spacing: 10.0,
            padding: 10.0,
        }
    }
    
    pub fn add_child(mut self, child: Box<dyn Widget>) -> Self {
        self.children.push(child);
        self
    }
    
    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
    
    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }
}

impl Default for Column {
    fn default() -> Self { Self::new() }
}

impl Widget for Column {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        
        let mut cursor = origin + Vec2::splat(self.padding);
        let content_width = max_size.x - self.padding * 2.0;
        let mut max_width = 0.0f32;
        
        for child in &mut self.children {
            let used_height = cursor.y - origin.y - self.padding;
            let remaining_height = (max_size.y - self.padding * 2.0 - used_height).max(0.0);
            
            let child_size = child.layout(cursor, Vec2::new(content_width, remaining_height)); 
            cursor.y += child_size.y + self.spacing;
            max_width = max_width.max(child_size.x);
        }
        
        self.size = Vec2::new(max_width + self.padding * 2.0, cursor.y - origin.y + self.padding - self.spacing);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let mut handled = false;
        for child in &mut self.children {
            if child.handle_event(event, mouse_pos) {
                handled = true;
            }
        }
        handled
    }

    fn update(&mut self, dt: f32) {
        for child in &mut self.children {
            child.update(dt);
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        for child in &self.children {
            child.render(renderer);
        }
    }
}

// =============================================================================
// ROW
// =============================================================================

/// Horizontal layout container
pub struct Row {
    pub position: Vec2,
    pub size: Vec2,
    pub children: Vec<Box<dyn Widget>>,
    pub spacing: f32,
    pub padding: f32,
}

impl Row {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            children: Vec::new(),
            spacing: 10.0,
            padding: 10.0,
        }
    }
    
    pub fn add_child(mut self, child: Box<dyn Widget>) -> Self {
        self.children.push(child);
        self
    }
    
    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl Default for Row {
    fn default() -> Self { Self::new() }
}

impl Widget for Row {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        
        let mut cursor = origin + Vec2::splat(self.padding);
        let mut max_height = 0.0f32;
        
        for child in &mut self.children {
            let used_width = cursor.x - origin.x - self.padding;
            let remaining_width = (max_size.x - self.padding * 2.0 - used_width).max(0.0);
            
            let child_size = child.layout(cursor, Vec2::new(remaining_width, max_size.y)); 
            cursor.x += child_size.x + self.spacing;
            max_height = max_height.max(child_size.y);
        }
        
        self.size = Vec2::new(cursor.x - origin.x + self.padding - self.spacing, max_height + self.padding * 2.0);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let mut handled = false;
        for child in &mut self.children {
            if child.handle_event(event, mouse_pos) {
                handled = true;
            }
        }
        handled
    }

    fn update(&mut self, dt: f32) {
        for child in &mut self.children {
            child.update(dt);
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        for child in &self.children {
            child.render(renderer);
        }
    }
}

// =============================================================================
// STACK
// =============================================================================

/// Z-layered overlay container (last child on top)
pub struct Stack {
    pub position: Vec2,
    pub size: Vec2,
    pub children: Vec<Box<dyn Widget>>,
}

impl Stack {
    pub fn new() -> Self {
        Self { position: Vec2::ZERO, size: Vec2::ZERO, children: Vec::new() }
    }
    
    pub fn add_child(mut self, child: Box<dyn Widget>) -> Self {
        self.children.push(child);
        self
    }
}

impl Default for Stack {
    fn default() -> Self { Self::new() }
}

impl Widget for Stack {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = max_size;
        
        for child in &mut self.children {
            child.layout(origin, max_size);
        }
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        // Reverse order for events (top-most first)
        let mut handled_index = None;
        let len = self.children.len();
        
        for i in (0..len).rev() {
            if self.children[i].handle_event(event, mouse_pos) {
                handled_index = Some(i);
                break;
            }
        }
        
        if let Some(index) = handled_index {
            // Bring to front (end of list) if interaction happened
            if index != len - 1 {
                let child = self.children.remove(index);
                self.children.push(child);
            }
            return true;
        }
        
        false
    }
    
    fn update(&mut self, dt: f32) {
        for child in &mut self.children {
            child.update(dt);
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        for child in &self.children {
            child.render(renderer);
        }
    }
}

// =============================================================================
// SPACER
// =============================================================================

/// Flexible spacing element
pub struct Spacer {
    pub size: Vec2,
}

impl Spacer {
    pub fn new(size: Vec2) -> Self {
        Self { size }
    }
    
    pub fn horizontal(width: f32) -> Self {
        Self { size: Vec2::new(width, 0.0) }
    }
    
    pub fn vertical(height: f32) -> Self {
        Self { size: Vec2::new(0.0, height) }
    }
}

impl Widget for Spacer {
    fn layout(&mut self, _origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.size
    }
    fn handle_event(&mut self, _event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool { false }
    fn update(&mut self, _dt: f32) {}
    fn render(&self, _renderer: &mut GlassRenderer) {}
}

// =============================================================================
// ALIGN
// =============================================================================

/// Alignment options for Align widget
pub enum Alignment {
    Center,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Alignment wrapper
pub struct Align {
    pub alignment: Alignment,
    pub child: Box<dyn Widget>,
    pub position: Vec2,
    pub size: Vec2,
}

impl Align {
    pub fn new(alignment: Alignment, child: Box<dyn Widget>) -> Self {
        Self {
            alignment,
            child,
            position: Vec2::ZERO,
            size: Vec2::ZERO,
        }
    }
    
    pub fn center(child: Box<dyn Widget>) -> Self {
        Self::new(Alignment::Center, child)
    }
}

impl Widget for Align {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = max_size;
        
        let child_size = self.child.layout(origin, max_size);
        
        let final_pos = match self.alignment {
            Alignment::Center => origin + (max_size - child_size) * 0.5,
            Alignment::TopLeft => origin,
            Alignment::TopRight => Vec2::new(origin.x + max_size.x - child_size.x, origin.y),
            Alignment::BottomLeft => Vec2::new(origin.x, origin.y + max_size.y - child_size.y),
            Alignment::BottomRight => origin + max_size - child_size,
        };
        
        self.child.layout(final_pos, max_size);
        
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        self.child.handle_event(event, mouse_pos)
    }

    fn update(&mut self, dt: f32) {
        self.child.update(dt);
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        self.child.render(renderer);
    }
}

// =============================================================================
// GRID
// =============================================================================

/// CSS Grid-inspired multi-column layout
pub struct Grid {
    pub position: Vec2,
    pub size: Vec2,
    pub children: Vec<Box<dyn Widget>>,
    pub columns: usize,
    pub gap: f32,
    pub padding: f32,
}

impl Grid {
    pub fn new(columns: usize) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            children: Vec::new(),
            columns: columns.max(1),
            gap: 10.0,
            padding: 10.0,
        }
    }
    
    pub fn add_child(mut self, child: Box<dyn Widget>) -> Self {
        self.children.push(child);
        self
    }
    
    pub fn with_gap(mut self, gap: f32) -> Self { self.gap = gap; self }
}

impl Widget for Grid {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        let cols = self.columns.max(1);
        let content_width = max_size.x - self.padding * 2.0;
        let cell_width = (content_width - (cols - 1) as f32 * self.gap) / cols as f32;
        
        let mut max_row_height = 0.0f32;
        let mut total_height = self.padding;
        let child_count = self.children.len();
        
        for (i, child) in self.children.iter_mut().enumerate() {
            let col = i % cols;
            let x = origin.x + self.padding + col as f32 * (cell_width + self.gap);
            let y = origin.y + total_height;
            
            let child_size = child.layout(Vec2::new(x, y), Vec2::new(cell_width, 10000.0));
            max_row_height = max_row_height.max(child_size.y);
            
            if col == cols - 1 || i == child_count - 1 {
                total_height += max_row_height + self.gap;
                max_row_height = 0.0;
            }
        }
        
        self.size = Vec2::new(max_size.x, total_height + self.padding);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        self.children.iter_mut().any(|c| c.handle_event(event, mouse_pos))
    }
    
    fn update(&mut self, dt: f32) { 
        self.children.iter_mut().for_each(|c| c.update(dt)); 
    }
    
    fn render(&self, renderer: &mut GlassRenderer) { 
        self.children.iter().for_each(|c| c.render(renderer)); 
    }
}

// =============================================================================
// FLEX
// =============================================================================

#[derive(Clone, Copy, PartialEq)]
pub enum FlexDirection { Row, Column }

#[derive(Clone, Copy, PartialEq)]
pub enum FlexJustify { Start, Center, End, SpaceBetween, SpaceAround }

#[derive(Clone, Copy, PartialEq)]
pub enum FlexAlign { Start, Center, End, Stretch }

/// Flexbox-style layout with justify/align options
pub struct Flex {
    pub position: Vec2,
    pub size: Vec2,
    pub children: Vec<Box<dyn Widget>>,
    pub direction: FlexDirection,
    pub justify: FlexJustify,
    pub align: FlexAlign,
    pub gap: f32,
    pub padding: f32,
}

impl Flex {
    pub fn new(direction: FlexDirection) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            children: Vec::new(),
            direction,
            justify: FlexJustify::Start,
            align: FlexAlign::Start,
            gap: 10.0,
            padding: 10.0,
        }
    }
    
    pub fn row() -> Self { Self::new(FlexDirection::Row) }
    pub fn column() -> Self { Self::new(FlexDirection::Column) }
    
    pub fn add_child(mut self, child: Box<dyn Widget>) -> Self { 
        self.children.push(child); 
        self 
    }
    
    pub fn with_justify(mut self, j: FlexJustify) -> Self { self.justify = j; self }
    pub fn with_align(mut self, a: FlexAlign) -> Self { self.align = a; self }
    pub fn with_gap(mut self, g: f32) -> Self { self.gap = g; self }
}

impl Widget for Flex {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        let is_row = self.direction == FlexDirection::Row;
        let content_area = max_size - Vec2::splat(self.padding * 2.0);
        
        // First pass: measure children
        let mut child_sizes = Vec::new();
        let mut total_main = 0.0f32;
        let mut max_cross = 0.0f32;
        
        for child in &mut self.children {
            let size = child.layout(origin, content_area);
            let (main, cross) = if is_row { (size.x, size.y) } else { (size.y, size.x) };
            total_main += main;
            max_cross = max_cross.max(cross);
            child_sizes.push(size);
        }
        
        let main_axis = if is_row { content_area.x } else { content_area.y };
        let total_gaps = self.gap * (self.children.len().saturating_sub(1)) as f32;
        let free_space = (main_axis - total_main - total_gaps).max(0.0);
        
        // Calculate start offset and spacing
        let (start_offset, item_spacing) = match self.justify {
            FlexJustify::Start => (0.0, self.gap),
            FlexJustify::End => (free_space, self.gap),
            FlexJustify::Center => (free_space / 2.0, self.gap),
            FlexJustify::SpaceBetween if self.children.len() > 1 => 
                (0.0, self.gap + free_space / (self.children.len() - 1) as f32),
            FlexJustify::SpaceAround if !self.children.is_empty() => {
                let s = free_space / self.children.len() as f32;
                (s / 2.0, self.gap + s)
            },
            _ => (0.0, self.gap),
        };
        
        // Second pass: position children
        let mut cursor = start_offset;
        for (i, child) in self.children.iter_mut().enumerate() {
            let size = child_sizes[i];
            let (main_size, cross_size) = if is_row { (size.x, size.y) } else { (size.y, size.x) };
            
            let cross_offset = match self.align {
                FlexAlign::Start => 0.0,
                FlexAlign::End => max_cross - cross_size,
                FlexAlign::Center => (max_cross - cross_size) / 2.0,
                FlexAlign::Stretch => 0.0,
            };
            
            let pos = if is_row {
                Vec2::new(origin.x + self.padding + cursor, origin.y + self.padding + cross_offset)
            } else {
                Vec2::new(origin.x + self.padding + cross_offset, origin.y + self.padding + cursor)
            };
            
            child.layout(pos, size);
            cursor += main_size + item_spacing;
        }
        
        self.size = if is_row {
            Vec2::new(max_size.x, max_cross + self.padding * 2.0)
        } else {
            Vec2::new(max_cross + self.padding * 2.0, cursor - item_spacing + self.padding * 2.0)
        };
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        self.children.iter_mut().any(|c| c.handle_event(event, mouse_pos))
    }
    
    fn update(&mut self, dt: f32) { 
        self.children.iter_mut().for_each(|c| c.update(dt)); 
    }
    
    fn render(&self, renderer: &mut GlassRenderer) { 
        self.children.iter().for_each(|c| c.render(renderer)); 
    }
}
