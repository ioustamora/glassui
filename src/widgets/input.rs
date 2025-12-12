//! GlassUI Input Widgets
//!
//! Text and data input controls: TextInput, Dropdown

use glam::{Vec2, Vec4};
use winit::event::{ElementState, MouseButton};
use crate::renderer::GlassRenderer;
use super::core::{Widget, get_theme};

// =============================================================================
// TEXT INPUT
// =============================================================================

/// Single-line text input field
pub struct TextInput {
    pub position: Vec2,
    pub size: Vec2,
    pub text: String,
    pub placeholder: String,
    pub focused: bool,
    pub cursor_visible: bool,
    pub cursor_timer: f32,
    pub corner_radius: f32,
}

impl TextInput {
    pub fn new(placeholder: &str) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            text: String::new(),
            placeholder: placeholder.to_string(),
            focused: false,
            cursor_visible: true,
            cursor_timer: 0.0,
            corner_radius: 6.0,
        }
    }
    
    pub fn with_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }
    
    pub fn get_text(&self) -> &str {
        &self.text
    }
    
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}

impl Widget for TextInput {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(200.0, 36.0);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let inside = mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.size.x &&
                     mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.size.y;

        match event {
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. }, .. } => {
                if inside {
                    self.focused = true;
                    return true;
                } else {
                    self.focused = false;
                }
            },
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::KeyboardInput { event: key_event, .. }, .. } => {
                if self.focused && key_event.state.is_pressed() {
                    if let winit::keyboard::Key::Named(winit::keyboard::NamedKey::Backspace) = key_event.logical_key {
                        self.text.pop();
                        return true;
                    }
                    
                    if let Some(text) = &key_event.text {
                        if let Some(c) = text.chars().next() {
                            if !c.is_control() {
                                self.text.push_str(text);
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
                self.cursor_visible = !self.cursor_visible;
                self.cursor_timer = 0.0;
            }
        } else {
            self.cursor_visible = false;
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Background
        let bg_col = if self.focused { 
            Vec4::new(0.1, 0.1, 0.12, 0.95) 
        } else { 
            Vec4::new(0.08, 0.08, 0.1, 0.9) 
        };
        renderer.draw_rounded_rect(self.position, self.size, bg_col, self.corner_radius);
        
        // Border
        if self.focused {
            renderer.draw_rounded_rect(
                self.position - Vec2::splat(1.0), 
                self.size + Vec2::splat(2.0), 
                Vec4::new(theme.primary.x, theme.primary.y, theme.primary.z, 0.6),
                self.corner_radius + 1.0
            );
        }

        // Text or placeholder
        let display_text = if self.text.is_empty() && !self.focused {
            &self.placeholder
        } else {
            &self.text
        };
        let text_color = if self.text.is_empty() && !self.focused {
            theme.text_secondary
        } else {
            theme.text
        };
        
        renderer.draw_text(display_text, self.position + Vec2::new(10.0, 8.0), 18.0, text_color);
        
        // Cursor
        if self.focused && self.cursor_visible {
            let text_width = self.text.len() as f32 * 9.0;
            let cursor_pos = self.position + Vec2::new(10.0 + text_width, 6.0);
            renderer.draw_rounded_rect(cursor_pos, Vec2::new(2.0, 22.0), theme.primary, 1.0);
        }
    }
}

// =============================================================================
// DROPDOWN
// =============================================================================

/// Dropdown select box
pub struct Dropdown {
    pub position: Vec2,
    pub size: Vec2,
    pub options: Vec<String>,
    pub selected_index: usize,
    pub open: bool,
    pub hovered_index: Option<usize>,
    pub corner_radius: f32,
}

impl Dropdown {
    pub fn new(options: Vec<String>) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            options,
            selected_index: 0,
            open: false,
            hovered_index: None,
            corner_radius: 6.0,
        }
    }
    
    pub fn with_selected(mut self, index: usize) -> Self {
        if index < self.options.len() {
            self.selected_index = index;
        }
        self
    }
    
    pub fn selected(&self) -> Option<&str> {
        self.options.get(self.selected_index).map(|s| s.as_str())
    }
}

impl Widget for Dropdown {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(180.0, 36.0);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let in_header = mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.size.x &&
                        mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.size.y;
        
