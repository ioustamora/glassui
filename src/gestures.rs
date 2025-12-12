//! GlassUI Gesture Recognition System
//!
//! Provides gesture detection for touch and pen input:
//! - Tap, Double Tap, Long Press
//! - Drag, Pan, Swipe
//! - Pinch, Rotate
//! - Gesture state machine

use glam::Vec2;
use std::time::{Duration, Instant};

// =============================================================================
// GESTURE TYPES
// =============================================================================

/// Type of detected gesture
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GestureType {
    Tap,
    DoubleTap,
    LongPress,
    Pan,
    Swipe,
    Pinch,
    Rotate,
}

/// Direction of a swipe gesture
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwipeDirection {
    Left,
    Right,
    Up,
    Down,
}

/// State of an ongoing gesture
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GestureState {
    /// Gesture may start
    Possible,
    /// Gesture has started
    Began,
    /// Gesture is ongoing
    Changed,
    /// Gesture completed successfully
    Ended,
    /// Gesture was cancelled
    Cancelled,
    /// Gesture failed recognition
    Failed,
}

// =============================================================================
// GESTURE EVENT
// =============================================================================

/// A recognized gesture event
#[derive(Clone, Debug)]
pub struct GestureEvent {
    /// Type of gesture
    pub gesture_type: GestureType,
    /// Current state
    pub state: GestureState,
    /// Current position (or center for multi-touch)
    pub position: Vec2,
    /// Starting position
    pub start_position: Vec2,
    /// Delta from last event
    pub delta: Vec2,
    /// Velocity (units per second)
    pub velocity: Vec2,
    /// Scale factor (for pinch, 1.0 = no change)
    pub scale: f32,
    /// Rotation angle in radians (for rotate)
    pub rotation: f32,
    /// Number of touches involved
    pub touch_count: u32,
    /// Time since gesture started
    pub duration: Duration,
}

impl GestureEvent {
    fn new(gesture_type: GestureType, state: GestureState) -> Self {
        Self {
            gesture_type,
            state,
            position: Vec2::ZERO,
            start_position: Vec2::ZERO,
            delta: Vec2::ZERO,
            velocity: Vec2::ZERO,
            scale: 1.0,
            rotation: 0.0,
            touch_count: 1,
            duration: Duration::ZERO,
        }
    }
}

// =============================================================================
// TOUCH POINT
// =============================================================================

/// A single touch point
#[derive(Clone, Debug)]
pub struct TouchPoint {
    /// Unique identifier for this touch
    pub id: u64,
    /// Current position
    pub position: Vec2,
    /// Previous position
    pub previous_position: Vec2,
    /// Starting position
    pub start_position: Vec2,
    /// Time when touch started
    pub start_time: Instant,
    /// Pressure (0.0 to 1.0)
    pub pressure: f32,
    /// Whether this is a pen/stylus
    pub is_pen: bool,
}

impl TouchPoint {
    pub fn new(id: u64, position: Vec2) -> Self {
        Self {
            id,
            position,
            previous_position: position,
            start_position: position,
            start_time: Instant::now(),
            pressure: 1.0,
            is_pen: false,
        }
    }
    
    pub fn delta(&self) -> Vec2 {
        self.position - self.previous_position
    }
    
    pub fn total_delta(&self) -> Vec2 {
        self.position - self.start_position
    }
    
    pub fn duration(&self) -> Duration {
        self.start_time.elapsed()
    }
}

// =============================================================================
// GESTURE RECOGNIZER
// =============================================================================

/// Configuration for gesture recognition
#[derive(Clone, Debug)]
pub struct GestureConfig {
    /// Maximum movement for a tap to register
    pub tap_max_distance: f32,
    /// Maximum duration for a tap  
    pub tap_max_duration: Duration,
    /// Maximum time between taps for double tap
    pub double_tap_max_interval: Duration,
    /// Minimum duration for long press
    pub long_press_min_duration: Duration,
    /// Minimum velocity for swipe (pixels per second)
    pub swipe_min_velocity: f32,
    /// Minimum distance for swipe
    pub swipe_min_distance: f32,
}

