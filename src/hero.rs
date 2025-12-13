//! GlassUI Hero/Shared Element Transitions
//!
//! Provides smooth animated transitions where elements morph between
//! different states/views. Like Flutter's Hero widget or Android's
//! shared element transitions.
//!
//! # Usage
//! ```rust
//! // Wrap elements with matching HeroIds
//! let thumbnail = HeroScope::new("avatar-1", Box::new(Image::new(...)));
//! let expanded = HeroScope::new("avatar-1", Box::new(Image::new(...)));
//!
//! // When navigating, the HeroController animates between them
//! hero_controller.start_transition("avatar-1");
//! ```

use std::collections::HashMap;
use std::time::Duration;
use glam::{Vec2, Vec4};
use crate::animation::{AnimationController, Curve, Lerp};
use crate::renderer::GlassRenderer;
use crate::layout::{BoxConstraints, Size, Offset};
use crate::widgets::Widget;

// =============================================================================
// HERO ID
// =============================================================================

/// Unique identifier for hero elements
/// Elements with the same HeroId will animate between each other during transitions
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HeroId(pub String);

impl HeroId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl From<&str> for HeroId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for HeroId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

// =============================================================================
// HERO RECT (Bounds for transitions)
// =============================================================================

/// Rectangle bounds for hero element positioning
#[derive(Clone, Copy, Debug, Default)]
pub struct HeroRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl HeroRect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn from_pos_size(pos: Vec2, size: Vec2) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            width: size.x,
            height: size.y,
        }
    }

    pub fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}

impl Lerp for HeroRect {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        Self {
            x: f32::lerp(&a.x, &b.x, t),
            y: f32::lerp(&a.y, &b.y, t),
            width: f32::lerp(&a.width, &b.width, t),
            height: f32::lerp(&a.height, &b.height, t),
        }
    }
}

// =============================================================================
// HERO FLIGHT (Active transition)
// =============================================================================

/// Represents an active transition between two hero positions
#[derive(Clone, Debug)]
pub struct HeroFlight {
    /// The hero being animated
    pub hero_id: HeroId,
    /// Starting bounds
    pub source: HeroRect,
    /// Ending bounds
    pub destination: HeroRect,
    /// Animation controller for timing
    pub animation: AnimationController,
    /// Starting opacity
    pub source_opacity: f32,
    /// Ending opacity
    pub dest_opacity: f32,
    /// Corner radius transition
    pub source_radius: f32,
    pub dest_radius: f32,
    /// Optional color tint transition
    pub source_tint: Option<Vec4>,
    pub dest_tint: Option<Vec4>,
}

impl HeroFlight {
    pub fn new(
        hero_id: HeroId,
        source: HeroRect,
        destination: HeroRect,
        duration: Duration,
        curve: Curve,
    ) -> Self {
        let mut animation = AnimationController::new(duration).with_curve(curve);
        animation.forward();
        
        Self {
            hero_id,
            source,
            destination,
            animation,
            source_opacity: 1.0,
            dest_opacity: 1.0,
            source_radius: 0.0,
            dest_radius: 0.0,
            source_tint: None,
            dest_tint: None,
        }
    }

    /// Get current interpolated bounds
    pub fn current_rect(&self) -> HeroRect {
        HeroRect::lerp(&self.source, &self.destination, self.animation.value())
    }

    /// Get current interpolated opacity
    pub fn current_opacity(&self) -> f32 {
        f32::lerp(&self.source_opacity, &self.dest_opacity, self.animation.value())
    }

    /// Get current interpolated corner radius
    pub fn current_radius(&self) -> f32 {
        f32::lerp(&self.source_radius, &self.dest_radius, self.animation.value())
    }

