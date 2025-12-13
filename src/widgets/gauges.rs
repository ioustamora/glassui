//! GlassUI Progress and Gauge Widgets
//!
//! Visual data indicators:
//! - ProgressBar with animation
//! - CircularGauge (radial progress)
//! - Sparkline mini chart
//! - MetricDisplay with trend

use glam::{Vec2, Vec4};
use crate::renderer::GlassRenderer;
use crate::widget_id::WidgetId;
use crate::widgets::core::{Widget, get_theme};

// =============================================================================
// PROGRESS BAR
// =============================================================================

/// Animated progress bar with smooth transitions
pub struct AnimatedProgressBar {
    pub id: WidgetId,
    pub position: Vec2,
    pub size: Vec2,
    pub value: f32,           // 0.0 to 1.0
    pub target_value: f32,    // For animation
    pub color: Vec4,
    pub background_color: Vec4,
    pub show_percentage: bool,
    pub animated: bool,
    pub striped: bool,
    corner_radius: f32,
}

impl AnimatedProgressBar {
    pub fn new(value: f32) -> Self {
        let theme = get_theme();
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::new(200.0, 20.0),
            value: value.clamp(0.0, 1.0),
            target_value: value.clamp(0.0, 1.0),
            color: theme.primary,
            background_color: Vec4::new(0.15, 0.15, 0.18, 0.8),
            show_percentage: true,
            animated: true,
            striped: false,
            corner_radius: 4.0,
        }
    }
    
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }
    
    pub fn striped(mut self) -> Self {
        self.striped = true;
        self
    }
    
    pub fn set_value(&mut self, value: f32) {
        self.target_value = value.clamp(0.0, 1.0);
        if !self.animated {
            self.value = self.target_value;
        }
    }
    
    /// Get color based on value (green -> yellow -> red)
    pub fn auto_color(&mut self) {
        self.color = if self.value < 0.5 {
            Vec4::new(0.3, 0.8, 0.4, 1.0)  // Green
        } else if self.value < 0.8 {
            Vec4::new(0.9, 0.7, 0.2, 1.0)  // Yellow
        } else {
            Vec4::new(0.9, 0.3, 0.3, 1.0)  // Red
        };
    }
}

impl Widget for AnimatedProgressBar {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(max_size.x.min(self.size.x), self.size.y);
        self.size
    }

    fn handle_event(&mut self, _event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool {
        false
    }

    fn update(&mut self, dt: f32) {
        // Smooth animation
        if self.animated && (self.value - self.target_value).abs() > 0.001 {
            self.value += (self.target_value - self.value) * 8.0 * dt;
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Background
        renderer.draw_rounded_rect(self.position, self.size, self.background_color, self.corner_radius);
        
        // Progress fill
        let fill_width = self.size.x * self.value;
        if fill_width > 0.0 {
            renderer.draw_rounded_rect(
                self.position,
                Vec2::new(fill_width, self.size.y),
                self.color,
                self.corner_radius
            );
        }
        
        // Percentage text
        if self.show_percentage {
            let percent = format!("{}%", (self.value * 100.0) as i32);
            let text_x = self.position.x + self.size.x / 2.0 - percent.len() as f32 * 4.0;
            renderer.draw_text(&percent, Vec2::new(text_x, self.position.y + 3.0), 12.0, theme.text);
        }
    }
}

// =============================================================================
// CIRCULAR GAUGE
// =============================================================================

/// Circular progress gauge
pub struct CircularGauge {
    pub id: WidgetId,
    pub position: Vec2,
    pub radius: f32,
    pub value: f32,           // 0.0 to 1.0
    pub target_value: f32,
    pub color: Vec4,
    pub background_color: Vec4,
    pub label: String,
    pub show_value: bool,
    pub thickness: f32,
}

impl CircularGauge {
    pub fn new(value: f32) -> Self {
        let theme = get_theme();
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            radius: 50.0,
            value: value.clamp(0.0, 1.0),
            target_value: value.clamp(0.0, 1.0),
            color: theme.primary,
            background_color: Vec4::new(0.15, 0.15, 0.18, 0.5),
            label: String::new(),
            show_value: true,
            thickness: 8.0,
        }
    }
    
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }
    
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }
    
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
    
    pub fn set_value(&mut self, value: f32) {
        self.target_value = value.clamp(0.0, 1.0);
    }
}