impl Default for GestureConfig {
    fn default() -> Self {
        Self {
            tap_max_distance: 10.0,
            tap_max_duration: Duration::from_millis(300),
            double_tap_max_interval: Duration::from_millis(300),
            long_press_min_duration: Duration::from_millis(500),
            swipe_min_velocity: 200.0,
            swipe_min_distance: 50.0,
        }
    }
}

/// Gesture recognition state machine
pub struct GestureRecognizer {
    config: GestureConfig,
    touches: Vec<TouchPoint>,
    last_tap_time: Option<Instant>,
    last_tap_position: Vec2,
    pending_long_press: bool,
    gesture_started: bool,
}

impl GestureRecognizer {
    pub fn new() -> Self {
        Self::with_config(GestureConfig::default())
    }
    
    pub fn with_config(config: GestureConfig) -> Self {
        Self {
            config,
            touches: Vec::new(),
            last_tap_time: None,
            last_tap_position: Vec2::ZERO,
            pending_long_press: false,
            gesture_started: false,
        }
    }
    
    /// Handle touch start
    pub fn touch_start(&mut self, id: u64, position: Vec2) -> Vec<GestureEvent> {
        let touch = TouchPoint::new(id, position);
        self.touches.push(touch);
        self.pending_long_press = true;
        Vec::new() // No immediate event
    }
    
    /// Handle touch move
    pub fn touch_move(&mut self, id: u64, position: Vec2) -> Vec<GestureEvent> {
        let mut events = Vec::new();
        
        // First, update the touch and collect needed info
        let touch_info: Option<(Vec2, Vec2, Vec2, Duration, f32)> = {
            if let Some(touch) = self.touches.iter_mut().find(|t| t.id == id) {
                touch.previous_position = touch.position;
                touch.position = position;
                
                // Check if moved enough to cancel tap/long-press
                if touch.total_delta().length() > self.config.tap_max_distance {
                    self.pending_long_press = false;
                }
                
                Some((
                    touch.position,
                    touch.start_position,
                    touch.delta(),
                    touch.duration(),
                    touch.total_delta().length(),
                ))
            } else {
                None
            }
        };
        
        // Process pan gesture
        if let Some((pos, start_pos, delta, duration, total_dist)) = touch_info {
            if self.touches.len() == 1 && total_dist > self.config.tap_max_distance {
                let state = if !self.gesture_started {
                    self.gesture_started = true;
                    GestureState::Began
                } else {
                    GestureState::Changed
                };
                
                let mut event = GestureEvent::new(GestureType::Pan, state);
                event.position = pos;
                event.start_position = start_pos;
                event.delta = delta;
                event.duration = duration;
                events.push(event);
            }
        }
        
        // Two-finger gestures
        if self.touches.len() == 2 {
            let t0_pos = self.touches[0].position;
            let t1_pos = self.touches[1].position;
            let t0_start = self.touches[0].start_position;
            let t1_start = self.touches[1].start_position;
            
            let current_distance = (t0_pos - t1_pos).length();
            let start_distance = (t0_start - t1_start).length();
            
            if start_distance > 0.0 {
                let scale = current_distance / start_distance;
                
                let mut event = GestureEvent::new(
                    GestureType::Pinch,
                    if !self.gesture_started {
                        self.gesture_started = true;
                        GestureState::Began
                    } else {
                        GestureState::Changed
                    }
                );
                event.position = (t0_pos + t1_pos) / 2.0;
                event.scale = scale;
                event.touch_count = 2;
                events.push(event);
            }
        }
        
        events
    }
    
