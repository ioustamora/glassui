//! GlassUI Widget Core Types
//!
//! Contains the Widget trait, Theme system, and easing functions used by all widgets.

use glam::{Vec2, Vec4};
use std::cell::RefCell;
use crate::renderer::GlassRenderer;

// =============================================================================
// THEME SYSTEM
// =============================================================================

/// Theme colors for consistent styling across all widgets
#[derive(Clone)]
pub struct Theme {
    pub primary: Vec4,
    pub secondary: Vec4,
    pub accent: Vec4,
    pub background: Vec4,
    pub surface: Vec4,
    pub text: Vec4,
    pub text_secondary: Vec4,
    pub border: Vec4,
    pub hover: Vec4,
    pub pressed: Vec4,
    pub success: Vec4,
    pub error: Vec4,
    pub warning: Vec4,
}

impl Default for Theme {
    fn default() -> Self { Self::cyberpunk() }
}

impl Theme {
    /// Neon cyan/magenta futuristic theme (default)
    pub fn cyberpunk() -> Self {
        Self {
            primary: Vec4::new(0.0, 0.8, 1.0, 1.0),
            secondary: Vec4::new(0.5, 0.0, 1.0, 1.0),
            accent: Vec4::new(1.0, 0.0, 0.5, 1.0),
            background: Vec4::new(0.05, 0.05, 0.08, 0.9),
            surface: Vec4::new(0.1, 0.1, 0.15, 0.8),
            text: Vec4::new(1.0, 1.0, 1.0, 1.0),
            text_secondary: Vec4::new(0.7, 0.7, 0.7, 1.0),
            border: Vec4::new(0.3, 0.3, 0.4, 0.5),
            hover: Vec4::new(0.2, 0.4, 0.6, 0.8),
            pressed: Vec4::new(0.0, 0.3, 0.4, 0.8),
            success: Vec4::new(0.0, 1.0, 0.5, 1.0),
            error: Vec4::new(1.0, 0.3, 0.3, 1.0),
            warning: Vec4::new(1.0, 0.8, 0.0, 1.0),
        }
    }
    
    /// Modern dark mode with blue accents
    pub fn dark() -> Self {
        Self {
            primary: Vec4::new(0.3, 0.5, 1.0, 1.0),
            secondary: Vec4::new(0.6, 0.3, 0.9, 1.0),
            accent: Vec4::new(1.0, 0.5, 0.2, 1.0),
            background: Vec4::new(0.08, 0.08, 0.1, 0.95),
            surface: Vec4::new(0.12, 0.12, 0.15, 0.9),
            text: Vec4::new(0.95, 0.95, 0.95, 1.0),
            text_secondary: Vec4::new(0.6, 0.6, 0.65, 1.0),
            border: Vec4::new(0.25, 0.25, 0.3, 0.6),
            hover: Vec4::new(0.2, 0.3, 0.5, 0.7),
            pressed: Vec4::new(0.15, 0.25, 0.4, 0.8),
            success: Vec4::new(0.2, 0.9, 0.4, 1.0),
            error: Vec4::new(0.9, 0.25, 0.25, 1.0),
            warning: Vec4::new(0.95, 0.75, 0.1, 1.0),
        }
    }
    
    /// Clean light theme for accessibility
    pub fn light() -> Self {
        Self {
            primary: Vec4::new(0.1, 0.4, 0.8, 1.0),
            secondary: Vec4::new(0.5, 0.2, 0.7, 1.0),
            accent: Vec4::new(0.9, 0.3, 0.1, 1.0),
            background: Vec4::new(0.95, 0.95, 0.97, 0.95),
            surface: Vec4::new(1.0, 1.0, 1.0, 0.9),
            text: Vec4::new(0.1, 0.1, 0.1, 1.0),
            text_secondary: Vec4::new(0.4, 0.4, 0.45, 1.0),
            border: Vec4::new(0.8, 0.8, 0.85, 0.6),
            hover: Vec4::new(0.85, 0.9, 0.95, 0.8),
            pressed: Vec4::new(0.75, 0.85, 0.95, 0.9),
            success: Vec4::new(0.1, 0.7, 0.3, 1.0),
            error: Vec4::new(0.8, 0.2, 0.2, 1.0),
            warning: Vec4::new(0.85, 0.65, 0.0, 1.0),
        }
    }
    
    /// Premium glassmorphism theme exceeding macOS/iOS quality
    pub fn glass() -> Self {
        Self {
            primary: Vec4::new(0.4, 0.7, 1.0, 1.0),
            secondary: Vec4::new(0.7, 0.4, 1.0, 1.0),
            accent: Vec4::new(1.0, 0.6, 0.4, 1.0),
            background: Vec4::new(0.02, 0.02, 0.04, 0.85),
            surface: Vec4::new(0.08, 0.08, 0.12, 0.7),
            text: Vec4::new(1.0, 1.0, 1.0, 0.95),
            text_secondary: Vec4::new(0.8, 0.8, 0.85, 0.8),
            border: Vec4::new(1.0, 1.0, 1.0, 0.1),
            hover: Vec4::new(1.0, 1.0, 1.0, 0.15),
            pressed: Vec4::new(0.4, 0.7, 1.0, 0.3),
            success: Vec4::new(0.3, 1.0, 0.6, 1.0),
            error: Vec4::new(1.0, 0.4, 0.4, 1.0),
            warning: Vec4::new(1.0, 0.85, 0.3, 1.0),
        }
    }
}

thread_local! {
    static CURRENT_THEME: RefCell<Theme> = RefCell::new(Theme::default());
}

/// Set the global theme
pub fn set_theme(theme: Theme) {
    CURRENT_THEME.with(|t| *t.borrow_mut() = theme);
}

/// Get a clone of the current theme
pub fn get_theme() -> Theme {
    CURRENT_THEME.with(|t| t.borrow().clone())
}

// =============================================================================
// WIDGET TRAIT
// =============================================================================

/// Core trait that all UI components implement
pub trait Widget {
    /// Calculate layout and return the size used
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2;
    
    /// Handle input events, return true if consumed
    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool;
    
    /// Update animations and state
    fn update(&mut self, dt: f32);
    
    /// Render the widget
    fn render(&self, renderer: &mut GlassRenderer);
}

// =============================================================================
// EASING FUNCTIONS
// =============================================================================

/// Premium animation easing functions
pub mod easing {
    /// Smooth deceleration (ease-out cubic)
    pub fn ease_out_cubic(t: f32) -> f32 {
        1.0 - (1.0 - t).powi(3)
    }
    
    /// Spring-like overshoot
    pub fn ease_out_back(t: f32) -> f32 {
        let c1 = 1.70158;
        let c3 = c1 + 1.0;
        1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
    }
    
    /// Elastic bounce
    pub fn ease_out_elastic(t: f32) -> f32 {
        if t == 0.0 { return 0.0; }
        if t == 1.0 { return 1.0; }
        let c4 = (2.0 * std::f32::consts::PI) / 3.0;
        2.0_f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
    }
    
    /// Smooth acceleration and deceleration
    pub fn ease_in_out_quart(t: f32) -> f32 {
        if t < 0.5 {
            8.0 * t * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
        }
    }
    
    /// Linear interpolation helper
    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }
}
