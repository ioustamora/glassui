//! GlassUI Live Widgets
//!
//! Enhanced widgets with reactive data binding:
//! - LivePanel: Panel with dynamic color and pulse animation
//! - LiveLabel: Label with reactive text
//! - LiveKpi: KPI card with sparkline

use glam::{Vec2, Vec4};
use crate::renderer::GlassRenderer;
use crate::reactive::{Reactive, ColorSource, Property};
use crate::panel_style::{PanelPreset, PanelStyle};
use crate::widget_id::WidgetId;
use crate::widgets::core::{Widget, get_theme};

// =============================================================================
// LIVE PANEL
// =============================================================================

/// Panel with reactive color binding and pulse animation
pub struct LivePanel {
    pub id: WidgetId,
    pub position: Vec2,
    pub size: Vec2,
    pub content: Option<Box<dyn Widget>>,
    
    // Style
    pub style: PanelStyle,
    
    // Reactive color
    pub color_source: ColorSource,
    
    // Pulse animation
    pub pulse_enabled: bool,
    pub pulse_rate: f32,  // BPM
    pub pulse_intensity: f32,  // 0.0-1.0
    pulse_phase: f32,
    
    // Glow effect
    pub glow_enabled: bool,
    pub glow_color: Vec4,
    pub glow_intensity: f32,
    
    // Layout
    pub padding: f32,
    pub fill: bool,
}

impl LivePanel {
    /// Create a new live panel
    pub fn new(content: Box<dyn Widget>) -> Self {
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            content: Some(content),
            style: PanelStyle::default(),
            color_source: ColorSource::Static(Vec4::new(0.1, 0.1, 0.15, 0.4)),
            pulse_enabled: false,
            pulse_rate: 60.0,
            pulse_intensity: 0.2,
            pulse_phase: 0.0,
            glow_enabled: false,
            glow_color: Vec4::new(0.5, 0.5, 1.0, 0.5),
            glow_intensity: 0.0,
            padding: 20.0,
            fill: false,
        }
    }
    
    /// Create from preset
    pub fn with_preset(mut self, preset: PanelPreset) -> Self {
        self.style = PanelStyle::from_preset(preset);
        self.color_source = ColorSource::Static(preset.tint_color());
        self
    }
    
    /// Bind color to a reactive value
    pub fn bind_color(mut self, source: ColorSource) -> Self {
        self.color_source = source;
        self
    }
    
    /// Bind color to a reactive Vec4
    pub fn bind_reactive_color(mut self, color: Reactive<Vec4>) -> Self {
        self.color_source = ColorSource::Reactive(color);
        self
    }
    
    /// Enable pulse animation
    pub fn with_pulse(mut self, bpm: f32) -> Self {
        self.pulse_enabled = true;
        self.pulse_rate = bpm;
        self
    }
    
    /// Set pulse intensity
    pub fn with_pulse_intensity(mut self, intensity: f32) -> Self {
        self.pulse_intensity = intensity.clamp(0.0, 1.0);
        self
    }
    
    /// Enable glow effect
    pub fn with_glow(mut self, color: Vec4, intensity: f32) -> Self {
        self.glow_enabled = true;
        self.glow_color = color;
        self.glow_intensity = intensity;
        self
    }
    
    /// Set fill mode
    pub fn with_fill(mut self, fill: bool) -> Self {
        self.fill = fill;
        self
    }
    
    /// Set padding
    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }
    
    /// Get current color (with pulse applied)
    fn get_current_color(&self) -> Vec4 {
        let base_color = self.color_source.get();
        
        if self.pulse_enabled {
            // Calculate pulse factor (0.0 to 1.0)
            let pulse = (self.pulse_phase.sin() * 0.5 + 0.5) * self.pulse_intensity;
            
            // Brighten the color
            Vec4::new(
                (base_color.x + pulse * 0.2).min(1.0),
                (base_color.y + pulse * 0.2).min(1.0),
                (base_color.z + pulse * 0.2).min(1.0),
                (base_color.w + pulse * 0.1).min(1.0),
            )
        } else {
            base_color
        }
    }
}

impl Widget for LivePanel {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        
        let content_available = max_size - Vec2::splat(self.padding * 2.0);
        
