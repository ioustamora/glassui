pub mod renderer;
pub mod widget;       // Legacy - will be deprecated
pub mod widgets;      // New modular widget system
pub mod text;
pub mod state;
pub mod property;
pub mod layout;       // New constraint-based layout primitives
pub mod focus;        // Focus management and keyboard navigation

use winit::window::Window;
// use winit::event::Event;
pub use glam::{Vec2, Vec4, Mat4};

// Re-export layout primitives for convenience
pub use layout::{Size, Offset, BoxConstraints, EdgeInsets, LayoutResult};

// Re-export focus primitives
pub use focus::{FocusId, FocusManager, FocusNode, Focusable};

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
