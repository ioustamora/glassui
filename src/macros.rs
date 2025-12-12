//! GlassUI Declarative Macros
//!
//! Provides ergonomic macros for building widget hierarchies:
//! - `ui!` - Main declarative macro for widget trees
//! - `column!` / `row!` - Quick layout helpers
//! - `style!` - Style builder macro
//!
//! # Example
//! ```rust
//! use glassui::macros::*;
//!
//! let ui = column![
//!     Label::new("Hello, GlassUI!"),
//!     row![
//!         Button::new("OK").with_callback(|| {}),
//!         Button::new("Cancel"),
//!     ],
//! ];
//! ```

/// Create a Column layout with children
/// 
/// # Example
/// ```rust
/// let col = column![
///     Label::new("First"),
///     Label::new("Second"),
///     Button::new("Click"),
/// ];
/// ```
#[macro_export]
macro_rules! column {
    // Empty column
    () => {
        $crate::widgets::Column::new()
    };
    
    // Column with spacing
    (spacing: $spacing:expr; $($child:expr),* $(,)?) => {{
        let mut col = $crate::widgets::Column::new().with_spacing($spacing);
        $(
            col = col.add_child(Box::new($child));
        )*
        col
    }};
    
    // Column with children
    ($($child:expr),* $(,)?) => {{
        let mut col = $crate::widgets::Column::new();
        $(
            col = col.add_child(Box::new($child));
        )*
        col
    }};
}

/// Create a Row layout with children
/// 
/// # Example
/// ```rust
/// let row = row![
///     Button::new("Left"),
///     Spacer::new(),
///     Button::new("Right"),
/// ];
/// ```
#[macro_export]
macro_rules! row {
    // Empty row
    () => {
        $crate::widgets::Row::new()
    };
    
    // Row with spacing
    (spacing: $spacing:expr; $($child:expr),* $(,)?) => {{
        let mut r = $crate::widgets::Row::new().with_spacing($spacing);
        $(
            r = r.add_child(Box::new($child));
        )*
        r
    }};
    
    // Row with children
    ($($child:expr),* $(,)?) => {{
        let mut r = $crate::widgets::Row::new();
        $(
            r = r.add_child(Box::new($child));
        )*
        r
    }};
}

/// Create a Stack layout with children
/// 
/// # Example
/// ```rust
/// let stack = stack![
///     Image::placeholder(),
///     Label::new("Overlay Text"),
/// ];
/// ```
#[macro_export]
macro_rules! stack {
    ($($child:expr),* $(,)?) => {{
        let mut s = $crate::widgets::Stack::new();
        $(
            s = s.add_child(Box::new($child));
        )*
        s
    }};
}

/// Create a Grid layout
/// 
/// # Example
/// ```rust
/// let grid = grid!(2, 2;
///     Button::new("1"),
///     Button::new("2"),
///     Button::new("3"),
///     Button::new("4"),
/// );
/// ```
#[macro_export]
macro_rules! grid {
    ($cols:expr, $rows:expr; $($child:expr),* $(,)?) => {{
        let mut g = $crate::widgets::Grid::new($cols, $rows);
        $(
            g = g.add_child(Box::new($child));
        )*
        g
    }};
}

/// Create a centered widget
/// 
/// # Example
/// ```rust
/// let centered = center!(Button::new("Centered"));
/// ```
#[macro_export]
macro_rules! center {
    ($child:expr) => {
        $crate::widgets::Align::new(
            Box::new($child),
            $crate::widgets::Alignment::Center
        )
    };
}

/// Create padding around a widget
/// 
/// # Example
/// ```rust
/// let padded = padding!(16.0; Button::new("Padded"));
/// let padded = padding!(10.0, 20.0; Button::new("H/V Padded"));
/// ```
#[macro_export]
macro_rules! padding {
    // All sides
    ($all:expr; $child:expr) => {{
        // TODO: Implement Padding widget wrapper
        // For now, we'll return the child as-is
        // In a full implementation, this would wrap with EdgeInsets
        $child
    }};
    
    // Horizontal, Vertical
    ($h:expr, $v:expr; $child:expr) => {{
        $child
    }};
}

/// Quick label creation
/// 
/// # Example
/// ```rust
/// let label = label!("Hello");
/// let styled = label!("Big", 24.0);
/// ```
#[macro_export]
macro_rules! label {
    ($text:expr) => {
        $crate::widgets::Label::new($text)
    };
    
    ($text:expr, $size:expr) => {
        $crate::widgets::Label::new($text).with_size($size)
    };
}

/// Quick button creation with callback
/// 
/// # Example
/// ```rust
/// let btn = button!("Click Me" => || println!("Clicked!"));
/// let btn = button!("Plain");
/// ```
#[macro_export]
macro_rules! button {
    ($text:expr) => {
        $crate::widgets::Button::new($text)
    };
    
    ($text:expr => $callback:expr) => {
        $crate::widgets::Button::new($text).with_callback($callback)
    };
}

