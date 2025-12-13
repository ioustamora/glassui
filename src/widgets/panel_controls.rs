//! GlassUI Panel Controls
//!
//! Keyboard and mouse-based panel manipulation:
//! - Hold R + mouse/arrows = resize
//! - Hold M + mouse/arrows = move
//! - Hold C + click = cycle colors
//! - Control buttons on panel corners

use glam::{Vec2, Vec4};
use crate::renderer::GlassRenderer;
use crate::widget_id::WidgetId;
use crate::widgets::core::{Widget, get_theme};
use crate::panel_style::PanelPreset;

// =============================================================================
// PANEL CONTROL MODE
// =============================================================================

/// Current control mode based on held keys
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum PanelControlMode {
    #[default]
    None,
    /// Hold R - resize mode
    Resize,
    /// Hold M - move mode
    Move,
    /// Hold C - color cycle mode
    Color,
}

impl PanelControlMode {
    /// Get the key for this mode
    pub fn key_hint(&self) -> &'static str {
        match self {
            PanelControlMode::None => "",
            PanelControlMode::Resize => "R",
            PanelControlMode::Move => "M",
            PanelControlMode::Color => "C",
        }
    }
    
    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            PanelControlMode::None => "",
            PanelControlMode::Resize => "Resize",
            PanelControlMode::Move => "Move",
            PanelControlMode::Color => "Color",
        }
    }
}

// =============================================================================
// CONTROL BUTTON
// =============================================================================

/// Small control button on panel corner with joyful animations
#[derive(Clone, Copy, Debug)]
pub struct ControlButton {
    pub mode: PanelControlMode,
    pub position: Vec2,
    pub size: Vec2,
    pub hovered: bool,
    pub active: bool,
    /// Hover animation progress (0.0 to 1.0)
    pub hover_t: f32,
    /// Press bounce animation
    pub press_t: f32,
    /// Pulse animation for active state
    pub pulse_t: f32,
}

impl ControlButton {
    pub fn new(mode: PanelControlMode) -> Self {
        Self {
            mode,
            position: Vec2::ZERO,
            size: Vec2::new(24.0, 24.0),
            hovered: false,
            active: false,
            hover_t: 0.0,
            press_t: 0.0,
            pulse_t: 0.0,
        }
    }
    
    /// Get icon for this button
    pub fn icon(&self) -> &'static str {
        match self.mode {
            PanelControlMode::None => "",
            PanelControlMode::Resize => "⤡",  // Resize diagonal
            PanelControlMode::Move => "✥",    // Move crosshair
            PanelControlMode::Color => "◐",   // Color half-circle
        }
    }
    
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.position.x && point.x <= self.position.x + self.size.x &&
        point.y >= self.position.y && point.y <= self.position.y + self.size.y
    }
    
    /// Update animations
    pub fn update(&mut self, dt: f32) {
        // Smooth hover animation
        let target_hover = if self.hovered { 1.0 } else { 0.0 };
        self.hover_t += (target_hover - self.hover_t) * 12.0 * dt;
        
        // Bounce back from press
        if self.press_t > 0.0 {
            self.press_t = (self.press_t - dt * 8.0).max(0.0);
        }
        
        // Pulse when active
        if self.active {
            self.pulse_t += dt * 6.0;
            if self.pulse_t > std::f32::consts::TAU {
                self.pulse_t -= std::f32::consts::TAU;
            }
        } else {
            self.pulse_t = 0.0;
        }
    }
    
    /// Get the current scale (with bounce effect)
    pub fn current_scale(&self) -> f32 {
        let hover_scale = 1.0 + self.hover_t * 0.15;
        let press_bounce = (self.press_t * std::f32::consts::PI).sin() * 0.2;
        hover_scale + press_bounce
    }
    
    /// Trigger press animation
    pub fn trigger_press(&mut self) {
        self.press_t = 1.0;
    }
}

// =============================================================================
// CONTROLLABLE PANEL
// =============================================================================

