//! GlassUI Tab View Widget
//!
//! Tabbed container for organizing content:
//! - Horizontal and vertical tabs
//! - Animated tab switching
//! - Closeable tabs
//! - Tab overflow handling

use glam::{Vec2, Vec4};
use crate::renderer::GlassRenderer;
use crate::widget_id::WidgetId;
use crate::widgets::core::{Widget, get_theme};

// =============================================================================
// TAB
// =============================================================================

/// Single tab definition
#[derive(Clone, Debug)]
pub struct Tab {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub closeable: bool,
    pub badge: Option<String>,
}

impl Tab {
    pub fn new(id: &str, label: &str) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            icon: None,
            closeable: false,
            badge: None,
        }
    }
    
    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }
    
    pub fn closeable(mut self) -> Self {
        self.closeable = true;
        self
    }
    
    pub fn with_badge(mut self, badge: &str) -> Self {
        self.badge = Some(badge.to_string());
        self
    }
}

// =============================================================================
// TAB BAR POSITION
// =============================================================================

/// Position of the tab bar
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TabPosition {
    Top,
    Bottom,
    Left,
    Right,
}

// =============================================================================
// TAB VIEW
// =============================================================================

/// Tabbed container widget
pub struct TabView {
    pub id: WidgetId,
    pub position: Vec2,
    pub size: Vec2,
    pub tabs: Vec<Tab>,
    pub selected_index: usize,
    pub tab_position: TabPosition,
    pub hovered_tab: Option<usize>,
    pub hovered_close: Option<usize>,
    // Animation
    indicator_x: f32,
    target_indicator_x: f32,
    // Callbacks
    pub on_tab_change: Option<Box<dyn FnMut(usize, &str)>>,
    pub on_tab_close: Option<Box<dyn FnMut(usize, &str)>>,
}

impl TabView {
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::new(400.0, 300.0),
            tabs: Vec::new(),
            selected_index: 0,
            tab_position: TabPosition::Top,
            hovered_tab: None,
            hovered_close: None,
            indicator_x: 0.0,
            target_indicator_x: 0.0,
            on_tab_change: None,
            on_tab_close: None,
        }
    }
    
    /// Add a tab
    pub fn add_tab(&mut self, tab: Tab) {
        self.tabs.push(tab);
    }
    
    /// Remove a tab by index
    pub fn remove_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.tabs.remove(index);
            if self.selected_index >= self.tabs.len() && self.selected_index > 0 {
                self.selected_index -= 1;
            }
        }
    }
    
    /// Select a tab by index
    pub fn select(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.selected_index = index;
            self.update_indicator_target();
        }
    }
    
    /// Select a tab by ID
    pub fn select_by_id(&mut self, id: &str) {
        if let Some(index) = self.tabs.iter().position(|t| t.id == id) {
            self.select(index);
        }
    }
    
    /// Get currently selected tab
    pub fn selected_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.selected_index)
    }
    
    fn tab_width(&self) -> f32 {
        if self.tabs.is_empty() { return 0.0; }
        (self.size.x / self.tabs.len() as f32).min(120.0)
    }
    
    fn tab_height(&self) -> f32 {
        36.0
    }
    
    fn update_indicator_target(&mut self) {
        self.target_indicator_x = self.selected_index as f32 * self.tab_width();
    }
    
    fn content_area(&self) -> (Vec2, Vec2) {
        let tab_h = self.tab_height();
        match self.tab_position {
            TabPosition::Top => (
                self.position + Vec2::new(0.0, tab_h),
                self.size - Vec2::new(0.0, tab_h)
            ),
            TabPosition::Bottom => (
                self.position,
                self.size - Vec2::new(0.0, tab_h)
            ),
            _ => (self.position, self.size),
        }
    }
}

impl Default for TabView {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for TabView {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = max_size;
        self.update_indicator_target();
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let tab_w = self.tab_width();
        let tab_h = self.tab_height();
        let tabs_y = self.position.y;
        
        // Update hover state
        self.hovered_tab = None;
        self.hovered_close = None;
        
        if mouse_pos.y >= tabs_y && mouse_pos.y <= tabs_y + tab_h {
            for i in 0..self.tabs.len() {
                let tab_x = self.position.x + i as f32 * tab_w;
                if mouse_pos.x >= tab_x && mouse_pos.x <= tab_x + tab_w {
                    self.hovered_tab = Some(i);
                    
                    // Check close button area
                    if self.tabs[i].closeable {
                        let close_x = tab_x + tab_w - 20.0;
                        if mouse_pos.x >= close_x && mouse_pos.x <= close_x + 16.0 {
                            self.hovered_close = Some(i);
                        }
                    }
                    break;
                }
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
            if let Some(index) = self.hovered_close {
                // Close tab
                let tab_id = self.tabs[index].id.clone();
                if let Some(callback) = &mut self.on_tab_close {
                    callback(index, &tab_id);
                }
                self.remove_tab(index);
                return true;
            } else if let Some(index) = self.hovered_tab {
                // Select tab
                self.select(index);
                if let Some(callback) = &mut self.on_tab_change {
                    callback(index, &self.tabs[index].id);
                }
                return true;
            }
        }
        
        false
    }