    /// Get current interpolated tint (if any)
    pub fn current_tint(&self) -> Option<Vec4> {
        match (&self.source_tint, &self.dest_tint) {
            (Some(src), Some(dst)) => Some(src.lerp(*dst, self.animation.value())),
            (Some(src), None) => {
                let mut result = *src;
                result.w *= 1.0 - self.animation.value(); // Fade out
                Some(result)
            }
            (None, Some(dst)) => {
                let mut result = *dst;
                result.w *= self.animation.value(); // Fade in
                Some(result)
            }
            (None, None) => None,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.animation.update(dt);
    }

    pub fn is_completed(&self) -> bool {
        self.animation.is_completed()
    }

    pub fn progress(&self) -> f32 {
        self.animation.value()
    }
}

// =============================================================================
// HERO CONTROLLER
// =============================================================================

/// Manages hero transitions across the application
/// 
/// Register hero elements, then trigger transitions when navigating between views.
pub struct HeroController {
    /// Registry of hero bounds by ID (updated each frame during layout)
    registry: HashMap<HeroId, HeroRect>,
    /// Currently active flights (transitions)
    flights: Vec<HeroFlight>,
    /// Default transition duration
    pub default_duration: Duration,
    /// Default easing curve
    pub default_curve: Curve,
    /// Placeholder color for transitioning elements
    pub placeholder_color: Vec4,
}

impl HeroController {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
            flights: Vec::new(),
            default_duration: Duration::from_millis(300),
            default_curve: Curve::EaseOutCubic,
            placeholder_color: Vec4::new(0.2, 0.2, 0.25, 0.9),
        }
    }

    /// Register a hero's current bounds (call during layout)
    pub fn register(&mut self, id: HeroId, bounds: HeroRect) {
        self.registry.insert(id, bounds);
    }

    /// Clear registry (call at start of each layout pass)
    pub fn clear_registry(&mut self) {
        self.registry.clear();
    }

    /// Get registered bounds for a hero
    pub fn get_bounds(&self, id: &HeroId) -> Option<&HeroRect> {
        self.registry.get(id)
    }

    /// Start a transition from source to destination bounds
    pub fn start_flight(
        &mut self,
        hero_id: HeroId,
        source: HeroRect,
        destination: HeroRect,
    ) {
        let flight = HeroFlight::new(
            hero_id,
            source,
            destination,
            self.default_duration,
            self.default_curve,
        );
        self.flights.push(flight);
    }

    /// Start a transition with custom configuration
    pub fn start_flight_custom(
        &mut self,
        hero_id: HeroId,
        source: HeroRect,
        destination: HeroRect,
        duration: Duration,
        curve: Curve,
    ) -> &mut HeroFlight {
        let flight = HeroFlight::new(hero_id, source, destination, duration, curve);
        self.flights.push(flight);
        self.flights.last_mut().unwrap()
    }

    /// Start a transition using registered bounds
    /// Returns true if both source and destination were found
    pub fn start_registered_flight(&mut self, hero_id: HeroId, dest_bounds: HeroRect) -> bool {
        if let Some(source) = self.registry.get(&hero_id).cloned() {
            self.start_flight(hero_id, source, dest_bounds);
            true
        } else {
            false
        }
    }

    /// Check if a hero is currently in flight
    pub fn is_in_flight(&self, hero_id: &HeroId) -> bool {
        self.flights.iter().any(|f| &f.hero_id == hero_id)
    }

    /// Get active flight for a hero
    pub fn get_flight(&self, hero_id: &HeroId) -> Option<&HeroFlight> {
        self.flights.iter().find(|f| &f.hero_id == hero_id)
    }

    /// Update all active transitions
    pub fn update(&mut self, dt: f32) {
        for flight in &mut self.flights {
            flight.update(dt);
        }
        // Remove completed flights
        self.flights.retain(|f| !f.is_completed());
    }

    /// Check if any transitions are active
    pub fn has_active_flights(&self) -> bool {
        !self.flights.is_empty()
    }

    /// Number of active transitions
    pub fn active_flight_count(&self) -> usize {
        self.flights.len()
    }

    /// Render all active hero flights as overlays
    /// Call this AFTER rendering the main UI to draw transitioning elements on top
    pub fn render(&self, renderer: &mut GlassRenderer) {
        for flight in &self.flights {
            let rect = flight.current_rect();
            let opacity = flight.current_opacity();
            let radius = flight.current_radius();
            
            // Draw the transitioning element
            let color = flight.current_tint().unwrap_or(self.placeholder_color);
            let color = Vec4::new(color.x, color.y, color.z, color.w * opacity);
            
            renderer.draw_rounded_rect(rect.position(), rect.size(), color, radius);
        }
    }

    /// Abort all active flights
    pub fn abort_all(&mut self) {
        self.flights.clear();
    }

    /// Abort a specific flight
    pub fn abort_flight(&mut self, hero_id: &HeroId) {
        self.flights.retain(|f| &f.hero_id != hero_id);
    }
}

