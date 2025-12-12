//! GlassUI Style System
//!
//! Provides CSS-like styling capabilities:
//! - `WidgetStyle` - Style properties for widgets
//! - `StyleSheet` - Collection of named styles
//! - Style variants (primary, secondary, danger, etc.)
//! - Style inheritance and merging

use glam::Vec4;
use crate::layout::EdgeInsets;

// =============================================================================
// COLORS
// =============================================================================

/// Color utilities
pub mod colors {
    use glam::Vec4;
    
    /// Create color from hex value (e.g., 0xFF5733)
    pub const fn from_hex(hex: u32) -> Vec4 {
        let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let b = (hex & 0xFF) as f32 / 255.0;
        Vec4::new(r, g, b, 1.0)
    }
    
    /// Create color from hex with alpha
    pub const fn from_hex_alpha(hex: u32, alpha: f32) -> Vec4 {
        let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let b = (hex & 0xFF) as f32 / 255.0;
        Vec4::new(r, g, b, alpha)
    }
    
    /// Create color from RGBA (0-255)
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Vec4 {
        Vec4::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }
    
    /// Lighten a color
    pub fn lighten(color: Vec4, amount: f32) -> Vec4 {
        Vec4::new(
            (color.x + amount).min(1.0),
            (color.y + amount).min(1.0),
            (color.z + amount).min(1.0),
            color.w,
        )
    }
    
    /// Darken a color
    pub fn darken(color: Vec4, amount: f32) -> Vec4 {
        Vec4::new(
            (color.x - amount).max(0.0),
            (color.y - amount).max(0.0),
            (color.z - amount).max(0.0),
            color.w,
        )
    }
    
    /// Set alpha of a color
    pub fn with_alpha(color: Vec4, alpha: f32) -> Vec4 {
        Vec4::new(color.x, color.y, color.z, alpha)
    }
}

// =============================================================================
// BORDER
// =============================================================================

/// Border style properties
#[derive(Clone, Copy, Debug, Default)]
pub struct Border {
    pub width: f32,
    pub color: Vec4,
    pub radius: f32,
}

impl Border {
    pub const NONE: Border = Border {
        width: 0.0,
        color: Vec4::ZERO,
        radius: 0.0,
    };
    
    pub fn new(width: f32, color: Vec4) -> Self {
        Self { width, color, radius: 0.0 }
    }
    
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}

// =============================================================================
// SHADOW
// =============================================================================

/// Box shadow properties
#[derive(Clone, Copy, Debug, Default)]
pub struct Shadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur: f32,
    pub spread: f32,
    pub color: Vec4,
}

impl Shadow {
    pub const NONE: Shadow = Shadow {
        offset_x: 0.0,
        offset_y: 0.0,
        blur: 0.0,
        spread: 0.0,
        color: Vec4::ZERO,
    };
    
    pub fn new(offset_x: f32, offset_y: f32, blur: f32, color: Vec4) -> Self {
        Self { offset_x, offset_y, blur, spread: 0.0, color }
    }
    
    /// Preset: subtle elevation
    pub fn sm() -> Self {
        Self::new(0.0, 1.0, 2.0, Vec4::new(0.0, 0.0, 0.0, 0.1))
    }
    
    /// Preset: medium elevation
    pub fn md() -> Self {
        Self::new(0.0, 4.0, 6.0, Vec4::new(0.0, 0.0, 0.0, 0.15))
    }
    
    /// Preset: large elevation
    pub fn lg() -> Self {
        Self::new(0.0, 10.0, 15.0, Vec4::new(0.0, 0.0, 0.0, 0.2))
    }
}

// =============================================================================
// TEXT STYLE
// =============================================================================

/// Text styling properties
#[derive(Clone, Debug)]
pub struct TextStyle {
    pub font_size: f32,
    pub color: Vec4,
    pub font_weight: FontWeight,
    pub letter_spacing: f32,
    pub line_height: f32,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font_size: 14.0,
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
            font_weight: FontWeight::Regular,
            letter_spacing: 0.0,
            line_height: 1.4,
        }
    }
}

impl TextStyle {
    pub fn new(size: f32, color: Vec4) -> Self {
        Self {
            font_size: size,
            color,
            ..Default::default()
        }
    }
    