        let content_size = if let Some(content) = &mut self.content {
            let content_origin = origin + Vec2::splat(self.padding);
            content.layout(content_origin, content_available)
        } else {
            Vec2::ZERO
        };
        
        self.size = if self.fill {
            max_size
        } else {
            content_size + Vec2::splat(self.padding * 2.0)
        };
        
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        if let Some(content) = &mut self.content {
            content.handle_event(event, mouse_pos)
        } else {
            false
        }
    }

    fn update(&mut self, dt: f32) {
        // Update pulse phase
        if self.pulse_enabled {
            // Convert BPM to radians per second
            let radians_per_second = self.pulse_rate / 60.0 * std::f32::consts::TAU;
            self.pulse_phase += radians_per_second * dt;
            if self.pulse_phase > std::f32::consts::TAU {
                self.pulse_phase -= std::f32::consts::TAU;
            }
        }
        
        if let Some(content) = &mut self.content {
            content.update(dt);
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let color = self.get_current_color();
        let radius = self.style.corner_radius;
        
        // Draw glow if enabled
        if self.glow_enabled && self.glow_intensity > 0.0 {
            let glow_size = Vec2::splat(self.glow_intensity * 8.0);
            renderer.draw_rounded_rect(
                self.position - glow_size,
                self.size + glow_size * 2.0,
                Vec4::new(self.glow_color.x, self.glow_color.y, self.glow_color.z, self.glow_intensity * 0.3),
                radius + glow_size.x,
            );
        }
        
        // Draw main panel
        renderer.draw_rounded_rect(self.position, self.size, color, radius);
        
        // Draw border
        let border_color = self.style.border_color;
        renderer.draw_rounded_rect(
            self.position - Vec2::splat(1.0),
            self.size + Vec2::splat(2.0),
            Vec4::new(border_color.x, border_color.y, border_color.z, 0.3),
            radius + 1.0,
        );
        
        // Draw content
        if let Some(content) = &self.content {
            content.render(renderer);
        }
    }
}

// =============================================================================
// LIVE LABEL
// =============================================================================

/// Label with reactive text binding
pub struct LiveLabel {
    pub id: WidgetId,
    pub position: Vec2,
    pub size: Vec2,
    pub text_source: TextSource,
    pub font_size: f32,
    pub color: Vec4,
}

/// Source of text content
pub enum TextSource {
    Static(String),
    Reactive(Reactive<String>),
}

impl LiveLabel {
    pub fn new(text: &str) -> Self {
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            text_source: TextSource::Static(text.to_string()),
            font_size: 16.0,
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
        }
    }
    
    pub fn bind(text: Reactive<String>) -> Self {
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            text_source: TextSource::Reactive(text),
            font_size: 16.0,
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
        }
    }
    
    pub fn with_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }
    
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }
    
    fn get_text(&self) -> String {
        match &self.text_source {
            TextSource::Static(s) => s.clone(),
            TextSource::Reactive(r) => r.get(),
        }
    }
}

impl Widget for LiveLabel {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        let text = self.get_text();
        self.size = Vec2::new(text.len() as f32 * (self.font_size * 0.6), self.font_size + 4.0);
        self.size
    }

    fn handle_event(&mut self, _event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool {
        false
    }

    fn update(&mut self, _dt: f32) {}

    fn render(&self, renderer: &mut GlassRenderer) {
        let text = self.get_text();
        renderer.draw_text(&text, self.position, self.font_size, self.color);
    }
}

// =============================================================================
// KPI CARD
// =============================================================================

/// KPI display card with value, trend, and optional sparkline
pub struct KpiCard {
    pub id: WidgetId,
    pub position: Vec2,
    pub size: Vec2,
    pub label: String,
    pub value_source: ValueSource,
    pub trend: Trend,
    pub sparkline_data: Vec<f32>,
    pub preset: PanelPreset,
}

/// Source of numeric value
pub enum ValueSource {
    Static(f32),
    Reactive(Reactive<f32>),
}

/// Trend indicator
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Trend {
    Up,
    Down,
    Stable,
    None,
}

