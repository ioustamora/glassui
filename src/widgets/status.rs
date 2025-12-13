//! GlassUI Status Bar and Notifications
//!
//! Dashboard status components:
//! - StatusBar for system status
//! - Toast notifications
//! - AgentCard for AI agent display

use glam::{Vec2, Vec4};
use std::time::Instant;
use crate::renderer::GlassRenderer;
use crate::widget_id::WidgetId;
use crate::widgets::core::{Widget, get_theme};
use crate::panel_style::PanelPreset;
use crate::ai::AgentState;

// =============================================================================
// STATUS ITEM
// =============================================================================

/// Individual status item for the status bar
#[derive(Clone, Debug)]
pub struct StatusItem {
    pub id: String,
    pub label: String,
    pub value: String,
    pub icon: Option<String>,
    pub color: Option<Vec4>,
}

impl StatusItem {
    pub fn new(id: &str, label: &str, value: &str) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            value: value.to_string(),
            icon: None,
            color: None,
        }
    }
    
    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }
    
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
    
    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
    }
}

// =============================================================================
// STATUS BAR
// =============================================================================

/// Horizontal status bar with multiple status items
pub struct StatusBar {
    pub id: WidgetId,
    pub position: Vec2,
    pub size: Vec2,
    pub items: Vec<StatusItem>,
    pub background_color: Vec4,
    item_spacing: f32,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::new(800.0, 28.0),
            items: Vec::new(),
            background_color: Vec4::new(0.05, 0.05, 0.08, 0.9),
            item_spacing: 24.0,
        }
    }
    
    /// Add a status item
    pub fn add_item(&mut self, item: StatusItem) {
        self.items.push(item);
    }
    
    /// Update a status item by ID
    pub fn update_item(&mut self, id: &str, value: &str) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.set_value(value);
        }
    }
    
    /// Create a common status bar for dashboards
    pub fn dashboard_default() -> Self {
        let mut bar = Self::new();
        bar.add_item(StatusItem::new("time", "Time", "00:00").with_icon("🕐"));
        bar.add_item(StatusItem::new("cpu", "CPU", "0%").with_icon("💻"));
        bar.add_item(StatusItem::new("mem", "Mem", "0%").with_icon("🧠"));
        bar.add_item(StatusItem::new("tasks", "Tasks", "0").with_icon("📋"));
        bar.add_item(StatusItem::new("agents", "Agents", "0").with_icon("🤖"));
        bar
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for StatusBar {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(max_size.x, 28.0);
        self.size
    }

    fn handle_event(&mut self, _event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool {
        false
    }

    fn update(&mut self, _dt: f32) {}

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Background
        renderer.draw_rounded_rect(self.position, self.size, self.background_color, 4.0);
        
        // Items
        let mut x = self.position.x + 12.0;
        for item in &self.items {
            // Icon
            if let Some(icon) = &item.icon {
                renderer.draw_text(icon, Vec2::new(x, self.position.y + 5.0), 14.0, theme.text_secondary);
                x += 20.0;
            }
            
            // Label
            renderer.draw_text(&item.label, Vec2::new(x, self.position.y + 6.0), 12.0, theme.text_secondary);
            x += item.label.len() as f32 * 7.0 + 4.0;
            
            // Value
            let value_color = item.color.unwrap_or(theme.text);
            renderer.draw_text(&item.value, Vec2::new(x, self.position.y + 6.0), 12.0, value_color);
            x += item.value.len() as f32 * 7.0 + self.item_spacing;
        }
    }
}

// =============================================================================
// TOAST NOTIFICATION
// =============================================================================

/// Toast notification type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToastType {
    Info,
    Success,
    Warning,
    Error,
}

