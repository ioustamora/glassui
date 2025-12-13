//! GlassUI Panel Presets and Shapes
//!
//! Pre-designed panel styles for quick dashboard creation:
//! - Panel presets for different data types
//! - Custom panel shapes (rect, circle, hex, SVG paths)
//! - Quick styling methods

use glam::Vec4;

// =============================================================================
// PANEL PRESET
// =============================================================================

/// Pre-designed panel styles for different use cases
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PanelPreset {
    /// Default glass panel
    Default,
    /// Blue tint - data display
    Data,
    /// Green tint - success/status
    Status,
    /// Orange/amber tint - warnings
    Warning,
    /// Red tint - errors/alerts
    Alert,
    /// Purple tint - creative/media
    Media,
    /// Cyan tint - technical/code
    Technical,
    /// Dark minimal
    Minimal,
    /// Bright accent
    Accent,
}

impl PanelPreset {
    /// Get the tint color for this preset
    pub fn tint_color(&self) -> Vec4 {
        match self {
            PanelPreset::Default => Vec4::new(0.1, 0.1, 0.15, 0.4),
            PanelPreset::Data => Vec4::new(0.1, 0.15, 0.25, 0.5),
            PanelPreset::Status => Vec4::new(0.1, 0.2, 0.15, 0.5),
            PanelPreset::Warning => Vec4::new(0.25, 0.18, 0.1, 0.5),
            PanelPreset::Alert => Vec4::new(0.25, 0.1, 0.1, 0.5),
            PanelPreset::Media => Vec4::new(0.18, 0.1, 0.22, 0.5),
            PanelPreset::Technical => Vec4::new(0.1, 0.18, 0.2, 0.5),
            PanelPreset::Minimal => Vec4::new(0.05, 0.05, 0.06, 0.7),
            PanelPreset::Accent => Vec4::new(0.15, 0.12, 0.2, 0.6),
        }
    }
    
    /// Get the border/glow color for this preset
    pub fn border_color(&self) -> Vec4 {
        match self {
            PanelPreset::Default => Vec4::new(0.3, 0.3, 0.35, 0.5),
            PanelPreset::Data => Vec4::new(0.3, 0.5, 0.9, 0.6),
            PanelPreset::Status => Vec4::new(0.3, 0.8, 0.4, 0.6),
            PanelPreset::Warning => Vec4::new(0.9, 0.7, 0.2, 0.6),
            PanelPreset::Alert => Vec4::new(0.9, 0.3, 0.3, 0.6),
            PanelPreset::Media => Vec4::new(0.7, 0.3, 0.9, 0.6),
            PanelPreset::Technical => Vec4::new(0.2, 0.8, 0.9, 0.6),
            PanelPreset::Minimal => Vec4::new(0.2, 0.2, 0.25, 0.3),
            PanelPreset::Accent => Vec4::new(0.8, 0.4, 0.9, 0.7),
        }
    }
    
    /// Get default corner radius for this preset
    pub fn corner_radius(&self) -> f32 {
        match self {
            PanelPreset::Minimal => 4.0,
            PanelPreset::Technical => 6.0,
            _ => 12.0,
        }
    }
}

// =============================================================================
// PANEL SHAPE
// =============================================================================

/// Shape of a panel
#[derive(Clone, Debug)]
pub enum PanelShape {
    /// Standard rectangle with corner radius
    Rectangle { corner_radius: f32 },
    /// Rectangle with per-corner radii [top-left, top-right, bottom-right, bottom-left]
    RoundedRect { radii: [f32; 4] },
    /// Perfect circle (width = height, radius = width/2)
    Circle,
    /// Hexagon
    Hexagon,
    /// Custom SVG-like path
    Custom { path: Vec<PathCommand> },
}

impl Default for PanelShape {
    fn default() -> Self {
        PanelShape::Rectangle { corner_radius: 12.0 }
    }
}

/// SVG-like path commands for custom shapes
#[derive(Clone, Debug)]
pub enum PathCommand {
    MoveTo(f32, f32),
    LineTo(f32, f32),
    QuadraticTo { control: (f32, f32), end: (f32, f32) },
    CubicTo { control1: (f32, f32), control2: (f32, f32), end: (f32, f32) },
    Close,
}

// =============================================================================
// PANEL STYLE
// =============================================================================

/// Complete panel style configuration
#[derive(Clone, Debug)]
pub struct PanelStyle {
    pub tint_color: Vec4,
    pub border_color: Vec4,
    pub border_width: f32,
    pub corner_radius: f32,
    pub shadow_blur: f32,
    pub glow_intensity: f32,
    pub shape: PanelShape,
    pub title_bar: bool,
    pub title_font_size: f32,
}

impl Default for PanelStyle {
    fn default() -> Self {
        Self::from_preset(PanelPreset::Default)
    }
}

impl PanelStyle {
    /// Create style from preset
    pub fn from_preset(preset: PanelPreset) -> Self {
        Self {
            tint_color: preset.tint_color(),
            border_color: preset.border_color(),
            border_width: 1.0,
            corner_radius: preset.corner_radius(),
            shadow_blur: 8.0,
            glow_intensity: 0.3,
            shape: PanelShape::Rectangle { corner_radius: preset.corner_radius() },
            title_bar: false,
            title_font_size: 16.0,
        }
    }
    
    /// Set tint color
    pub fn with_tint(mut self, color: Vec4) -> Self {
        self.tint_color = color;
        self
    }
    
    /// Set border color
    pub fn with_border(mut self, color: Vec4) -> Self {
        self.border_color = color;
        self
    }
    
    /// Set corner radius
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self.shape = PanelShape::Rectangle { corner_radius: radius };
        self
    }
    
    /// Enable title bar
    pub fn with_title_bar(mut self) -> Self {
        self.title_bar = true;
        self
    }
    
    /// Set shape to circle
    pub fn as_circle(mut self) -> Self {
        self.shape = PanelShape::Circle;
        self
    }
    
    /// Set shape to hexagon
    pub fn as_hexagon(mut self) -> Self {
        self.shape = PanelShape::Hexagon;
        self
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_preset_colors() {
        let data = PanelPreset::Data;
        assert!(data.tint_color().w > 0.0);
        assert!(data.border_color().w > 0.0);
    }
    
    #[test]
    fn test_style_builder() {
        let style = PanelStyle::from_preset(PanelPreset::Alert)
            .with_radius(20.0)
            .with_title_bar();
        
        assert_eq!(style.corner_radius, 20.0);
        assert!(style.title_bar);
    }
}
