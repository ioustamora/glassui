//! GlassUI Media Widgets
//!
//! Media display widgets: Image, Icon
//!
//! Note: Heavy image loading is typically done via the Resource async system
//! in state.rs. This module provides the widgets to display loaded images.

use glam::{Vec2, Vec4};
use crate::renderer::GlassRenderer;
use crate::layout::{BoxConstraints, Size, Offset};
use crate::widgets::core::{Widget, get_theme};

// =============================================================================
// BOX FIT
// =============================================================================

/// How an image should be scaled to fit its container
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BoxFit {
    /// Scale to fill container while maintaining aspect ratio (may crop)
    #[default]
    Cover,
    /// Scale to fit entirely within container (may letterbox)
    Contain,
    /// Stretch to fill container (may distort)
    Fill,
    /// Show at native size, may be clipped
    None,
    /// Scale down only if larger than container
    ScaleDown,
}

// =============================================================================
// IMAGE SOURCE
// =============================================================================

/// Source for image data
#[derive(Clone, Debug)]
pub enum ImageSource {
    /// Path to asset file
    Asset(String),
    /// URL for network image (requires async loading)
    Network(String),
    /// Raw pixel data (RGBA8)
    Memory {
        data: Vec<u8>,
        width: u32,
        height: u32,
    },
    /// Placeholder while loading
    Placeholder,
}

impl Default for ImageSource {
    fn default() -> Self {
        ImageSource::Placeholder
    }
}

// =============================================================================
// IMAGE WIDGET
// =============================================================================

/// Image display widget
///
/// # Example
/// ```rust
/// let img = Image::new(ImageSource::Asset("logo.png".into()))
///     .with_fit(BoxFit::Contain)
///     .with_size(200.0, 150.0);
/// ```
///
/// # Loading Pattern
/// For async image loading, use with Resource:
/// ```rust
/// let avatar = Resource::new(|| async {
///     load_image_from_url("https://example.com/avatar.png").await
/// });
/// ```
pub struct Image {
    pub position: Vec2,
    pub size: Vec2,
    pub source: ImageSource,
    pub fit: BoxFit,
    pub tint: Option<Vec4>,
    pub corner_radius: f32,
    /// Fixed width (None = use intrinsic or constraints)
    pub width: Option<f32>,
    /// Fixed height (None = use intrinsic or constraints)
    pub height: Option<f32>,
    /// Native image dimensions (set when image loads)
    pub native_size: Option<(u32, u32)>,
    /// Error loading image
    pub error: Option<String>,
    /// Whether image is currently loading
    pub loading: bool,
}

impl Image {
    pub fn new(source: ImageSource) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::new(100.0, 100.0), // Default placeholder size
            source,
            fit: BoxFit::Cover,
            tint: None,
            corner_radius: 0.0,
            width: None,
            height: None,
            native_size: None,
            error: None,
            loading: false,
        }
    }
    
    /// Create a placeholder image
    pub fn placeholder() -> Self {
        Self::new(ImageSource::Placeholder)
    }
    
    pub fn with_fit(mut self, fit: BoxFit) -> Self {
        self.fit = fit;
        self
    }
    
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }
    
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }
    
    pub fn with_tint(mut self, tint: Vec4) -> Self {
        self.tint = Some(tint);
        self
    }
    
    /// Calculate display size based on constraints and fit mode
    fn calculate_display_size(&self, available: Size) -> Size {
        let native = self.native_size.unwrap_or((100, 100));
        let native_w = native.0 as f32;
        let native_h = native.1 as f32;
        let aspect = native_w / native_h;
        
        // Use explicit size if provided
        let target_w = self.width.unwrap_or(available.width);
        let target_h = self.height.unwrap_or(available.height);
        
        match self.fit {
            BoxFit::Fill => Size::new(target_w, target_h),
            BoxFit::None => Size::new(native_w.min(target_w), native_h.min(target_h)),
            BoxFit::Contain => {
                // Fit entirely within bounds
                let scale = (target_w / native_w).min(target_h / native_h);
                Size::new(native_w * scale, native_h * scale)
            }
            BoxFit::Cover => {
                // Fill bounds, may crop
                let scale = (target_w / native_w).max(target_h / native_h);
                Size::new(native_w * scale, native_h * scale)
            }
            BoxFit::ScaleDown => {
                // Only scale down, never up
                if native_w <= target_w && native_h <= target_h {
                    Size::new(native_w, native_h)
                } else {
                    let scale = (target_w / native_w).min(target_h / native_h);
                    Size::new(native_w * scale, native_h * scale)
                }
            }
        }
    }
}