    fn update(&mut self, dt: f32) {
        // Animate indicator
        self.indicator_x += (self.target_indicator_x - self.indicator_x) * 12.0 * dt;
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        let tab_w = self.tab_width();
        let tab_h = self.tab_height();
        
        // Tab bar background
        renderer.draw_rounded_rect(
            self.position,
            Vec2::new(self.size.x, tab_h),
            Vec4::new(0.08, 0.08, 0.1, 0.9),
            8.0
        );
        
        // Tabs
        for (i, tab) in self.tabs.iter().enumerate() {
            let tab_x = self.position.x + i as f32 * tab_w;
            let is_selected = i == self.selected_index;
            let is_hovered = self.hovered_tab == Some(i);
            
            // Tab background
            if is_selected {
                renderer.draw_rounded_rect(
                    Vec2::new(tab_x + 2.0, self.position.y + 2.0),
                    Vec2::new(tab_w - 4.0, tab_h - 4.0),
                    Vec4::new(theme.primary.x, theme.primary.y, theme.primary.z, 0.3),
                    6.0
                );
            } else if is_hovered {
                renderer.draw_rounded_rect(
                    Vec2::new(tab_x + 2.0, self.position.y + 2.0),
                    Vec2::new(tab_w - 4.0, tab_h - 4.0),
                    Vec4::new(1.0, 1.0, 1.0, 0.1),
                    6.0
                );
            }
            
            // Icon
            let mut text_x = tab_x + 12.0;
            if let Some(icon) = &tab.icon {
                renderer.draw_text(icon, Vec2::new(text_x, self.position.y + 10.0), 14.0, theme.text);
                text_x += 20.0;
            }
            
            // Label
            let label_color = if is_selected { theme.text } else { theme.text_secondary };
            renderer.draw_text(&tab.label, Vec2::new(text_x, self.position.y + 10.0), 13.0, label_color);
            
            // Badge
            if let Some(badge) = &tab.badge {
                let badge_x = tab_x + tab_w - 30.0;
                renderer.draw_rounded_rect(
                    Vec2::new(badge_x, self.position.y + 8.0),
                    Vec2::new(18.0, 18.0),
                    theme.primary,
                    9.0
                );
                renderer.draw_text(badge, Vec2::new(badge_x + 5.0, self.position.y + 10.0), 10.0, theme.text);
            }
            
            // Close button
            if tab.closeable {
                let close_x = tab_x + tab_w - 20.0;
                let close_hovered = self.hovered_close == Some(i);
                let close_color = if close_hovered { 
                    Vec4::new(0.9, 0.3, 0.3, 1.0) 
                } else { 
                    theme.text_secondary 
                };
                renderer.draw_text("×", Vec2::new(close_x, self.position.y + 8.0), 16.0, close_color);
            }
        }
        
        // Selection indicator
        renderer.draw_rounded_rect(
            Vec2::new(self.position.x + self.indicator_x + 4.0, self.position.y + tab_h - 3.0),
            Vec2::new(tab_w - 8.0, 2.0),
            theme.primary,
            1.0
        );
        
        // Content area
        let (content_pos, content_size) = self.content_area();
        renderer.draw_rounded_rect(
            content_pos,
            content_size,
            Vec4::new(0.06, 0.06, 0.08, 0.85),
            8.0
        );
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tab_view() {
        let mut tabs = TabView::new();
        tabs.add_tab(Tab::new("home", "Home").with_icon("🏠"));
        tabs.add_tab(Tab::new("settings", "Settings").closeable());
        
        assert_eq!(tabs.tabs.len(), 2);
        assert_eq!(tabs.selected_index, 0);
        
        tabs.select(1);
        assert_eq!(tabs.selected_index, 1);
    }
    
    #[test]
    fn test_tab_remove() {
        let mut tabs = TabView::new();
        tabs.add_tab(Tab::new("a", "A"));
        tabs.add_tab(Tab::new("b", "B"));
        tabs.select(1);
        
        tabs.remove_tab(1);
        assert_eq!(tabs.tabs.len(), 1);
        assert_eq!(tabs.selected_index, 0);
    }
}
