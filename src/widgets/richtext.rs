//! GlassUI RichText System
//!
//! Provides styled text rendering with formatting:
//! - `TextSpan` - Styled text segment
//! - `RichText` - Compound styled text
//! - `RichTextEditor` - Editable rich text widget

use glam::{Vec2, Vec4};
use crate::widgets::Widget;
use crate::renderer::GlassRenderer;
use crate::layout::{Size, Offset};

// =============================================================================
// TEXT STYLE (Rich)
// =============================================================================

/// Font weight
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FontWeight {
    Thin,
    Light,
    #[default]
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
}

/// Text decoration
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextDecoration {
    #[default]
    None,
    Underline,
    Strikethrough,
    Overline,
}

/// Style for a text span
#[derive(Clone, Debug)]
pub struct SpanStyle {
    pub color: Vec4,
    pub font_size: f32,
    pub font_weight: FontWeight,
    pub italic: bool,
    pub decoration: TextDecoration,
    pub background: Option<Vec4>,
    pub letter_spacing: f32,
}

impl Default for SpanStyle {
    fn default() -> Self {
        Self {
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
            font_size: 16.0,
            font_weight: FontWeight::Regular,
            italic: false,
            decoration: TextDecoration::None,
            background: None,
            letter_spacing: 0.0,
        }
    }
}

impl SpanStyle {
    pub fn new() -> Self { Self::default() }
    
    pub fn color(mut self, color: Vec4) -> Self { self.color = color; self }
    pub fn size(mut self, size: f32) -> Self { self.font_size = size; self }
    pub fn bold(mut self) -> Self { self.font_weight = FontWeight::Bold; self }
    pub fn italic(mut self) -> Self { self.italic = true; self }
    pub fn underline(mut self) -> Self { self.decoration = TextDecoration::Underline; self }
    pub fn strikethrough(mut self) -> Self { self.decoration = TextDecoration::Strikethrough; self }
    pub fn background(mut self, color: Vec4) -> Self { self.background = Some(color); self }
}

// =============================================================================
// TEXT SPAN
// =============================================================================

/// A styled segment of text
#[derive(Clone, Debug)]
pub struct TextSpan {
    pub text: String,
    pub style: SpanStyle,
    pub children: Vec<TextSpan>,
}

impl TextSpan {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            style: SpanStyle::default(),
            children: Vec::new(),
        }
    }
    
    pub fn styled(text: &str, style: SpanStyle) -> Self {
        Self {
            text: text.to_string(),
            style,
            children: Vec::new(),
        }
    }
    
    pub fn with_style(mut self, style: SpanStyle) -> Self {
        self.style = style;
        self
    }
    
    pub fn with_child(mut self, child: TextSpan) -> Self {
        self.children.push(child);
        self
    }
    
    pub fn bold(text: &str) -> Self {
        Self::styled(text, SpanStyle::new().bold())
    }
    
    pub fn italic(text: &str) -> Self {
        Self::styled(text, SpanStyle::new().italic())
    }
    
    pub fn colored(text: &str, color: Vec4) -> Self {
        Self::styled(text, SpanStyle::new().color(color))
    }
    
    /// Get total character count including children
    pub fn char_count(&self) -> usize {
        self.text.len() + self.children.iter().map(|c| c.char_count()).sum::<usize>()
    }
    
    /// Get plain text content
    pub fn plain_text(&self) -> String {
        let mut result = self.text.clone();
        for child in &self.children {
            result.push_str(&child.plain_text());
        }
        result
    }
}

// =============================================================================
// RICH TEXT (Display Only)
// =============================================================================

/// Rich text widget for displaying formatted text
pub struct RichText {
    spans: Vec<TextSpan>,
    position: Vec2,
    size: Size,
    line_height: f32,
    max_width: f32,
}