impl Default for HeroController {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// HERO SCOPE WIDGET
// =============================================================================

/// Wrapper widget that marks a child as a hero element
/// 
/// During layout, automatically registers bounds with the global HeroController.
/// During transitions, the child is hidden and a placeholder is shown while
/// the HeroController renders the animated version as an overlay.
pub struct HeroScope {
    /// Unique hero identifier
    pub hero_id: HeroId,
    /// Child widget
    child: Box<dyn Widget>,
    /// Cached position
    position: Vec2,
    /// Cached size
    size: Vec2,
    /// Whether this hero is currently being animated (hide child)
    pub in_flight: bool,
    /// Corner radius for transitions
    pub corner_radius: f32,
    /// Tint color for transitions
    pub tint: Option<Vec4>,
}

impl HeroScope {
    pub fn new(id: impl Into<HeroId>, child: Box<dyn Widget>) -> Self {
        Self {
            hero_id: id.into(),
            child,
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            in_flight: false,
            corner_radius: 0.0,
            tint: None,
        }
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    pub fn with_tint(mut self, tint: Vec4) -> Self {
        self.tint = Some(tint);
        self
    }

    /// Get current bounds as HeroRect
    pub fn bounds(&self) -> HeroRect {
        HeroRect::from_pos_size(self.position, self.size)
    }

    /// Set flight status (call from HeroController when transition starts/ends)
    pub fn set_in_flight(&mut self, in_flight: bool) {
        self.in_flight = in_flight;
    }
}

impl Widget for HeroScope {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = self.child.layout(origin, max_size);
        self.size
    }

    fn layout_with_constraints(&mut self, constraints: BoxConstraints) -> Size {
        let child_size = self.child.layout_with_constraints(constraints);
        self.size = Vec2::new(child_size.width, child_size.height);
        child_size
    }

    fn set_position(&mut self, position: Offset) {
        self.position = Vec2::new(position.x, position.y);
        self.child.set_position(position);
    }

    fn get_position(&self) -> Offset {
        Offset::new(self.position.x, self.position.y)
    }

    fn get_size(&self) -> Size {
        Size::new(self.size.x, self.size.y)
    }

    fn intrinsic_width(&self, height: f32) -> Option<f32> {
        self.child.intrinsic_width(height)
    }

    fn intrinsic_height(&self, width: f32) -> Option<f32> {
        self.child.intrinsic_height(width)
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        // Don't handle events during flight
        if self.in_flight {
            return false;
        }
        self.child.handle_event(event, mouse_pos)
    }

    fn update(&mut self, dt: f32) {
        self.child.update(dt);
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        // If in flight, the HeroController renders the animated version
        // We render a placeholder or nothing
        if self.in_flight {
            // Optionally render a placeholder
            // For now, just hide the original
            return;
        }
        
        self.child.render(renderer);
    }
}

// =============================================================================
// SHARED ELEMENT TRANSITION
// =============================================================================

/// High-level API for page/route transitions with hero elements
pub struct SharedElementTransition {
    /// Hero controller managing the animations
    controller: HeroController,
    /// Whether a transition is currently active
    transitioning: bool,
    /// Direction of transition (true = forward, false = backward)
    forward: bool,
}

impl SharedElementTransition {
    pub fn new() -> Self {
        Self {
            controller: HeroController::new(),
            transitioning: false,
            forward: true,
        }
    }

