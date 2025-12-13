//! GlassUI Reactive Binding System
//!
//! Provides reactive data binding for automatic UI updates:
//! - `Reactive<T>` - Observable values with subscribers
//! - `Computed<T>` - Derived values with caching
//! - `Binding<T>` - Two-way widget-data binding
//! - Property bindings with transformations
//! - Animation triggers on data changes

use std::cell::RefCell;
use std::rc::Rc;

// =============================================================================
// REACTIVE VALUE
// =============================================================================

/// Observable value that notifies subscribers on change
/// 
/// # Example
/// ```rust
/// let count = Reactive::new(0);
/// count.subscribe(|v| println!("Count: {}", v));
/// count.set(5);  // Prints "Count: 5"
/// ```
pub struct Reactive<T> {
    inner: Rc<RefCell<ReactiveInner<T>>>,
}

struct ReactiveInner<T> {
    value: T,
    subscribers: Vec<Box<dyn Fn(&T)>>,
    version: u64,
}

impl<T: Clone + 'static> Reactive<T> {
    /// Create a new reactive value
    pub fn new(value: T) -> Self {
        Self {
            inner: Rc::new(RefCell::new(ReactiveInner {
                value,
                subscribers: Vec::new(),
                version: 0,
            })),
        }
    }
    
    /// Get the current value
    pub fn get(&self) -> T {
        self.inner.borrow().value.clone()
    }
    
    /// Set a new value and notify subscribers
    pub fn set(&self, value: T) {
        let mut inner = self.inner.borrow_mut();
        inner.value = value;
        inner.version += 1;
        
        // Notify subscribers
        let value_ref = &inner.value;
        for subscriber in &inner.subscribers {
            subscriber(value_ref);
        }
    }
    
    /// Update value with a function
    pub fn update(&self, f: impl FnOnce(&T) -> T) {
        let new_value = {
            let inner = self.inner.borrow();
            f(&inner.value)
        };
        self.set(new_value);
    }
    
    /// Subscribe to value changes
    pub fn subscribe(&self, f: impl Fn(&T) + 'static) {
        self.inner.borrow_mut().subscribers.push(Box::new(f));
    }
    
    /// Get current version (increments on each change)
    pub fn version(&self) -> u64 {
        self.inner.borrow().version
    }
    
    /// Map to a new reactive with transformation
    pub fn map<U: Clone + 'static>(&self, f: impl Fn(&T) -> U + 'static) -> Reactive<U> {
        let initial = f(&self.get());
        let mapped = Reactive::new(initial);
        
        let mapped_clone = mapped.clone();
        self.subscribe(move |v| {
            mapped_clone.set(f(v));
        });
        
        mapped
    }
}

impl<T: Clone + 'static> Clone for Reactive<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}

// =============================================================================
// COMPUTED VALUE
// =============================================================================

/// Derived value that recomputes when dependencies change
/// 
/// # Example
/// ```rust
/// let a = Reactive::new(5);
/// let b = Reactive::new(10);
/// let sum = Computed::new(|| a.get() + b.get());
/// ```
pub struct Computed<T> {
    compute: Box<dyn Fn() -> T>,
    cached: RefCell<Option<T>>,
}

impl<T: Clone + 'static> Computed<T> {
    /// Create a new computed value
    pub fn new(compute: impl Fn() -> T + 'static) -> Self {
        Self {
            compute: Box::new(compute),
            cached: RefCell::new(None),
        }
    }
    
    /// Get the computed value (recomputes each time for now)
    pub fn get(&self) -> T {
        (self.compute)()
    }
}

// =============================================================================
// PROPERTY ENUM
// =============================================================================

/// Widget properties that can be bound to reactive values
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Property {
    Color,
    BackgroundColor,
    BorderColor,
    Opacity,
    Scale,
    Rotation,
    PositionX,
    PositionY,
    Width,
    Height,
    GlowIntensity,
    PulseRate,
    Visibility,
    Enabled,
}

use glam::Vec4;

/// Dynamic color source for reactive panels
#[derive(Clone)]
pub enum ColorSource {
    /// Fixed color
    Static(Vec4),
    /// Bound to a reactive value
    Reactive(Reactive<Vec4>),
    /// Data-driven with transformation (low -> mid -> high)
    DataDriven {
        value: Reactive<f32>,
        low_color: Vec4,
        mid_color: Vec4,
        high_color: Vec4,
        low_threshold: f32,
        high_threshold: f32,
    },
    /// Follows time of day
    TimeOfDay {
        dawn: Vec4,
        day: Vec4,
        dusk: Vec4,
        night: Vec4,
    },
}

impl ColorSource {
    /// Get current color value
    pub fn get(&self) -> Vec4 {
        match self {
            ColorSource::Static(c) => *c,
            ColorSource::Reactive(r) => r.get(),
            ColorSource::DataDriven { value, low_color, mid_color, high_color, low_threshold, high_threshold } => {
                let v = value.get();
                let mid_threshold = (low_threshold + high_threshold) / 2.0;
                if v < *low_threshold {
                    *low_color
                } else if v < mid_threshold {
                    // Interpolate between low and mid
                    let t = (v - low_threshold) / (mid_threshold - low_threshold);
                    low_color.lerp(*mid_color, t)
                } else if v < *high_threshold {
                    // Interpolate between mid and high
                    let t = (v - mid_threshold) / (high_threshold - mid_threshold);
                    mid_color.lerp(*high_color, t)
                } else {
                    *high_color
                }
            },
            ColorSource::TimeOfDay { day, .. } => *day, // TODO: implement time logic
        }
    }
    
    /// Create a data-driven color source (green -> yellow -> red)
    pub fn traffic_light(value: Reactive<f32>) -> Self {
        ColorSource::DataDriven {
            value,
            low_color: Vec4::new(0.2, 0.8, 0.2, 1.0),  // Green
            mid_color: Vec4::new(0.9, 0.8, 0.1, 1.0),  // Yellow
            high_color: Vec4::new(0.9, 0.2, 0.2, 1.0), // Red
            low_threshold: 0.3,
            high_threshold: 0.7,
        }
    }
}

// =============================================================================
// ANIMATION TRIGGER
// =============================================================================

/// Types of animation triggers based on data changes
#[derive(Clone, Copy, Debug)]
pub enum AnimationTrigger {
    /// Trigger on any value change
    OnChange,
    /// Trigger when value increases
    OnIncrease,
    /// Trigger when value decreases
    OnDecrease,
    /// Trigger when value crosses threshold
    OnThreshold(f32),
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_reactive_basic() {
        let value = Reactive::new(42);
        assert_eq!(value.get(), 42);
        
        value.set(100);
        assert_eq!(value.get(), 100);
    }
    
    #[test]
    fn test_reactive_map() {
        let number = Reactive::new(5);
        let doubled = number.map(|n| n * 2);
        
        assert_eq!(doubled.get(), 10);
        
        number.set(10);
        assert_eq!(doubled.get(), 20);
    }
    
    #[test]
    fn test_computed() {
        let a = Reactive::new(5);
        let b = Reactive::new(10);
        
        let a_clone = a.clone();
        let b_clone = b.clone();
        let sum = Computed::new(move || a_clone.get() + b_clone.get());
        
        assert_eq!(sum.get(), 15);
        
        a.set(20);
        assert_eq!(sum.get(), 30);
    }
}
