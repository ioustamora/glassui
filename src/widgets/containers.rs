//! GlassUI Container Widgets
//!
//! Container widgets that manage child content: ScrollArea, TabBar

use glam::{Vec2, Vec4};
use winit::event::{ElementState, MouseButton};
use crate::renderer::GlassRenderer;
use super::core::{Widget, get_theme};

// =============================================================================
// SCROLL AREA
// =============================================================================

/// Scrollable container with optional scrollbar
pub struct ScrollArea {
    pub position: Vec2,
    pub size: Vec2,
    pub child: Box<dyn Widget>,
    pub scroll_offset: f32,
    pub content_height: f32,
    pub scrollbar_hovered: bool,
    pub scrollbar_dragging: bool,
    pub drag_start_y: f32,
    pub drag_start_offset: f32,
}

impl ScrollArea {
    pub fn new(child: Box<dyn Widget>) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            child,
            scroll_offset: 0.0,
            content_height: 0.0,
            scrollbar_hovered: false,
            scrollbar_dragging: false,
            drag_start_y: 0.0,
            drag_start_offset: 0.0,
        }
    }
}

impl Widget for ScrollArea {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = max_size;
        
        let child_origin = origin + Vec2::new(0.0, -self.scroll_offset);
        let child_size = self.child.layout(child_origin, Vec2::new(max_size.x - 12.0, 10000.0));
        self.content_height = child_size.y;
        
        let max_scroll = (self.content_height - self.size.y).max(0.0);
        self.scroll_offset = self.scroll_offset.clamp(0.0, max_scroll);
        
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let inside = mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.size.x &&
                     mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.size.y;

        // Pass to child first
        if inside && self.child.handle_event(event, mouse_pos) {
            return true;
        }
        
        // Scrollbar handling
        let scrollbar_x = self.position.x + self.size.x - 10.0;
        let visible_ratio = (self.size.y / self.content_height).min(1.0);
        let thumb_height = (self.size.y * visible_ratio).max(30.0);
        let max_scroll = (self.content_height - self.size.y).max(0.0);
        let scroll_ratio = if max_scroll > 0.0 { self.scroll_offset / max_scroll } else { 0.0 };
        let thumb_y = self.position.y + scroll_ratio * (self.size.y - thumb_height);
        
        let in_scrollbar = mouse_pos.x >= scrollbar_x && mouse_pos.x <= scrollbar_x + 10.0 &&
                           mouse_pos.y >= thumb_y && mouse_pos.y <= thumb_y + thumb_height;
        
        self.scrollbar_hovered = in_scrollbar;
        
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state, button: MouseButton::Left, .. }, .. } = event {
            if *state == ElementState::Pressed && in_scrollbar {
                self.scrollbar_dragging = true;
                self.drag_start_y = mouse_pos.y;
                self.drag_start_offset = self.scroll_offset;
                return true;
            } else if *state == ElementState::Released {
                self.scrollbar_dragging = false;
            }
        }
        
        if self.scrollbar_dragging {
            let delta_y = mouse_pos.y - self.drag_start_y;
            let scroll_range = self.size.y - thumb_height;
            if scroll_range > 0.0 {
                let delta_scroll = (delta_y / scroll_range) * max_scroll;
                self.scroll_offset = (self.drag_start_offset + delta_scroll).clamp(0.0, max_scroll);
            }
            return true;
        }
        
        // Mouse wheel
        if inside {
            if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseWheel { delta, .. }, .. } = event {
                let scroll_amount = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => y * 40.0,
                    winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
                };
                
                let old_scroll = self.scroll_offset;
                self.scroll_offset = (self.scroll_offset - scroll_amount).clamp(0.0, max_scroll);
                
                if (self.scroll_offset - old_scroll).abs() > 0.1 {
                    return true;
                }
            }
        }
        
        false
    }

    fn update(&mut self, dt: f32) {
        self.child.update(dt);
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Draw background frame BEFORE scissor (so border is always visible)
        renderer.draw_rounded_rect(
            self.position - Vec2::splat(1.0),
            self.size + Vec2::splat(2.0),
            Vec4::new(0.2, 0.2, 0.25, 0.3),
            8.0
        );
        
        // Set scissor for clipping content
        renderer.set_scissor([
            self.position.x as u32,
            self.position.y as u32,
            self.size.x as u32,
            self.size.y as u32,
        ]);
        
        self.child.render(renderer);
        
        renderer.clear_scissor();
        
        // Draw visible border frame AFTER scissor cleared (renders on top)
        renderer.draw_rounded_rect(
            self.position - Vec2::splat(1.0),
            self.size + Vec2::splat(2.0),
            Vec4::new(theme.border.x, theme.border.y, theme.border.z, 0.4),
            8.0
        );
        
        // Scrollbar
        if self.content_height > self.size.y {
            let scrollbar_x = self.position.x + self.size.x - 8.0;
            let visible_ratio = (self.size.y / self.content_height).min(1.0);
            let thumb_height = (self.size.y * visible_ratio).max(30.0);
            let max_scroll = self.content_height - self.size.y;
            let scroll_ratio = if max_scroll > 0.0 { self.scroll_offset / max_scroll } else { 0.0 };
            let thumb_y = self.position.y + scroll_ratio * (self.size.y - thumb_height);
            
            // Track
            renderer.draw_rounded_rect(
                Vec2::new(scrollbar_x, self.position.y + 4.0),
                Vec2::new(6.0, self.size.y - 8.0),
                Vec4::new(0.1, 0.1, 0.1, 0.3),
                3.0
            );
            
            // Thumb
            let thumb_color = if self.scrollbar_dragging {
                theme.primary
            } else if self.scrollbar_hovered {
                Vec4::new(0.6, 0.6, 0.6, 0.8)
            } else {
                Vec4::new(0.4, 0.4, 0.4, 0.6)
            };
            
            renderer.draw_rounded_rect(
                Vec2::new(scrollbar_x, thumb_y),
                Vec2::new(6.0, thumb_height),
                thumb_color,
                3.0
            );
        }
    }
}