    pub fn with_weight(mut self, weight: FontWeight) -> Self {
        self.font_weight = weight;
        self
    }
}

/// Font weight
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FontWeight {
    Thin,
    Light,
    #[default]
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

// =============================================================================
// WIDGET STYLE
// =============================================================================

/// Complete style for a widget
#[derive(Clone, Debug, Default)]
pub struct WidgetStyle {
    // Layout
    pub padding: Option<EdgeInsets>,
    pub margin: Option<EdgeInsets>,
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    
    // Background
    pub background: Option<Vec4>,
    pub background_hover: Option<Vec4>,
    pub background_pressed: Option<Vec4>,
    pub background_disabled: Option<Vec4>,
    
    // Border
    pub border: Option<Border>,
    pub border_hover: Option<Border>,
    
    // Shadow
    pub shadow: Option<Shadow>,
    
    // Text
    pub text_style: Option<TextStyle>,
    
    // Effects
    pub opacity: Option<f32>,
    pub corner_radius: Option<f32>,
}

impl WidgetStyle {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Merge another style on top of this one (other takes precedence)
    pub fn merge(&self, other: &WidgetStyle) -> WidgetStyle {
        WidgetStyle {
            padding: other.padding.or(self.padding),
            margin: other.margin.or(self.margin),
            min_width: other.min_width.or(self.min_width),
            max_width: other.max_width.or(self.max_width),
            min_height: other.min_height.or(self.min_height),
            max_height: other.max_height.or(self.max_height),
            background: other.background.or(self.background),
            background_hover: other.background_hover.or(self.background_hover),
            background_pressed: other.background_pressed.or(self.background_pressed),
            background_disabled: other.background_disabled.or(self.background_disabled),
            border: other.border.or(self.border),
            border_hover: other.border_hover.or(self.border_hover),
            shadow: other.shadow.or(self.shadow),
            text_style: other.text_style.clone().or(self.text_style.clone()),
            opacity: other.opacity.or(self.opacity),
            corner_radius: other.corner_radius.or(self.corner_radius),
        }
    }
    
    // Builder methods
    
    pub fn padding(mut self, padding: EdgeInsets) -> Self {
        self.padding = Some(padding);
        self
    }
    
    pub fn margin(mut self, margin: EdgeInsets) -> Self {
        self.margin = Some(margin);
        self
    }
    
    pub fn background(mut self, color: Vec4) -> Self {
        self.background = Some(color);
        self
    }
    
    pub fn background_states(mut self, normal: Vec4, hover: Vec4, pressed: Vec4) -> Self {
        self.background = Some(normal);
        self.background_hover = Some(hover);
        self.background_pressed = Some(pressed);
        self
    }
    
    pub fn border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }
    
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = Some(radius);
        self
    }
    
    pub fn shadow(mut self, shadow: Shadow) -> Self {
        self.shadow = Some(shadow);
        self
    }
    
    pub fn text(mut self, style: TextStyle) -> Self {
        self.text_style = Some(style);
        self
    }
    
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity);
        self
    }
}

// =============================================================================
// STYLE VARIANTS
// =============================================================================

/// Predefined button style variants
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Outline,
    Ghost,
    Danger,
    Success,
    Warning,
}