impl KpiCard {
    pub fn new(label: &str, value: f32) -> Self {
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::new(160.0, 100.0),
            label: label.to_string(),
            value_source: ValueSource::Static(value),
            trend: Trend::None,
            sparkline_data: Vec::new(),
            preset: PanelPreset::Data,
        }
    }
    
    pub fn bind(label: &str, value: Reactive<f32>) -> Self {
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::new(160.0, 100.0),
            label: label.to_string(),
            value_source: ValueSource::Reactive(value),
            trend: Trend::None,
            sparkline_data: Vec::new(),
            preset: PanelPreset::Data,
        }
    }
    
    pub fn with_trend(mut self, trend: Trend) -> Self {
        self.trend = trend;
        self
    }
    
    pub fn with_sparkline(mut self, data: Vec<f32>) -> Self {
        self.sparkline_data = data;
        self
    }
    
    pub fn with_preset(mut self, preset: PanelPreset) -> Self {
        self.preset = preset;
        self
    }
    
    fn get_value(&self) -> f32 {
        match &self.value_source {
            ValueSource::Static(v) => *v,
            ValueSource::Reactive(r) => r.get(),
        }
    }
}

impl Widget for KpiCard {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size
    }

    fn handle_event(&mut self, _event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool {
        false
    }

    fn update(&mut self, _dt: f32) {}

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        let style = PanelStyle::from_preset(self.preset);
        
        // Background
        renderer.draw_rounded_rect(self.position, self.size, style.tint_color, style.corner_radius);
        
        // Label
        renderer.draw_text(&self.label, self.position + Vec2::new(12.0, 8.0), 14.0, theme.text_secondary);
        
        // Value
        let value = self.get_value();
        let value_str = format!("{:.1}", value);
        renderer.draw_text(&value_str, self.position + Vec2::new(12.0, 30.0), 28.0, theme.text);
        
        // Trend indicator
        let trend_text = match self.trend {
            Trend::Up => "▲",
            Trend::Down => "▼",
            Trend::Stable => "─",
            Trend::None => "",
        };
        let trend_color = match self.trend {
            Trend::Up => Vec4::new(0.3, 0.9, 0.4, 1.0),
            Trend::Down => Vec4::new(0.9, 0.3, 0.3, 1.0),
            _ => theme.text_secondary,
        };
        if !trend_text.is_empty() {
            renderer.draw_text(trend_text, self.position + Vec2::new(self.size.x - 24.0, 35.0), 16.0, trend_color);
        }
        
        // Sparkline (simple line)
        if !self.sparkline_data.is_empty() {
            let sparkline_y = self.position.y + self.size.y - 25.0;
            let sparkline_width = self.size.x - 24.0;
            let step = sparkline_width / (self.sparkline_data.len() - 1).max(1) as f32;
            
            let max_val = self.sparkline_data.iter().cloned().fold(f32::MIN, f32::max);
            let min_val = self.sparkline_data.iter().cloned().fold(f32::MAX, f32::min);
            let range = (max_val - min_val).max(0.001);
            
            for i in 0..self.sparkline_data.len() {
                let x = self.position.x + 12.0 + i as f32 * step;
                let normalized = (self.sparkline_data[i] - min_val) / range;
                let y = sparkline_y + 15.0 - normalized * 15.0;
                
                renderer.draw_rounded_rect(
                    Vec2::new(x, y),
                    Vec2::new(2.0, 2.0),
                    style.border_color,
                    1.0,
                );
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
    use crate::widgets::Label;
    
    #[test]
    fn test_live_panel_pulse() {
        let label = Box::new(Label::new("Test"));
        let mut panel = LivePanel::new(label)
            .with_pulse(60.0)
            .with_pulse_intensity(0.3);
        
        assert!(panel.pulse_enabled);
        assert_eq!(panel.pulse_rate, 60.0);
        
        // Simulate update
        panel.update(0.5);
        assert!(panel.pulse_phase > 0.0);
    }
    
    #[test]
    fn test_kpi_card() {
        let value = Reactive::new(42.5);
        let kpi = KpiCard::bind("CPU", value.clone())
            .with_trend(Trend::Up)
            .with_sparkline(vec![10.0, 20.0, 15.0, 25.0]);
        
        assert_eq!(kpi.get_value(), 42.5);
    }
}
