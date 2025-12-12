//! GlassUI Animation System
//!
//! Provides a centralized animation system with:
//! - `AnimationController` - Drives animations with duration/curve
//! - `Tween<T>` - Interpolates between values
//! - `SpringAnimation` - Physics-based spring animations
//! - Predefined easing curves

use std::time::Duration;

// =============================================================================
// ANIMATION STATUS
// =============================================================================

/// Current status of an animation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimationStatus {
    /// Animation has not started
    Idle,
    /// Animation is running forward
    Forward,
    /// Animation is running backward
    Reverse,
    /// Animation completed (at end)
    Completed,
    /// Animation dismissed (at start)
    Dismissed,
}

impl AnimationStatus {
    pub fn is_animating(&self) -> bool {
        matches!(self, Self::Forward | Self::Reverse)
    }
    
    pub fn is_completed(&self) -> bool {
        matches!(self, Self::Completed | Self::Dismissed)
    }
}

// =============================================================================
// EASING CURVES
// =============================================================================

/// Animation easing curve
#[derive(Clone, Copy, Debug, Default)]
pub enum Curve {
    #[default]
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,
    EaseOutBack,
    EaseOutElastic,
    EaseOutBounce,
    /// Custom cubic bezier curve
    CubicBezier(f32, f32, f32, f32),
}

impl Curve {
    /// Apply the curve to a linear progress value (0.0 to 1.0)
    pub fn transform(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        
        match self {
            Curve::Linear => t,
            Curve::EaseIn => t * t,
            Curve::EaseOut => 1.0 - (1.0 - t).powi(2),
            Curve::EaseInOut => {
                if t < 0.5 { 2.0 * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(2) / 2.0 }
            }
            Curve::EaseInQuad => t * t,
            Curve::EaseOutQuad => 1.0 - (1.0 - t) * (1.0 - t),
            Curve::EaseInOutQuad => {
                if t < 0.5 { 2.0 * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(2) / 2.0 }
            }
            Curve::EaseInCubic => t * t * t,
            Curve::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            Curve::EaseInOutCubic => {
                if t < 0.5 { 4.0 * t * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(3) / 2.0 }
            }
            Curve::EaseInQuart => t * t * t * t,
            Curve::EaseOutQuart => 1.0 - (1.0 - t).powi(4),
            Curve::EaseInOutQuart => {
                if t < 0.5 { 8.0 * t.powi(4) } else { 1.0 - (-2.0 * t + 2.0).powi(4) / 2.0 }
            }
            Curve::EaseInExpo => {
                if t == 0.0 { 0.0 } else { 2.0_f32.powf(10.0 * t - 10.0) }
            }
            Curve::EaseOutExpo => {
                if t == 1.0 { 1.0 } else { 1.0 - 2.0_f32.powf(-10.0 * t) }
            }
            Curve::EaseInOutExpo => {
                if t == 0.0 { 0.0 }
                else if t == 1.0 { 1.0 }
                else if t < 0.5 { 2.0_f32.powf(20.0 * t - 10.0) / 2.0 }
                else { (2.0 - 2.0_f32.powf(-20.0 * t + 10.0)) / 2.0 }
            }
            Curve::EaseOutBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
            }
            Curve::EaseOutElastic => {
                if t == 0.0 { 0.0 }
                else if t == 1.0 { 1.0 }
                else {
                    let c4 = (2.0 * std::f32::consts::PI) / 3.0;
                    2.0_f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
                }
            }
            Curve::EaseOutBounce => {
                let n1 = 7.5625;
                let d1 = 2.75;
                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    let t = t - 1.5 / d1;
                    n1 * t * t + 0.75
                } else if t < 2.5 / d1 {
                    let t = t - 2.25 / d1;
                    n1 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / d1;
                    n1 * t * t + 0.984375
                }
            }
            Curve::CubicBezier(x1, y1, x2, y2) => {
                // Approximate cubic bezier using Newton-Raphson
                cubic_bezier_at(t, *x1, *y1, *x2, *y2)
            }
        }
    }
}

/// Cubic bezier approximation
fn cubic_bezier_at(t: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    // Simplified - for accurate results use Newton-Raphson
    let cx = 3.0 * x1;
    let bx = 3.0 * (x2 - x1) - cx;
    let ax = 1.0 - cx - bx;
    
    let cy = 3.0 * y1;
    let by = 3.0 * (y2 - y1) - cy;
    let ay = 1.0 - cy - by;
    
    // Sample the curve (could be improved with proper solving)
    let sample_x = |t: f32| ((ax * t + bx) * t + cx) * t;
    let sample_y = |t: f32| ((ay * t + by) * t + cy) * t;
    
    // Binary search to find t for x
    let mut low = 0.0_f32;
    let mut high = 1.0_f32;
    let target_x = t;
    
    for _ in 0..16 {
        let mid = (low + high) / 2.0;
        let x = sample_x(mid);
        if x < target_x {
            low = mid;
        } else {
            high = mid;
        }
    }
    
    sample_y((low + high) / 2.0)
}

// =============================================================================
// ANIMATION CONTROLLER
// =============================================================================

