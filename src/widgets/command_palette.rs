//! GlassUI Command Palette
//!
//! Quick action command palette (Ctrl+K / Cmd+K):
//! - Fuzzy search commands
//! - Keyboard navigation
//! - Action execution

use glam::{Vec2, Vec4};
use crate::renderer::GlassRenderer;
use crate::widget_id::WidgetId;
use crate::widgets::core::{Widget, get_theme};

// =============================================================================
// COMMAND
// =============================================================================

/// A command that can be executed
#[derive(Clone, Debug)]
pub struct Command {
    pub id: String,
    pub label: String,
    pub description: String,
    pub shortcut: Option<String>,
    pub icon: Option<String>,
    pub category: String,
}

impl Command {
    pub fn new(id: &str, label: &str) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            description: String::new(),
            shortcut: None,
            icon: None,
            category: "General".to_string(),
        }
    }
    
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }
    
    pub fn with_shortcut(mut self, shortcut: &str) -> Self {
        self.shortcut = Some(shortcut.to_string());
        self
    }
    
    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }
    
    pub fn with_category(mut self, category: &str) -> Self {
        self.category = category.to_string();
        self
    }
    
    /// Check if command matches search query
    pub fn matches(&self, query: &str) -> bool {
        if query.is_empty() { return true; }
        let query_lower = query.to_lowercase();
        self.label.to_lowercase().contains(&query_lower) ||
        self.description.to_lowercase().contains(&query_lower) ||
        self.category.to_lowercase().contains(&query_lower)
    }
    
    /// Get match score for sorting (higher is better)
    pub fn match_score(&self, query: &str) -> i32 {
        if query.is_empty() { return 0; }
        let query_lower = query.to_lowercase();
        let label_lower = self.label.to_lowercase();
        
        if label_lower == query_lower { return 100; }
        if label_lower.starts_with(&query_lower) { return 80; }
        if label_lower.contains(&query_lower) { return 60; }
        if self.description.to_lowercase().contains(&query_lower) { return 40; }
        0
    }
}

// =============================================================================
// COMMAND PALETTE
// =============================================================================

/// Quick command palette widget
pub struct CommandPalette {
    pub id: WidgetId,
    pub position: Vec2,
    pub size: Vec2,
    pub visible: bool,
    pub query: String,
    pub commands: Vec<Command>,
    pub filtered: Vec<usize>,  // Indices into commands
    pub selected_index: usize,
    cursor_pos: usize,
    cursor_visible: bool,
    cursor_timer: f32,
    pub on_execute: Option<Box<dyn FnMut(&Command)>>,
}