impl Widget for CircularGauge {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        Vec2::splat(self.radius * 2.0)
    }

    fn handle_event(&mut self, _event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool {
        false
    }

    fn update(&mut self, dt: f32) {
        // Smooth animation
        if (self.value - self.target_value).abs() > 0.001 {
            self.value += (self.target_value - self.value) * 6.0 * dt;
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        let center = self.position + Vec2::splat(self.radius);
        
        // Background circle (approximated with rounded rect)
        renderer.draw_rounded_rect(
            self.position,
            Vec2::splat(self.radius * 2.0),
            self.background_color,
            self.radius
        );
        
        // Inner circle (creates ring effect)
        let inner_radius = self.radius - self.thickness;
        renderer.draw_rounded_rect(
            self.position + Vec2::splat(self.thickness),
            Vec2::splat(inner_radius * 2.0),
            Vec4::new(0.05, 0.05, 0.07, 0.95),
            inner_radius
        );
        
        // Value arc (simplified as a partial fill for now)
        // In a real implementation, this would use arc rendering
        if self.value > 0.0 {
            // Approximate with a colored indicator
            let indicator_angle = self.value * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2;
            let indicator_pos = center + Vec2::new(
                indicator_angle.cos() * (self.radius - self.thickness / 2.0),
                indicator_angle.sin() * (self.radius - self.thickness / 2.0)
            );
            renderer.draw_rounded_rect(
                indicator_pos - Vec2::splat(self.thickness / 2.0),
                Vec2::splat(self.thickness),
                self.color,
                self.thickness / 2.0
            );
        }
        
        // Value text
        if self.show_value {
            let value_text = format!("{}%", (self.value * 100.0) as i32);
            let text_x = center.x - value_text.len() as f32 * 5.0;
            renderer.draw_text(&value_text, Vec2::new(text_x, center.y - 8.0), 16.0, theme.text);
        }
        
        // Label
        if !self.label.is_empty() {
            let label_x = center.x - self.label.len() as f32 * 3.0;
            renderer.draw_text(&self.label, Vec2::new(label_x, center.y + 8.0), 11.0, theme.text_secondary);
        }
    }
}

// =============================================================================
// SPARKLINE
// =============================================================================

/// Mini line chart for inline display
pub struct MiniSparkline {
    pub id: WidgetId,
    pub position: Vec2,
    pub size: Vec2,
    pub data: Vec<f32>,
    pub max_points: usize,
    pub color: Vec4,
    pub fill: bool,
    pub show_last_value: bool,
}

impl MiniSparkline {
    pub fn new() -> Self {
        let theme = get_theme();
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::new(100.0, 30.0),
            data: Vec::new(),
            max_points: 30,
            color: theme.primary,
            fill: true,
            show_last_value: false,
        }
    }
    
    pub fn with_data(mut self, data: Vec<f32>) -> Self {
        self.data = data;
        self
    }
    
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }
    
    /// Add a new data point
    pub fn push(&mut self, value: f32) {
        self.data.push(value);
        if self.data.len() > self.max_points {
            self.data.remove(0);
        }
    }
    
    /// Get min and max values
    fn range(&self) -> (f32, f32) {
        if self.data.is_empty() {
            return (0.0, 1.0);
        }
        let min = self.data.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = self.data.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        if (max - min).abs() < 0.001 {
            (min - 0.5, max + 0.5)
        } else {
            (min, max)
        }
    }
}

impl Default for MiniSparkline {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for MiniSparkline {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(max_size.x.min(self.size.x), self.size.y);
        self.size
    }

    fn handle_event(&mut self, _event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool {
        false
    }

    fn update(&mut self, _dt: f32) {}

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        if self.data.len() < 2 { return; }
        
        let (min, max) = self.range();
        let range = max - min;
        let point_width = self.size.x / (self.data.len() - 1) as f32;
        
        // Draw line segments
        for i in 0..self.data.len() - 1 {
            let x1 = self.position.x + i as f32 * point_width;
            let x2 = self.position.x + (i + 1) as f32 * point_width;
            let y1 = self.position.y + self.size.y - ((self.data[i] - min) / range) * self.size.y;
            let y2 = self.position.y + self.size.y - ((self.data[i + 1] - min) / range) * self.size.y;
            
            // Line segment (approximated)
            let dx = x2 - x1;
            let dy = y2 - y1;
            let len = (dx * dx + dy * dy).sqrt();
            let angle = dy.atan2(dx);
            
            // Draw as thin rectangle rotated (simplified: just draw points)
            renderer.draw_rounded_rect(
                Vec2::new(x1, y1.min(y2)),
                Vec2::new(point_width, 2.0),
                self.color,
                1.0
            );
        }
        
        // End point dot
        if let Some(&last) = self.data.last() {
            let last_x = self.position.x + self.size.x;
            let last_y = self.position.y + self.size.y - ((last - min) / range) * self.size.y;
            renderer.draw_rounded_rect(
                Vec2::new(last_x - 3.0, last_y - 3.0),
                Vec2::splat(6.0),
                self.color,
                3.0
            );
            
            // Show value
            if self.show_last_value {
                let value_text = format!("{:.1}", last);
                renderer.draw_text(&value_text, Vec2::new(last_x + 4.0, last_y - 6.0), 10.0, theme.text_secondary);
            }
        }
    }
}