// =============================================================================
// TAB BAR
// =============================================================================

/// Tab bar for switching between views
pub struct TabBar {
    pub position: Vec2,
    pub size: Vec2,
    pub tabs: Vec<String>,
    pub children: Vec<Box<dyn Widget>>,
    pub active_index: usize,
    pub hovered_index: Option<usize>,
    pub tab_animated_t: f32,
}

impl TabBar {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            tabs: Vec::new(),
            children: Vec::new(),
            active_index: 0,
            hovered_index: None,
            tab_animated_t: 0.0,
        }
    }
    
    pub fn add_tab(mut self, name: &str, content: Box<dyn Widget>) -> Self {
        self.tabs.push(name.to_string());
        self.children.push(content);
        self
    }
}

impl Default for TabBar {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for TabBar {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = max_size;
        
        let tab_height = 40.0;
        let content_origin = origin + Vec2::new(0.0, tab_height);
        let content_size = max_size - Vec2::new(0.0, tab_height);
        
        if let Some(child) = self.children.get_mut(self.active_index) {
            child.layout(content_origin, content_size);
        }
        
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let tab_height = 40.0;
        let tab_width = if self.tabs.is_empty() { 0.0 } else { self.size.x / self.tabs.len() as f32 };
        
        // Check if in tab bar
        let in_tab_bar = mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + tab_height &&
                         mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.size.x;
        
        if in_tab_bar {
            let relative_x = mouse_pos.x - self.position.x;
            let index = (relative_x / tab_width) as usize;
            if index < self.tabs.len() {
                self.hovered_index = Some(index);
            }
            
            if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. }, .. } = event {
                if let Some(idx) = self.hovered_index {
                    self.active_index = idx;
                    return true;
                }
            }
        } else {
            self.hovered_index = None;
        }
        
        // Pass to active child
        if let Some(child) = self.children.get_mut(self.active_index) {
            if child.handle_event(event, mouse_pos) {
                return true;
            }
        }
        
        false
    }

    fn update(&mut self, dt: f32) {
        // Animate tab indicator
        let target = self.active_index as f32;
        self.tab_animated_t += (target - self.tab_animated_t) * 12.0 * dt;
        
        if let Some(child) = self.children.get_mut(self.active_index) {
            child.update(dt);
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        let tab_height = 40.0;
        let tab_width = if self.tabs.is_empty() { 0.0 } else { self.size.x / self.tabs.len() as f32 };
        
        // Tab bar background
        renderer.draw_rounded_rect(
            self.position,
            Vec2::new(self.size.x, tab_height),
            Vec4::new(0.06, 0.06, 0.08, 0.9),
            8.0
        );
        
        // Active indicator (animated)
        let indicator_x = self.position.x + self.tab_animated_t * tab_width;
        renderer.draw_rounded_rect(
            Vec2::new(indicator_x + 4.0, self.position.y + tab_height - 4.0),
            Vec2::new(tab_width - 8.0, 3.0),
            theme.primary,
            2.0
        );
        
        // Tab labels
        for (i, tab) in self.tabs.iter().enumerate() {
            let tab_x = self.position.x + i as f32 * tab_width;
            
            // Hover bg
            if self.hovered_index == Some(i) {
                renderer.draw_rounded_rect(
                    Vec2::new(tab_x + 2.0, self.position.y + 2.0),
                    Vec2::new(tab_width - 4.0, tab_height - 4.0),
                    Vec4::new(1.0, 1.0, 1.0, 0.05),
                    6.0
                );
            }
            
            let color = if i == self.active_index { theme.text } else { theme.text_secondary };
            let text_x = tab_x + (tab_width - tab.len() as f32 * 8.0) / 2.0;
            renderer.draw_text(tab, Vec2::new(text_x, self.position.y + 10.0), 16.0, color);
        }
        
        // Active content
        if let Some(child) = self.children.get(self.active_index) {
            child.render(renderer);
        }
    }
}
