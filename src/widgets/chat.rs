//! GlassUI Chat Widgets
//!
//! AI chat interface components:
//! - ChatMessage widget
//! - ChatView with message list
//! - PromptInput for user input

use glam::{Vec2, Vec4};
use crate::renderer::GlassRenderer;
use crate::widget_id::WidgetId;
use crate::widgets::core::{Widget, get_theme};
use crate::ai::{MessageRole, ChatMessage as AiChatMessage};
use crate::panel_style::PanelPreset;

// =============================================================================
// CHAT MESSAGE WIDGET
// =============================================================================

/// Single chat message display
pub struct ChatMessageWidget {
    pub id: WidgetId,
    pub position: Vec2,
    pub size: Vec2,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: Option<String>,
    pub is_streaming: bool,
    stream_cursor_visible: bool,
    stream_cursor_timer: f32,
}

impl ChatMessageWidget {
    pub fn new(message: &AiChatMessage) -> Self {
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            role: message.role.clone(),
            content: message.content.clone(),
            timestamp: None,
            is_streaming: false,
            stream_cursor_visible: true,
            stream_cursor_timer: 0.0,
        }
    }
    
    pub fn user(content: &str) -> Self {
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            role: MessageRole::User,
            content: content.to_string(),
            timestamp: None,
            is_streaming: false,
            stream_cursor_visible: true,
            stream_cursor_timer: 0.0,
        }
    }
    
    pub fn assistant(content: &str) -> Self {
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            role: MessageRole::Assistant,
            content: content.to_string(),
            timestamp: None,
            is_streaming: false,
            stream_cursor_visible: true,
            stream_cursor_timer: 0.0,
        }
    }
    
    pub fn streaming() -> Self {
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            role: MessageRole::Assistant,
            content: String::new(),
            timestamp: None,
            is_streaming: true,
            stream_cursor_visible: true,
            stream_cursor_timer: 0.0,
        }
    }
    
    pub fn append_content(&mut self, text: &str) {
        self.content.push_str(text);
    }
    
    pub fn finish_streaming(&mut self) {
        self.is_streaming = false;
    }
    
    fn get_background_color(&self) -> Vec4 {
        match self.role {
            MessageRole::User => Vec4::new(0.15, 0.2, 0.3, 0.6),
            MessageRole::Assistant => Vec4::new(0.1, 0.15, 0.2, 0.5),
            MessageRole::System => Vec4::new(0.2, 0.15, 0.1, 0.4),
        }
    }
    
    fn calculate_height(&self) -> f32 {
        // Approximate: 20px per line, assume 40 chars per line
        let lines = (self.content.len() / 40).max(1) as f32;
        let padding = 24.0;
        lines * 20.0 + padding
    }
}