/// Controls an animation's progress over time
/// 
/// # Example
/// ```rust
/// let mut controller = AnimationController::new(Duration::from_millis(300))
///     .with_curve(Curve::EaseOutCubic);
/// 
/// // In update loop:
/// controller.update(dt);
/// 
/// // Get current value (0.0 to 1.0)
/// let progress = controller.value();
/// ```
#[derive(Clone, Debug)]
pub struct AnimationController {
    /// Animation duration
    duration: Duration,
    /// Easing curve
    curve: Curve,
    /// Current progress (0.0 to 1.0, before curve applied)
    progress: f32,
    /// Current status
    status: AnimationStatus,
    /// Whether to repeat
    repeat: bool,
    /// Whether to reverse on repeat
    reverse_on_repeat: bool,
}

impl AnimationController {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            curve: Curve::Linear,
            progress: 0.0,
            status: AnimationStatus::Idle,
            repeat: false,
            reverse_on_repeat: false,
        }
    }
    
    pub fn with_curve(mut self, curve: Curve) -> Self {
        self.curve = curve;
        self
    }
    
    pub fn with_repeat(mut self, repeat: bool) -> Self {
        self.repeat = repeat;
        self
    }
    
    pub fn with_reverse(mut self, reverse: bool) -> Self {
        self.reverse_on_repeat = reverse;
        self
    }
    
    /// Start animation forward (0 -> 1)
    pub fn forward(&mut self) {
        self.status = AnimationStatus::Forward;
    }
    
    /// Start animation backward (1 -> 0)
    pub fn reverse(&mut self) {
        self.status = AnimationStatus::Reverse;
    }
    
    /// Reset to beginning
    pub fn reset(&mut self) {
        self.progress = 0.0;
        self.status = AnimationStatus::Idle;
    }
    
    /// Stop animation at current position
    pub fn stop(&mut self) {
        self.status = AnimationStatus::Idle;
    }
    
    /// Set progress directly (0.0 to 1.0)
    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }
    
    /// Update animation by delta time
    pub fn update(&mut self, dt: f32) {
        if !self.status.is_animating() {
            return;
        }
        
        let duration_secs = self.duration.as_secs_f32();
        let delta = if duration_secs > 0.0 { dt / duration_secs } else { 1.0 };
        
        match self.status {
            AnimationStatus::Forward => {
                self.progress += delta;
                if self.progress >= 1.0 {
                    self.progress = 1.0;
                    if self.repeat {
                        if self.reverse_on_repeat {
                            self.status = AnimationStatus::Reverse;
                        } else {
                            self.progress = 0.0;
                        }
                    } else {
                        self.status = AnimationStatus::Completed;
                    }
                }
            }
            AnimationStatus::Reverse => {
                self.progress -= delta;
                if self.progress <= 0.0 {
                    self.progress = 0.0;
                    if self.repeat {
                        if self.reverse_on_repeat {
                            self.status = AnimationStatus::Forward;
                        } else {
                            self.progress = 1.0;
                        }
                    } else {
                        self.status = AnimationStatus::Dismissed;
                    }
                }
            }
            _ => {}
        }
    }
    
    /// Get current value with curve applied (0.0 to 1.0)
    pub fn value(&self) -> f32 {
        self.curve.transform(self.progress)
    }
    
    /// Get raw progress without curve (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        self.progress
    }
    
    /// Get current status
    pub fn status(&self) -> AnimationStatus {
        self.status
    }
    
    /// Check if animation is running
    pub fn is_animating(&self) -> bool {
        self.status.is_animating()
    }
    
    /// Check if animation completed
    pub fn is_completed(&self) -> bool {
        self.status == AnimationStatus::Completed
    }
    
    /// Check if animation dismissed
    pub fn is_dismissed(&self) -> bool {
        self.status == AnimationStatus::Dismissed
    }
}

// =============================================================================
// TWEEN
// =============================================================================

/// Interpolates between two values of type T
pub struct Tween<T> {
    pub begin: T,
    pub end: T,
}

impl<T: Lerp> Tween<T> {
    pub fn new(begin: T, end: T) -> Self {
        Self { begin, end }
    }
    
    /// Get interpolated value at progress t (0.0 to 1.0)
    pub fn lerp(&self, t: f32) -> T {
        T::lerp(&self.begin, &self.end, t)
    }
    
    /// Get value using an animation controller
    pub fn evaluate(&self, controller: &AnimationController) -> T {
        self.lerp(controller.value())
    }
}

/// Trait for values that can be linearly interpolated
pub trait Lerp {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        a + (b - a) * t
    }
}

impl Lerp for f64 {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        a + (b - a) * t as f64
    }
}

impl Lerp for glam::Vec2 {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        *a + (*b - *a) * t
    }
}

impl Lerp for glam::Vec3 {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        *a + (*b - *a) * t
    }
}

impl Lerp for glam::Vec4 {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        *a + (*b - *a) * t
    }
}

// =============================================================================
// SPRING ANIMATION
// =============================================================================