        let item_height = 32.0;
        let dropdown_height = self.options.len() as f32 * item_height;
        let dropdown_y = self.position.y + self.size.y;
        
        let in_dropdown = self.open && 
            mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.size.x &&
            mouse_pos.y >= dropdown_y && mouse_pos.y <= dropdown_y + dropdown_height;
        
        // Update hovered
        if in_dropdown {
            let relative_y = mouse_pos.y - dropdown_y;
            let index = (relative_y / item_height) as usize;
            if index < self.options.len() {
                self.hovered_index = Some(index);
            }
        } else {
            self.hovered_index = None;
        }
        
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. }, .. } = event {
            if in_header {
                self.open = !self.open;
                return true;
            } else if in_dropdown {
                if let Some(index) = self.hovered_index {
                    self.selected_index = index;
                    self.open = false;
                    return true;
                }
            } else {
                self.open = false;
            }
        }
        
        false
    }

    fn update(&mut self, _dt: f32) {}

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Header (normal rendering)
        renderer.draw_rounded_rect(self.position, self.size, Vec4::new(0.1, 0.1, 0.12, 0.9), self.corner_radius);
        
        // Selected text
        if let Some(text) = self.options.get(self.selected_index) {
            renderer.draw_text(text, self.position + Vec2::new(10.0, 8.0), 16.0, theme.text);
        }
        
        // Arrow
        let arrow = if self.open { "▲" } else { "▼" };
        renderer.draw_text(arrow, Vec2::new(self.position.x + self.size.x - 24.0, self.position.y + 10.0), 14.0, theme.text_secondary);
        
        // Dropdown list - USE OVERLAY RENDERING (on top of everything)
        if self.open {
            let item_height = 32.0;
            let list_y = self.position.y + self.size.y + 2.0;
            let list_height = self.options.len() as f32 * item_height;
            
            // Background - use overlay (renders on top)
            renderer.draw_overlay_rect(
                Vec2::new(self.position.x, list_y),
                Vec2::new(self.size.x, list_height),
                Vec4::new(0.08, 0.08, 0.1, 0.98),
                self.corner_radius
            );
            
            // Items
            for (i, option) in self.options.iter().enumerate() {
                let item_y = list_y + i as f32 * item_height;
                
                // Hover highlight
                if self.hovered_index == Some(i) {
                    renderer.draw_overlay_rect(
                        Vec2::new(self.position.x + 2.0, item_y + 2.0),
                        Vec2::new(self.size.x - 4.0, item_height - 4.0),
                        Vec4::new(theme.primary.x, theme.primary.y, theme.primary.z, 0.3),
                        4.0
                    );
                }
                
                // Text - use overlay
                let color = if i == self.selected_index { theme.primary } else { theme.text };
                renderer.draw_overlay_text(option, Vec2::new(self.position.x + 10.0, item_y + 6.0), 16.0, color);
            }
        }
    }
}

// =============================================================================
// DATE PICKER
// =============================================================================

/// Simple date representation (no external chrono dependency)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SimpleDate {
    pub year: i32,
    pub month: u32,  // 1-12
    pub day: u32,    // 1-31
}

