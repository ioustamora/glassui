//! GlassUI Timeline Widget
//!
//! Activity timeline for showing task history and events:
//! - Vertical timeline with entries
//! - Time-based grouping
//! - Visual connections

use glam::{Vec2, Vec4};
use crate::renderer::GlassRenderer;
use crate::widget_id::WidgetId;
use crate::widgets::core::{Widget, get_theme};

// =============================================================================
// TIMELINE ENTRY
// =============================================================================

/// Type of timeline entry for styling
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TimelineEntryType {
    Task,
    Message,
    Event,
    Alert,
    Milestone,
}

impl TimelineEntryType {
    pub fn color(&self) -> Vec4 {
        match self {
            TimelineEntryType::Task => Vec4::new(0.3, 0.6, 0.9, 1.0),
            TimelineEntryType::Message => Vec4::new(0.5, 0.5, 0.7, 1.0),
            TimelineEntryType::Event => Vec4::new(0.4, 0.8, 0.5, 1.0),
            TimelineEntryType::Alert => Vec4::new(0.9, 0.5, 0.3, 1.0),
            TimelineEntryType::Milestone => Vec4::new(0.9, 0.8, 0.2, 1.0),
        }
    }
    
    pub fn icon(&self) -> &'static str {
        match self {
            TimelineEntryType::Task => "✓",
            TimelineEntryType::Message => "💬",
            TimelineEntryType::Event => "⚡",
            TimelineEntryType::Alert => "⚠",
            TimelineEntryType::Milestone => "★",
        }
    }
}

/// Single timeline entry
#[derive(Clone, Debug)]
pub struct TimelineEntry {
    pub id: u64,
    pub entry_type: TimelineEntryType,
    pub title: String,
    pub description: String,
    pub time: String,  // e.g., "2m ago", "10:45"
    pub completed: bool,
}

impl TimelineEntry {
    pub fn new(entry_type: TimelineEntryType, title: &str, time: &str) -> Self {
        static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        Self {
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            entry_type,
            title: title.to_string(),
            description: String::new(),
            time: time.to_string(),
            completed: false,
        }
    }
    
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }
    
    pub fn completed(mut self) -> Self {
        self.completed = true;
        self
    }
    
    pub fn task(title: &str, time: &str) -> Self {
        Self::new(TimelineEntryType::Task, title, time)
    }
    
    pub fn message(title: &str, time: &str) -> Self {
        Self::new(TimelineEntryType::Message, title, time)
    }
    
    pub fn event(title: &str, time: &str) -> Self {
        Self::new(TimelineEntryType::Event, title, time)
    }
    
    pub fn alert(title: &str, time: &str) -> Self {
        Self::new(TimelineEntryType::Alert, title, time)
    }
    
    pub fn milestone(title: &str, time: &str) -> Self {
        Self::new(TimelineEntryType::Milestone, title, time)
    }
}

// =============================================================================
// TIMELINE
// =============================================================================

