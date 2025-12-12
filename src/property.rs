//! Property System for GlassUI
//!
//! Provides introspection capabilities for widgets, enabling:
//! - Object Inspector (like Delphi's property editor)
//! - Visual form designers
//! - Serialization/deserialization of widget properties
//!
//! # Example
//! ```rust
//! #[derive(Inspectable)]
//! struct Button {
//!     #[property(name = "Text", category = "Content")]
//!     text: String,
//!     #[property(name = "Width", category = "Layout", min = 0.0)]
//!     width: f32,
//! }
//! ```

use std::any::Any;
use std::collections::HashMap;

/// Supported property value types
#[derive(Clone, Debug)]
pub enum PropertyValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Color([f32; 4]),
    Vec2([f32; 2]),
    Enum { value: String, options: Vec<String> },
}

impl PropertyValue {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            PropertyValue::String(s) => Some(s),
            _ => None,
        }
    }
    
    pub fn as_float(&self) -> Option<f64> {
        match self {
            PropertyValue::Float(f) => Some(*f),
            PropertyValue::Int(i) => Some(*i as f64),
            _ => None,
        }
    }
    
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            PropertyValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

/// Property categories for grouping in Object Inspector
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PropertyCategory {
    /// General properties (name, tag, etc.)
    General,
    /// Layout properties (position, size, alignment)
    Layout,
    /// Visual properties (color, font, border)
    Appearance,
    /// Behavioral properties (enabled, visible)
    Behavior,
    /// Data binding properties
    Data,
    /// Event handlers
    Events,
    /// Custom category
    Custom(String),
}

impl Default for PropertyCategory {
    fn default() -> Self {
        PropertyCategory::General
    }
}

/// Constraints for property values
#[derive(Clone, Debug, Default)]
pub struct PropertyConstraints {
    /// Minimum value (for numeric types)
    pub min: Option<f64>,
    /// Maximum value (for numeric types)
    pub max: Option<f64>,
    /// Step value for numeric inputs
    pub step: Option<f64>,
    /// Is this property read-only?
    pub read_only: bool,
    /// Regex pattern for string validation
    pub pattern: Option<String>,
    /// Allowed enum values
    pub enum_values: Option<Vec<String>>,
}

/// Describes a single property of a widget
#[derive(Clone, Debug)]
pub struct PropertyDescriptor {
    /// Internal property name (used in code)
    pub id: String,
    /// Display name (shown in Object Inspector)
    pub display_name: String,
    /// Description/tooltip
    pub description: String,
    /// Property category for grouping
    pub category: PropertyCategory,
    /// Type of the property value
    pub value_type: PropertyType,
    /// Validation constraints
    pub constraints: PropertyConstraints,
    /// Default value
    pub default: Option<PropertyValue>,
}

/// Type information for property values
#[derive(Clone, Debug, PartialEq)]
pub enum PropertyType {
    String,
    Int,
    Float,
    Bool,
    Color,
    Vec2,
    Vec4,
    Enum(Vec<String>),
    /// Reference to another component
    ComponentRef,
    /// Event handler
    EventHandler,
}

impl PropertyDescriptor {
    /// Create a new property descriptor
    pub fn new(id: impl Into<String>, display_name: impl Into<String>) -> Self {
        let id = id.into();
        Self {
            display_name: display_name.into(),
            description: String::new(),
            category: PropertyCategory::General,
            value_type: PropertyType::String,
            constraints: PropertyConstraints::default(),
            default: None,
            id,
        }
    }
    
    pub fn with_type(mut self, value_type: PropertyType) -> Self {
        self.value_type = value_type;
        self
    }
    
    pub fn with_category(mut self, category: PropertyCategory) -> Self {
        self.category = category;
        self
    }
    
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
    
    pub fn with_range(mut self, min: f64, max: f64) -> Self {
        self.constraints.min = Some(min);
        self.constraints.max = Some(max);
        self
    }
    
    pub fn with_default(mut self, default: PropertyValue) -> Self {
        self.default = Some(default);
        self
    }
    
    pub fn read_only(mut self) -> Self {
        self.constraints.read_only = true;
        self
    }
}

/// Event descriptor for Object Inspector
#[derive(Clone, Debug)]
pub struct EventDescriptor {
    /// Event name (e.g., "on_click")
    pub name: String,
    /// Display name (e.g., "OnClick")
    pub display_name: String,
    /// Description
    pub description: String,
    /// Parameter types
    pub parameters: Vec<(String, PropertyType)>,
}

