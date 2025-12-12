//! GlassUI Control Widgets
//!
//! Interactive controls: Button, Label, Slider, Checkbox

use glam::{Vec2, Vec4};
use winit::event::{ElementState, MouseButton};
use crate::renderer::GlassRenderer;
use super::core::{Widget, get_theme};

// =============================================================================
// BUTTON
// =============================================================================

/// Interactive button with hover/press animations
pub struct Button {
    pub position: Vec2,
    pub size: Vec2,
    pub text: String,
    pub hovered: bool,
    pub pressed: bool,
    pub hover_t: f32,
    pub press_t: f32,
    pub on_click: Option<Box<dyn FnMut()>>,
    pub corner_radius: f32,
}

impl Button {
    pub fn new(text: &str) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            text: text.to_string(),
            hovered: false,
            pressed: false,
            hover_t: 0.0,
            press_t: 0.0,
            on_click: None,
            corner_radius: 8.0,
        }
    }
    
    pub fn with_callback(mut self, callback: impl FnMut() + 'static) -> Self {
        self.on_click = Some(Box::new(callback));
        self
    }
    
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }
}

impl Widget for Button {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(200.0, 50.0);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let inside = mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.size.x &&
                     mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.size.y;
        
        self.hovered = inside;
        
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state, button: MouseButton::Left, .. }, .. } = event {
            if *state == ElementState::Pressed && inside {
                self.pressed = true;
                return true;
            } else if *state == ElementState::Released {
                if self.pressed && inside {
                    if let Some(callback) = &mut self.on_click {
                        callback();
                    }
                }
                self.pressed = false;
            }
        }
        false
    }

    fn update(&mut self, dt: f32) {
        let hover_target = if self.hovered { 1.0 } else { 0.0 };
        self.hover_t += (hover_target - self.hover_t) * 15.0 * dt;
        
        let press_target = if self.pressed { 1.0 } else { 0.0 };
        self.press_t += (press_target - self.press_t) * 20.0 * dt;
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        let base_col = Vec4::new(0.0, 0.0, 0.0, 0.3);
        let hover_col = theme.hover;
        let press_col = theme.pressed;
        
        let mut color = base_col.lerp(hover_col, self.hover_t);
        color = color.lerp(press_col, self.press_t);
        
        let scale = self.hover_t * 4.0 - self.press_t * 2.0;
        
        // Glow effect
        if self.hover_t > 0.01 {
            renderer.draw_rounded_rect(
                self.position - Vec2::splat(scale),
                self.size + Vec2::splat(scale * 2.0),
                Vec4::new(theme.primary.x, theme.primary.y, theme.primary.z, self.hover_t * 0.3),
                self.corner_radius + 4.0
            );
        }
        
        // Button body
        renderer.draw_rounded_rect(
            self.position + Vec2::splat(self.press_t * 2.0), 
            self.size - Vec2::splat(self.press_t * 4.0), 
            color,
            self.corner_radius
        );
        
        // Text
        let text_len = self.text.len() as f32 * 10.0;
        let text_pos = self.position + (self.size - Vec2::new(text_len, 20.0)) * 0.5 + Vec2::new(0.0, self.press_t * 2.0);
        renderer.draw_text(&self.text, text_pos, 20.0, theme.text);
    }
}

// =============================================================================
// LABEL
// =============================================================================

/// Simple text label
pub struct Label {
    pub position: Vec2,
    pub text: String,
    pub font_size: f32,
    pub color: Option<Vec4>,
}

impl Label {
    pub fn new(text: &str) -> Self {
        Self { 
            position: Vec2::ZERO, 
            text: text.to_string(),
            font_size: 24.0,
            color: None,
        }
    }
    
    pub fn with_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }
    
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
    
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}

impl Widget for Label {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        Vec2::new(self.text.len() as f32 * (self.font_size * 0.5), self.font_size)
    }
    fn handle_event(&mut self, _event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool { false }
    fn update(&mut self, _dt: f32) {}
    fn render(&self, renderer: &mut GlassRenderer) {
        let color = self.color.unwrap_or_else(|| get_theme().text);
        renderer.draw_text(&self.text, self.position, self.font_size, color);
    }
}

// =============================================================================
// SLIDER
// =============================================================================

/// Horizontal slider for value selection
pub struct Slider {
    pub position: Vec2,
    pub size: Vec2,
    pub value: f32,
    pub dragging: bool,
    pub hovered: bool,
    pub corner_radius: f32,
}

impl Slider {
    pub fn new(value: f32) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            value: value.clamp(0.0, 1.0),
            dragging: false,
            hovered: false,
            corner_radius: 4.0,
        }
    }
    
    pub fn get_value(&self) -> f32 {
        self.value
    }
}

impl Widget for Slider {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(200.0, 20.0);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let inside = mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.size.x &&
                     mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.size.y;
        
        self.hovered = inside;