impl CommandPalette {
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::new(500.0, 400.0),
            visible: false,
            query: String::new(),
            commands: Vec::new(),
            filtered: Vec::new(),
            selected_index: 0,
            cursor_pos: 0,
            cursor_visible: true,
            cursor_timer: 0.0,
            on_execute: None,
        }
    }
    
    /// Add a command
    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
        self.update_filtered();
    }
    
    /// Add multiple commands
    pub fn add_commands(&mut self, commands: Vec<Command>) {
        self.commands.extend(commands);
        self.update_filtered();
    }
    
    /// Set callback for command execution
    pub fn on_execute(mut self, callback: impl FnMut(&Command) + 'static) -> Self {
        self.on_execute = Some(Box::new(callback));
        self
    }
    
    /// Show the palette
    pub fn show(&mut self) {
        self.visible = true;
        self.query.clear();
        self.cursor_pos = 0;
        self.selected_index = 0;
        self.update_filtered();
    }
    
    /// Hide the palette
    pub fn hide(&mut self) {
        self.visible = false;
    }
    
    /// Toggle visibility
    pub fn toggle(&mut self) {
        if self.visible { self.hide(); } else { self.show(); }
    }
    
    /// Execute the selected command
    pub fn execute_selected(&mut self) {
        if let Some(idx) = self.filtered.get(self.selected_index) {
            if let Some(callback) = &mut self.on_execute {
                let cmd = self.commands[*idx].clone();
                callback(&cmd);
            }
        }
        self.hide();
    }
    
    /// Update filtered commands based on query
    fn update_filtered(&mut self) {
        self.filtered = self.commands
            .iter()
            .enumerate()
            .filter(|(_, cmd)| cmd.matches(&self.query))
            .map(|(i, _)| i)
            .collect();
        
        // Sort by match score
        let query = self.query.clone();
        self.filtered.sort_by(|a, b| {
            let score_a = self.commands[*a].match_score(&query);
            let score_b = self.commands[*b].match_score(&query);
            score_b.cmp(&score_a)
        });
        
        // Reset selection if out of bounds
        if self.selected_index >= self.filtered.len() {
            self.selected_index = 0;
        }
    }
    
    /// Center on screen
    pub fn center_on_screen(mut self, screen_size: Vec2) -> Self {
        self.position = (screen_size - self.size) / 2.0;
        self
    }
    
    /// Add default dashboard commands
    pub fn with_dashboard_commands(mut self) -> Self {
        self.add_commands(vec![
            Command::new("new_panel", "New Panel").with_icon("➕").with_shortcut("Ctrl+N").with_category("Panels"),
            Command::new("close_panel", "Close Panel").with_icon("✕").with_shortcut("Ctrl+W").with_category("Panels"),
            Command::new("maximize_panel", "Maximize Panel").with_icon("⤢").with_category("Panels"),
            Command::new("tile_panels", "Tile All Panels").with_icon("▦").with_category("Layout"),
            Command::new("save_workspace", "Save Workspace").with_icon("💾").with_shortcut("Ctrl+S").with_category("Workspace"),
            Command::new("load_workspace", "Load Workspace").with_icon("📂").with_shortcut("Ctrl+O").with_category("Workspace"),
            Command::new("new_agent", "New AI Agent").with_icon("🤖").with_category("AI"),
            Command::new("clear_chat", "Clear Chat").with_icon("🗑").with_category("AI"),
            Command::new("toggle_theme", "Toggle Theme").with_icon("🎨").with_category("Settings"),
            Command::new("toggle_sound", "Toggle Sounds").with_icon("🔊").with_category("Settings"),
        ]);
        self
    }
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for CommandPalette {
    fn layout(&mut self, _origin: Vec2, _max_size: Vec2) -> Vec2 {
        // Palette uses its own positioning
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        if !self.visible { return false; }
        
        match event {
            winit::event::Event::WindowEvent { 
                event: winit::event::WindowEvent::KeyboardInput { event: key_event, .. }, 
                .. 
            } => {
                if key_event.state.is_pressed() {
                    // Escape to close
                    if let winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape) = key_event.logical_key {
                        self.hide();
                        return true;
                    }
                    
                    // Enter to execute
                    if let winit::keyboard::Key::Named(winit::keyboard::NamedKey::Enter) = key_event.logical_key {
                        self.execute_selected();
                        return true;
                    }
                    
                    // Arrow navigation
                    if let winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowUp) = key_event.logical_key {
                        if self.selected_index > 0 {
                            self.selected_index -= 1;
                        }
                        return true;
                    }
                    if let winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowDown) = key_event.logical_key {
                        if self.selected_index < self.filtered.len().saturating_sub(1) {
                            self.selected_index += 1;
                        }
                        return true;
                    }
                    
                    // Backspace
                    if let winit::keyboard::Key::Named(winit::keyboard::NamedKey::Backspace) = key_event.logical_key {
                        if self.cursor_pos > 0 {
                            self.cursor_pos -= 1;
                            self.query.remove(self.cursor_pos);
                            self.update_filtered();
                        }
                        return true;
                    }
                    
                    // Text input
                    if let Some(text) = &key_event.text {
                        if let Some(c) = text.chars().next() {
                            if !c.is_control() {
                                self.query.insert(self.cursor_pos, c);
                                self.cursor_pos += 1;
                                self.update_filtered();
                                return true;
                            }
                        }
                    }
                }
            },
            
            // Click to select
            winit::event::Event::WindowEvent { 
                event: winit::event::WindowEvent::MouseInput { 
                    state: winit::event::ElementState::Pressed,
                    button: winit::event::MouseButton::Left,
                    ..
                }, .. 
            } => {
                let list_y = self.position.y + 52.0;
                let item_height = 48.0;
                
                if mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.size.x {
                    for i in 0..self.filtered.len().min(7) {
                        let y = list_y + i as f32 * item_height;
                        if mouse_pos.y >= y && mouse_pos.y <= y + item_height {
                            self.selected_index = i;
                            self.execute_selected();
                            return true;
                        }
                    }
                }
                
                // Click outside to close
                let inside = mouse_pos.x >= self.position.x && 
                             mouse_pos.x <= self.position.x + self.size.x &&
                             mouse_pos.y >= self.position.y && 
                             mouse_pos.y <= self.position.y + self.size.y;
                if !inside {
                    self.hide();
                    return true;
                }
            },
            
            _ => {}
        }
        
        false
    }

    fn update(&mut self, dt: f32) {
        if self.visible {
            self.cursor_timer += dt;
            if self.cursor_timer > 0.5 {
                self.cursor_timer = 0.0;
                self.cursor_visible = !self.cursor_visible;
            }
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        if !self.visible { return; }
        
        let theme = get_theme();
        
        // Backdrop
        renderer.draw_rounded_rect(
            Vec2::ZERO,
            Vec2::new(2000.0, 2000.0),
            Vec4::new(0.0, 0.0, 0.0, 0.5),
            0.0
        );
        
        // Main container
        renderer.draw_rounded_rect(self.position, self.size, Vec4::new(0.08, 0.08, 0.1, 0.98), 12.0);
        
        // Border
        renderer.draw_rounded_rect(
            self.position - Vec2::splat(1.0),
            self.size + Vec2::splat(2.0),
            Vec4::new(theme.primary.x, theme.primary.y, theme.primary.z, 0.3),
            13.0
        );
        
        // Search input
        let input_pos = self.position + Vec2::new(16.0, 12.0);
        let input_size = Vec2::new(self.size.x - 32.0, 36.0);
        renderer.draw_rounded_rect(input_pos, input_size, Vec4::new(0.05, 0.05, 0.07, 1.0), 6.0);
        
        // Search icon
        renderer.draw_text("🔍", input_pos + Vec2::new(10.0, 8.0), 16.0, theme.text_secondary);
        
        // Query text with cursor
        let text_pos = input_pos + Vec2::new(36.0, 10.0);
        if self.query.is_empty() {
            renderer.draw_text("Type a command...", text_pos, 14.0, theme.text_secondary);
        } else {
            let mut display = self.query.clone();
            if self.cursor_visible {
                display.insert(self.cursor_pos, '|');
            }
            renderer.draw_text(&display, text_pos, 14.0, theme.text);
        }
        
        // Results
        let list_y = self.position.y + 56.0;
        let item_height = 48.0;
        
        for (i, &cmd_idx) in self.filtered.iter().take(7).enumerate() {
            let cmd = &self.commands[cmd_idx];
            let y = list_y + i as f32 * item_height;
            let item_pos = Vec2::new(self.position.x + 8.0, y);
            let item_size = Vec2::new(self.size.x - 16.0, item_height - 4.0);
            
            // Selected highlight
            if i == self.selected_index {
                renderer.draw_rounded_rect(
                    item_pos,
                    item_size,
                    Vec4::new(theme.primary.x, theme.primary.y, theme.primary.z, 0.2),
                    6.0
                );
            }
            
            // Icon
            if let Some(icon) = &cmd.icon {
                renderer.draw_text(icon, item_pos + Vec2::new(12.0, 12.0), 18.0, theme.text);
            }
            
            // Label
            renderer.draw_text(&cmd.label, item_pos + Vec2::new(44.0, 8.0), 14.0, theme.text);
            
            // Description
            if !cmd.description.is_empty() {
                renderer.draw_text(&cmd.description, item_pos + Vec2::new(44.0, 26.0), 11.0, theme.text_secondary);
            }
            
            // Shortcut
            if let Some(shortcut) = &cmd.shortcut {
                let shortcut_x = item_pos.x + item_size.x - shortcut.len() as f32 * 7.0 - 12.0;
                renderer.draw_text(shortcut, Vec2::new(shortcut_x, item_pos.y + 16.0), 11.0, theme.text_secondary);
            }
        }
        
        // No results message
        if self.filtered.is_empty() && !self.query.is_empty() {
            renderer.draw_text(
                "No commands found", 
                Vec2::new(self.position.x + 16.0, list_y + 16.0), 
                14.0, 
                theme.text_secondary
            );
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
    fn test_command_matching() {
        let cmd = Command::new("test", "Test Command")
            .with_description("A test command");
        
        assert!(cmd.matches("test"));
        assert!(cmd.matches("command"));
        assert!(!cmd.matches("xyz"));
    }
    
    #[test]
    fn test_palette_filtering() {
        let mut palette = CommandPalette::new();
        palette.add_command(Command::new("save", "Save"));
        palette.add_command(Command::new("load", "Load"));
        palette.add_command(Command::new("new", "New"));
        
        palette.query = "sa".to_string();
        palette.update_filtered();
        
        assert_eq!(palette.filtered.len(), 1);
    }
}
