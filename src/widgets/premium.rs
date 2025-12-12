//! GlassUI Premium Widgets
//!
//! High-quality widgets with advanced animations: ProgressBar, Toggle, RadioGroup, NumberInput

use glam::{Vec2, Vec4};
use winit::event::{ElementState, MouseButton};
use crate::renderer::GlassRenderer;
use super::core::{Widget, get_theme, easing};

// =============================================================================
// PROGRESS BAR
// =============================================================================

/// Premium animated progress bar with glassmorphism
pub struct ProgressBar {
    pub position: Vec2,
    pub size: Vec2,
    pub value: f32,
    pub target_value: f32,
    pub animated_value: f32,
    pub indeterminate: bool,
    pub indeterminate_phase: f32,
    pub show_percentage: bool,
    pub color: Option<Vec4>,
    pub glow_intensity: f32,
    pub corner_radius: f32,
}

impl ProgressBar {
    pub fn new(value: f32) -> Self {
        let v = value.clamp(0.0, 1.0);
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            value: v,
            target_value: v,
            animated_value: v,
            indeterminate: false,
            indeterminate_phase: 0.0,
            show_percentage: true,
            color: None,
            glow_intensity: 0.0,
            corner_radius: 6.0,
        }
    }
    
    pub fn indeterminate() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            value: 0.0,
            target_value: 0.0,
            animated_value: 0.0,
            indeterminate: true,
            indeterminate_phase: 0.0,
            show_percentage: false,
            color: None,
            glow_intensity: 0.0,
            corner_radius: 6.0,
        }
    }
    
    pub fn set_value(&mut self, value: f32) {
        self.target_value = value.clamp(0.0, 1.0);
    }
    
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

impl Widget for ProgressBar {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(250.0, 24.0);
        self.size
    }

    fn handle_event(&mut self, _event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool {
        false
    }

    fn update(&mut self, dt: f32) {
        let diff = self.target_value - self.animated_value;
        self.animated_value += diff * 8.0 * dt;
        
        if diff.abs() > 0.001 {
            self.glow_intensity = (self.glow_intensity + dt * 3.0).min(1.0);
        } else {
            self.glow_intensity = (self.glow_intensity - dt * 2.0).max(0.0);
        }
        
        if self.indeterminate {
            self.indeterminate_phase = (self.indeterminate_phase + dt * 1.5) % 1.0;
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Track
        renderer.draw_rounded_rect(self.position, self.size, Vec4::new(0.0, 0.0, 0.0, 0.4), self.corner_radius);
        renderer.draw_rounded_rect(
            self.position + Vec2::new(2.0, 2.0),
            self.size - Vec2::new(4.0, 4.0),
            Vec4::new(0.05, 0.05, 0.08, 0.8),
            self.corner_radius - 2.0
        );
        
        let bar_color = self.color.unwrap_or(theme.primary);
        
        if self.indeterminate {
            let bar_width = self.size.x * 0.3;
            let travel = self.size.x - bar_width - 4.0;
            let ease_t = easing::ease_in_out_quart((self.indeterminate_phase * 2.0 - 1.0).abs());
            let x_offset = ease_t * travel;
            
            renderer.draw_rounded_rect(
                self.position + Vec2::new(2.0 + x_offset, 2.0),
                Vec2::new(bar_width, self.size.y - 4.0),
                bar_color,
                self.corner_radius - 2.0
            );
        } else {
            let fill_width = (self.size.x - 4.0) * self.animated_value;
            
            if fill_width > 0.0 {
                if self.glow_intensity > 0.0 {
                    renderer.draw_rounded_rect(
                        self.position + Vec2::new(0.0, -2.0),
                        Vec2::new(fill_width + 4.0, self.size.y + 4.0),
                        Vec4::new(bar_color.x, bar_color.y, bar_color.z, 0.2 * self.glow_intensity),
                        self.corner_radius + 2.0
                    );
                }
                
                renderer.draw_rounded_rect(
                    self.position + Vec2::new(2.0, 2.0),
                    Vec2::new(fill_width, self.size.y - 4.0),
                    bar_color,
                    self.corner_radius - 2.0
                );
            }
            
            if self.show_percentage {
                let pct = format!("{}%", (self.animated_value * 100.0).round() as i32);
                renderer.draw_text(&pct, Vec2::new(self.position.x + self.size.x + 10.0, self.position.y + 2.0), 16.0, theme.text);
            }
        }
    }
}

// =============================================================================
// TOGGLE
// =============================================================================

/// iOS-style toggle switch with spring animation
pub struct Toggle {
    pub position: Vec2,
    pub size: Vec2,
    pub checked: bool,
    pub label: String,
    pub animated_t: f32,
    pub spring_velocity: f32,
    pub hovered: bool,
    pub pressed: bool,
}

impl Toggle {
    pub fn new(label: &str, checked: bool) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            checked,
            label: label.to_string(),
            animated_t: if checked { 1.0 } else { 0.0 },
            spring_velocity: 0.0,
            hovered: false,
            pressed: false,
        }
    }
    
    pub fn is_checked(&self) -> bool {
        self.checked
    }
}