/// Panel with keyboard/mouse control system
pub struct ControllablePanel {
    pub id: WidgetId,
    pub position: Vec2,
    pub size: Vec2,
    pub min_size: Vec2,
    pub max_size: Vec2,
    pub content: Option<Box<dyn Widget>>,
    
    // Style
    pub preset_index: usize,
    pub color: Vec4,
    pub corner_radius: f32,
    pub padding: f32,
    
    // Control state
    pub control_mode: PanelControlMode,
    pub buttons: Vec<ControlButton>,
    pub show_controls: bool,
    pub selected: bool,
    
    // Drag state
    drag_start: Option<Vec2>,
    drag_start_pos: Vec2,
    drag_start_size: Vec2,
}

/// Available presets for cycling
const PRESETS: [PanelPreset; 6] = [
    PanelPreset::Default,
    PanelPreset::Data,
    PanelPreset::Status,
    PanelPreset::Alert,
    PanelPreset::Technical,
    PanelPreset::Media,
];

impl ControllablePanel {
    pub fn new(content: Box<dyn Widget>) -> Self {
        let preset = PanelPreset::Default;
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::new(300.0, 200.0),
            min_size: Vec2::new(100.0, 80.0),
            max_size: Vec2::new(800.0, 600.0),
            content: Some(content),
            preset_index: 0,
            color: preset.tint_color(),
            corner_radius: 12.0,
            padding: 16.0,
            control_mode: PanelControlMode::None,
            buttons: vec![
                ControlButton::new(PanelControlMode::Resize),
                ControlButton::new(PanelControlMode::Move),
                ControlButton::new(PanelControlMode::Color),
            ],
            show_controls: false,
            selected: false,
            drag_start: None,
            drag_start_pos: Vec2::ZERO,
            drag_start_size: Vec2::ZERO,
        }
    }
    
    pub fn new_empty() -> Self {
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::new(300.0, 200.0),
            min_size: Vec2::new(100.0, 80.0),
            max_size: Vec2::new(800.0, 600.0),
            content: None,
            preset_index: 0,
            color: PanelPreset::Default.tint_color(),
            corner_radius: 12.0,
            padding: 16.0,
            control_mode: PanelControlMode::None,
            buttons: vec![
                ControlButton::new(PanelControlMode::Resize),
                ControlButton::new(PanelControlMode::Move),
                ControlButton::new(PanelControlMode::Color),
            ],
            show_controls: false,
            selected: false,
            drag_start: None,
            drag_start_pos: Vec2::ZERO,
            drag_start_size: Vec2::ZERO,
        }
    }
    
    /// Set initial position
    pub fn at(mut self, x: f32, y: f32) -> Self {
        self.position = Vec2::new(x, y);
        self
    }
    
    /// Set initial size
    pub fn sized(mut self, w: f32, h: f32) -> Self {
        self.size = Vec2::new(w, h);
        self
    }
    
    /// Set preset
    pub fn with_preset(mut self, preset: PanelPreset) -> Self {
        self.color = preset.tint_color();
        self.preset_index = PRESETS.iter().position(|p| *p == preset).unwrap_or(0);
        self
    }
    
    /// Cycle to next color preset
    pub fn cycle_color(&mut self) {
        self.preset_index = (self.preset_index + 1) % PRESETS.len();
        self.color = PRESETS[self.preset_index].tint_color();
    }
    
    /// Cycle corner radius
    pub fn cycle_shape(&mut self) {
        self.corner_radius = match self.corner_radius as i32 {
            0..=4 => 12.0,
            5..=15 => 24.0,
            16..=30 => 0.0,
            _ => 12.0,
        };
    }
    
    /// Move by delta
    pub fn move_by(&mut self, delta: Vec2) {
        self.position += delta;
    }
    
    /// Resize by delta
    pub fn resize_by(&mut self, delta: Vec2) {
        self.size = (self.size + delta).clamp(self.min_size, self.max_size);
    }
    
    /// Update button positions based on panel position
    fn update_button_positions(&mut self) {
        let button_size = 20.0;
        let spacing = 4.0;
        let start_x = self.position.x + self.size.x - (self.buttons.len() as f32 * (button_size + spacing));
        let y = self.position.y + 4.0;
        
        for (i, button) in self.buttons.iter_mut().enumerate() {
            button.position = Vec2::new(start_x + i as f32 * (button_size + spacing), y);
            button.size = Vec2::new(button_size, button_size);
        }
    }
    
    /// Check if point is inside panel
    fn contains(&self, point: Vec2) -> bool {
        point.x >= self.position.x && point.x <= self.position.x + self.size.x &&
        point.y >= self.position.y && point.y <= self.position.y + self.size.y
    }
}