impl EventDescriptor {
    pub fn new(name: impl Into<String>, display_name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            display_name: display_name.into(),
            description: String::new(),
            parameters: Vec::new(),
        }
    }
    
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
}

/// Component metadata for the visual designer
#[derive(Clone, Debug)]
pub struct ComponentDescriptor {
    /// Component type name (e.g., "Button", "TextInput")
    pub type_name: String,
    /// Display name for palette
    pub display_name: String,
    /// Category in component palette
    pub palette_category: String,
    /// Icon name for palette
    pub icon: String,
    /// Description
    pub description: String,
    /// Property descriptors
    pub properties: Vec<PropertyDescriptor>,
    /// Event descriptors
    pub events: Vec<EventDescriptor>,
    /// Can contain children?
    pub is_container: bool,
    /// Default size
    pub default_size: [f32; 2],
}

impl ComponentDescriptor {
    pub fn new(type_name: impl Into<String>) -> Self {
        let name = type_name.into();
        Self {
            display_name: name.clone(),
            type_name: name,
            palette_category: "General".to_string(),
            icon: "component".to_string(),
            description: String::new(),
            properties: Vec::new(),
            events: Vec::new(),
            is_container: false,
            default_size: [100.0, 30.0],
        }
    }
    
    pub fn with_property(mut self, prop: PropertyDescriptor) -> Self {
        self.properties.push(prop);
        self
    }
    
    pub fn with_event(mut self, event: EventDescriptor) -> Self {
        self.events.push(event);
        self
    }
    
    pub fn container(mut self) -> Self {
        self.is_container = true;
        self
    }
    
    pub fn with_category(mut self, cat: impl Into<String>) -> Self {
        self.palette_category = cat.into();
        self
    }
    
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.default_size = [width, height];
        self
    }
}

/// Trait for widgets that can be inspected by the Object Inspector
pub trait Inspectable {
    /// Get the component descriptor
    fn descriptor() -> ComponentDescriptor where Self: Sized;
    
    /// Get a property value by name
    fn get_property(&self, name: &str) -> Option<PropertyValue>;
    
    /// Set a property value by name
    fn set_property(&mut self, name: &str, value: PropertyValue) -> Result<(), String>;
    
    /// Get all property values
    fn get_all_properties(&self) -> HashMap<String, PropertyValue> {
        HashMap::new()
    }
}

/// Component registry for the visual designer
pub struct ComponentRegistry {
    components: HashMap<String, ComponentDescriptor>,
    factories: HashMap<String, Box<dyn Fn() -> Box<dyn Any>>>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            factories: HashMap::new(),
        }
    }
    
    /// Register a component type
    pub fn register<T: Inspectable + Default + 'static>(&mut self) {
        let descriptor = T::descriptor();
        let type_name = descriptor.type_name.clone();
        self.components.insert(type_name.clone(), descriptor);
        self.factories.insert(type_name, Box::new(|| {
            Box::new(T::default()) as Box<dyn Any>
        }));
    }
    
    /// Get component descriptor by type name
    pub fn get_descriptor(&self, type_name: &str) -> Option<&ComponentDescriptor> {
        self.components.get(type_name)
    }
    
    /// Get all registered component descriptors
    pub fn all_descriptors(&self) -> impl Iterator<Item = &ComponentDescriptor> {
        self.components.values()
    }
    
    /// Create a new instance of a component
    pub fn create(&self, type_name: &str) -> Option<Box<dyn Any>> {
        self.factories.get(type_name).map(|f| f())
    }
    
    /// Get descriptors grouped by palette category
    pub fn by_category(&self) -> HashMap<String, Vec<&ComponentDescriptor>> {
        let mut result: HashMap<String, Vec<&ComponentDescriptor>> = HashMap::new();
        for desc in self.components.values() {
            result.entry(desc.palette_category.clone())
                .or_default()
                .push(desc);
        }
        result
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Serialize component registry to JSON for IDE consumption
pub fn export_component_catalog(registry: &ComponentRegistry) -> String {
    // Simple JSON serialization for IDE protocol
    let mut json = String::from("{\n  \"components\": [\n");
    
    let components: Vec<_> = registry.all_descriptors().collect();
    for (i, desc) in components.iter().enumerate() {
        json.push_str(&format!(
            "    {{\n      \"type\": \"{}\",\n      \"displayName\": \"{}\",\n      \"category\": \"{}\",\n      \"isContainer\": {}\n    }}",
            desc.type_name,
            desc.display_name,
            desc.palette_category,
            desc.is_container
        ));
        if i < components.len() - 1 {
            json.push_str(",\n");
        }
    }
    
    json.push_str("\n  ]\n}");
    json
}