/// Vertical timeline widget
pub struct Timeline {
    pub id: WidgetId,
    pub position: Vec2,
    pub size: Vec2,
    pub entries: Vec<TimelineEntry>,
    pub scroll_offset: f32,
    pub max_scroll: f32,
    entry_height: f32,
    line_x: f32,  // X position of the vertical line
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::new(300.0, 400.0),
            entries: Vec::new(),
            scroll_offset: 0.0,
            max_scroll: 0.0,
            entry_height: 64.0,
            line_x: 24.0,
        }
    }
    
    /// Add an entry to the timeline
    pub fn add_entry(&mut self, entry: TimelineEntry) {
        self.entries.push(entry);
        self.update_scroll_limits();
    }
    
    /// Add entry at the beginning (newest first)
    pub fn prepend_entry(&mut self, entry: TimelineEntry) {
        self.entries.insert(0, entry);
        self.update_scroll_limits();
    }
    
    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
        self.scroll_offset = 0.0;
        self.max_scroll = 0.0;
    }
    
    fn update_scroll_limits(&mut self) {
        let content_height = self.entries.len() as f32 * self.entry_height;
        self.max_scroll = (content_height - self.size.y + 16.0).max(0.0);
    }
    
    /// Create a sample timeline for demo
    pub fn sample() -> Self {
        let mut timeline = Self::new();
        timeline.add_entry(TimelineEntry::task("Initialize dashboard", "Just now").completed());
        timeline.add_entry(TimelineEntry::message("AI Agent connected", "1m ago"));
        timeline.add_entry(TimelineEntry::event("System started", "2m ago"));
        timeline.add_entry(TimelineEntry::milestone("Version 2.0", "5m ago"));
        timeline.add_entry(TimelineEntry::alert("High CPU usage", "10m ago")
            .with_description("CPU usage exceeded 90% threshold"));
        timeline
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Timeline {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = max_size;
        self.update_scroll_limits();
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool {
        // Handle scroll wheel
        if let winit::event::Event::WindowEvent { 
            event: winit::event::WindowEvent::MouseWheel { delta, .. }, 
            .. 
        } = event {
            let scroll_amount = match delta {
                winit::event::MouseScrollDelta::LineDelta(_, y) => *y * 30.0,
                winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
            };
            self.scroll_offset = (self.scroll_offset - scroll_amount).clamp(0.0, self.max_scroll);
            return true;
        }
        false
    }

    fn update(&mut self, _dt: f32) {}

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Background
        renderer.draw_rounded_rect(
            self.position,
            self.size,
            Vec4::new(0.06, 0.06, 0.08, 0.8),
            8.0
        );
        
        // Title
        renderer.draw_text("Activity", self.position + Vec2::new(12.0, 8.0), 14.0, theme.text);
        
        // Vertical line
        let line_x = self.position.x + self.line_x;
        let line_start = self.position.y + 36.0;
        let line_height = self.size.y - 44.0;
        renderer.draw_rounded_rect(
            Vec2::new(line_x - 1.0, line_start),
            Vec2::new(2.0, line_height),
            Vec4::new(0.3, 0.3, 0.4, 0.5),
            1.0
        );
        
        // Entries
        let content_start = self.position.y + 40.0;
        for (i, entry) in self.entries.iter().enumerate() {
            let y = content_start + i as f32 * self.entry_height - self.scroll_offset;
            
            // Clip check
            if y + self.entry_height < self.position.y + 36.0 || y > self.position.y + self.size.y {
                continue;
            }
            
            // Dot on the line
            let dot_color = entry.entry_type.color();
            let dot_pos = Vec2::new(line_x - 5.0, y + 8.0);
            renderer.draw_rounded_rect(dot_pos, Vec2::splat(10.0), dot_color, 5.0);
            
            // Icon
            let icon_pos = Vec2::new(self.position.x + 40.0, y + 4.0);
            renderer.draw_text(entry.entry_type.icon(), icon_pos, 14.0, dot_color);
            
            // Title
            let title_color = if entry.completed {
                theme.text_secondary
            } else {
                theme.text
            };
            renderer.draw_text(&entry.title, Vec2::new(self.position.x + 60.0, y + 4.0), 13.0, title_color);
            
            // Time
            let time_x = self.position.x + self.size.x - entry.time.len() as f32 * 6.0 - 12.0;
            renderer.draw_text(&entry.time, Vec2::new(time_x, y + 4.0), 11.0, theme.text_secondary);
            
            // Description
            if !entry.description.is_empty() {
                renderer.draw_text(
                    &entry.description, 
                    Vec2::new(self.position.x + 60.0, y + 24.0), 
                    11.0, 
                    theme.text_secondary
                );
            }
            
            // Completed strikethrough effect (just dimmer)
            if entry.completed {
                let line_pos = Vec2::new(self.position.x + 60.0, y + 12.0);
                let line_width = entry.title.len() as f32 * 7.0;
                renderer.draw_rounded_rect(line_pos, Vec2::new(line_width, 1.0), theme.text_secondary, 0.0);
            }
        }
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_timeline() {
        let mut timeline = Timeline::new();
        timeline.add_entry(TimelineEntry::task("Test", "now"));
        timeline.prepend_entry(TimelineEntry::event("Start", "ago"));
        
        assert_eq!(timeline.entries.len(), 2);
        assert_eq!(timeline.entries[0].title, "Start");
    }
    
    #[test]
    fn test_entry_types() {
        let entry = TimelineEntry::milestone("Release", "today").completed();
        
        assert!(entry.completed);
        assert_eq!(entry.entry_type, TimelineEntryType::Milestone);
    }
}
