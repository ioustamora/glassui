//! GlassUI Advanced Wrapper Widgets
//!
//! Widgets that modify child behavior: Draggable, Resizable

use glam::{Vec2, Vec4};
use winit::event::{ElementState, MouseButton};
use crate::renderer::GlassRenderer;
use super::core::{Widget, get_theme};

// =============================================================================
// DRAGGABLE
// =============================================================================

/// Wrapper that makes a child widget freely draggable
pub struct Draggable {
    pub position: Vec2,
    pub size: Vec2,
    pub child: Box<dyn Widget>,
    pub dragging: bool,
    drag_start_mouse: Vec2,
    drag_start_pos: Vec2,
    initialized: bool,
}

impl Draggable {
    pub fn new(child: Box<dyn Widget>) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            child,
            dragging: false,
            drag_start_mouse: Vec2::ZERO,
            drag_start_pos: Vec2::ZERO,
            initialized: false,
        }
    }
    
    /// Set initial position
    pub fn at(mut self, pos: Vec2) -> Self {
        self.position = pos;
        self.initialized = true;
        self
    }
}

impl Widget for Draggable {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        // First layout: accept parent's suggested position
        if !self.initialized && !self.dragging {
            self.position = origin;
            self.initialized = true;
        }
        
        // Layout child at our current position
        let child_size = self.child.layout(self.position, max_size);
        self.size = child_size;
        child_size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        // Child gets first chance to handle events
        if self.child.handle_event(event, mouse_pos) {
            return true;
        }
        
        // Check if mouse is inside
        let inside = mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.size.x &&
                     mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.size.y;
        
        match event {
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state, button: MouseButton::Left, .. }, .. } => {
                if *state == ElementState::Pressed && inside {
                    self.dragging = true;
                    self.drag_start_mouse = mouse_pos;
                    self.drag_start_pos = self.position;
                    return true;
                } else if *state == ElementState::Released {
                    self.dragging = false;
                }
            },
            _ => {}
        }
        
        // Handle drag motion
        if self.dragging {
            let delta = mouse_pos - self.drag_start_mouse;
            self.position = self.drag_start_pos + delta;
            return true;
        }
        
        false
    }

    fn update(&mut self, dt: f32) {
        self.child.update(dt);
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        self.child.render(renderer);
    }
}

// =============================================================================
// RESIZABLE
// =============================================================================

/// Wrapper that makes a child widget resizable with a corner handle
pub struct Resizable {
    pub position: Vec2,
    pub current_size: Vec2,
    pub min_size: Vec2,
    pub max_size: Vec2,
    pub child: Box<dyn Widget>,
    pub resizing: bool,
    resize_start_mouse: Vec2,
    resize_start_size: Vec2,
    handle_size: f32,
    corner_radius: f32,
}

impl Resizable {
    pub fn new(child: Box<dyn Widget>, initial_size: Vec2) -> Self {
        Self {
            position: Vec2::ZERO,
            current_size: initial_size,
            min_size: Vec2::new(100.0, 100.0),
            max_size: Vec2::new(2000.0, 2000.0),
            child,
            resizing: false,
            resize_start_mouse: Vec2::ZERO,
            resize_start_size: Vec2::ZERO,
            handle_size: 20.0,
            corner_radius: 4.0,
        }
    }
    
    pub fn with_min_size(mut self, min: Vec2) -> Self {
        self.min_size = min;
        self
    }
    
    pub fn with_max_size(mut self, max: Vec2) -> Self {
        self.max_size = max;
        self
    }
}

impl Widget for Resizable {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        // Constrain child to our current size
        self.child.layout(origin, self.current_size);
        self.current_size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        // Check if in resize handle (bottom-right corner)
        let handle_pos = self.position + self.current_size - Vec2::splat(self.handle_size);
        let in_handle = mouse_pos.x >= handle_pos.x && mouse_pos.y >= handle_pos.y &&
                        mouse_pos.x <= self.position.x + self.current_size.x &&
                        mouse_pos.y <= self.position.y + self.current_size.y;
        
        match event {
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state, button: MouseButton::Left, .. }, .. } => {
                if *state == ElementState::Pressed && in_handle {
                    self.resizing = true;
                    self.resize_start_mouse = mouse_pos;
                    self.resize_start_size = self.current_size;
                    return true;
                } else if *state == ElementState::Released {
                    self.resizing = false;
                }
            },
            _ => {}
        }
        
        // Handle resize motion
        if self.resizing {
            let delta = mouse_pos - self.resize_start_mouse;
            self.current_size = (self.resize_start_size + delta)
                .max(self.min_size)
                .min(self.max_size);
            return true;
        }
        
        // Child events
        if self.child.handle_event(event, mouse_pos) {
            return true;
        }
        
        false
    }

    fn update(&mut self, dt: f32) {
        self.child.update(dt);
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        self.child.render(renderer);
        
        // Draw resize handle
        let handle_pos = self.position + self.current_size - Vec2::new(14.0, 14.0);
        
        // Three diagonal lines (resize grip)
        let grip_color = if self.resizing {
            theme.primary
        } else {
            Vec4::new(0.5, 0.5, 0.5, 0.6)
        };
        
        for i in 0..3 {
            let offset = i as f32 * 4.0;
            renderer.draw_rounded_rect(
                handle_pos + Vec2::new(offset + 4.0, 10.0 - offset),
                Vec2::new(2.0, 2.0),
                grip_color,
                1.0
            );
        }
    }
}