impl ToastType {
    pub fn color(&self) -> Vec4 {
        match self {
            ToastType::Info => Vec4::new(0.3, 0.5, 0.9, 0.9),
            ToastType::Success => Vec4::new(0.3, 0.8, 0.4, 0.9),
            ToastType::Warning => Vec4::new(0.9, 0.7, 0.2, 0.9),
            ToastType::Error => Vec4::new(0.9, 0.3, 0.3, 0.9),
        }
    }
    
    pub fn icon(&self) -> &'static str {
        match self {
            ToastType::Info => "ℹ️",
            ToastType::Success => "✓",
            ToastType::Warning => "⚠",
            ToastType::Error => "✕",
        }
    }
}

/// Single toast notification
#[derive(Clone, Debug)]
pub struct Toast {
    pub id: u64,
    pub toast_type: ToastType,
    pub title: String,
    pub message: String,
    pub duration: f32,
    pub time_remaining: f32,
    pub dismissed: bool,
    // Animation
    pub slide_t: f32,
    pub fade_t: f32,
}

impl Toast {
    pub fn new(toast_type: ToastType, title: &str, message: &str) -> Self {
        static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        Self {
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            toast_type,
            title: title.to_string(),
            message: message.to_string(),
            duration: 4.0,
            time_remaining: 4.0,
            dismissed: false,
            slide_t: 0.0,
            fade_t: 1.0,
        }
    }
    
    pub fn info(title: &str, message: &str) -> Self {
        Self::new(ToastType::Info, title, message)
    }
    
    pub fn success(title: &str, message: &str) -> Self {
        Self::new(ToastType::Success, title, message)
    }
    
    pub fn warning(title: &str, message: &str) -> Self {
        Self::new(ToastType::Warning, title, message)
    }
    
    pub fn error(title: &str, message: &str) -> Self {
        Self::new(ToastType::Error, title, message)
    }
    
    pub fn with_duration(mut self, seconds: f32) -> Self {
        self.duration = seconds;
        self.time_remaining = seconds;
        self
    }
    
    pub fn dismiss(&mut self) {
        self.dismissed = true;
    }
    
    pub fn is_expired(&self) -> bool {
        self.dismissed || self.time_remaining <= 0.0
    }
}

// =============================================================================
// TOAST CONTAINER
// =============================================================================

/// Container that manages and displays toast notifications
pub struct ToastContainer {
    pub id: WidgetId,
    pub position: Vec2,
    pub toasts: Vec<Toast>,
    pub max_visible: usize,
    pub toast_width: f32,
    pub toast_height: f32,
    pub spacing: f32,
}

impl ToastContainer {
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            toasts: Vec::new(),
            max_visible: 5,
            toast_width: 320.0,
            toast_height: 72.0,
            spacing: 8.0,
        }
    }
    
    /// Add a toast notification
    pub fn push(&mut self, toast: Toast) {
        self.toasts.push(toast);
    }
    
    /// Add an info toast
    pub fn info(&mut self, title: &str, message: &str) {
        self.push(Toast::info(title, message));
    }
    
    /// Add a success toast
    pub fn success(&mut self, title: &str, message: &str) {
        self.push(Toast::success(title, message));
    }
    
    /// Add a warning toast
    pub fn warning(&mut self, title: &str, message: &str) {
        self.push(Toast::warning(title, message));
    }
    
    /// Add an error toast
    pub fn error(&mut self, title: &str, message: &str) {
        self.push(Toast::error(title, message));
    }
    
    /// Clear all toasts
    pub fn clear(&mut self) {
        self.toasts.clear();
    }
    
    /// Position at top-right of screen
    pub fn position_top_right(mut self, screen_width: f32, margin: f32) -> Self {
        self.position = Vec2::new(screen_width - self.toast_width - margin, margin);
        self
    }
}