    /// Start a forward transition (push new view)
    pub fn push(&mut self, heroes: Vec<(HeroId, HeroRect, HeroRect)>) {
        self.transitioning = true;
        self.forward = true;
        
        for (id, source, dest) in heroes {
            self.controller.start_flight(id, source, dest);
        }
    }

    /// Start a backward transition (pop view)
    pub fn pop(&mut self, heroes: Vec<(HeroId, HeroRect, HeroRect)>) {
        self.transitioning = true;
        self.forward = false;
        
        for (id, source, dest) in heroes {
            self.controller.start_flight(id, source, dest);
        }
    }

    /// Update transitions
    pub fn update(&mut self, dt: f32) {
        self.controller.update(dt);
        if !self.controller.has_active_flights() {
            self.transitioning = false;
        }
    }

    /// Render transition overlays
    pub fn render(&self, renderer: &mut GlassRenderer) {
        self.controller.render(renderer);
    }

    /// Check if transition is active
    pub fn is_transitioning(&self) -> bool {
        self.transitioning
    }

    /// Get progress of the transition (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        if self.controller.flights.is_empty() {
            if self.transitioning { 0.0 } else { 1.0 }
        } else {
            // Return average progress
            let sum: f32 = self.controller.flights.iter().map(|f| f.progress()).sum();
            sum / self.controller.flights.len() as f32
        }
    }

    /// Get the controller for fine-grained access
    pub fn controller(&self) -> &HeroController {
        &self.controller
    }

    /// Get mutable controller
    pub fn controller_mut(&mut self) -> &mut HeroController {
        &mut self.controller
    }
}

impl Default for SharedElementTransition {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hero_rect_lerp() {
        let a = HeroRect::new(0.0, 0.0, 100.0, 100.0);
        let b = HeroRect::new(100.0, 50.0, 200.0, 150.0);
        
        let mid = HeroRect::lerp(&a, &b, 0.5);
        assert!((mid.x - 50.0).abs() < 0.01);
        assert!((mid.y - 25.0).abs() < 0.01);
        assert!((mid.width - 150.0).abs() < 0.01);
        assert!((mid.height - 125.0).abs() < 0.01);
    }

    #[test]
    fn test_hero_flight_lifecycle() {
        let mut flight = HeroFlight::new(
            HeroId::new("test"),
            HeroRect::new(0.0, 0.0, 50.0, 50.0),
            HeroRect::new(100.0, 100.0, 200.0, 200.0),
            Duration::from_millis(100),
            Curve::Linear,
        );

        assert!(!flight.is_completed());
        
        // Simulate time passing
        for _ in 0..10 {
            flight.update(0.02);
        }
        
        assert!(flight.is_completed());
    }

    #[test]
    fn test_hero_controller() {
        let mut controller = HeroController::new();
        
        let id = HeroId::new("avatar");
        controller.register(id.clone(), HeroRect::new(10.0, 10.0, 50.0, 50.0));
        
        assert!(controller.get_bounds(&id).is_some());
        
        controller.start_flight(
            id.clone(),
            HeroRect::new(10.0, 10.0, 50.0, 50.0),
            HeroRect::new(200.0, 200.0, 300.0, 300.0),
        );
        
        assert!(controller.is_in_flight(&id));
        assert_eq!(controller.active_flight_count(), 1);
        
        // Complete the flight
        for _ in 0..50 {
            controller.update(0.016);
        }
        
        assert!(!controller.has_active_flights());
    }
}