impl Widget for ChatMessageWidget {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(max_size.x, self.calculate_height());
        self.size
    }

    fn handle_event(&mut self, _event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool {
        false
    }

    fn update(&mut self, dt: f32) {
        if self.is_streaming {
            self.stream_cursor_timer += dt;
            if self.stream_cursor_timer > 0.5 {
                self.stream_cursor_timer = 0.0;
                self.stream_cursor_visible = !self.stream_cursor_visible;
            }
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        let bg_color = self.get_background_color();
        
        // Background
        renderer.draw_rounded_rect(self.position, self.size, bg_color, 8.0);
        
        // Role indicator
        let role_text = match self.role {
            MessageRole::User => "You",
            MessageRole::Assistant => "AI",
            MessageRole::System => "System",
        };
        renderer.draw_text(role_text, self.position + Vec2::new(12.0, 6.0), 12.0, theme.text_secondary);
        
        // Content
        let mut display_content = self.content.clone();
        if self.is_streaming && self.stream_cursor_visible {
            display_content.push('▌');
        }
        renderer.draw_text(&display_content, self.position + Vec2::new(12.0, 22.0), 14.0, theme.text);
    }
}

// =============================================================================
// CHAT VIEW
// =============================================================================

/// Scrollable chat message list
pub struct ChatView {
    pub id: WidgetId,
    pub position: Vec2,
    pub size: Vec2,
    pub messages: Vec<ChatMessageWidget>,
    pub scroll_offset: f32,
    pub max_scroll: f32,
}

impl ChatView {
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::new(400.0, 500.0),
            messages: Vec::new(),
            scroll_offset: 0.0,
            max_scroll: 0.0,
        }
    }
    
    pub fn add_message(&mut self, message: ChatMessageWidget) {
        self.messages.push(message);
        self.scroll_to_bottom();
    }
    
    pub fn add_user_message(&mut self, content: &str) {
        self.add_message(ChatMessageWidget::user(content));
    }
    
    pub fn add_assistant_message(&mut self, content: &str) {
        self.add_message(ChatMessageWidget::assistant(content));
    }
    
    pub fn start_streaming(&mut self) {
        self.add_message(ChatMessageWidget::streaming());
    }
    
    pub fn append_to_stream(&mut self, text: &str) {
        if let Some(last) = self.messages.last_mut() {
            if last.is_streaming {
                last.append_content(text);
            }
        }
    }
    
    pub fn finish_streaming(&mut self) {
        if let Some(last) = self.messages.last_mut() {
            last.finish_streaming();
        }
    }
    
    fn scroll_to_bottom(&mut self) {
        self.scroll_offset = self.max_scroll;
    }
    
    fn calculate_content_height(&self) -> f32 {
        let mut height = 0.0;
        for msg in &self.messages {
            height += msg.calculate_height() + 8.0; // gap
        }
        height
    }
}

impl Default for ChatView {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for ChatView {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = max_size;
        
        // Layout messages
        let mut y_offset = 0.0;
        for msg in &mut self.messages {
            let msg_origin = Vec2::new(origin.x + 8.0, origin.y + y_offset - self.scroll_offset + 8.0);
            let msg_size = msg.layout(msg_origin, Vec2::new(max_size.x - 16.0, 0.0));
            y_offset += msg_size.y + 8.0;
        }
        
        // Update scroll limits
        let content_height = self.calculate_content_height();
        self.max_scroll = (content_height - self.size.y + 16.0).max(0.0);
        
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool {
        // Handle scroll wheel
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseWheel { delta, .. }, .. } = event {
            let scroll_amount = match delta {
                winit::event::MouseScrollDelta::LineDelta(_, y) => *y * 30.0,
                winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
            };
            self.scroll_offset = (self.scroll_offset - scroll_amount).clamp(0.0, self.max_scroll);
            return true;
        }
        false
    }

    fn update(&mut self, dt: f32) {
        for msg in &mut self.messages {
            msg.update(dt);
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Background
        renderer.draw_rounded_rect(
            self.position, 
            self.size, 
            Vec4::new(0.05, 0.05, 0.08, 0.8),
            12.0
        );
        
        // Messages (with clipping - approximate)
        for msg in &self.messages {
            // Only render if visible
            if msg.position.y + msg.size.y > self.position.y && 
               msg.position.y < self.position.y + self.size.y {
                msg.render(renderer);
            }
        }
    }
}

// =============================================================================
// PROMPT INPUT
// =============================================================================

/// Text input for chat prompt
pub struct PromptInput {
    pub id: WidgetId,
    pub position: Vec2,
    pub size: Vec2,
    pub text: String,
    pub placeholder: String,
    pub focused: bool,
    pub cursor_pos: usize,
    cursor_visible: bool,
    cursor_timer: f32,
    pub on_submit: Option<Box<dyn FnMut(String)>>,
}

impl PromptInput {
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::new(400.0, 44.0),
            text: String::new(),
            placeholder: "Type a message...".to_string(),
            focused: false,
            cursor_pos: 0,
            cursor_visible: true,
            cursor_timer: 0.0,
            on_submit: None,
        }
    }
    
    pub fn with_placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }
    
    pub fn on_submit(mut self, callback: impl FnMut(String) + 'static) -> Self {
        self.on_submit = Some(Box::new(callback));
        self
    }
    
    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor_pos = 0;
    }
    
    pub fn get_and_clear(&mut self) -> String {
        let text = self.text.clone();
        self.clear();
        text
    }
}