impl Default for ToastContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for ToastContainer {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        if self.position == Vec2::ZERO {
            self.position = origin;
        }
        Vec2::new(self.toast_width, self.toasts.len() as f32 * (self.toast_height + self.spacing))
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        // Click to dismiss
        if let winit::event::Event::WindowEvent { 
            event: winit::event::WindowEvent::MouseInput { 
                state: winit::event::ElementState::Pressed,
                button: winit::event::MouseButton::Left,
                ..
            }, .. 
        } = event {
            for (i, toast) in self.toasts.iter_mut().enumerate() {
                if i >= self.max_visible { break; }
                
                let y = self.position.y + i as f32 * (self.toast_height + self.spacing);
                let toast_pos = Vec2::new(self.position.x, y);
                let toast_size = Vec2::new(self.toast_width, self.toast_height);
                
                if mouse_pos.x >= toast_pos.x && mouse_pos.x <= toast_pos.x + toast_size.x &&
                   mouse_pos.y >= toast_pos.y && mouse_pos.y <= toast_pos.y + toast_size.y {
                    toast.dismiss();
                    return true;
                }
            }
        }
        false
    }

    fn update(&mut self, dt: f32) {
        // Update toast timers and animations
        for toast in &mut self.toasts {
            // Slide in animation
            toast.slide_t = (toast.slide_t + dt * 8.0).min(1.0);
            
            // Count down
            toast.time_remaining -= dt;
            
            // Fade out when almost expired
            if toast.time_remaining < 0.5 {
                toast.fade_t = (toast.time_remaining / 0.5).max(0.0);
            }
        }
        
        // Remove expired toasts
        self.toasts.retain(|t| !t.is_expired());
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        for (i, toast) in self.toasts.iter().enumerate() {
            if i >= self.max_visible { break; }
            
            let y = self.position.y + i as f32 * (self.toast_height + self.spacing);
            
            // Slide animation offset
            let slide_offset = (1.0 - toast.slide_t) * self.toast_width;
            let toast_pos = Vec2::new(self.position.x + slide_offset, y);
            let toast_size = Vec2::new(self.toast_width, self.toast_height);
            
            // Background with fade
            let mut bg_color = toast.toast_type.color();
            bg_color.w *= toast.fade_t;
            renderer.draw_rounded_rect(toast_pos, toast_size, bg_color, 8.0);
            
            // Icon
            let icon_pos = toast_pos + Vec2::new(12.0, 12.0);
            let mut icon_color = theme.text;
            icon_color.w *= toast.fade_t;
            renderer.draw_text(toast.toast_type.icon(), icon_pos, 20.0, icon_color);
            
            // Title
            let title_pos = toast_pos + Vec2::new(40.0, 10.0);
            let mut title_color = theme.text;
            title_color.w *= toast.fade_t;
            renderer.draw_text(&toast.title, title_pos, 14.0, title_color);
            
            // Message
            let msg_pos = toast_pos + Vec2::new(40.0, 30.0);
            let mut msg_color = theme.text_secondary;
            msg_color.w *= toast.fade_t;
            renderer.draw_text(&toast.message, msg_pos, 12.0, msg_color);
            
            // Progress bar
            let progress = toast.time_remaining / toast.duration;
            let bar_width = (toast_size.x - 24.0) * progress;
            let bar_pos = toast_pos + Vec2::new(12.0, toast_size.y - 6.0);
            let mut bar_color = Vec4::new(1.0, 1.0, 1.0, 0.3 * toast.fade_t);
            renderer.draw_rounded_rect(bar_pos, Vec2::new(bar_width, 3.0), bar_color, 2.0);
        }
    }
}

// =============================================================================
// AGENT CARD
// =============================================================================

/// Visual card for an AI agent
pub struct AgentCard {
    pub id: WidgetId,
    pub position: Vec2,
    pub size: Vec2,
    pub name: String,
    pub model: String,
    pub state: AgentState,
    pub message_count: usize,
    // Animation
    thinking_dots: u8,
    thinking_timer: f32,
}

