//! GlassUI Overlay Widgets
//!
//! Widgets that render on top of other content: Tooltip, ContextMenu, Modal

use glam::{Vec2, Vec4};
use winit::event::{ElementState, MouseButton};
use crate::renderer::GlassRenderer;
use super::core::{Widget, get_theme};

// =============================================================================
// TOOLTIP
// =============================================================================

/// Wraps a child widget and shows a tooltip on hover
pub struct Tooltip {
    pub position: Vec2,
    pub size: Vec2,
    pub child: Box<dyn Widget>,
    pub text: String,
    pub hovered: bool,
    pub hover_time: f32,
    pub mouse_pos: Vec2,
    pub delay: f32,
}

impl Tooltip {
    pub fn new(child: Box<dyn Widget>, text: &str) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            child,
            text: text.to_string(),
            hovered: false,
            hover_time: 0.0,
            mouse_pos: Vec2::ZERO,
            delay: 0.5,
        }
    }
    
    pub fn with_delay(mut self, delay: f32) -> Self {
        self.delay = delay;
        self
    }
}

impl Widget for Tooltip {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = self.child.layout(origin, max_size);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        self.mouse_pos = mouse_pos;
        let handled = self.child.handle_event(event, mouse_pos);
        
        let inside = mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.size.x &&
                     mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.size.y;
        
        self.hovered = inside;
        handled
    }

    fn update(&mut self, dt: f32) {
        self.child.update(dt);
        if self.hovered {
            self.hover_time += dt;
        } else {
            self.hover_time = 0.0;
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        self.child.render(renderer);
        
        // Render tooltip using overlay layer (on top of everything)
        if self.hovered && self.hover_time > self.delay {
            let theme = get_theme();
            let padding = 8.0;
            let text_width = self.text.len() as f32 * 8.0;
            let tooltip_size = Vec2::new(text_width + padding * 2.0, 28.0);
            let tooltip_pos = self.mouse_pos + Vec2::new(12.0, 12.0);
            
            // Background
            renderer.draw_overlay_rect(
                tooltip_pos,
                tooltip_size,
                Vec4::new(0.05, 0.05, 0.08, 0.95),
                6.0
            );
            
            // Border
            renderer.draw_overlay_rect(
                tooltip_pos - Vec2::splat(1.0),
                tooltip_size + Vec2::splat(2.0),
                Vec4::new(theme.border.x, theme.border.y, theme.border.z, 0.5),
                7.0
            );
            
            // Text
            renderer.draw_overlay_text(&self.text, tooltip_pos + Vec2::new(padding, 6.0), 14.0, theme.text);
        }
    }
}

// =============================================================================
// MENU ITEM
// =============================================================================

/// A single item in a context menu
pub struct MenuItem {
    pub label: String,
    pub shortcut: Option<String>,
    pub on_click: Option<Box<dyn FnMut()>>,
}

impl MenuItem {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            shortcut: None,
            on_click: None,
        }
    }
    
    pub fn with_shortcut(mut self, shortcut: &str) -> Self {
        self.shortcut = Some(shortcut.to_string());
        self
    }
    
    pub fn with_callback<F: FnMut() + 'static>(mut self, callback: F) -> Self {
        self.on_click = Some(Box::new(callback));
        self
    }
}

// =============================================================================
// CONTEXT MENU
// =============================================================================

/// Popup context menu (right-click menu)
pub struct ContextMenu {
    pub position: Vec2,
    pub items: Vec<MenuItem>,
    pub visible: bool,
    pub item_height: f32,
    pub width: f32,
    pub hovered_index: Option<usize>,
    pub corner_radius: f32,
}

impl ContextMenu {
    pub fn new(items: Vec<MenuItem>) -> Self {
        Self {
            position: Vec2::ZERO,
            items,
            visible: false,
            item_height: 32.0,
            width: 180.0,
            hovered_index: None,
            corner_radius: 8.0,
        }
    }
    
    pub fn show(&mut self, pos: Vec2) {
        self.position = pos;
        self.visible = true;
    }
    
    pub fn hide(&mut self) {
        self.visible = false;
    }
    
