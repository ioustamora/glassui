//! GlassUI Layout Primitives
//!
//! Provides Flutter-inspired constraint-based layout system with:
//! - `BoxConstraints` - Min/max width/height constraints
//! - `Size` - 2D dimensions
//! - `EdgeInsets` - Padding/margin values
//! - `Offset` - 2D position

use glam::Vec2;

// =============================================================================
// SIZE
// =============================================================================

/// 2D size with width and height
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub const ZERO: Size = Size { width: 0.0, height: 0.0 };
    pub const INFINITE: Size = Size { width: f32::INFINITY, height: f32::INFINITY };
    
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
    
    pub fn square(dimension: f32) -> Self {
        Self { width: dimension, height: dimension }
    }
    
    /// Check if size has finite dimensions
    pub fn is_finite(&self) -> bool {
        self.width.is_finite() && self.height.is_finite()
    }
    
    /// Check if size is empty (zero area)
    pub fn is_empty(&self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }
    
    /// Return size constrained to the given constraints
    pub fn constrain(&self, constraints: BoxConstraints) -> Size {
        Size {
            width: self.width.clamp(constraints.min_width, constraints.max_width),
            height: self.height.clamp(constraints.min_height, constraints.max_height),
        }
    }
    
    /// Aspect ratio (width / height)
    pub fn aspect_ratio(&self) -> f32 {
        if self.height == 0.0 { 0.0 } else { self.width / self.height }
    }
}

impl From<Vec2> for Size {
    fn from(v: Vec2) -> Self {
        Size::new(v.x, v.y)
    }
}

impl From<Size> for Vec2 {
    fn from(s: Size) -> Self {
        Vec2::new(s.width, s.height)
    }
}

// =============================================================================
// OFFSET
// =============================================================================

/// 2D offset/position
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Offset {
    pub x: f32,
    pub y: f32,
}

