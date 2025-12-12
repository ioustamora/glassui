//! GlassUI Widget Library
//!
//! All widgets are re-exported from this module for convenient access.

mod core;
mod layout;
mod controls;
mod premium;
mod input;
mod containers;
mod overlays;
mod advanced;
mod data;

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

// Re-export overlay widgets
pub use overlays::{
    Tooltip, MenuItem, ContextMenu, ContextMenuTrigger, Modal,
};

// Re-export advanced widgets
pub use advanced::{
    Draggable, Resizable,
};

// Re-export data widgets
pub use data::{
    Table, TableColumn, TableRow,
    ListView, ListItem,
    TreeView, TreeNode,
};