    fn total_height(&self) -> f32 {
        self.items.len() as f32 * self.item_height
    }
}

impl Widget for ContextMenu {
    fn layout(&mut self, _origin: Vec2, _max_size: Vec2) -> Vec2 {
        Vec2::new(self.width, self.total_height())
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        if !self.visible {
            return false;
        }
        
        let inside = mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.width &&
                     mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.total_height();
        
        if inside {
            let relative_y = mouse_pos.y - self.position.y;
            let index = (relative_y / self.item_height) as usize;
            self.hovered_index = if index < self.items.len() { Some(index) } else { None };
        } else {
            self.hovered_index = None;
        }
        
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. }, .. } = event {
            if inside {
                if let Some(index) = self.hovered_index {
                    if let Some(callback) = &mut self.items[index].on_click {
                        callback();
                    }
                    println!("Menu: '{}'", self.items[index].label);
                    self.hide();
                    return true;
                }
            } else {
                self.hide();
                return true;
            }
        }
        
        inside
    }

    fn update(&mut self, _dt: f32) {}

    fn render(&self, renderer: &mut GlassRenderer) {
        if !self.visible {
            return;
        }
        
        let theme = get_theme();
        
        // Background - use overlay layer
        renderer.draw_overlay_rect(
            self.position,
            Vec2::new(self.width, self.total_height()),
            Vec4::new(0.08, 0.08, 0.1, 0.97),
            self.corner_radius
        );
        
        // Border
        renderer.draw_overlay_rect(
            self.position - Vec2::splat(1.0),
            Vec2::new(self.width + 2.0, self.total_height() + 2.0),
            Vec4::new(theme.border.x, theme.border.y, theme.border.z, 0.4),
            self.corner_radius + 1.0
        );
        
        // Items
        for (i, item) in self.items.iter().enumerate() {
            let item_y = self.position.y + i as f32 * self.item_height;
            
            // Hover highlight
            if self.hovered_index == Some(i) {
                renderer.draw_overlay_rect(
                    Vec2::new(self.position.x + 4.0, item_y + 2.0),
                    Vec2::new(self.width - 8.0, self.item_height - 4.0),
                    Vec4::new(theme.primary.x, theme.primary.y, theme.primary.z, 0.3),
                    4.0
                );
            }
            
            // Label
            renderer.draw_overlay_text(&item.label, Vec2::new(self.position.x + 12.0, item_y + 8.0), 15.0, theme.text);
            
            // Shortcut
            if let Some(shortcut) = &item.shortcut {
                let shortcut_x = self.position.x + self.width - 12.0 - shortcut.len() as f32 * 7.0;
                renderer.draw_overlay_text(shortcut, Vec2::new(shortcut_x, item_y + 8.0), 13.0, theme.text_secondary);
            }
        }
    }
}

// =============================================================================
// CONTEXT MENU TRIGGER
// =============================================================================

/// Wrapper that shows a context menu on right-click
pub struct ContextMenuTrigger {
    pub position: Vec2,
    pub size: Vec2,
    pub child: Box<dyn Widget>,
    pub menu: ContextMenu,
}

impl ContextMenuTrigger {
    pub fn new(child: Box<dyn Widget>, items: Vec<MenuItem>) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            child,
            menu: ContextMenu::new(items),
        }
    }
}

impl Widget for ContextMenuTrigger {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = self.child.layout(origin, max_size);
        self.menu.layout(Vec2::ZERO, max_size);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        // Menu gets priority if visible
        if self.menu.visible && self.menu.handle_event(event, mouse_pos) {
            return true;
        }
        
        // Child events
        if self.child.handle_event(event, mouse_pos) {
            return true;
        }
        
        // Right-click detection
        let inside = mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.size.x &&
                     mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.size.y;
        
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Right, .. }, .. } = event {
            if inside {
                self.menu.show(mouse_pos);
                return true;
            }
        }
        
        false
    }

    fn update(&mut self, dt: f32) {
        self.child.update(dt);
        self.menu.update(dt);
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        self.child.render(renderer);
        self.menu.render(renderer);
    }
}

