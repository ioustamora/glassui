//! GlassUI Widget Library
//!
//! All widgets are re-exported from this module for convenient access.

mod core;
mod layout;
mod controls;
mod premium;
mod input;
mod containers;

// Re-export core types
pub use core::{Theme, Widget, set_theme, get_theme, easing};

// Re-export layout widgets
pub use layout::{
    Column, Row, Stack, Spacer, Align, Alignment,
    Grid, Flex, FlexDirection, FlexJustify, FlexAlign,
};

// Re-export control widgets
pub use controls::{
    Button, Label, Slider, Checkbox, Panel,
};

// Re-export premium widgets
pub use premium::{
    ProgressBar, Toggle, RadioGroup, NumberInput,
};

// Re-export input widgets
pub use input::{
    TextInput, Dropdown,
};

// Re-export container widgets
pub use containers::{
    ScrollArea, TabBar,
};

// TODO: Remaining modules
// mod overlays;   // Modal, Tooltip, ContextMenu
// mod advanced;   // Draggable, Resizable, DragSource, DropTarget