impl SimpleDate {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }
    
    /// Get today's date (uses system time)
    pub fn today() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Rough calculation (not accounting for leap seconds, etc.)
        let days_since_epoch = secs / 86400;
        let mut year = 1970;
        let mut remaining_days = days_since_epoch as i32;
        
        loop {
            let days_in_year = if Self::is_leap_year(year) { 366 } else { 365 };
            if remaining_days < days_in_year {
                break;
            }
            remaining_days -= days_in_year;
            year += 1;
        }
        
        let mut month = 1;
        loop {
            let days_in_month = Self::days_in_month(year, month);
            if remaining_days < days_in_month as i32 {
                break;
            }
            remaining_days -= days_in_month as i32;
            month += 1;
        }
        
        Self {
            year,
            month,
            day: remaining_days as u32 + 1,
        }
    }
    
    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
    
    fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if Self::is_leap_year(year) { 29 } else { 28 },
            _ => 30,
        }
    }
    
    /// Get day of week for first day of month (0 = Sunday, 6 = Saturday)
    fn first_day_of_month(&self) -> u32 {
        // Zeller's congruence (simplified)
        let y = if self.month < 3 { self.year - 1 } else { self.year };
        let m = if self.month < 3 { self.month as i32 + 12 } else { self.month as i32 };
        let q: i32 = 1; // first day
        let k = y % 100;
        let j = y / 100;
        
        let h = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;
        ((h + 6) % 7) as u32 // Convert to 0=Sunday
    }
    
    pub fn format(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl Default for SimpleDate {
    fn default() -> Self {
        Self::today()
    }
}

/// Date picker with calendar popup
pub struct DatePicker {
    pub position: Vec2,
    pub size: Vec2,
    pub value: SimpleDate,
    pub display_month: SimpleDate, // Month being displayed in calendar
    pub open: bool,
    pub hovered_day: Option<u32>,
    pub corner_radius: f32,
}

impl DatePicker {
    pub fn new() -> Self {
        let today = SimpleDate::today();
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            value: today,
            display_month: today,
            open: false,
            hovered_day: None,
            corner_radius: 6.0,
        }
    }
    
    pub fn with_value(mut self, date: SimpleDate) -> Self {
        self.value = date;
        self.display_month = date;
        self
    }
    
    fn prev_month(&mut self) {
        if self.display_month.month == 1 {
            self.display_month.month = 12;
            self.display_month.year -= 1;
        } else {
            self.display_month.month -= 1;
        }
    }
    
    fn next_month(&mut self) {
        if self.display_month.month == 12 {
            self.display_month.month = 1;
            self.display_month.year += 1;
        } else {
            self.display_month.month += 1;
        }
    }
    
    fn calendar_size(&self) -> Vec2 {
        Vec2::new(240.0, 220.0)
    }
}

impl Default for DatePicker {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for DatePicker {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(160.0, 36.0);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        // Header click
        let in_header = mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.size.x &&
                        mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.size.y;
        
        let cal_pos = Vec2::new(self.position.x, self.position.y + self.size.y + 2.0);
        let cal_size = self.calendar_size();
        
        let in_calendar = self.open &&
            mouse_pos.x >= cal_pos.x && mouse_pos.x <= cal_pos.x + cal_size.x &&
            mouse_pos.y >= cal_pos.y && mouse_pos.y <= cal_pos.y + cal_size.y;
        