impl RichText {
    pub fn new() -> Self {
        Self {
            spans: Vec::new(),
            position: Vec2::ZERO,
            size: Size::ZERO,
            line_height: 1.4,
            max_width: f32::INFINITY,
        }
    }
    
    pub fn with_span(mut self, span: TextSpan) -> Self {
        self.spans.push(span);
        self
    }
    
    pub fn with_text(mut self, text: &str) -> Self {
        self.spans.push(TextSpan::new(text));
        self
    }
    
    pub fn with_max_width(mut self, width: f32) -> Self {
        self.max_width = width;
        self
    }
    
    /// Parse simple markdown-like formatting
    /// **bold**, *italic*, `code`, ~~strike~~
    pub fn from_markdown(text: &str) -> Self {
        let mut rich = RichText::new();
        let mut current = String::new();
        let mut chars = text.chars().peekable();
        
        while let Some(c) = chars.next() {
            match c {
                '*' if chars.peek() == Some(&'*') => {
                    // Bold
                    if !current.is_empty() {
                        rich.spans.push(TextSpan::new(&current));
                        current.clear();
                    }
                    chars.next(); // consume second *
                    let mut bold_text = String::new();
                    while let Some(&next) = chars.peek() {
                        if next == '*' {
                            chars.next();
                            if chars.peek() == Some(&'*') {
                                chars.next();
                                break;
                            }
                            bold_text.push('*');
                        } else {
                            bold_text.push(chars.next().unwrap());
                        }
                    }
                    rich.spans.push(TextSpan::bold(&bold_text));
                }
                '*' => {
                    // Italic
                    if !current.is_empty() {
                        rich.spans.push(TextSpan::new(&current));
                        current.clear();
                    }
                    let mut italic_text = String::new();
                    while let Some(&next) = chars.peek() {
                        if next == '*' {
                            chars.next();
                            break;
                        }
                        italic_text.push(chars.next().unwrap());
                    }
                    rich.spans.push(TextSpan::italic(&italic_text));
                }
                '`' => {
                    // Code
                    if !current.is_empty() {
                        rich.spans.push(TextSpan::new(&current));
                        current.clear();
                    }
                    let mut code_text = String::new();
                    while let Some(&next) = chars.peek() {
                        if next == '`' {
                            chars.next();
                            break;
                        }
                        code_text.push(chars.next().unwrap());
                    }
                    rich.spans.push(TextSpan::styled(&code_text, 
                        SpanStyle::new().background(Vec4::new(0.2, 0.2, 0.25, 0.8))));
                }
                _ => current.push(c),
            }
        }
        
        if !current.is_empty() {
            rich.spans.push(TextSpan::new(&current));
        }
        
        rich
    }
}

impl Default for RichText { fn default() -> Self { Self::new() } }

impl Widget for RichText {
    fn layout(&mut self, _origin: Vec2, available: Vec2) -> Vec2 {
        // Calculate size based on content
        let mut total_width = 0.0f32;
        let base_height = 20.0; // Approximate
        
        for span in &self.spans {
            total_width += span.text.len() as f32 * span.style.font_size * 0.6;
        }
        
        let width = total_width.min(available.x).min(self.max_width);
        let lines = (total_width / width).ceil().max(1.0);
        let height = lines * base_height * self.line_height;
        
        self.size = Size::new(width, height);
        Vec2::new(width, height)
    }
    