impl Widget for Image {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        let available = Size::new(max_size.x, max_size.y);
        let display = self.calculate_display_size(available);
        self.size = Vec2::new(display.width, display.height);
        self.size
    }
    
    fn layout_with_constraints(&mut self, constraints: BoxConstraints) -> Size {
        let available = constraints.biggest();
        let display = self.calculate_display_size(available);
        let constrained = constraints.constrain(display);
        self.size = Vec2::new(constrained.width, constrained.height);
        constrained
    }
    
    fn set_position(&mut self, position: Offset) {
        self.position = Vec2::new(position.x, position.y);
    }
    
    fn get_position(&self) -> Offset {
        Offset::new(self.position.x, self.position.y)
    }
    
    fn get_size(&self) -> Size {
        Size::new(self.size.x, self.size.y)
    }
    
    fn intrinsic_width(&self, _height: f32) -> Option<f32> {
        self.width.or_else(|| self.native_size.map(|(w, _)| w as f32))
    }
    
    fn intrinsic_height(&self, _width: f32) -> Option<f32> {
        self.height.or_else(|| self.native_size.map(|(_, h)| h as f32))
    }
    
    fn handle_event(&mut self, _event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool {
        false // Images don't handle events by default
    }
    
    fn update(&mut self, _dt: f32) {
        // Could animate loading spinner here
    }
    
    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        match &self.source {
            ImageSource::Placeholder | ImageSource::Network(_) if self.loading => {
                // Draw loading placeholder
                renderer.draw_rounded_rect(
                    self.position,
                    self.size,
                    Vec4::new(0.1, 0.1, 0.12, 0.8),
                    self.corner_radius
                );
                // Loading indicator (simple pulsing dot pattern)
                let cx = self.position.x + self.size.x / 2.0;
                let cy = self.position.y + self.size.y / 2.0;
                renderer.draw_rounded_rect(
                    Vec2::new(cx - 4.0, cy - 4.0),
                    Vec2::new(8.0, 8.0),
                    Vec4::new(0.4, 0.4, 0.45, 0.8),
                    4.0
                );
            }
            ImageSource::Placeholder => {
                // Draw placeholder with icon
                renderer.draw_rounded_rect(
                    self.position,
                    self.size,
                    Vec4::new(0.15, 0.15, 0.18, 0.9),
                    self.corner_radius
                );
                // Draw image icon in center
                let icon_size = self.size.x.min(self.size.y) * 0.3;
                let icon_pos = self.position + (self.size - Vec2::splat(icon_size)) / 2.0;
                renderer.draw_text(
                    "🖼",
                    icon_pos,
                    icon_size,
                    theme.text_secondary
                );
            }
            ImageSource::Asset(path) => {
                // For now, draw placeholder with path hint
                // Real implementation would use cached GPU texture
                renderer.draw_rounded_rect(
                    self.position,
                    self.size,
                    self.tint.unwrap_or(Vec4::new(0.2, 0.2, 0.25, 0.9)),
                    self.corner_radius
                );
                // Show filename at bottom
                let filename = path.split('/').last().unwrap_or(path);
                if self.size.y > 40.0 {
                    renderer.draw_text(
                        filename,
                        Vec2::new(self.position.x + 8.0, self.position.y + self.size.y - 20.0),
                        12.0,
                        theme.text_secondary
                    );
                }
            }
            ImageSource::Memory { .. } => {
                // Real implementation would upload to GPU texture
                // For now, draw as colored rect with tint
                renderer.draw_rounded_rect(
                    self.position,
                    self.size,
                    self.tint.unwrap_or(theme.surface),
                    self.corner_radius
                );
            }
            ImageSource::Network(url) => {
                // Draw placeholder with URL hint
                renderer.draw_rounded_rect(
                    self.position,
                    self.size,
                    Vec4::new(0.12, 0.12, 0.15, 0.9),
                    self.corner_radius
                );
                if self.error.is_some() {
                    // Error state
                    let cx = self.position.x + self.size.x / 2.0 - 10.0;
                    let cy = self.position.y + self.size.y / 2.0 - 10.0;
                    renderer.draw_text("⚠", Vec2::new(cx, cy), 20.0, theme.error);
                }
            }
        }
    }
}

// =============================================================================
// ICON WIDGET
// =============================================================================

/// Simple icon widget using text glyphs
///
/// Uses Unicode/emoji glyphs for now. Future: support icon fonts like
/// Material Icons or FontAwesome.
pub struct Icon {
    pub position: Vec2,
    pub size: f32,
    pub glyph: String,
    pub color: Option<Vec4>,
}

impl Icon {
    pub fn new(glyph: &str) -> Self {
        Self {
            position: Vec2::ZERO,
            size: 24.0,
            glyph: glyph.to_string(),
            color: None,
        }
    }
    
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

impl Widget for Icon {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        Vec2::new(self.size, self.size)
    }
    
    fn layout_with_constraints(&mut self, constraints: BoxConstraints) -> Size {
        let s = constraints.constrain(Size::square(self.size));
        s
    }
    
    fn set_position(&mut self, position: Offset) {
        self.position = Vec2::new(position.x, position.y);
    }
    
    fn get_position(&self) -> Offset {
        Offset::new(self.position.x, self.position.y)
    }
    
    fn get_size(&self) -> Size {
        Size::square(self.size)
    }
    
    fn intrinsic_width(&self, _height: f32) -> Option<f32> {
        Some(self.size)
    }
    
    fn intrinsic_height(&self, _width: f32) -> Option<f32> {
        Some(self.size)
    }
    
    fn handle_event(&mut self, _event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool {
        false
    }
    
    fn update(&mut self, _dt: f32) {}
    
    fn render(&self, renderer: &mut GlassRenderer) {
        let color = self.color.unwrap_or_else(|| get_theme().text);
        renderer.draw_text(&self.glyph, self.position, self.size, color);
    }
}
