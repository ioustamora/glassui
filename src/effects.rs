//! GlassUI Shader Effects
//!
//! GPU-accelerated visual effects:
//! - Glow effect parameters
//! - Blur configuration
//! - Glassmorphism settings
//! - Gradient definitions

use glam::{Vec2, Vec4};

// =============================================================================
// GLOW EFFECT
// =============================================================================

/// Glow effect configuration
#[derive(Clone, Debug)]
pub struct GlowEffect {
    pub enabled: bool,
    pub color: Vec4,
    pub intensity: f32,
    pub radius: f32,
    pub spread: f32,
}

impl Default for GlowEffect {
    fn default() -> Self {
        Self {
            enabled: false,
            color: Vec4::new(0.4, 0.6, 1.0, 0.8),
            intensity: 1.0,
            radius: 8.0,
            spread: 0.0,
        }
    }
}

impl GlowEffect {
    pub fn new(color: Vec4) -> Self {
        Self {
            enabled: true,
            color,
            ..Default::default()
        }
    }
    
    pub fn primary() -> Self {
        Self::new(Vec4::new(0.4, 0.6, 1.0, 0.8))
    }
    
    pub fn success() -> Self {
        Self::new(Vec4::new(0.3, 0.9, 0.4, 0.8))
    }
    
    pub fn warning() -> Self {
        Self::new(Vec4::new(0.9, 0.7, 0.2, 0.8))
    }
    
    pub fn error() -> Self {
        Self::new(Vec4::new(0.9, 0.3, 0.3, 0.8))
    }
    
    pub fn with_intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity;
        self
    }
    
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
    
    /// Generate WGSL shader code for this glow
    pub fn wgsl_params(&self) -> String {
        format!(
            "GlowParams {{ color: vec4<f32>({}, {}, {}, {}), intensity: {}, radius: {} }}",
            self.color.x, self.color.y, self.color.z, self.color.w,
            self.intensity, self.radius
        )
    }
}

// =============================================================================
// BLUR EFFECT
// =============================================================================

/// Blur effect configuration
#[derive(Clone, Debug)]
pub struct BlurEffect {
    pub enabled: bool,
    pub radius: f32,
    pub quality: BlurQuality,
    pub tint: Option<Vec4>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlurQuality {
    Low,    // 5 samples
    Medium, // 9 samples
    High,   // 13 samples
    Ultra,  // 17 samples
}

impl BlurQuality {
    pub fn samples(&self) -> u32 {
        match self {
            BlurQuality::Low => 5,
            BlurQuality::Medium => 9,
            BlurQuality::High => 13,
            BlurQuality::Ultra => 17,
        }
    }
}

impl Default for BlurEffect {
    fn default() -> Self {
        Self {
            enabled: false,
            radius: 10.0,
            quality: BlurQuality::Medium,
            tint: None,
        }
    }
}

impl BlurEffect {
    pub fn new(radius: f32) -> Self {
        Self {
            enabled: true,
            radius,
            ..Default::default()
        }
    }
    
    pub fn with_quality(mut self, quality: BlurQuality) -> Self {
        self.quality = quality;
        self
    }
    
    pub fn with_tint(mut self, tint: Vec4) -> Self {
        self.tint = Some(tint);
        self
    }
}

// =============================================================================
// GLASSMORPHISM
// =============================================================================

/// Glassmorphism effect settings
#[derive(Clone, Debug)]
pub struct GlassmorphismEffect {
    pub enabled: bool,
    pub blur_radius: f32,
    pub opacity: f32,
    pub border_opacity: f32,
    pub tint: Vec4,
    pub saturation: f32,
}

impl Default for GlassmorphismEffect {
    fn default() -> Self {
        Self {
            enabled: true,
            blur_radius: 20.0,
            opacity: 0.25,
            border_opacity: 0.3,
            tint: Vec4::new(1.0, 1.0, 1.0, 0.1),
            saturation: 1.2,
        }
    }
}

impl GlassmorphismEffect {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn subtle() -> Self {
        Self {
            blur_radius: 10.0,
            opacity: 0.15,
            border_opacity: 0.2,
            ..Default::default()
        }
    }
    
    pub fn frosted() -> Self {
        Self {
            blur_radius: 30.0,
            opacity: 0.4,
            border_opacity: 0.4,
            saturation: 0.8,
            ..Default::default()
        }
    }
    
    pub fn dark() -> Self {
        Self {
            tint: Vec4::new(0.0, 0.0, 0.0, 0.3),
            opacity: 0.5,
            ..Default::default()
        }
    }
    
    /// Generate CSS-like string for debugging
    pub fn to_css(&self) -> String {
        format!(
            "backdrop-filter: blur({}px) saturate({}); background: rgba({},{},{},{}); border: 1px solid rgba(255,255,255,{});",
            self.blur_radius,
            self.saturation,
            (self.tint.x * 255.0) as u8,
            (self.tint.y * 255.0) as u8,
            (self.tint.z * 255.0) as u8,
            self.opacity,
            self.border_opacity
        )
    }
}

// =============================================================================
// GRADIENT
// =============================================================================

/// Gradient type
#[derive(Clone, Debug)]
pub enum GradientType {
    Linear { angle: f32 },
    Radial { center: Vec2 },
    Conic { center: Vec2, start_angle: f32 },
}

/// Color stop in a gradient
#[derive(Clone, Debug)]
pub struct ColorStop {
    pub color: Vec4,
    pub position: f32,  // 0.0 to 1.0
}

impl ColorStop {
    pub fn new(color: Vec4, position: f32) -> Self {
        Self { color, position }
    }
}

/// Gradient definition
#[derive(Clone, Debug)]
pub struct Gradient {
    pub gradient_type: GradientType,
    pub stops: Vec<ColorStop>,
}

impl Gradient {
    pub fn linear(angle: f32) -> Self {
        Self {
            gradient_type: GradientType::Linear { angle },
            stops: Vec::new(),
        }
    }
    