impl Widget for Toggle {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(200.0, 28.0);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let track_width = 50.0;
        let inside = mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + track_width &&
                     mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.size.y;
        
        self.hovered = inside;
        
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state, button: MouseButton::Left, .. }, .. } = event {
            if *state == ElementState::Pressed && inside {
                self.pressed = true;
                return true;
            } else if *state == ElementState::Released {
                if self.pressed && inside {
                    self.checked = !self.checked;
                    self.spring_velocity = if self.checked { 8.0 } else { -8.0 };
                }
                self.pressed = false;
            }
        }
        false
    }

    fn update(&mut self, dt: f32) {
        let target = if self.checked { 1.0 } else { 0.0 };
        let spring_k = 180.0;
        let damping = 12.0;
        
        let displacement = target - self.animated_t;
        let spring_force = displacement * spring_k;
        let damping_force = -self.spring_velocity * damping;
        
        self.spring_velocity += (spring_force + damping_force) * dt;
        self.animated_t += self.spring_velocity * dt;
        self.animated_t = self.animated_t.clamp(0.0, 1.0);
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        let track_width = 50.0;
        let track_height = 28.0;
        let knob_size = 22.0;
        let padding = 3.0;
        
        let off_color = Vec4::new(0.15, 0.15, 0.18, 0.9);
        let on_color = theme.primary;
        let track_color = off_color.lerp(on_color, self.animated_t);
        
        // Track glow
        if self.animated_t > 0.1 {
            renderer.draw_rounded_rect(
                self.position - Vec2::splat(3.0),
                Vec2::new(track_width + 6.0, track_height + 6.0),
                Vec4::new(on_color.x, on_color.y, on_color.z, 0.2 * self.animated_t),
                17.0
            );
        }
        
        // Track
        renderer.draw_rounded_rect(self.position, Vec2::new(track_width, track_height), track_color, 14.0);
        
        // Knob
        let knob_travel = track_width - knob_size - padding * 2.0;
        let knob_x = self.position.x + padding + knob_travel * self.animated_t;
        let knob_y = self.position.y + padding;
        
        // Shadow
        renderer.draw_rounded_rect(
            Vec2::new(knob_x + 1.0, knob_y + 2.0),
            Vec2::new(knob_size, knob_size),
            Vec4::new(0.0, 0.0, 0.0, 0.3),
            11.0
        );
        
        // Knob
        let knob_color = if self.pressed { Vec4::new(0.85, 0.85, 0.85, 1.0) } else { Vec4::new(0.98, 0.98, 0.98, 1.0) };
        renderer.draw_rounded_rect(Vec2::new(knob_x, knob_y), Vec2::new(knob_size, knob_size), knob_color, 11.0);
        
        // Label
        renderer.draw_text(&self.label, self.position + Vec2::new(track_width + 12.0, 4.0), 16.0, theme.text);
    }
}

// =============================================================================
// RADIO GROUP
// =============================================================================

/// Radio button group for exclusive selection
pub struct RadioGroup {
    pub position: Vec2,
    pub size: Vec2,
    pub options: Vec<String>,
    pub selected_index: usize,
    pub hovered_index: Option<usize>,
    pub item_height: f32,
}

impl RadioGroup {
    pub fn new(options: Vec<String>) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            options,
            selected_index: 0,
            hovered_index: None,
            item_height: 32.0,
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

impl Widget for RadioGroup {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(220.0, self.options.len() as f32 * self.item_height);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let inside_x = mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.size.x;
        
        if inside_x && mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.size.y {
            let relative_y = mouse_pos.y - self.position.y;
            let index = (relative_y / self.item_height) as usize;
            if index < self.options.len() {
                self.hovered_index = Some(index);
            } else {
                self.hovered_index = None;
            }
        } else {
            self.hovered_index = None;
        }
        
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. }, .. } = event {
            if let Some(index) = self.hovered_index {
                self.selected_index = index;
                return true;
            }
        }
        false
    }

    fn update(&mut self, _dt: f32) {}

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        let radio_size = 20.0;
        let inner_size = 10.0;
        
        for (i, option) in self.options.iter().enumerate() {
            let item_y = self.position.y + i as f32 * self.item_height;
            let radio_y = item_y + (self.item_height - radio_size) / 2.0;
            
            // Hover
            if self.hovered_index == Some(i) {
                renderer.draw_rounded_rect(
                    Vec2::new(self.position.x - 5.0, item_y),
                    Vec2::new(self.size.x + 10.0, self.item_height),
                    Vec4::new(theme.primary.x, theme.primary.y, theme.primary.z, 0.1),
                    4.0
                );
            }
            
            let is_selected = i == self.selected_index;
            let outer_color = if is_selected { theme.primary } else { theme.border };
            
            // Outer ring
            renderer.draw_rounded_rect(Vec2::new(self.position.x, radio_y), Vec2::new(radio_size, radio_size), outer_color, 10.0);
            renderer.draw_rounded_rect(Vec2::new(self.position.x + 2.0, radio_y + 2.0), Vec2::new(radio_size - 4.0, radio_size - 4.0), theme.background, 8.0);
            
            // Selected dot
            if is_selected {
                let center_offset = (radio_size - inner_size) / 2.0;
                renderer.draw_rounded_rect(
                    Vec2::new(self.position.x + center_offset, radio_y + center_offset),
                    Vec2::new(inner_size, inner_size),
                    theme.primary,
                    5.0
                );
            }
            
            // Label
            renderer.draw_text(option, Vec2::new(self.position.x + radio_size + 12.0, item_y + 6.0), 16.0, 
                if is_selected { theme.text } else { theme.text_secondary });
        }
    }
}