        // Detect hovered day
        if in_calendar {
            let grid_y = cal_pos.y + 50.0; // Below header and weekday labels
            let cell_w = cal_size.x / 7.0;
            let cell_h = 24.0;
            
            if mouse_pos.y >= grid_y {
                let col = ((mouse_pos.x - cal_pos.x) / cell_w) as u32;
                let row = ((mouse_pos.y - grid_y) / cell_h) as u32;
                let first_dow = self.display_month.first_day_of_month();
                let day_idx = row * 7 + col;
                
                if day_idx >= first_dow {
                    let day = day_idx - first_dow + 1;
                    let max_days = SimpleDate::days_in_month(self.display_month.year, self.display_month.month);
                    if day >= 1 && day <= max_days {
                        self.hovered_day = Some(day);
                    } else {
                        self.hovered_day = None;
                    }
                } else {
                    self.hovered_day = None;
                }
            } else {
                self.hovered_day = None;
            }
        } else {
            self.hovered_day = None;
        }
        
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. }, .. } = event {
            if in_header {
                self.open = !self.open;
                return true;
            } else if in_calendar {
                // Check nav buttons (top of calendar)
                let nav_y = cal_pos.y + 4.0;
                if mouse_pos.y >= nav_y && mouse_pos.y <= nav_y + 24.0 {
                    // Previous month button (left side)
                    if mouse_pos.x >= cal_pos.x + 4.0 && mouse_pos.x <= cal_pos.x + 30.0 {
                        self.prev_month();
                        return true;
                    }
                    // Next month button (right side)
                    if mouse_pos.x >= cal_pos.x + cal_size.x - 30.0 && mouse_pos.x <= cal_pos.x + cal_size.x - 4.0 {
                        self.next_month();
                        return true;
                    }
                }
                
                // Select day
                if let Some(day) = self.hovered_day {
                    self.value = SimpleDate::new(self.display_month.year, self.display_month.month, day);
                    self.open = false;
                    return true;
                }
            } else {
                self.open = false;
            }
        }
        
        false
    }

    fn update(&mut self, _dt: f32) {}

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Header (shows selected date)
        renderer.draw_rounded_rect(self.position, self.size, Vec4::new(0.1, 0.1, 0.12, 0.9), self.corner_radius);
        renderer.draw_text(&self.value.format(), self.position + Vec2::new(10.0, 8.0), 16.0, theme.text);
        
        // Calendar icon
        renderer.draw_text("📅", Vec2::new(self.position.x + self.size.x - 28.0, self.position.y + 8.0), 16.0, theme.text_secondary);
        
        // Calendar popup
        if self.open {
            let cal_pos = Vec2::new(self.position.x, self.position.y + self.size.y + 2.0);
            let cal_size = self.calendar_size();
            
            // Background
            renderer.draw_overlay_rect(cal_pos, cal_size, Vec4::new(0.08, 0.08, 0.1, 0.98), 8.0);
            
            // Border
            renderer.draw_overlay_rect(
                cal_pos - Vec2::splat(1.0),
                cal_size + Vec2::splat(2.0),
                Vec4::new(theme.border.x, theme.border.y, theme.border.z, 0.4),
                9.0
            );
            
            // Month/Year header with nav
            let month_names = ["", "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
            let month_str = format!("{} {}", month_names[self.display_month.month as usize], self.display_month.year);
            
            // Nav arrows
            renderer.draw_overlay_text("◀", cal_pos + Vec2::new(10.0, 8.0), 14.0, theme.text_secondary);
            renderer.draw_overlay_text("▶", Vec2::new(cal_pos.x + cal_size.x - 22.0, cal_pos.y + 8.0), 14.0, theme.text_secondary);
            
            // Month title
            let title_x = cal_pos.x + (cal_size.x - month_str.len() as f32 * 8.0) / 2.0;
            renderer.draw_overlay_text(&month_str, Vec2::new(title_x, cal_pos.y + 8.0), 16.0, theme.text);
            
            // Weekday headers
            let weekdays = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];
            let cell_w = cal_size.x / 7.0;
            for (i, wd) in weekdays.iter().enumerate() {
                let x = cal_pos.x + i as f32 * cell_w + cell_w / 2.0 - 8.0;
                renderer.draw_overlay_text(wd, Vec2::new(x, cal_pos.y + 32.0), 12.0, theme.text_secondary);
            }
            
            // Day grid
            let first_dow = self.display_month.first_day_of_month();
            let days_in_month = SimpleDate::days_in_month(self.display_month.year, self.display_month.month);
            let cell_h = 24.0;
            let grid_y = cal_pos.y + 50.0;
            
            for day in 1..=days_in_month {
                let cell_idx = first_dow + day - 1;
                let col = cell_idx % 7;
                let row = cell_idx / 7;
                
                let x = cal_pos.x + col as f32 * cell_w;
                let y = grid_y + row as f32 * cell_h;
                
                // Highlight selected day
                let is_selected = self.value.year == self.display_month.year &&
                                  self.value.month == self.display_month.month &&
                                  self.value.day == day;
                
                let is_hovered = self.hovered_day == Some(day);
                
                if is_selected {
                    renderer.draw_overlay_rect(
                        Vec2::new(x + 2.0, y),
                        Vec2::new(cell_w - 4.0, cell_h - 2.0),
                        theme.primary,
                        4.0
                    );
                } else if is_hovered {
                    renderer.draw_overlay_rect(
                        Vec2::new(x + 2.0, y),
                        Vec2::new(cell_w - 4.0, cell_h - 2.0),
                        Vec4::new(theme.primary.x, theme.primary.y, theme.primary.z, 0.3),
                        4.0
                    );
                }
                
                let day_str = format!("{}", day);
                let text_color = if is_selected { Vec4::new(1.0, 1.0, 1.0, 1.0) } else { theme.text };
                let text_x = x + cell_w / 2.0 - if day >= 10 { 8.0 } else { 4.0 };
                renderer.draw_overlay_text(&day_str, Vec2::new(text_x, y + 4.0), 14.0, text_color);
            }
        }
    }
}
