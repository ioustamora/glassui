//! GlassUI Hover Effects System
//!
//! Joyful hover interactions:
//! - Hover scaling with spring physics
//! - Glow effects
//! - Ripple effects
//! - Cursor changes

use glam::{Vec2, Vec4};

// =============================================================================
// HOVER STATE
// =============================================================================

/// Hover animation state for a widget
#[derive(Clone, Debug)]
pub struct HoverState {
    /// Is currently hovered
    pub hovered: bool,
    /// Hover animation progress (0.0 to 1.0)
    pub hover_t: f32,
    /// Press animation progress
    pub press_t: f32,
    /// Is currently pressed
    pub pressed: bool,
    /// Ripple effect position and progress
    pub ripple: Option<RippleEffect>,
}

impl Default for HoverState {
    fn default() -> Self {
        Self {
            hovered: false,
            hover_t: 0.0,
            press_t: 0.0,
            pressed: false,
            ripple: None,
        }
    }
}

impl HoverState {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Update hover state based on mouse position
    pub fn update_hover(&mut self, is_inside: bool) {
        self.hovered = is_inside;
    }
    
    /// Trigger press
    pub fn press(&mut self, pos: Vec2) {
        self.pressed = true;
        self.press_t = 1.0;
        self.ripple = Some(RippleEffect::new(pos));
    }
    
    /// Release press
    pub fn release(&mut self) {
        self.pressed = false;
    }
    
    /// Update animations
    pub fn update(&mut self, dt: f32) {
        // Smooth hover animation with spring-like feel
        let target = if self.hovered { 1.0 } else { 0.0 };
        self.hover_t += (target - self.hover_t) * 10.0 * dt;
        
        // Press bounce animation
        if self.press_t > 0.0 {
            self.press_t = (self.press_t - dt * 6.0).max(0.0);
        }
        
        // Update ripple
        if let Some(ripple) = &mut self.ripple {
            ripple.update(dt);
            if ripple.is_finished() {
                self.ripple = None;
            }
        }
    }
    
    /// Get current scale multiplier (for hover effect)
    pub fn scale(&self) -> f32 {
        1.0 + self.hover_t * 0.05 - self.press_t * 0.02
    }
    
    /// Get glow intensity (0.0 to 1.0)
    pub fn glow_intensity(&self) -> f32 {
        self.hover_t * 0.5 + self.press_t * 0.3
    }
}

// =============================================================================
// RIPPLE EFFECT
// =============================================================================

/// Ripple effect emanating from a point
#[derive(Clone, Debug)]
pub struct RippleEffect {
    pub center: Vec2,
    pub radius: f32,
    pub max_radius: f32,
    pub alpha: f32,
    pub progress: f32,
}

impl RippleEffect {
    pub fn new(center: Vec2) -> Self {
        Self {
            center,
            radius: 0.0,
            max_radius: 100.0,
            alpha: 0.4,
            progress: 0.0,
        }
    }
    
    pub fn with_max_radius(mut self, radius: f32) -> Self {
        self.max_radius = radius;
        self
    }
    
    pub fn update(&mut self, dt: f32) {
        self.progress = (self.progress + dt * 3.0).min(1.0);
        
        // Ease out cubic
        let t = 1.0 - (1.0 - self.progress).powi(3);
        self.radius = t * self.max_radius;
        
        // Fade out
        self.alpha = 0.4 * (1.0 - self.progress);
    }
    
    pub fn is_finished(&self) -> bool {
        self.progress >= 1.0
    }
}

// =============================================================================
// HOVER EFFECT PRESETS
// =============================================================================

/// Preset hover effect styles
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HoverEffectStyle {
    /// Subtle scale and glow
    Subtle,
    /// More pronounced bounce
    Bouncy,
    /// Lift with shadow
    Lift,
    /// Color shift
    ColorShift,
    /// Pulse glow
    Pulse,
    /// No effect
    None,
}

impl HoverEffectStyle {
    /// Get scale multiplier for this style
    pub fn scale(&self, hover_t: f32) -> f32 {
        match self {
            HoverEffectStyle::Subtle => 1.0 + hover_t * 0.02,
            HoverEffectStyle::Bouncy => 1.0 + hover_t * 0.08,
            HoverEffectStyle::Lift => 1.0 + hover_t * 0.03,
            HoverEffectStyle::ColorShift => 1.0,
            HoverEffectStyle::Pulse => 1.0 + hover_t * 0.02,
            HoverEffectStyle::None => 1.0,
        }
    }
    
    /// Get glow intensity for this style
    pub fn glow(&self, hover_t: f32) -> f32 {
        match self {
            HoverEffectStyle::Subtle => hover_t * 0.3,
            HoverEffectStyle::Bouncy => hover_t * 0.2,
            HoverEffectStyle::Lift => hover_t * 0.5,
            HoverEffectStyle::ColorShift => 0.0,
            HoverEffectStyle::Pulse => hover_t * 0.6,
            HoverEffectStyle::None => 0.0,
        }
    }
    
    /// Get shadow offset for this style
    pub fn shadow_offset(&self, hover_t: f32) -> Vec2 {
        match self {
            HoverEffectStyle::Lift => Vec2::new(0.0, hover_t * 4.0),
            _ => Vec2::ZERO,
        }
    }
}

// =============================================================================
// HOVER EFFECT RENDERER
// =============================================================================

/// Helper to render hover effects
pub struct HoverEffectRenderer;

impl HoverEffectRenderer {
    /// Modify a color for hover effect
    pub fn modify_color(base: Vec4, hover_state: &HoverState, style: HoverEffectStyle) -> Vec4 {
        let t = hover_state.hover_t;
        
        match style {
            HoverEffectStyle::ColorShift => {
                // Shift hue slightly
                Vec4::new(
                    (base.x + t * 0.1).min(1.0),
                    base.y,
                    (base.z - t * 0.1).max(0.0),
                    base.w,
                )
            },
            HoverEffectStyle::Pulse => {
                // Increase brightness
                let brightness = 1.0 + t * 0.2;
                Vec4::new(
                    (base.x * brightness).min(1.0),
                    (base.y * brightness).min(1.0),
                    (base.z * brightness).min(1.0),
                    base.w,
                )
            },
            _ => base,
        }
    }
    
    /// Get glow color for widget
    pub fn glow_color(base: Vec4, intensity: f32) -> Vec4 {
        Vec4::new(base.x, base.y, base.z, intensity * 0.5)
    }
}

// =============================================================================
// CURSOR TYPE
// =============================================================================

/// Cursor type suggestions for widgets
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CursorType {
    Default,
    Pointer,
    Text,
    Move,
    ResizeNS,
    ResizeEW,
    ResizeNESW,
    ResizeNWSE,
    Grab,
    Grabbing,
    NotAllowed,
    Wait,
    Crosshair,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hover_state() {
        let mut state = HoverState::new();
        state.update_hover(true);
        state.update(0.1);
        
        assert!(state.hover_t > 0.0);
        assert!(state.scale() > 1.0);
    }
    
    #[test]
    fn test_ripple() {
        let mut ripple = RippleEffect::new(Vec2::new(50.0, 50.0));
        ripple.update(0.1);  // Smaller dt to not finish immediately
        
        assert!(ripple.radius > 0.0);
        assert!(!ripple.is_finished());
    }
}