// =============================================================================
// METRIC DISPLAY
// =============================================================================

/// Trend direction for metrics
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MetricTrend {
    Up,
    Down,
    Stable,
}

impl MetricTrend {
    pub fn icon(&self) -> &'static str {
        match self {
            MetricTrend::Up => "↑",
            MetricTrend::Down => "↓",
            MetricTrend::Stable => "→",
        }
    }
    
    pub fn color(&self) -> Vec4 {
        match self {
            MetricTrend::Up => Vec4::new(0.3, 0.8, 0.4, 1.0),
            MetricTrend::Down => Vec4::new(0.9, 0.3, 0.3, 1.0),
            MetricTrend::Stable => Vec4::new(0.6, 0.6, 0.6, 1.0),
        }
    }
}

/// Display a metric with value, trend, and sparkline
pub struct MetricDisplay {
    pub id: WidgetId,
    pub position: Vec2,
    pub size: Vec2,
    pub label: String,
    pub value: String,
    pub trend: MetricTrend,
    pub trend_value: String,
    pub sparkline: MiniSparkline,
}

impl MetricDisplay {
    pub fn new(label: &str, value: &str) -> Self {
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::new(180.0, 80.0),
            label: label.to_string(),
            value: value.to_string(),
            trend: MetricTrend::Stable,
            trend_value: String::new(),
            sparkline: MiniSparkline::new(),
        }
    }
    
    pub fn with_trend(mut self, trend: MetricTrend, value: &str) -> Self {
        self.trend = trend;
        self.trend_value = value.to_string();
        self
    }
    
    pub fn with_sparkline_data(mut self, data: Vec<f32>) -> Self {
        self.sparkline = MiniSparkline::new().with_data(data);
        self
    }
    
    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
    }
    
    pub fn add_data_point(&mut self, value: f32) {
        self.sparkline.push(value);
    }
}

impl Widget for MetricDisplay {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        
        // Layout sparkline at bottom
        let sparkline_size = Vec2::new(self.size.x - 16.0, 24.0);
        self.sparkline.position = self.position + Vec2::new(8.0, self.size.y - 32.0);
        self.sparkline.size = sparkline_size;
        
        self.size
    }

    fn handle_event(&mut self, _event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool {
        false
    }

    fn update(&mut self, dt: f32) {
        self.sparkline.update(dt);
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Background
        renderer.draw_rounded_rect(
            self.position,
            self.size,
            Vec4::new(0.08, 0.08, 0.1, 0.85),
            8.0
        );
        
        // Label
        renderer.draw_text(&self.label, self.position + Vec2::new(12.0, 8.0), 12.0, theme.text_secondary);
        
        // Value
        renderer.draw_text(&self.value, self.position + Vec2::new(12.0, 24.0), 20.0, theme.text);
        
        // Trend
        let trend_x = self.position.x + self.size.x - 50.0;
        renderer.draw_text(self.trend.icon(), Vec2::new(trend_x, self.position.y + 24.0), 16.0, self.trend.color());
        if !self.trend_value.is_empty() {
            renderer.draw_text(&self.trend_value, Vec2::new(trend_x + 16.0, self.position.y + 26.0), 12.0, self.trend.color());
        }
        
        // Sparkline
        self.sparkline.render(renderer);
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_progress_bar() {
        let mut bar = AnimatedProgressBar::new(0.5);
        bar.set_value(0.75);
        bar.update(0.1);
        
        assert!(bar.value > 0.5);
    }
    
    #[test]
    fn test_sparkline() {
        let mut spark = MiniSparkline::new();
        spark.push(10.0);
        spark.push(20.0);
        spark.push(15.0);
        
        let (min, max) = spark.range();
        assert_eq!(min, 10.0);
        assert_eq!(max, 20.0);
    }
    
    #[test]
    fn test_metric_display() {
        let metric = MetricDisplay::new("CPU", "45%")
            .with_trend(MetricTrend::Up, "+5%");
        
        assert_eq!(metric.label, "CPU");
        assert_eq!(metric.trend, MetricTrend::Up);
    }
}
