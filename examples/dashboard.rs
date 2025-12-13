//! GlassUI Dashboard Example
//!
//! A complete dashboard demonstrating all widgets working together.
//! Run with: cargo run --example dashboard

use glam::{Vec2, Vec4};

#[cfg(feature = "example")]
use glassui::{
    GlassContext,
    widgets::{
        // Panel controls
        ControllablePanel, PanelControlMode,
        // Status widgets
        StatusBar, ToastContainer, AgentCard, Toast, ToastType,
        // Command palette
        CommandPalette, Command,
        // Timeline
        Timeline, TimelineEntry,
        // Gauges
        ProgressBar, CircularGauge, Sparkline, MetricDisplay, MetricTrend,
        // Chat
        ChatView, PromptInput,
        // Core
        Label, Panel, Theme,
    },
    ai::AgentState,
};

/// Dashboard state
pub struct DashboardState {
    // Metrics
    pub cpu_usage: f32,
    pub mem_usage: f32,
    pub task_count: usize,
    
    // Time
    pub elapsed: f32,
    
    // UI state
    pub command_palette_open: bool,
}

impl Default for DashboardState {
    fn default() -> Self {
        Self {
            cpu_usage: 0.45,
            mem_usage: 0.62,
            task_count: 3,
            elapsed: 0.0,
            command_palette_open: false,
        }
    }
}

impl DashboardState {
    /// Simulate some changing values
    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;
        
        // Simulate fluctuating values
        self.cpu_usage = (0.4 + 0.3 * (self.elapsed * 0.5).sin()).clamp(0.0, 1.0);
        self.mem_usage = (0.5 + 0.2 * (self.elapsed * 0.3).cos()).clamp(0.0, 1.0);
    }
}

/// Example of setting up a complete dashboard
pub fn create_example_dashboard() -> String {
    // This is a code example showing how to build a dashboard
    r#"
// Create main widgets
let mut status_bar = StatusBar::dashboard_default();
let mut toasts = ToastContainer::new().position_top_right(1920.0, 16.0);
let mut command_palette = CommandPalette::new()
    .with_dashboard_commands()
    .center_on_screen(Vec2::new(1920.0, 1080.0));
let mut timeline = Timeline::sample();

// Create panels
let cpu_panel = ControllablePanel::new(Box::new(
    MetricDisplay::new("CPU Usage", "45%")
        .with_trend(MetricTrend::Up, "+5%")
        .with_sparkline_data(vec![0.3, 0.4, 0.35, 0.45, 0.5])
)).at(50.0, 100.0).sized(200.0, 120.0);

let mem_panel = ControllablePanel::new(Box::new(
    MetricDisplay::new("Memory", "62%")
        .with_trend(MetricTrend::Stable, "0%")
)).at(270.0, 100.0).sized(200.0, 120.0);

let agent_panel = ControllablePanel::new(Box::new(
    AgentCard::new("Assistant", "phi3")
)).at(50.0, 240.0).sized(220.0, 120.0);

// Create AI chat panel
let mut chat = ChatView::new();
chat.add_user_message("Hello!");
chat.add_assistant_message("Hi! How can I help?");

let chat_panel = ControllablePanel::new(Box::new(chat))
    .at(490.0, 100.0).sized(400.0, 500.0);

// Update loop
loop {
    // Handle Ctrl+K for command palette
    if ctrl_k_pressed {
        command_palette.toggle();
    }
    
    // Update status bar
    status_bar.update_item("cpu", &format!("{}%", (cpu_usage * 100.0) as i32));
    status_bar.update_item("mem", &format!("{}%", (mem_usage * 100.0) as i32));
    
    // Show notifications
    if task_completed {
        toasts.success("Done", "Task completed successfully");
    }
    
    // Render all
    status_bar.render(&mut renderer);
    cpu_panel.render(&mut renderer);
    mem_panel.render(&mut renderer);
    agent_panel.render(&mut renderer);
    chat_panel.render(&mut renderer);
    timeline.render(&mut renderer);
    toasts.render(&mut renderer);
    
    // Command palette on top
    command_palette.render(&mut renderer);
}
"#.to_string()
}

/// Quick start guide
pub fn quick_start_guide() -> &'static str {
    r#"
# GlassUI v2 Quick Start

## 1. Create a window
```rust
let (window, event_loop) = create_window();
let mut ctx = GlassContext::new(&window).await;
```

## 2. Create widgets
```rust
let mut panel = ControllablePanel::new_empty()
    .at(100.0, 100.0)
    .sized(300.0, 200.0);

let mut toasts = ToastContainer::new();
let mut palette = CommandPalette::new().with_dashboard_commands();
```

## 3. Handle input
```rust
// Panel controls: R=resize, M=move, C=color
// Command palette: Ctrl+K
// Escape: deselect/close
```

## 4. Render
```rust
panel.layout(origin, max_size);
panel.update(dt);
panel.render(&mut ctx.renderer);
```
"#
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dashboard_state() {
        let mut state = DashboardState::default();
        state.update(1.0);
        
        assert!(state.elapsed > 0.0);
    }
}