impl Offset {
    pub const ZERO: Offset = Offset { x: 0.0, y: 0.0 };
    
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<Vec2> for Offset {
    fn from(v: Vec2) -> Self {
        Offset::new(v.x, v.y)
    }
}

impl From<Offset> for Vec2 {
    fn from(o: Offset) -> Self {
        Vec2::new(o.x, o.y)
    }
}

impl std::ops::Add for Offset {
    type Output = Offset;
    fn add(self, rhs: Offset) -> Offset {
        Offset::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Offset {
    type Output = Offset;
    fn sub(self, rhs: Offset) -> Offset {
        Offset::new(self.x - rhs.x, self.y - rhs.y)
    }
}

// =============================================================================
// EDGE INSETS
// =============================================================================

/// Padding/margin values for all four edges
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EdgeInsets {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl EdgeInsets {
    pub const ZERO: EdgeInsets = EdgeInsets { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 };
    
    /// Same value on all sides
    pub fn all(value: f32) -> Self {
        Self { top: value, right: value, bottom: value, left: value }
    }
    
    /// Symmetric horizontal and vertical values
    pub fn symmetric(horizontal: f32, vertical: f32) -> Self {
        Self { top: vertical, right: horizontal, bottom: vertical, left: horizontal }
    }
    
    /// Only specific sides
    pub fn only(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self { top, right, bottom, left }
    }
    
    /// Horizontal insets (left + right)
    pub fn horizontal(&self) -> f32 {
        self.left + self.right
    }
    
    /// Vertical insets (top + bottom)
    pub fn vertical(&self) -> f32 {
        self.top + self.bottom
    }
    
    /// Total size consumed by insets
    pub fn size(&self) -> Size {
        Size::new(self.horizontal(), self.vertical())
    }
    
    /// Offset for the top-left corner
    pub fn top_left(&self) -> Offset {
        Offset::new(self.left, self.top)
    }
}

// =============================================================================
// BOX CONSTRAINTS
// =============================================================================

/// Immutable layout constraints for widgets
/// 
/// Widgets receive constraints from their parent and must return a size
/// that satisfies those constraints. This is inspired by Flutter's layout
/// protocol.
///
/// # Invariants
/// - `0 <= min_width <= max_width <= infinity`
/// - `0 <= min_height <= max_height <= infinity`
///
/// # Example
/// ```rust
/// let constraints = BoxConstraints::tight(Size::new(100.0, 50.0));
/// assert!(constraints.is_tight());
/// 
/// let loose = BoxConstraints::loose(Size::new(200.0, 200.0));
/// assert!(!loose.is_tight());
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoxConstraints {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
}

impl Default for BoxConstraints {
    fn default() -> Self {
        Self::expand()
    }
}

impl BoxConstraints {
    /// Create constraints with exact values
    pub fn new(min_width: f32, max_width: f32, min_height: f32, max_height: f32) -> Self {
        debug_assert!(min_width >= 0.0, "min_width must be >= 0");
        debug_assert!(max_width >= min_width, "max_width must be >= min_width");
        debug_assert!(min_height >= 0.0, "min_height must be >= 0");
        debug_assert!(max_height >= min_height, "max_height must be >= min_height");
        
        Self { min_width, max_width, min_height, max_height }
    }
    
    /// Tight constraints - widget must be exactly this size
    pub fn tight(size: Size) -> Self {
        Self {
            min_width: size.width,
            max_width: size.width,
            min_height: size.height,
            max_height: size.height,
        }
    }
    
    /// Tight width constraint with flexible height
    pub fn tight_width(width: f32) -> Self {
        Self {
            min_width: width,
            max_width: width,
            min_height: 0.0,
            max_height: f32::INFINITY,
        }
    }
    
    /// Tight height constraint with flexible width
    pub fn tight_height(height: f32) -> Self {
        Self {
            min_width: 0.0,
            max_width: f32::INFINITY,
            min_height: height,
            max_height: height,
        }
    }
    
    /// Loose constraints - widget can be any size up to max
    pub fn loose(size: Size) -> Self {
        Self {
            min_width: 0.0,
            max_width: size.width,
            min_height: 0.0,
            max_height: size.height,
        }
    }
    
    /// Expand to fill available space (unbounded)
    pub fn expand() -> Self {
        Self {
            min_width: 0.0,
            max_width: f32::INFINITY,
            min_height: 0.0,
            max_height: f32::INFINITY,
        }
    }
    
    /// Expand width, flexible height
    pub fn expand_width(height: f32) -> Self {
        Self {
            min_width: 0.0,
            max_width: f32::INFINITY,
            min_height: 0.0,
            max_height: height,
        }
    }
    
    /// Expand height, flexible width
    pub fn expand_height(width: f32) -> Self {
        Self {
            min_width: 0.0,
            max_width: width,
            min_height: 0.0,
            max_height: f32::INFINITY,
        }
    }
    
    // -------------------------------------------------------------------------
    // Query methods
    // -------------------------------------------------------------------------
    
    /// Widget has no choice in size (min == max for both dimensions)
    pub fn is_tight(&self) -> bool {
        self.min_width == self.max_width && self.min_height == self.max_height
    }
    
    /// Widget can be any size within bounds
    pub fn is_loose(&self) -> bool {
        self.min_width == 0.0 && self.min_height == 0.0
    }
    
    /// Constraints have finite bounds
    pub fn is_bounded(&self) -> bool {
        self.max_width.is_finite() && self.max_height.is_finite()
    }
    
    /// Constraints have infinite bounds
    pub fn is_unbounded(&self) -> bool {
        !self.max_width.is_finite() || !self.max_height.is_finite()
    }
    
    /// Check if a size satisfies these constraints
    pub fn is_satisfied_by(&self, size: Size) -> bool {
        size.width >= self.min_width && size.width <= self.max_width &&
        size.height >= self.min_height && size.height <= self.max_height
    }
    
    /// The biggest size that satisfies the constraints
    pub fn biggest(&self) -> Size {
        Size::new(
            if self.max_width.is_finite() { self.max_width } else { 0.0 },
            if self.max_height.is_finite() { self.max_height } else { 0.0 },
        )
    }
    
    /// The smallest size that satisfies the constraints
    pub fn smallest(&self) -> Size {
        Size::new(self.min_width, self.min_height)
    }
    
    // -------------------------------------------------------------------------
    // Transformation methods
    // -------------------------------------------------------------------------
    
    /// Constrain a size to fit within these constraints
    pub fn constrain(&self, size: Size) -> Size {
        Size::new(
            size.width.clamp(self.min_width, self.max_width),
            size.height.clamp(self.min_height, self.max_height),
        )
    }
    
    /// Constrain width only
    pub fn constrain_width(&self, width: f32) -> f32 {
        width.clamp(self.min_width, self.max_width)
    }
    
    /// Constrain height only
    pub fn constrain_height(&self, height: f32) -> f32 {
        height.clamp(self.min_height, self.max_height)
    }
    
    /// Create new constraints with reduced size (for padding/borders)
    pub fn deflate(&self, insets: EdgeInsets) -> Self {
        let horizontal = insets.horizontal();
        let vertical = insets.vertical();
        
        Self {
            min_width: (self.min_width - horizontal).max(0.0),
            max_width: (self.max_width - horizontal).max(0.0),
            min_height: (self.min_height - vertical).max(0.0),
            max_height: (self.max_height - vertical).max(0.0),
        }
    }
    
    /// Remove the minimum constraints
    pub fn loosen(&self) -> Self {
        Self {
            min_width: 0.0,
            max_width: self.max_width,
            min_height: 0.0,
            max_height: self.max_height,
        }
    }
    
    /// Apply the tightest of this and other constraints
    pub fn enforce(&self, other: BoxConstraints) -> Self {
        Self {
            min_width: self.min_width.max(other.min_width),
            max_width: self.max_width.min(other.max_width),
            min_height: self.min_height.max(other.min_height),
            max_height: self.max_height.min(other.max_height),
        }
    }
    
    /// Create constraints that respect aspect ratio
    pub fn constrain_dimensions(&self, width: f32, height: f32) -> Size {
        Size::new(
            width.clamp(self.min_width, self.max_width),
            height.clamp(self.min_height, self.max_height),
        )
    }
}

// =============================================================================
// LAYOUT RESULT
// =============================================================================

/// Result of widget layout containing size and child positions
#[derive(Clone, Debug)]
pub struct LayoutResult {
    /// The size this widget decided to be
    pub size: Size,
    /// Baseline for text alignment (optional)
    pub baseline: Option<f32>,
}

impl LayoutResult {
    pub fn new(size: Size) -> Self {
        Self { size, baseline: None }
    }
    
    pub fn with_baseline(size: Size, baseline: f32) -> Self {
        Self { size, baseline: Some(baseline) }
    }
}

impl From<Size> for LayoutResult {
    fn from(size: Size) -> Self {
        LayoutResult::new(size)
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tight_constraints() {
        let c = BoxConstraints::tight(Size::new(100.0, 50.0));
        assert!(c.is_tight());
        assert_eq!(c.biggest(), Size::new(100.0, 50.0));
        assert_eq!(c.smallest(), Size::new(100.0, 50.0));
    }
    
    #[test]
    fn test_loose_constraints() {
        let c = BoxConstraints::loose(Size::new(200.0, 100.0));
        assert!(c.is_loose());
        assert_eq!(c.smallest(), Size::ZERO);
        assert_eq!(c.biggest(), Size::new(200.0, 100.0));
    }
    
    #[test]
    fn test_constrain() {
        let c = BoxConstraints::new(50.0, 150.0, 30.0, 80.0);
        
        assert_eq!(c.constrain(Size::new(100.0, 50.0)), Size::new(100.0, 50.0));
        assert_eq!(c.constrain(Size::new(10.0, 10.0)), Size::new(50.0, 30.0));
        assert_eq!(c.constrain(Size::new(200.0, 200.0)), Size::new(150.0, 80.0));
    }
    
    #[test]
    fn test_deflate() {
        let c = BoxConstraints::tight(Size::new(100.0, 100.0));
        let deflated = c.deflate(EdgeInsets::all(10.0));
        
        assert_eq!(deflated.max_width, 80.0);
        assert_eq!(deflated.max_height, 80.0);
    }
    
    #[test]
    fn test_edge_insets() {
        let insets = EdgeInsets::symmetric(10.0, 20.0);
        assert_eq!(insets.horizontal(), 20.0);
        assert_eq!(insets.vertical(), 40.0);
        assert_eq!(insets.top_left(), Offset::new(10.0, 20.0));
    }
}