impl Default for PromptInput {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for PromptInput {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(max_size.x, 44.0);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        match event {
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { 
                state: winit::event::ElementState::Pressed, 
                button: winit::event::MouseButton::Left, 
                .. 
            }, .. } => {
                let inside = mouse_pos.x >= self.position.x && 
                             mouse_pos.x <= self.position.x + self.size.x &&
                             mouse_pos.y >= self.position.y && 
                             mouse_pos.y <= self.position.y + self.size.y;
                self.focused = inside;
                return inside;
            },
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::KeyboardInput { event: key_event, .. }, .. } => {
                if self.focused && key_event.state.is_pressed() {
                    // Handle backspace
                    if let winit::keyboard::Key::Named(winit::keyboard::NamedKey::Backspace) = key_event.logical_key {
                        if self.cursor_pos > 0 {
                            self.cursor_pos -= 1;
                            self.text.remove(self.cursor_pos);
                        }
                        return true;
                    }
                    
                    // Handle Enter (submit)
                    if let winit::keyboard::Key::Named(winit::keyboard::NamedKey::Enter) = key_event.logical_key {
                        if !self.text.is_empty() {
                            if let Some(callback) = &mut self.on_submit {
                                let text = self.text.clone();
                                callback(text);
                            }
                            self.clear();
                        }
                        return true;
                    }
                    
                    // Handle arrow keys
                    if let winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowLeft) = key_event.logical_key {
                        if self.cursor_pos > 0 {
                            self.cursor_pos -= 1;
                        }
                        return true;
                    }
                    if let winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowRight) = key_event.logical_key {
                        if self.cursor_pos < self.text.len() {
                            self.cursor_pos += 1;
                        }
                        return true;
                    }
                    
                    // Regular text input
                    if let Some(text) = &key_event.text {
                        if let Some(c) = text.chars().next() {
                            if !c.is_control() {
                                self.text.insert(self.cursor_pos, c);
                                self.cursor_pos += 1;
                                self.cursor_visible = true;
                                self.cursor_timer = 0.0;
                                return true;
                            }
                        }
                    }
                }
            },
            _ => {}
        }
        false
    }

    fn update(&mut self, dt: f32) {
        if self.focused {
            self.cursor_timer += dt;
            if self.cursor_timer > 0.5 {
                self.cursor_timer = 0.0;
                self.cursor_visible = !self.cursor_visible;
            }
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Background
        let bg_color = if self.focused {
            Vec4::new(0.12, 0.12, 0.15, 0.9)
        } else {
            Vec4::new(0.08, 0.08, 0.1, 0.8)
        };
        renderer.draw_rounded_rect(self.position, self.size, bg_color, 8.0);
        
        // Border
        if self.focused {
            renderer.draw_rounded_rect(
                self.position - Vec2::splat(1.0),
                self.size + Vec2::splat(2.0),
                Vec4::new(theme.primary.x, theme.primary.y, theme.primary.z, 0.5),
                9.0
            );
        }
        
        // Text or placeholder
        let text_pos = self.position + Vec2::new(16.0, 14.0);
        if self.text.is_empty() {
            renderer.draw_text(&self.placeholder, text_pos, 16.0, theme.text_secondary);
        } else {
            let mut display_text = self.text.clone();
            if self.focused && self.cursor_visible {
                display_text.insert(self.cursor_pos, '|');
            }
            renderer.draw_text(&display_text, text_pos, 16.0, theme.text);
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
    fn test_chat_view() {
        let mut chat = ChatView::new();
        chat.add_user_message("Hello!");
        chat.add_assistant_message("Hi there!");
        
        assert_eq!(chat.messages.len(), 2);
    }
    
    #[test]
    fn test_streaming() {
        let mut chat = ChatView::new();
        chat.start_streaming();
        chat.append_to_stream("Hello ");
        chat.append_to_stream("World!");
        chat.finish_streaming();
        
        assert_eq!(chat.messages[0].content, "Hello World!");
        assert!(!chat.messages[0].is_streaming);
    }
}