/// Physics-based spring animation
/// 
/// Uses a damped harmonic oscillator model for natural-feeling motion.
#[derive(Clone, Debug)]
pub struct SpringAnimation {
    /// Current value
    value: f32,
    /// Current velocity
    velocity: f32,
    /// Target value
    target: f32,
    /// Spring stiffness (higher = faster)
    stiffness: f32,
    /// Damping ratio (1.0 = critically damped)
    damping: f32,
    /// Mass
    mass: f32,
    /// Threshold for considering animation complete
    threshold: f32,
}

impl SpringAnimation {
    /// Create with default spring parameters
    pub fn new(initial: f32) -> Self {
        Self {
            value: initial,
            velocity: 0.0,
            target: initial,
            stiffness: 200.0,
            damping: 20.0,
            mass: 1.0,
            threshold: 0.001,
        }
    }
    
    /// Create with custom spring parameters
    pub fn with_config(initial: f32, stiffness: f32, damping: f32) -> Self {
        Self {
            value: initial,
            velocity: 0.0,
            target: initial,
            stiffness,
            damping,
            mass: 1.0,
            threshold: 0.001,
        }
    }
    
    /// Preset: Bouncy spring
    pub fn bouncy(initial: f32) -> Self {
        Self::with_config(initial, 300.0, 10.0)
    }
    
    /// Preset: Gentle spring  
    pub fn gentle(initial: f32) -> Self {
        Self::with_config(initial, 100.0, 15.0)
    }
    
    /// Preset: Stiff spring (quick, no overshoot)
    pub fn stiff(initial: f32) -> Self {
        Self::with_config(initial, 400.0, 30.0)
    }
    
    /// Set target value
    pub fn animate_to(&mut self, target: f32) {
        self.target = target;
    }
    
    /// Set value immediately (no animation)
    pub fn set(&mut self, value: f32) {
        self.value = value;
        self.velocity = 0.0;
        self.target = value;
    }
    
    /// Update spring physics
    pub fn update(&mut self, dt: f32) {
        // Spring force
        let displacement = self.value - self.target;
        let spring_force = -self.stiffness * displacement;
        
        // Damping force
        let damping_force = -self.damping * self.velocity;
        
        // Apply forces
        let acceleration = (spring_force + damping_force) / self.mass;
        self.velocity += acceleration * dt;
        self.value += self.velocity * dt;
    }
    
    /// Check if animation is essentially complete
    pub fn is_at_rest(&self) -> bool {
        (self.value - self.target).abs() < self.threshold && 
        self.velocity.abs() < self.threshold
    }
    
    /// Get current value
    pub fn value(&self) -> f32 {
        self.value
    }
    
    /// Get current velocity
    pub fn velocity(&self) -> f32 {
        self.velocity
    }
    
    /// Get target value
    pub fn target(&self) -> f32 {
        self.target
    }
}

// =============================================================================
// ANIMATED VALUE
// =============================================================================

/// A value that automatically animates to its target
pub struct AnimatedValue {
    spring: SpringAnimation,
}

impl AnimatedValue {
    pub fn new(initial: f32) -> Self {
        Self {
            spring: SpringAnimation::new(initial),
        }
    }
    
    pub fn set(&mut self, target: f32) {
        self.spring.animate_to(target);
    }
    
    pub fn set_immediate(&mut self, value: f32) {
        self.spring.set(value);
    }
    
    pub fn update(&mut self, dt: f32) {
        self.spring.update(dt);
    }
    
    pub fn get(&self) -> f32 {
        self.spring.value()
    }
    
    pub fn is_animating(&self) -> bool {
        !self.spring.is_at_rest()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_animation_controller() {
        let mut controller = AnimationController::new(Duration::from_secs(1))
            .with_curve(Curve::Linear);
        
        controller.forward();
        assert!(controller.is_animating());
        
        // Simulate half a second
        controller.update(0.5);
        assert!((controller.value() - 0.5).abs() < 0.01);
        
        // Complete animation
        controller.update(0.6);
        assert!(controller.is_completed());
        assert!((controller.value() - 1.0).abs() < 0.01);
    }
    
    #[test]
    fn test_spring_animation() {
        let mut spring = SpringAnimation::new(0.0);
        spring.animate_to(1.0);
        
        // Run physics for a while
        for _ in 0..100 {
            spring.update(0.016);
        }
        
        // Should be close to target
        assert!((spring.value() - 1.0).abs() < 0.1);
    }
    
    #[test]
    fn test_tween() {
        let tween = Tween::new(0.0f32, 100.0f32);
        
        assert_eq!(tween.lerp(0.0), 0.0);
        assert_eq!(tween.lerp(0.5), 50.0);
        assert_eq!(tween.lerp(1.0), 100.0);
    }
    
    #[test]
    fn test_curves() {
        // All curves should map 0 -> 0 and 1 -> 1
        let curves = [
            Curve::Linear,
            Curve::EaseIn,
            Curve::EaseOut,
            Curve::EaseInOut,
            Curve::EaseOutCubic,
        ];
        
        for curve in curves {
            assert!((curve.transform(0.0) - 0.0).abs() < 0.001, "{:?}", curve);
            assert!((curve.transform(1.0) - 1.0).abs() < 0.001, "{:?}", curve);
        }
    }
}