impl AgentCard {
    pub fn new(name: &str, model: &str) -> Self {
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::new(200.0, 100.0),
            name: name.to_string(),
            model: model.to_string(),
            state: AgentState::Idle,
            message_count: 0,
            thinking_dots: 0,
            thinking_timer: 0.0,
        }
    }
    
    pub fn set_state(&mut self, state: AgentState) {
        self.state = state;
    }
    
    pub fn set_message_count(&mut self, count: usize) {
        self.message_count = count;
    }
    
    fn state_color(&self) -> Vec4 {
        match self.state {
            AgentState::Idle => Vec4::new(0.5, 0.5, 0.5, 1.0),
            AgentState::Thinking => Vec4::new(0.9, 0.7, 0.2, 1.0),
            AgentState::Responding => Vec4::new(0.3, 0.9, 0.4, 1.0),
            AgentState::Error => Vec4::new(0.9, 0.3, 0.3, 1.0),
        }
    }
    
    fn state_text(&self) -> &'static str {
        match self.state {
            AgentState::Idle => "Idle",
            AgentState::Thinking => "Thinking",
            AgentState::Responding => "Responding",
            AgentState::Error => "Error",
        }
    }
}

impl Widget for AgentCard {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size
    }

    fn handle_event(&mut self, _event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool {
        false
    }

    fn update(&mut self, dt: f32) {
        // Animated thinking dots
        if self.state == AgentState::Thinking {
            self.thinking_timer += dt;
            if self.thinking_timer > 0.3 {
                self.thinking_timer = 0.0;
                self.thinking_dots = (self.thinking_dots + 1) % 4;
            }
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Background
        renderer.draw_rounded_rect(
            self.position, 
            self.size, 
            Vec4::new(0.1, 0.12, 0.15, 0.8),
            12.0
        );
        
        // Status indicator (left edge)
        let indicator_pos = self.position + Vec2::new(0.0, 20.0);
        let indicator_size = Vec2::new(4.0, self.size.y - 40.0);
        renderer.draw_rounded_rect(indicator_pos, indicator_size, self.state_color(), 2.0);
        
        // Avatar circle
        let avatar_pos = self.position + Vec2::new(16.0, 16.0);
        renderer.draw_rounded_rect(avatar_pos, Vec2::splat(40.0), theme.primary, 20.0);
        renderer.draw_text("🤖", avatar_pos + Vec2::new(10.0, 8.0), 20.0, theme.text);
        
        // Name
        renderer.draw_text(&self.name, self.position + Vec2::new(70.0, 16.0), 16.0, theme.text);
        
        // Model
        renderer.draw_text(&self.model, self.position + Vec2::new(70.0, 36.0), 12.0, theme.text_secondary);
        
        // State with animated dots
        let mut state_text = self.state_text().to_string();
        if self.state == AgentState::Thinking {
            for _ in 0..self.thinking_dots {
                state_text.push('.');
            }
        }
        renderer.draw_text(&state_text, self.position + Vec2::new(70.0, 56.0), 12.0, self.state_color());
        
        // Message count
        let count_text = format!("{} msgs", self.message_count);
        renderer.draw_text(&count_text, self.position + Vec2::new(self.size.x - 60.0, 76.0), 11.0, theme.text_secondary);
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_status_bar() {
        let mut bar = StatusBar::dashboard_default();
        bar.update_item("cpu", "45%");
        
        let item = bar.items.iter().find(|i| i.id == "cpu").unwrap();
        assert_eq!(item.value, "45%");
    }
    
    #[test]
    fn test_toast() {
        let mut container = ToastContainer::new();
        container.success("Done", "Task completed");
        container.error("Failed", "Something went wrong");
        
        assert_eq!(container.toasts.len(), 2);
    }
    
    #[test]
    fn test_agent_card() {
        let mut card = AgentCard::new("Assistant", "phi3");
        card.set_state(AgentState::Thinking);
        card.set_message_count(5);
        
        assert_eq!(card.state, AgentState::Thinking);
        assert_eq!(card.message_count, 5);
    }
}