        match event {
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state, button: MouseButton::Left, .. }, .. } => {
                if *state == ElementState::Pressed && inside {
                    self.dragging = true;
                } else if *state == ElementState::Released {
                    self.dragging = false;
                }
            }
            _ => {}
        }
        
        if self.dragging {
            let relative_x = (mouse_pos.x - self.position.x).clamp(0.0, self.size.x);
            self.value = relative_x / self.size.x;
            return true;
        }
        
        false
    }

    fn update(&mut self, _dt: f32) {}

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Track
        renderer.draw_rounded_rect(self.position, self.size, Vec4::new(0.0, 0.0, 0.0, 0.5), self.corner_radius);
        
        // Fill
        let fill_width = self.size.x * self.value;
        if fill_width > 0.0 {
            renderer.draw_rounded_rect(self.position, Vec2::new(fill_width, self.size.y), theme.primary * Vec4::new(1.0, 1.0, 1.0, 0.6), self.corner_radius);
        }
        
        // Handle
        let handle_size = Vec2::new(12.0, self.size.y + 8.0);
        let handle_pos = Vec2::new(
            self.position.x + fill_width - handle_size.x * 0.5, 
            self.position.y - 4.0
        );
        
        // Handle glow
        if self.hovered || self.dragging {
            renderer.draw_rounded_rect(
                handle_pos - Vec2::splat(2.0), 
                handle_size + Vec2::splat(4.0), 
                Vec4::new(theme.primary.x, theme.primary.y, theme.primary.z, 0.4),
                6.0
            );
        }
        
        renderer.draw_rounded_rect(handle_pos, handle_size, Vec4::new(1.0, 1.0, 1.0, 0.95), 4.0);
    }
}

// =============================================================================
// CHECKBOX
// =============================================================================

/// Toggle checkbox with label
pub struct Checkbox {
    pub position: Vec2,
    pub size: Vec2,
    pub checked: bool,
    pub label: String,
    pub hovered: bool,
    pub check_t: f32,
}

impl Checkbox {
    pub fn new(label: &str, checked: bool) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            checked,
            label: label.to_string(),
            hovered: false,
            check_t: if checked { 1.0 } else { 0.0 },
        }
    }
    
    pub fn is_checked(&self) -> bool {
        self.checked
    }
}

impl Widget for Checkbox {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(24.0, 24.0);
        Vec2::new(200.0, 24.0)
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let full_width = 200.0; // Label area too
        let inside = mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + full_width &&
                     mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.size.y;
        
        self.hovered = inside;
                     
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. }, .. } = event {
            if inside {
                self.checked = !self.checked;
                return true;
            }
        }
        false
    }

    fn update(&mut self, dt: f32) {
        let target = if self.checked { 1.0 } else { 0.0 };
        self.check_t += (target - self.check_t) * 15.0 * dt;
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Box background
        let bg_color = Vec4::new(0.1, 0.1, 0.12, 0.9).lerp(theme.primary, self.check_t * 0.3);
        renderer.draw_rounded_rect(self.position, self.size, bg_color, 4.0);
        
        // Border
        if self.hovered {
            renderer.draw_rounded_rect(
                self.position - Vec2::splat(1.0), 
                self.size + Vec2::splat(2.0), 
                Vec4::new(theme.primary.x, theme.primary.y, theme.primary.z, 0.5),
                5.0
            );
        }
        
        // Check mark (using inner rect for now)
        if self.check_t > 0.01 {
            let inner_size = self.size * 0.5 * self.check_t;
            let inner_pos = self.position + (self.size - inner_size) * 0.5;
            renderer.draw_rounded_rect(inner_pos, inner_size, theme.primary, 2.0);
        }
        
        // Label
        renderer.draw_text(
            &self.label, 
            self.position + Vec2::new(self.size.x + 10.0, 2.0), 
            16.0, 
            if self.checked { theme.text } else { theme.text_secondary }
        );
    }
}

// =============================================================================
// PANEL
// =============================================================================

/// Container panel with glassmorphism background
pub struct Panel {
    pub position: Vec2,
    pub size: Vec2,
    pub content: Option<Box<dyn Widget>>,
    pub color: Vec4,
    pub fill: bool,
    pub corner_radius: f32,
    pub padding: f32,
}

impl Panel {
    pub fn new(content: Box<dyn Widget>) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            content: Some(content),
            color: Vec4::new(1.0, 1.0, 1.0, 0.05),
            fill: false,
            corner_radius: 12.0,
            padding: 20.0,
        }
    }
    
    pub fn new_empty() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            content: None,
            color: Vec4::new(1.0, 1.0, 1.0, 0.05),
            fill: false,
            corner_radius: 12.0,
            padding: 20.0,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }
    
    pub fn with_fill(mut self, fill: bool) -> Self {
        self.fill = fill;
        self
    }
    
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }
}

impl Widget for Panel {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        
        let content_available = max_size - Vec2::splat(self.padding * 2.0);

        let content_size = if let Some(content) = &mut self.content {
            let content_origin = origin + Vec2::splat(self.padding);
            content.layout(content_origin, content_available)
        } else {
            Vec2::ZERO
        };
        
        self.size = if self.fill {
            max_size
        } else {
            content_size + Vec2::splat(self.padding * 2.0)
        };
        
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        if let Some(content) = &mut self.content {
            content.handle_event(event, mouse_pos)
        } else {
            false
        }
    }

    fn update(&mut self, dt: f32) {
        if let Some(content) = &mut self.content {
            content.update(dt);
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        renderer.draw_rounded_rect(self.position, self.size, self.color, self.corner_radius);
        
        if let Some(content) = &self.content {
            content.render(renderer);
        }
    }
}