impl Widget for ControllablePanel {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        // Use stored position if set, otherwise use origin
        if self.position == Vec2::ZERO {
            self.position = origin;
        }
        
        // Constrain size
        self.size = self.size.clamp(self.min_size, max_size);
        
        // Layout content
        if let Some(content) = &mut self.content {
            let content_origin = self.position + Vec2::splat(self.padding);
            let content_max = self.size - Vec2::splat(self.padding * 2.0);
            content.layout(content_origin, content_max);
        }
        
        // Update button positions
        self.update_button_positions();
        
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let inside = self.contains(mouse_pos);
        
        // Update show_controls based on hover
        self.show_controls = inside || self.selected;
        
        // Update button hover states
        for button in &mut self.buttons {
            button.hovered = inside && button.contains(mouse_pos);
        }
        
        match event {
            // Mouse click
            winit::event::Event::WindowEvent { 
                event: winit::event::WindowEvent::MouseInput { 
                    state, 
                    button: winit::event::MouseButton::Left, 
                    .. 
                }, 
                .. 
            } => {
                if state.is_pressed() && inside {
                    self.selected = true;
                    
                    // Check button clicks
                    for button in &mut self.buttons {
                        if button.contains(mouse_pos) {
                            button.active = true;
                            self.control_mode = button.mode;
                            return true;
                        }
                    }
                    
                    // Start drag based on mode
                    if self.control_mode != PanelControlMode::None {
                        self.drag_start = Some(mouse_pos);
                        self.drag_start_pos = self.position;
                        self.drag_start_size = self.size;
                    }
                    return true;
                } else if !state.is_pressed() {
                    // Release
                    self.drag_start = None;
                    for button in &mut self.buttons {
                        button.active = false;
                    }
                }
            },
            
            // Mouse move (drag)
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CursorMoved { .. },
                ..
            } => {
                if let Some(start) = self.drag_start {
                    let delta = mouse_pos - start;
                    
                    match self.control_mode {
                        PanelControlMode::Move => {
                            self.position = self.drag_start_pos + delta;
                            self.update_button_positions();
                            return true;
                        },
                        PanelControlMode::Resize => {
                            self.size = (self.drag_start_size + delta).clamp(self.min_size, self.max_size);
                            self.update_button_positions();
                            return true;
                        },
                        _ => {}
                    }
                }
            },
            