impl ButtonVariant {
    /// Get the style for this variant
    pub fn style(&self) -> WidgetStyle {
        match self {
            ButtonVariant::Primary => WidgetStyle::new()
                .background_states(
                    colors::from_hex(0x6366F1), // Indigo
                    colors::from_hex(0x818CF8), // Lighter
                    colors::from_hex(0x4F46E5), // Darker
                )
                .corner_radius(8.0)
                .padding(EdgeInsets::symmetric(16.0, 10.0)),
                
            ButtonVariant::Secondary => WidgetStyle::new()
                .background_states(
                    colors::from_hex_alpha(0x6366F1, 0.1),
                    colors::from_hex_alpha(0x6366F1, 0.2),
                    colors::from_hex_alpha(0x6366F1, 0.3),
                )
                .corner_radius(8.0)
                .padding(EdgeInsets::symmetric(16.0, 10.0)),
                
            ButtonVariant::Outline => WidgetStyle::new()
                .background(Vec4::ZERO)
                .border(Border::new(1.5, colors::from_hex(0x6366F1)).with_radius(8.0))
                .corner_radius(8.0)
                .padding(EdgeInsets::symmetric(16.0, 10.0)),
                
            ButtonVariant::Ghost => WidgetStyle::new()
                .background_states(
                    Vec4::ZERO,
                    colors::from_hex_alpha(0xFFFFFF, 0.1),
                    colors::from_hex_alpha(0xFFFFFF, 0.2),
                )
                .corner_radius(8.0)
                .padding(EdgeInsets::symmetric(16.0, 10.0)),
                
            ButtonVariant::Danger => WidgetStyle::new()
                .background_states(
                    colors::from_hex(0xEF4444), // Red
                    colors::from_hex(0xF87171),
                    colors::from_hex(0xDC2626),
                )
                .corner_radius(8.0)
                .padding(EdgeInsets::symmetric(16.0, 10.0)),
                
            ButtonVariant::Success => WidgetStyle::new()
                .background_states(
                    colors::from_hex(0x22C55E), // Green
                    colors::from_hex(0x4ADE80),
                    colors::from_hex(0x16A34A),
                )
                .corner_radius(8.0)
                .padding(EdgeInsets::symmetric(16.0, 10.0)),
                
            ButtonVariant::Warning => WidgetStyle::new()
                .background_states(
                    colors::from_hex(0xF59E0B), // Amber
                    colors::from_hex(0xFBBF24),
                    colors::from_hex(0xD97706),
                )
                .corner_radius(8.0)
                .padding(EdgeInsets::symmetric(16.0, 10.0)),
        }
    }
}

// =============================================================================
// STYLE SHEET
// =============================================================================

use std::collections::HashMap;

/// A collection of named styles
#[derive(Clone, Debug, Default)]
pub struct StyleSheet {
    styles: HashMap<String, WidgetStyle>,
}

impl StyleSheet {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a style with a name
    pub fn add(&mut self, name: impl Into<String>, style: WidgetStyle) {
        self.styles.insert(name.into(), style);
    }
    
    /// Get a style by name
    pub fn get(&self, name: &str) -> Option<&WidgetStyle> {
        self.styles.get(name)
    }
    
    /// Get a style, merging with a base style if it exists
    pub fn get_with_base(&self, name: &str, base: &str) -> Option<WidgetStyle> {
        let base_style = self.get(base)?;
        let override_style = self.get(name)?;
        Some(base_style.merge(override_style))
    }
    
    /// Builder pattern: add and return self
    pub fn with(mut self, name: impl Into<String>, style: WidgetStyle) -> Self {
        self.add(name, style);
        self
    }
}

// =============================================================================
// SIZE PRESETS
// =============================================================================

/// Size variant for consistent sizing
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SizeVariant {
    XSmall,
    Small,
    #[default]
    Medium,
    Large,
    XLarge,
}

impl SizeVariant {
    /// Get padding for this size
    pub fn padding(&self) -> EdgeInsets {
        match self {
            SizeVariant::XSmall => EdgeInsets::symmetric(8.0, 4.0),
            SizeVariant::Small => EdgeInsets::symmetric(12.0, 6.0),
            SizeVariant::Medium => EdgeInsets::symmetric(16.0, 10.0),
            SizeVariant::Large => EdgeInsets::symmetric(20.0, 12.0),
            SizeVariant::XLarge => EdgeInsets::symmetric(24.0, 16.0),
        }
    }
    
    /// Get font size for this size
    pub fn font_size(&self) -> f32 {
        match self {
            SizeVariant::XSmall => 12.0,
            SizeVariant::Small => 13.0,
            SizeVariant::Medium => 14.0,
            SizeVariant::Large => 16.0,
            SizeVariant::XLarge => 18.0,
        }
    }
    
    /// Get corner radius for this size
    pub fn corner_radius(&self) -> f32 {
        match self {
            SizeVariant::XSmall => 4.0,
            SizeVariant::Small => 6.0,
            SizeVariant::Medium => 8.0,
            SizeVariant::Large => 10.0,
            SizeVariant::XLarge => 12.0,
        }
    }
}

// =============================================================================
// SPACING
// =============================================================================

/// Consistent spacing values
pub mod spacing {
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 8.0;
    pub const MD: f32 = 16.0;
    pub const LG: f32 = 24.0;
    pub const XL: f32 = 32.0;
    pub const XXL: f32 = 48.0;
}
