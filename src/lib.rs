// Macros must be declared first for visibility in other modules
#[macro_use]
pub mod macros;       // Declarative macros for widget building

pub mod renderer;
pub mod widget;       // Legacy - will be deprecated
pub mod widgets;      // New modular widget system
pub mod text;
pub mod state;
pub mod property;
pub mod layout;       // New constraint-based layout primitives
pub mod focus;        // Focus management and keyboard navigation
pub mod clipboard;    // Clipboard copy/paste support
pub mod accessibility; // Screen reader and assistive technology support
pub mod animation;    // Animation system with curves and springs
pub mod style;        // CSS-like styling system
pub mod commands;     // Undo/redo command pattern
pub mod gestures;     // Touch/pen gesture recognition
pub mod hero;         // Hero/shared element transitions
pub mod video;        // Video playback abstraction

// === GlassUI v2 Modules ===
pub mod widget_id;    // Widget identity and context system
pub mod reactive;     // Reactive data binding system
pub mod panel_style;  // Panel presets and shapes
pub mod dashboard;    // Dashboard framework
pub mod ai;           // AI backend integration
pub mod task;         // Task system with notifications
pub mod workspace;    // Workspace management and layout
pub mod sound;        // Audio feedback system
pub mod persistence;  // Save/load workspace state

use winit::window::Window;
// use winit::event::Event;
pub use glam::{Vec2, Vec4, Mat4};

// Re-export layout primitives for convenience
pub use layout::{Size, Offset, BoxConstraints, EdgeInsets, LayoutResult};

// Re-export focus primitives
pub use focus::{FocusId, FocusManager, FocusNode, Focusable};

// Re-export clipboard functions
pub use clipboard::{copy_to_clipboard, paste_from_clipboard};

// Re-export animation types
pub use animation::{
    AnimationController, AnimationStatus, Curve, Tween, SpringAnimation,
    AnimationSequence, AnimationGroup, DelayedAnimation,
};

// Re-export style types
pub use style::{WidgetStyle, ButtonVariant, StyleSheet, SizeVariant};

// Re-export command types
pub use commands::{Command, CommandHistory};

// Re-export gesture types
pub use gestures::{GestureRecognizer, GestureEvent, GestureType, GestureState};

// Re-export hero transition types
pub use hero::{HeroId, HeroController, HeroScope, HeroRect, HeroFlight, SharedElementTransition};

// Re-export video types
pub use video::{VideoDecoder, VideoFrame, VideoMetadata, VideoSource, VideoError, PlaybackState};

// Re-export widget identity types (v2)
pub use widget_id::{WidgetId, WorkspaceId, WidgetContext};

// Re-export reactive types (v2)
pub use reactive::{Reactive, Computed, Property, ColorSource, AnimationTrigger};

// Re-export panel style types (v2)
pub use panel_style::{PanelPreset, PanelShape, PanelStyle, PathCommand};

// Re-export dashboard types (v2)
pub use dashboard::{Dashboard, DashboardPanel, DashboardLayout, DashboardTemplate, SizeHint, PositionHint, Edge};

// Re-export AI types (v2)
pub use ai::{AiBackend, NpuBackend, OllamaClient, LocalAiAgent, AgentId, AgentState, ChatMessage, MessageRole};

// Re-export task types (v2)
pub use task::{Task, TaskId, TaskStatus, TaskPanel, TaskManager, NotificationSound};

// Re-export workspace types (v2)
pub use workspace::{Workspace, WorkspacePanel, WorkspaceLayout, WorkspaceManager, SnapTarget, SnapEdge, TileMode};

pub struct GlassContext {
    pub renderer: renderer::GlassRenderer,
    pub width: u32,
    pub height: u32,
}

impl GlassContext {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let renderer = renderer::GlassRenderer::new(window).await;
        
        Self {
            renderer,
            width: size.width,
            height: size.height,
        }
    }
    
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.renderer.resize(width, height);
    }
    
    pub fn update(&mut self, dt: f32) {
        self.renderer.update(dt);
    }
    
    pub fn render(&mut self, root_widget: &mut dyn widget::Widget) {
        self.renderer.render(root_widget);
    }
}