// =============================================================================
// NUMBER INPUT
// =============================================================================

/// Numeric input with increment/decrement buttons
pub struct NumberInput {
    pub position: Vec2,
    pub size: Vec2,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub precision: usize,
    pub focused: bool,
    pub text_buffer: String,
    pub hovered_btn: Option<bool>,
    pub pressed_btn: Option<bool>,
    pub repeat_timer: f32,
}

impl NumberInput {
    pub fn new(value: f64) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            value,
            min: f64::MIN,
            max: f64::MAX,
            step: 1.0,
            precision: 2,
            focused: false,
            text_buffer: format!("{:.2}", value),
            hovered_btn: None,
            pressed_btn: None,
            repeat_timer: 0.0,
        }
    }
    
    pub fn with_range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self.value = self.value.clamp(min, max);
        self
    }
    
    pub fn with_step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }
    
    fn increment(&mut self) {
        self.value = (self.value + self.step).min(self.max);
        self.text_buffer = format!("{:.prec$}", self.value, prec = self.precision);
    }
    
    fn decrement(&mut self) {
        self.value = (self.value - self.step).max(self.min);
        self.text_buffer = format!("{:.prec$}", self.value, prec = self.precision);
    }
}

impl Widget for NumberInput {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(150.0, 32.0);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let btn_width = 28.0;
        let in_dec = mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + btn_width &&
                     mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.size.y;
        let in_inc = mouse_pos.x >= self.position.x + self.size.x - btn_width && mouse_pos.x <= self.position.x + self.size.x &&
                     mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.size.y;
        
        self.hovered_btn = if in_inc { Some(true) } else if in_dec { Some(false) } else { None };
        
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state, button: MouseButton::Left, .. }, .. } = event {
            if *state == ElementState::Pressed {
                if in_inc {
                    self.pressed_btn = Some(true);
                    self.increment();
                    self.repeat_timer = 0.0;
                    return true;
                } else if in_dec {
                    self.pressed_btn = Some(false);
                    self.decrement();
                    self.repeat_timer = 0.0;
                    return true;
                }
            } else {
                self.pressed_btn = None;
            }
        }
        false
    }

    fn update(&mut self, dt: f32) {
        if let Some(is_inc) = self.pressed_btn {
            self.repeat_timer += dt;
            if self.repeat_timer > 0.4 {
                if is_inc { self.increment(); } else { self.decrement(); }
                self.repeat_timer -= 0.08;
            }
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        let btn_width = 28.0;
        
        // Background
        renderer.draw_rounded_rect(self.position, self.size, Vec4::new(0.08, 0.08, 0.1, 0.9), 6.0);
        
        // Decrement button
        let dec_color = match (self.pressed_btn, self.hovered_btn) {
            (Some(false), _) => theme.pressed,
            (_, Some(false)) => theme.hover,
            _ => Vec4::new(0.15, 0.15, 0.18, 1.0),
        };
        renderer.draw_rounded_rect(self.position, Vec2::new(btn_width, self.size.y), dec_color, 6.0);
        renderer.draw_text("−", self.position + Vec2::new(9.0, 6.0), 18.0, theme.text);
        
        // Increment button
        let inc_color = match (self.pressed_btn, self.hovered_btn) {
            (Some(true), _) => theme.pressed,
            (_, Some(true)) => theme.hover,
            _ => Vec4::new(0.15, 0.15, 0.18, 1.0),
        };
        renderer.draw_rounded_rect(
            Vec2::new(self.position.x + self.size.x - btn_width, self.position.y),
            Vec2::new(btn_width, self.size.y),
            inc_color,
            6.0
        );
        renderer.draw_text("+", Vec2::new(self.position.x + self.size.x - btn_width + 8.0, self.position.y + 6.0), 18.0, theme.text);
        
        // Value
        renderer.draw_text(&self.text_buffer, Vec2::new(self.position.x + btn_width + 8.0, self.position.y + 7.0), 16.0, theme.text);
    }
}