    fn handle_event(&mut self, _event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool { false }
    fn update(&mut self, _dt: f32) {}
    
    fn render(&self, renderer: &mut GlassRenderer) {
        let mut x = self.position.x;
        let mut y = self.position.y;
        
        for span in &self.spans {
            self.render_span(renderer, span, &mut x, &mut y);
        }
    }
    
    fn set_position(&mut self, pos: Offset) { self.position = Vec2::new(pos.x, pos.y); }
    fn get_position(&self) -> Offset { Offset::new(self.position.x, self.position.y) }
    fn get_size(&self) -> Size { self.size }
    fn intrinsic_width(&self, _height: f32) -> Option<f32> { None }
    fn intrinsic_height(&self, _width: f32) -> Option<f32> { None }
}

impl RichText {
    fn render_span(&self, renderer: &mut GlassRenderer, span: &TextSpan, x: &mut f32, y: &mut f32) {
        let char_width = span.style.font_size * 0.6;
        let text_width = span.text.len() as f32 * char_width;
        
        // Background highlight
        if let Some(bg) = span.style.background {
            renderer.draw_rect(Vec2::new(*x, *y), Vec2::new(text_width, span.style.font_size + 4.0), bg);
        }
        
        // Text
        renderer.draw_text(&span.text, Vec2::new(*x, *y), span.style.font_size, span.style.color);
        
        // Underline
        if span.style.decoration == TextDecoration::Underline {
            renderer.draw_rect(
                Vec2::new(*x, *y + span.style.font_size + 2.0),
                Vec2::new(text_width, 1.0),
                span.style.color
            );
        }
        
        // Strikethrough
        if span.style.decoration == TextDecoration::Strikethrough {
            renderer.draw_rect(
                Vec2::new(*x, *y + span.style.font_size / 2.0),
                Vec2::new(text_width, 1.0),
                span.style.color
            );
        }
        
        *x += text_width;
        
        // Render children
        for child in &span.children {
            self.render_span(renderer, child, x, y);
        }
    }
}

// =============================================================================
// RICH TEXT EDITOR
// =============================================================================

/// Editable rich text widget with formatting support
pub struct RichTextEditor {
    content: Vec<TextSpan>,
    position: Vec2,
    size: Size,
    cursor_pos: usize,
    selection_start: Option<usize>,
    focused: bool,
    cursor_blink: f32,
    /// Current style to apply to new text
    current_style: SpanStyle,
}

impl RichTextEditor {
    pub fn new() -> Self {
        Self {
            content: vec![TextSpan::new("")],
            position: Vec2::ZERO,
            size: Size::new(400.0, 200.0),
            cursor_pos: 0,
            selection_start: None,
            focused: false,
            cursor_blink: 0.0,
            current_style: SpanStyle::default(),
        }
    }
    
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.size = Size::new(width, height);
        self
    }
    
    pub fn with_content(mut self, text: &str) -> Self {
        self.content = vec![TextSpan::new(text)];
        self
    }
    
    /// Get plain text content
    pub fn plain_text(&self) -> String {
        self.content.iter().map(|s| s.plain_text()).collect()
    }
    
    /// Set bold for current/future text
    pub fn toggle_bold(&mut self) {
        self.current_style.font_weight = match self.current_style.font_weight {
            FontWeight::Bold => FontWeight::Regular,
            _ => FontWeight::Bold,
        };
    }
    
    /// Set italic for current/future text
    pub fn toggle_italic(&mut self) {
        self.current_style.italic = !self.current_style.italic;
    }
    
    /// Set underline for current/future text
    pub fn toggle_underline(&mut self) {
        self.current_style.decoration = match self.current_style.decoration {
            TextDecoration::Underline => TextDecoration::None,
            _ => TextDecoration::Underline,
        };
    }
    
    fn insert_char(&mut self, c: char) {
        // For now, simple append to first span
        if let Some(span) = self.content.first_mut() {
            span.text.push(c);
            self.cursor_pos += 1;
        }
    }
    
    fn delete_char(&mut self) {
        if self.cursor_pos > 0 {
            if let Some(span) = self.content.first_mut() {
                if self.cursor_pos <= span.text.len() {
                    span.text.remove(self.cursor_pos - 1);
                    self.cursor_pos -= 1;
                }
            }
        }
    }
}

impl Default for RichTextEditor { fn default() -> Self { Self::new() } }