            // Keyboard
            winit::event::Event::WindowEvent { 
                event: winit::event::WindowEvent::KeyboardInput { event: key_event, .. }, 
                .. 
            } => {
                // Handle mode keys (R, M, C, S)
                if let winit::keyboard::Key::Character(ref c) = key_event.logical_key {
                    let mode = match c.as_str() {
                        "r" | "R" => Some(PanelControlMode::Resize),
                        "m" | "M" => Some(PanelControlMode::Move),
                        "c" | "C" => Some(PanelControlMode::Color),
                        _ => None,
                    };
                    
                    if let Some(m) = mode {
                        if key_event.state.is_pressed() {
                            if self.selected {
                                self.control_mode = m;
                                
                                // Immediate action for color
                                if m == PanelControlMode::Color {
                                    self.cycle_color();
                                }
                                
                                return true;
                            }
                        } else {
                            self.control_mode = PanelControlMode::None;
                        }
                    }
                }
                
                // Arrow keys for move/resize
                if self.selected && key_event.state.is_pressed() {
                    let step = 10.0;
                    let delta = match key_event.logical_key {
                        winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowUp) => Vec2::new(0.0, -step),
                        winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowDown) => Vec2::new(0.0, step),
                        winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowLeft) => Vec2::new(-step, 0.0),
                        winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowRight) => Vec2::new(step, 0.0),
                        _ => Vec2::ZERO,
                    };
                    
                    if delta != Vec2::ZERO {
                        match self.control_mode {
                            PanelControlMode::Resize => {
                                self.resize_by(delta);
                                self.update_button_positions();
                                return true;
                            },
                            PanelControlMode::Move => {
                                self.move_by(delta);
                                self.update_button_positions();
                                return true;
                            },
                            _ => {}
                        }
                    }
                }
                
                // Escape to deselect
                if let winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape) = key_event.logical_key {
                    if key_event.state.is_pressed() && self.selected {
                        self.selected = false;
                        self.control_mode = PanelControlMode::None;
                        return true;
                    }
                }
            },
            
            _ => {}
        }
        
        // Forward to content
        if let Some(content) = &mut self.content {
            if content.handle_event(event, mouse_pos) {
                return true;
            }
        }
        
        false
    }

    fn update(&mut self, dt: f32) {
        // Update button animations for joyful feedback
        for button in &mut self.buttons {
            button.update(dt);
        }
        
        if let Some(content) = &mut self.content {
            content.update(dt);
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Selection border
        if self.selected {
            renderer.draw_rounded_rect(
                self.position - Vec2::splat(2.0),
                self.size + Vec2::splat(4.0),
                Vec4::new(theme.primary.x, theme.primary.y, theme.primary.z, 0.6),
                self.corner_radius + 2.0,
            );
        }
        
        // Main panel
        renderer.draw_rounded_rect(self.position, self.size, self.color, self.corner_radius);
        
        // Control bar (top)
        if self.show_controls {
            // Subtle control bar background
            let bar_height = 28.0;
            renderer.draw_rounded_rect(
                self.position,
                Vec2::new(self.size.x, bar_height),
                Vec4::new(0.0, 0.0, 0.0, 0.3),
                self.corner_radius,
            );
            
            // Control buttons
            for button in &self.buttons {
                let bg_color = if button.active {
                    theme.primary
                } else if button.hovered {
                    Vec4::new(theme.primary.x, theme.primary.y, theme.primary.z, 0.5)
                } else {
                    Vec4::new(1.0, 1.0, 1.0, 0.2)
                };
                
                renderer.draw_rounded_rect(button.position, button.size, bg_color, 4.0);
                
                // Icon
                let icon_pos = button.position + Vec2::new(4.0, 2.0);
                renderer.draw_text(button.icon(), icon_pos, 12.0, theme.text);
            }
            
            // Mode indicator
            if self.control_mode != PanelControlMode::None {
                let mode_text = format!("[{}] {}", self.control_mode.key_hint(), self.control_mode.description());
                renderer.draw_text(&mode_text, self.position + Vec2::new(8.0, 6.0), 12.0, theme.text_secondary);
            }
        }
        
        // Content
        if let Some(content) = &self.content {
            content.render(renderer);
        }
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widgets::Label;
    
    #[test]
    fn test_panel_controls() {
        let label = Box::new(Label::new("Test"));
        let mut panel = ControllablePanel::new(label)
            .at(100.0, 100.0)
            .sized(300.0, 200.0);
        
        assert_eq!(panel.position, Vec2::new(100.0, 100.0));
        
        panel.control_mode = PanelControlMode::Move;
        panel.move_by(Vec2::new(50.0, 25.0));
        
        assert_eq!(panel.position, Vec2::new(150.0, 125.0));
    }
    
    #[test]
    fn test_color_cycle() {
        let mut panel = ControllablePanel::new_empty();
        let initial_color = panel.color;
        
        panel.cycle_color();
        assert_ne!(panel.color, initial_color);
    }
}