// =============================================================================
// MODAL
// =============================================================================

/// Modal dialog that overlays the entire screen with backdrop
pub struct Modal {
    pub visible: bool,
    pub title: String,
    pub content: Box<dyn Widget>,
    pub width: f32,
    pub height: f32,
    pub corner_radius: f32,
    pub backdrop_alpha: f32,
    screen_size: Vec2,
}

impl Modal {
    pub fn new(title: &str, content: Box<dyn Widget>) -> Self {
        Self {
            visible: false,
            title: title.to_string(),
            content,
            width: 400.0,
            height: 300.0,
            corner_radius: 12.0,
            backdrop_alpha: 0.6,
            screen_size: Vec2::ZERO,
        }
    }
    
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    
    pub fn show(&mut self) {
        self.visible = true;
    }
    
    pub fn hide(&mut self) {
        self.visible = false;
    }
    
    fn modal_position(&self) -> Vec2 {
        Vec2::new(
            (self.screen_size.x - self.width) / 2.0,
            (self.screen_size.y - self.height) / 2.0
        )
    }
}

impl Widget for Modal {
    fn layout(&mut self, _origin: Vec2, max_size: Vec2) -> Vec2 {
        self.screen_size = max_size;
        
        if self.visible {
            let content_origin = self.modal_position() + Vec2::new(16.0, 48.0);
            let content_size = Vec2::new(self.width - 32.0, self.height - 64.0);
            self.content.layout(content_origin, content_size);
        }
        
        Vec2::ZERO // Modal doesn't take layout space
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        if !self.visible {
            return false;
        }
        
        let modal_pos = self.modal_position();
        let in_modal = mouse_pos.x >= modal_pos.x && mouse_pos.x <= modal_pos.x + self.width &&
                       mouse_pos.y >= modal_pos.y && mouse_pos.y <= modal_pos.y + self.height;
        
        // Pass events to content
        if in_modal {
            if self.content.handle_event(event, mouse_pos) {
                return true;
            }
        }
        
        // Click on backdrop closes modal
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. }, .. } = event {
            if !in_modal {
                self.hide();
                return true;
            }
        }
        
        // Escape key closes modal
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::KeyboardInput { event: key_event, .. }, .. } = event {
            if key_event.state.is_pressed() {
                if let winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape) = key_event.logical_key {
                    self.hide();
                    return true;
                }
            }
        }
        
        true // Block all events when modal is open
    }

    fn update(&mut self, dt: f32) {
        if self.visible {
            self.content.update(dt);
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        if !self.visible {
            return;
        }
        
        let theme = get_theme();
        let modal_pos = self.modal_position();
        
        // Backdrop - full screen darkening
        renderer.draw_overlay_rect(
            Vec2::ZERO,
            self.screen_size,
            Vec4::new(0.0, 0.0, 0.0, self.backdrop_alpha),
            0.0
        );
        
        // Modal background
        renderer.draw_overlay_rect(
            modal_pos,
            Vec2::new(self.width, self.height),
            Vec4::new(0.1, 0.1, 0.12, 0.98),
            self.corner_radius
        );
        
        // Border glow
        renderer.draw_overlay_rect(
            modal_pos - Vec2::splat(1.0),
            Vec2::new(self.width + 2.0, self.height + 2.0),
            Vec4::new(theme.primary.x, theme.primary.y, theme.primary.z, 0.3),
            self.corner_radius + 1.0
        );
        
        // Title bar
        renderer.draw_overlay_rect(
            modal_pos,
            Vec2::new(self.width, 40.0),
            Vec4::new(0.06, 0.06, 0.08, 0.95),
            self.corner_radius
        );
        
        // Title text
        renderer.draw_overlay_text(&self.title, modal_pos + Vec2::new(16.0, 12.0), 18.0, theme.text);
        
        // Close button (X)
        let close_x = modal_pos.x + self.width - 32.0;
        renderer.draw_overlay_text("✕", Vec2::new(close_x, modal_pos.y + 12.0), 16.0, theme.text_secondary);
        
        // Content is rendered by the child widget (already laid out in correct position)
        self.content.render(renderer);
    }
}