    pub fn radial(center: Vec2) -> Self {
        Self {
            gradient_type: GradientType::Radial { center },
            stops: Vec::new(),
        }
    }
    
    pub fn add_stop(mut self, color: Vec4, position: f32) -> Self {
        self.stops.push(ColorStop::new(color, position));
        self
    }
    
    /// Sample gradient at position (0.0 to 1.0)
    pub fn sample(&self, t: f32) -> Vec4 {
        if self.stops.is_empty() {
            return Vec4::new(0.0, 0.0, 0.0, 1.0);
        }
        if self.stops.len() == 1 {
            return self.stops[0].color;
        }
        
        let t = t.clamp(0.0, 1.0);
        
        // Find surrounding stops
        let mut prev_stop = &self.stops[0];
        for stop in &self.stops {
            if stop.position >= t {
                let range = stop.position - prev_stop.position;
                if range <= 0.0 {
                    return stop.color;
                }
                let local_t = (t - prev_stop.position) / range;
                return Vec4::new(
                    prev_stop.color.x + (stop.color.x - prev_stop.color.x) * local_t,
                    prev_stop.color.y + (stop.color.y - prev_stop.color.y) * local_t,
                    prev_stop.color.z + (stop.color.z - prev_stop.color.z) * local_t,
                    prev_stop.color.w + (stop.color.w - prev_stop.color.w) * local_t,
                );
            }
            prev_stop = stop;
        }
        
        self.stops.last().map(|s| s.color).unwrap_or(Vec4::ONE)
    }
    
    /// Create a sunset gradient
    pub fn sunset() -> Self {
        Self::linear(45.0)
            .add_stop(Vec4::new(0.9, 0.4, 0.3, 1.0), 0.0)
            .add_stop(Vec4::new(0.9, 0.6, 0.2, 1.0), 0.5)
            .add_stop(Vec4::new(0.3, 0.2, 0.5, 1.0), 1.0)
    }
    
    /// Create an ocean gradient
    pub fn ocean() -> Self {
        Self::linear(135.0)
            .add_stop(Vec4::new(0.1, 0.3, 0.5, 1.0), 0.0)
            .add_stop(Vec4::new(0.2, 0.5, 0.7, 1.0), 0.5)
            .add_stop(Vec4::new(0.3, 0.7, 0.8, 1.0), 1.0)
    }
    
    /// Create a neon gradient
    pub fn neon() -> Self {
        Self::linear(90.0)
            .add_stop(Vec4::new(1.0, 0.0, 0.5, 1.0), 0.0)
            .add_stop(Vec4::new(0.5, 0.0, 1.0, 1.0), 0.5)
            .add_stop(Vec4::new(0.0, 0.5, 1.0, 1.0), 1.0)
    }
}

// =============================================================================
// EFFECT STACK
// =============================================================================

/// Combined effects for a widget
#[derive(Clone, Debug, Default)]
pub struct EffectStack {
    pub glow: Option<GlowEffect>,
    pub blur: Option<BlurEffect>,
    pub glass: Option<GlassmorphismEffect>,
    pub gradient: Option<Gradient>,
}

impl EffectStack {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_glow(mut self, glow: GlowEffect) -> Self {
        self.glow = Some(glow);
        self
    }
    
    pub fn with_blur(mut self, blur: BlurEffect) -> Self {
        self.blur = Some(blur);
        self
    }
    
    pub fn with_glass(mut self, glass: GlassmorphismEffect) -> Self {
        self.glass = Some(glass);
        self
    }
    
    pub fn with_gradient(mut self, gradient: Gradient) -> Self {
        self.gradient = Some(gradient);
        self
    }
    
    /// Preset: Glowing glass panel
    pub fn glowing_glass() -> Self {
        Self::new()
            .with_glass(GlassmorphismEffect::default())
            .with_glow(GlowEffect::primary().with_intensity(0.5))
    }
    
    /// Preset: Frosted card
    pub fn frosted_card() -> Self {
        Self::new()
            .with_glass(GlassmorphismEffect::frosted())
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gradient_sample() {
        let gradient = Gradient::linear(0.0)
            .add_stop(Vec4::new(0.0, 0.0, 0.0, 1.0), 0.0)
            .add_stop(Vec4::new(1.0, 1.0, 1.0, 1.0), 1.0);
        
        let mid = gradient.sample(0.5);
        assert!((mid.x - 0.5).abs() < 0.01);
    }
    
    #[test]
    fn test_effect_stack() {
        let effects = EffectStack::glowing_glass();
        assert!(effects.glass.is_some());
        assert!(effects.glow.is_some());
    }
}