/// Create a panel with children
/// 
/// # Example
/// ```rust
/// let panel = panel![
///     Label::new("Inside Panel"),
///     Button::new("Action"),
/// ];
/// ```
#[macro_export]
macro_rules! panel {
    ($($child:expr),* $(,)?) => {{
        // Panel takes a single child, so wrap in Column if multiple
        let content = column![$($child),*];
        $crate::widgets::Panel::new(Box::new(content))
    }};
    
    (size: $w:expr, $h:expr; $($child:expr),* $(,)?) => {{
        let content = column![$($child),*];
        $crate::widgets::Panel::new(Box::new(content))
            .with_size($w, $h)
    }};
}

/// Create a scrollable area
/// 
/// # Example
/// ```rust
/// let scroll = scroll!(height: 300.0;
///     column![
///         Label::new("Item 1"),
///         Label::new("Item 2"),
///         // ... many items
///     ]
/// );
/// ```
#[macro_export]
macro_rules! scroll {
    (height: $h:expr; $child:expr) => {
        $crate::widgets::ScrollArea::new(Box::new($child), $h)
    };
}

/// Create a TabBar with tabs
/// 
/// # Example
/// ```rust
/// let tabs = tabs![
///     "Home" => column![Label::new("Home content")],
///     "Settings" => column![Label::new("Settings content")],
/// ];
/// ```
#[macro_export]
macro_rules! tabs {
    ($($name:expr => $content:expr),* $(,)?) => {{
        let mut tb = $crate::widgets::TabBar::new();
        $(
            tb = tb.add_tab($name, Box::new($content));
        )*
        tb
    }};
}

/// Conditional widget inclusion
/// 
/// # Example
/// ```rust
/// let ui = column![
///     Label::new("Always shown"),
///     when!(show_button; Button::new("Conditional")),
/// ];
/// ```
#[macro_export]
macro_rules! when {
    ($cond:expr; $widget:expr) => {
        if $cond {
            Some(Box::new($widget) as Box<dyn $crate::widgets::Widget>)
        } else {
            None
        }
    };
}

/// Create a tooltip wrapper
/// 
/// # Example  
/// ```rust
/// let btn = tooltip!("Click to submit"; Button::new("Submit"));
/// ```
#[macro_export]
macro_rules! tooltip {
    ($text:expr; $child:expr) => {
        $crate::widgets::Tooltip::new(Box::new($child), $text)
    };
}

/// Quick spacing
#[macro_export]
macro_rules! spacer {
    () => {
        $crate::widgets::Spacer::new()
    };
    
    ($size:expr) => {
        $crate::widgets::Spacer::fixed($size)
    };
}

// =============================================================================
// FLUENT BUILDER EXTENSIONS
// =============================================================================

/// Trait extension for fluent widget building
pub trait WidgetExt: Sized {
    /// Wrap this widget in a Tooltip
    fn with_tooltip(self, text: &str) -> crate::widgets::Tooltip;
    
    /// Make this widget draggable
    fn draggable(self) -> crate::widgets::Draggable;
    
    /// Make this widget resizable
    fn resizable(self) -> crate::widgets::Resizable;
    
    /// Center this widget
    fn centered(self) -> crate::widgets::Align;
}

// Note: Implementation would require `where Self: Widget + 'static`
// which needs the Widget trait to be object-safe in specific ways.
// This serves as a design document for the API.

// =============================================================================
// STYLE MACRO
// =============================================================================

/// Quick style creation
/// 
/// # Example
/// ```rust
/// let style = style! {
///     background: colors::from_hex(0x1a1a2e),
///     corner_radius: 8.0,
///     padding: EdgeInsets::all(16.0),
/// };
/// ```
#[macro_export]
macro_rules! style {
    ($($field:ident : $value:expr),* $(,)?) => {{
        let mut s = $crate::style::WidgetStyle::new();
        $(
            s.$field = Some($value);
        )*
        s
    }};
}

/// Animation tween helper
/// 
/// # Example
/// ```rust
/// let tween = tween!(0.0 => 100.0);
/// let color_tween = tween!(RED => BLUE);
/// ```
#[macro_export]
macro_rules! tween {
    ($from:expr => $to:expr) => {
        $crate::animation::Tween::new($from, $to)
    };
}

/// Spring animation helper
/// 
/// # Example
/// ```rust
/// let spring = spring!(0.0);
/// let bouncy = spring!(0.0, bouncy);
/// ```
#[macro_export]
macro_rules! spring {
    ($initial:expr) => {
        $crate::animation::SpringAnimation::new($initial)
    };
    
    ($initial:expr, bouncy) => {
        $crate::animation::SpringAnimation::bouncy($initial)
    };
    
    ($initial:expr, gentle) => {
        $crate::animation::SpringAnimation::gentle($initial)
    };
    
    ($initial:expr, stiff) => {
        $crate::animation::SpringAnimation::stiff($initial)
    };
}

// =============================================================================
// RE-EXPORTS FOR CONVENIENCE
// =============================================================================

/// Prelude module - import everything commonly needed
pub mod prelude {
    pub use crate::widgets::*;
    pub use crate::layout::*;
    pub use crate::animation::*;
    pub use crate::style::*;
    pub use crate::focus::*;
    
    // Re-export macros (they're already at crate root due to #[macro_export])
}