    /// Handle touch end
    pub fn touch_end(&mut self, id: u64) -> Vec<GestureEvent> {
        let mut events = Vec::new();
        
        if let Some(idx) = self.touches.iter().position(|t| t.id == id) {
            let touch = self.touches.remove(idx);
            
            // Finish pan if active
            if self.gesture_started {
                let mut event = GestureEvent::new(GestureType::Pan, GestureState::Ended);
                event.position = touch.position;
                event.start_position = touch.start_position;
                
                // Calculate velocity
                let duration = touch.duration().as_secs_f32();
                if duration > 0.0 {
                    event.velocity = touch.total_delta() / duration;
                }
                
                // Check for swipe
                if event.velocity.length() > self.config.swipe_min_velocity 
                    && touch.total_delta().length() > self.config.swipe_min_distance 
                {
                    let direction = if touch.total_delta().x.abs() > touch.total_delta().y.abs() {
                        if touch.total_delta().x > 0.0 { SwipeDirection::Right } else { SwipeDirection::Left }
                    } else {
                        if touch.total_delta().y > 0.0 { SwipeDirection::Down } else { SwipeDirection::Up }
                    };
                    
                    let mut swipe_event = GestureEvent::new(GestureType::Swipe, GestureState::Ended);
                    swipe_event.position = touch.position;
                    swipe_event.start_position = touch.start_position;
                    swipe_event.velocity = event.velocity;
                    events.push(swipe_event);
                }
                
                events.push(event);
                self.gesture_started = false;
            }
            // Check for tap
            else if touch.total_delta().length() < self.config.tap_max_distance
                && touch.duration() < self.config.tap_max_duration
            {
                // Check for double tap
                if let Some(last_time) = self.last_tap_time {
                    if last_time.elapsed() < self.config.double_tap_max_interval
                        && (touch.position - self.last_tap_position).length() < self.config.tap_max_distance
                    {
                        let mut event = GestureEvent::new(GestureType::DoubleTap, GestureState::Ended);
                        event.position = touch.position;
                        events.push(event);
                        self.last_tap_time = None;
                    } else {
                        // Single tap
                        let mut event = GestureEvent::new(GestureType::Tap, GestureState::Ended);
                        event.position = touch.position;
                        events.push(event);
                        self.last_tap_time = Some(Instant::now());
                        self.last_tap_position = touch.position;
                    }
                } else {
                    // First tap
                    let mut event = GestureEvent::new(GestureType::Tap, GestureState::Ended);
                    event.position = touch.position;
                    events.push(event);
                    self.last_tap_time = Some(Instant::now());
                    self.last_tap_position = touch.position;
                }
            }
        }
        
        if self.touches.is_empty() {
            self.pending_long_press = false;
        }
        
        events
    }
    
    /// Check for long press (call periodically from update loop)
    pub fn check_long_press(&mut self) -> Option<GestureEvent> {
        if !self.pending_long_press || self.touches.is_empty() {
            return None;
        }
        
        let touch = &self.touches[0];
        if touch.duration() >= self.config.long_press_min_duration
            && touch.total_delta().length() < self.config.tap_max_distance
        {
            self.pending_long_press = false;
            let mut event = GestureEvent::new(GestureType::LongPress, GestureState::Ended);
            event.position = touch.position;
            event.duration = touch.duration();
            return Some(event);
        }
        
        None
    }
    
    /// Handle touch cancel
    pub fn touch_cancel(&mut self, id: u64) -> Vec<GestureEvent> {
        let mut events = Vec::new();
        
        if let Some(idx) = self.touches.iter().position(|t| t.id == id) {
            self.touches.remove(idx);
            
            if self.gesture_started {
                events.push(GestureEvent::new(GestureType::Pan, GestureState::Cancelled));
                self.gesture_started = false;
            }
        }
        
        self.pending_long_press = false;
        events
    }
    
    /// Get active touch count
    pub fn touch_count(&self) -> usize {
        self.touches.len()
    }
}

impl Default for GestureRecognizer {
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
    fn test_tap_detection() {
        let mut recognizer = GestureRecognizer::new();
        
        // Quick touch and release
        recognizer.touch_start(1, Vec2::new(100.0, 100.0));
        let events = recognizer.touch_end(1);
        
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].gesture_type, GestureType::Tap);
    }
    
    #[test]
    fn test_pan_detection() {
        let mut recognizer = GestureRecognizer::new();
        
        recognizer.touch_start(1, Vec2::new(100.0, 100.0));
        let events = recognizer.touch_move(1, Vec2::new(200.0, 100.0));
        
        assert!(!events.is_empty());
        assert_eq!(events[0].gesture_type, GestureType::Pan);
        assert_eq!(events[0].state, GestureState::Began);
    }
}