impl Widget for RichTextEditor {
    fn layout(&mut self, _origin: Vec2, _available: Vec2) -> Vec2 {
        Vec2::new(self.size.width, self.size.height)
    }
    
    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        use winit::event::{Event, WindowEvent, ElementState, MouseButton};
        
        match event {
            Event::WindowEvent { event: WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. }, .. } => {
                let in_bounds = mouse_pos.x >= self.position.x 
                    && mouse_pos.x <= self.position.x + self.size.width
                    && mouse_pos.y >= self.position.y 
                    && mouse_pos.y <= self.position.y + self.size.height;
                self.focused = in_bounds;
                in_bounds
            }
            Event::WindowEvent { event: WindowEvent::KeyboardInput { event: key_event, .. }, .. } if self.focused => {
                if key_event.state.is_pressed() {
                    use winit::keyboard::{Key, NamedKey};
                    
                    // Handle special keys
                    match &key_event.logical_key {
                        Key::Named(NamedKey::Backspace) => { self.delete_char(); return true; }
                        Key::Named(NamedKey::ArrowLeft) if self.cursor_pos > 0 => { self.cursor_pos -= 1; return true; }
                        Key::Named(NamedKey::ArrowRight) => { self.cursor_pos += 1; return true; }
                        Key::Named(NamedKey::Enter) => { self.insert_char('\n'); return true; }
                        _ => {}
                    }
                    
                    // Regular text input
                    if let Some(text) = &key_event.text {
                        for c in text.chars() {
                            if !c.is_control() {
                                self.insert_char(c);
                            }
                        }
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }
    
    fn update(&mut self, dt: f32) {
        if self.focused {
            self.cursor_blink += dt;
            if self.cursor_blink > 1.0 { self.cursor_blink = 0.0; }
        }
    }
    
    fn render(&self, renderer: &mut GlassRenderer) {
        let theme_bg = Vec4::new(0.08, 0.08, 0.1, 0.95);
        let theme_border = Vec4::new(0.3, 0.5, 0.9, 0.5);
        let theme_text = Vec4::new(1.0, 1.0, 1.0, 0.95);
        
        // Background
        renderer.draw_rounded_rect(self.position, Vec2::new(self.size.width, self.size.height), theme_bg, 6.0);
        
        // Focus border
        if self.focused {
            renderer.draw_rounded_rect(
                self.position - Vec2::splat(2.0),
                Vec2::new(self.size.width + 4.0, self.size.height + 4.0),
                theme_border,
                8.0
            );
        }
        
        // Content
        let text_pos = self.position + Vec2::new(12.0, 12.0);
        let mut x = text_pos.x;
        
        for span in &self.content {
            renderer.draw_text(&span.text, Vec2::new(x, text_pos.y), span.style.font_size, span.style.color);
            x += span.text.len() as f32 * span.style.font_size * 0.6;
        }
        
        // Cursor
        if self.focused && self.cursor_blink < 0.5 {
            let plain = self.plain_text();
            let cursor_x = text_pos.x + (self.cursor_pos.min(plain.len()) as f32) * 16.0 * 0.6;
            renderer.draw_rect(Vec2::new(cursor_x, text_pos.y), Vec2::new(2.0, 18.0), theme_text);
        }
        
        // Toolbar hint
        renderer.draw_text("Ctrl+B: Bold | Ctrl+I: Italic | Ctrl+U: Underline", 
            Vec2::new(self.position.x + 12.0, self.position.y + self.size.height - 24.0),
            12.0, Vec4::new(0.5, 0.5, 0.6, 0.7));
    }
    
    fn set_position(&mut self, pos: Offset) { self.position = Vec2::new(pos.x, pos.y); }
    fn get_position(&self) -> Offset { Offset::new(self.position.x, self.position.y) }
    fn get_size(&self) -> Size { self.size }
    fn intrinsic_width(&self, _height: f32) -> Option<f32> { Some(self.size.width) }
    fn intrinsic_height(&self, _width: f32) -> Option<f32> { Some(self.size.height) }
}
