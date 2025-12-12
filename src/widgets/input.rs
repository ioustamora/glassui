//! GlassUI Input Widgets
//!
//! Text and data input controls: TextInput, Dropdown

use glam::{Vec2, Vec4};
use winit::event::{ElementState, MouseButton};
use crate::renderer::GlassRenderer;
use super::core::{Widget, get_theme};

// =============================================================================
// TEXT INPUT
// =============================================================================

/// Single-line text input field
pub struct TextInput {
    pub position: Vec2,
    pub size: Vec2,
    pub text: String,
    pub placeholder: String,
    pub focused: bool,
    pub cursor_visible: bool,
    pub cursor_timer: f32,
    pub corner_radius: f32,
}

impl TextInput {
    pub fn new(placeholder: &str) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            text: String::new(),
            placeholder: placeholder.to_string(),
            focused: false,
            cursor_visible: true,
            cursor_timer: 0.0,
            corner_radius: 6.0,
        }
    }
    
    pub fn with_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }
    
    pub fn get_text(&self) -> &str {
        &self.text
    }
    
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}

impl Widget for TextInput {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(200.0, 36.0);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let inside = mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.size.x &&
                     mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.size.y;

        match event {
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. }, .. } => {
                if inside {
                    self.focused = true;
                    return true;
                } else {
                    self.focused = false;
                }
            },
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::KeyboardInput { event: key_event, .. }, .. } => {
                if self.focused && key_event.state.is_pressed() {
                    if let winit::keyboard::Key::Named(winit::keyboard::NamedKey::Backspace) = key_event.logical_key {
                        self.text.pop();
                        return true;
                    }
                    
                    if let Some(text) = &key_event.text {
                        if let Some(c) = text.chars().next() {
                            if !c.is_control() {
                                self.text.push_str(text);
                                return true;
                            }
                        }
                    }
                }
            },
            _ => {}
        }
        
        false
    }

    fn update(&mut self, dt: f32) {
        if self.focused {
            self.cursor_timer += dt;
            if self.cursor_timer > 0.5 {
                self.cursor_visible = !self.cursor_visible;
                self.cursor_timer = 0.0;
            }
        } else {
            self.cursor_visible = false;
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Background
        let bg_col = if self.focused { 
            Vec4::new(0.1, 0.1, 0.12, 0.95) 
        } else { 
            Vec4::new(0.08, 0.08, 0.1, 0.9) 
        };
        renderer.draw_rounded_rect(self.position, self.size, bg_col, self.corner_radius);
        
        // Border
        if self.focused {
            renderer.draw_rounded_rect(
                self.position - Vec2::splat(1.0), 
                self.size + Vec2::splat(2.0), 
                Vec4::new(theme.primary.x, theme.primary.y, theme.primary.z, 0.6),
                self.corner_radius + 1.0
            );
        }

        // Text or placeholder
        let display_text = if self.text.is_empty() && !self.focused {
            &self.placeholder
        } else {
            &self.text
        };
        let text_color = if self.text.is_empty() && !self.focused {
            theme.text_secondary
        } else {
            theme.text
        };
        
        renderer.draw_text(display_text, self.position + Vec2::new(10.0, 8.0), 18.0, text_color);
        
        // Cursor
        if self.focused && self.cursor_visible {
            let text_width = self.text.len() as f32 * 9.0;
            let cursor_pos = self.position + Vec2::new(10.0 + text_width, 6.0);
            renderer.draw_rounded_rect(cursor_pos, Vec2::new(2.0, 22.0), theme.primary, 1.0);
        }
    }
}

// =============================================================================
// DROPDOWN
// =============================================================================

/// Dropdown select box
pub struct Dropdown {
    pub position: Vec2,
    pub size: Vec2,
    pub options: Vec<String>,
    pub selected_index: usize,
    pub open: bool,
    pub hovered_index: Option<usize>,
    pub corner_radius: f32,
}

impl Dropdown {
    pub fn new(options: Vec<String>) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            options,
            selected_index: 0,
            open: false,
            hovered_index: None,
            corner_radius: 6.0,
        }
    }
    
    pub fn with_selected(mut self, index: usize) -> Self {
        if index < self.options.len() {
            self.selected_index = index;
        }
        self
    }
    
    pub fn selected(&self) -> Option<&str> {
        self.options.get(self.selected_index).map(|s| s.as_str())
    }
}

impl Widget for Dropdown {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(180.0, 36.0);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let in_header = mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.size.x &&
                        mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.size.y;
        
        let item_height = 32.0;
        let dropdown_height = self.options.len() as f32 * item_height;
        let dropdown_y = self.position.y + self.size.y;
        
        let in_dropdown = self.open && 
            mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.size.x &&
            mouse_pos.y >= dropdown_y && mouse_pos.y <= dropdown_y + dropdown_height;
        
        // Update hovered
        if in_dropdown {
            let relative_y = mouse_pos.y - dropdown_y;
            let index = (relative_y / item_height) as usize;
            if index < self.options.len() {
                self.hovered_index = Some(index);
            }
        } else {
            self.hovered_index = None;
        }
        
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. }, .. } = event {
            if in_header {
                self.open = !self.open;
                return true;
            } else if in_dropdown {
                if let Some(index) = self.hovered_index {
                    self.selected_index = index;
                    self.open = false;
                    return true;
                }
            } else {
                self.open = false;
            }
        }
        
        false
    }

    fn update(&mut self, _dt: f32) {}

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Header (normal rendering)
        renderer.draw_rounded_rect(self.position, self.size, Vec4::new(0.1, 0.1, 0.12, 0.9), self.corner_radius);
        
        // Selected text
        if let Some(text) = self.options.get(self.selected_index) {
            renderer.draw_text(text, self.position + Vec2::new(10.0, 8.0), 16.0, theme.text);
        }
        
        // Arrow
        let arrow = if self.open { "▲" } else { "▼" };
        renderer.draw_text(arrow, Vec2::new(self.position.x + self.size.x - 24.0, self.position.y + 10.0), 14.0, theme.text_secondary);
        
        // Dropdown list - USE OVERLAY RENDERING (on top of everything)
        if self.open {
            let item_height = 32.0;
            let list_y = self.position.y + self.size.y + 2.0;
            let list_height = self.options.len() as f32 * item_height;
            
            // Background - use overlay (renders on top)
            renderer.draw_overlay_rect(
                Vec2::new(self.position.x, list_y),
                Vec2::new(self.size.x, list_height),
                Vec4::new(0.08, 0.08, 0.1, 0.98),
                self.corner_radius
            );
            
            // Items
            for (i, option) in self.options.iter().enumerate() {
                let item_y = list_y + i as f32 * item_height;
                
                // Hover highlight
                if self.hovered_index == Some(i) {
                    renderer.draw_overlay_rect(
                        Vec2::new(self.position.x + 2.0, item_y + 2.0),
                        Vec2::new(self.size.x - 4.0, item_height - 4.0),
                        Vec4::new(theme.primary.x, theme.primary.y, theme.primary.z, 0.3),
                        4.0
                    );
                }
                
                // Text - use overlay
                let color = if i == self.selected_index { theme.primary } else { theme.text };
                renderer.draw_overlay_text(option, Vec2::new(self.position.x + 10.0, item_y + 6.0), 16.0, color);
            }
        }
    }
}
