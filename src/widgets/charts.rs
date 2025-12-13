//! GlassUI Chart Widgets
//!
//! Provides data visualization widgets:
//! - `LineChart` - Line/area charts
//! - `BarChart` - Vertical/horizontal bar charts  
//! - `PieChart` - Pie/donut charts
//! - `Sparkline` - Mini inline charts

use glam::{Vec2, Vec4};
use crate::widgets::Widget;
use crate::renderer::GlassRenderer;
use crate::layout::{Size, Offset};

// =============================================================================
// DATA POINT
// =============================================================================

/// A single data point
#[derive(Clone, Debug)]
pub struct DataPoint {
    pub value: f64,
    pub label: Option<String>,
    pub color: Option<Vec4>,
}

impl DataPoint {
    pub fn new(value: f64) -> Self {
        Self { value, label: None, color: None }
    }
    
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }
    
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

impl From<f64> for DataPoint {
    fn from(value: f64) -> Self { DataPoint::new(value) }
}

impl From<i32> for DataPoint {
    fn from(value: i32) -> Self { DataPoint::new(value as f64) }
}

/// A data series
#[derive(Clone, Debug)]
pub struct DataSeries {
    pub name: String,
    pub data: Vec<DataPoint>,
    pub color: Vec4,
}

impl DataSeries {
    pub fn new(name: &str, data: Vec<DataPoint>) -> Self {
        Self { name: name.to_string(), data, color: Vec4::new(0.4, 0.6, 1.0, 1.0) }
    }
    
    pub fn from_values<T: Into<DataPoint> + Clone>(name: &str, values: &[T]) -> Self {
        Self::new(name, values.iter().cloned().map(|v| v.into()).collect())
    }
    
    pub fn with_color(mut self, color: Vec4) -> Self { self.color = color; self }
    
    pub fn min(&self) -> f64 { self.data.iter().map(|d| d.value).fold(f64::INFINITY, f64::min) }
    pub fn max(&self) -> f64 { self.data.iter().map(|d| d.value).fold(f64::NEG_INFINITY, f64::max) }
}

// =============================================================================
// CHART CONFIG
// =============================================================================

#[derive(Clone, Debug)]
pub struct ChartConfig {
    pub background: Vec4,
    pub grid_color: Vec4,
    pub show_grid: bool,
    pub padding: f32,
}

impl Default for ChartConfig {
    fn default() -> Self {
        Self {
            background: Vec4::new(0.1, 0.1, 0.15, 0.8),
            grid_color: Vec4::new(1.0, 1.0, 1.0, 0.1),
            show_grid: true,
            padding: 40.0,
        }
    }
}

// =============================================================================
// LINE CHART
// =============================================================================

pub struct LineChart {
    series: Vec<DataSeries>,
    config: ChartConfig,
    position: Vec2,
    size: Size,
    line_width: f32,
    show_points: bool,
}

impl LineChart {
    pub fn new() -> Self {
        Self {
            series: Vec::new(),
            config: ChartConfig::default(),
            position: Vec2::ZERO,
            size: Size::new(300.0, 200.0),
            line_width: 2.0,
            show_points: true,
        }
    }
    
    pub fn with_data(mut self, name: &str, values: &[f64]) -> Self {
        self.series.push(DataSeries::from_values(name, values));
        self
    }
    
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.size = Size::new(width, height);
        self
    }
    
    fn get_bounds(&self) -> (f64, f64) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for series in &self.series {
            min = min.min(series.min());
            max = max.max(series.max());
        }
        let range = (max - min).max(0.001);
        (min - range * 0.1, max + range * 0.1)
    }
}

impl Default for LineChart { fn default() -> Self { Self::new() } }

impl Widget for LineChart {
    fn layout(&mut self, _origin: Vec2, _available: Vec2) -> Vec2 {
        Vec2::new(self.size.width, self.size.height)
    }
    
    fn handle_event(&mut self, _event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool { false }
    fn update(&mut self, _dt: f32) {}
    
    fn render(&self, renderer: &mut GlassRenderer) {
        let pos = self.position;
        let w = self.size.width;
        let h = self.size.height;
        let pad = self.config.padding;
        
        // Background
        renderer.draw_rounded_rect(pos, Vec2::new(w, h), self.config.background, 8.0);
        
        let chart_pos = pos + Vec2::new(pad, pad);
        let chart_w = w - pad * 2.0;
        let chart_h = h - pad * 2.0;
        
        // Grid
        if self.config.show_grid {
            for i in 0..=5 {
                let line_y = chart_pos.y + chart_h * i as f32 / 5.0;
                renderer.draw_rect(Vec2::new(chart_pos.x, line_y), Vec2::new(chart_w, 1.0), self.config.grid_color);
            }
        }
        
        let (min_val, max_val) = self.get_bounds();
        let value_range = (max_val - min_val).max(0.001);
        
        for series in &self.series {
            if series.data.is_empty() { continue; }
            let point_count = series.data.len();
            let x_step = chart_w / (point_count - 1).max(1) as f32;
            
            // Lines
            for i in 0..point_count.saturating_sub(1) {
                let v1 = series.data[i].value;
                let v2 = series.data[i + 1].value;
                let x1 = chart_pos.x + i as f32 * x_step;
                let x2 = chart_pos.x + (i + 1) as f32 * x_step;
                let y1 = chart_pos.y + chart_h - ((v1 - min_val) / value_range) as f32 * chart_h;
                let y2 = chart_pos.y + chart_h - ((v2 - min_val) / value_range) as f32 * chart_h;
                let len = ((x2-x1).powi(2) + (y2-y1).powi(2)).sqrt();
                renderer.draw_rect(Vec2::new((x1+x2)/2.0 - len/2.0, (y1+y2)/2.0 - self.line_width/2.0), Vec2::new(len, self.line_width), series.color);
            }
            
            // Points
            if self.show_points {
                for i in 0..point_count {
                    let v = series.data[i].value;
                    let px = chart_pos.x + i as f32 * x_step;
                    let py = chart_pos.y + chart_h - ((v - min_val) / value_range) as f32 * chart_h;
                    renderer.draw_rounded_rect(Vec2::new(px - 4.0, py - 4.0), Vec2::new(8.0, 8.0), series.color, 4.0);
                }
            }
        }
    }
    
    fn set_position(&mut self, pos: Offset) { self.position = Vec2::new(pos.x, pos.y); }
    fn get_position(&self) -> Offset { Offset::new(self.position.x, self.position.y) }
    fn get_size(&self) -> Size { self.size }
    fn intrinsic_width(&self, _height: f32) -> Option<f32> { Some(self.size.width) }
    fn intrinsic_height(&self, _width: f32) -> Option<f32> { Some(self.size.height) }
}

// =============================================================================
// BAR CHART
// =============================================================================

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BarOrientation { #[default] Vertical, Horizontal }

pub struct BarChart {
    series: Vec<DataSeries>,
    config: ChartConfig,
    position: Vec2,
    size: Size,
    orientation: BarOrientation,
    bar_width: f32,
    bar_gap: f32,
}

impl BarChart {
    pub fn new() -> Self {
        Self {
            series: Vec::new(),
            config: ChartConfig::default(),
            position: Vec2::ZERO,
            size: Size::new(300.0, 200.0),
            orientation: BarOrientation::Vertical,
            bar_width: 30.0,
            bar_gap: 10.0,
        }
    }
    
    pub fn with_data(mut self, name: &str, values: &[f64]) -> Self {
        self.series.push(DataSeries::from_values(name, values));
        self
    }
    
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.size = Size::new(width, height);
        self
    }
    
    pub fn horizontal(mut self) -> Self { self.orientation = BarOrientation::Horizontal; self }
    
    fn get_max(&self) -> f64 {
        self.series.iter().flat_map(|s| s.data.iter()).map(|d| d.value).fold(0.0, f64::max)
    }
}

impl Default for BarChart { fn default() -> Self { Self::new() } }

impl Widget for BarChart {
    fn layout(&mut self, _origin: Vec2, _available: Vec2) -> Vec2 {
        Vec2::new(self.size.width, self.size.height)
    }
    
    fn handle_event(&mut self, _event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool { false }
    fn update(&mut self, _dt: f32) {}
    
    fn render(&self, renderer: &mut GlassRenderer) {
        let pos = self.position;
        let w = self.size.width;
        let h = self.size.height;
        let pad = self.config.padding;
        
        renderer.draw_rounded_rect(pos, Vec2::new(w, h), self.config.background, 8.0);
        
        let chart_pos = pos + Vec2::new(pad, pad);
        let chart_w = w - pad * 2.0;
        let chart_h = h - pad * 2.0;
        
        let max_val = self.get_max();
        if max_val <= 0.0 { return; }
        
        for series in &self.series {
            let bar_count = series.data.len();
            if bar_count == 0 { continue; }
            
            match self.orientation {
                BarOrientation::Vertical => {
                    let total_w = bar_count as f32 * (self.bar_width + self.bar_gap) - self.bar_gap;
                    let start_x = chart_pos.x + (chart_w - total_w) / 2.0;
                    for (i, point) in series.data.iter().enumerate() {
                        let bar_x = start_x + i as f32 * (self.bar_width + self.bar_gap);
                        let bar_h = (point.value / max_val) as f32 * chart_h;
                        let color = point.color.unwrap_or(series.color);
                        renderer.draw_rounded_rect(Vec2::new(bar_x, chart_pos.y + chart_h - bar_h), Vec2::new(self.bar_width, bar_h), color, 4.0);
                    }
                }
                BarOrientation::Horizontal => {
                    let total_h = bar_count as f32 * (self.bar_width + self.bar_gap) - self.bar_gap;
                    let start_y = chart_pos.y + (chart_h - total_h) / 2.0;
                    for (i, point) in series.data.iter().enumerate() {
                        let bar_y = start_y + i as f32 * (self.bar_width + self.bar_gap);
                        let bar_w = (point.value / max_val) as f32 * chart_w;
                        let color = point.color.unwrap_or(series.color);
                        renderer.draw_rounded_rect(Vec2::new(chart_pos.x, bar_y), Vec2::new(bar_w, self.bar_width), color, 4.0);
                    }
                }
            }
        }
    }
    
    fn set_position(&mut self, pos: Offset) { self.position = Vec2::new(pos.x, pos.y); }
    fn get_position(&self) -> Offset { Offset::new(self.position.x, self.position.y) }
    fn get_size(&self) -> Size { self.size }
    fn intrinsic_width(&self, _height: f32) -> Option<f32> { Some(self.size.width) }
    fn intrinsic_height(&self, _width: f32) -> Option<f32> { Some(self.size.height) }
}

// =============================================================================
// PIE CHART
// =============================================================================

pub struct PieChart {
    data: Vec<DataPoint>,
    position: Vec2,
    size: Size,
    donut_ratio: f32,
    colors: Vec<Vec4>,
}

impl PieChart {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            position: Vec2::ZERO,
            size: Size::new(200.0, 200.0),
            donut_ratio: 0.0,
            colors: vec![
                Vec4::new(0.4, 0.6, 1.0, 1.0),
                Vec4::new(1.0, 0.4, 0.5, 1.0),
                Vec4::new(0.4, 0.9, 0.6, 1.0),
                Vec4::new(1.0, 0.7, 0.3, 1.0),
                Vec4::new(0.8, 0.5, 1.0, 1.0),
            ],
        }
    }
    
    pub fn with_values(mut self, values: &[f64]) -> Self {
        self.data = values.iter().map(|&v| DataPoint::new(v)).collect();
        self
    }
    
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = Size::new(size, size);
        self
    }
    
    pub fn donut(mut self, ratio: f32) -> Self {
        self.donut_ratio = ratio.clamp(0.0, 0.9);
        self
    }
    
    fn total(&self) -> f64 { self.data.iter().map(|d| d.value).sum() }
}

impl Default for PieChart { fn default() -> Self { Self::new() } }

impl Widget for PieChart {
    fn layout(&mut self, _origin: Vec2, _available: Vec2) -> Vec2 {
        Vec2::new(self.size.width, self.size.height)
    }
    
    fn handle_event(&mut self, _event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool { false }
    fn update(&mut self, _dt: f32) {}
    
    fn render(&self, renderer: &mut GlassRenderer) {
        let cx = self.position.x + self.size.width / 2.0;
        let cy = self.position.y + self.size.height / 2.0;
        let radius = self.size.width.min(self.size.height) / 2.0 - 10.0;
        
        let total = self.total();
        if total <= 0.0 { return; }
        
        let mut angle = -std::f32::consts::FRAC_PI_2;
        for (i, point) in self.data.iter().enumerate() {
            let slice_angle = (point.value / total) as f32 * std::f32::consts::TAU;
            let color = self.colors[i % self.colors.len()];
            let segments = ((slice_angle / 0.1) as i32).max(1);
            let seg_angle = slice_angle / segments as f32;
            
            for s in 0..segments {
                let mid_a = angle + (s as f32 + 0.5) * seg_angle;
                let inner_r = radius * self.donut_ratio;
                let r = (radius + inner_r) / 2.0;
                let px = cx + mid_a.cos() * r;
                let py = cy + mid_a.sin() * r;
                let seg_w = (radius - inner_r) * 0.7;
                renderer.draw_rounded_rect(Vec2::new(px - seg_w/2.0, py - seg_w/2.0), Vec2::new(seg_w, seg_w), color, seg_w/2.0);
            }
            angle += slice_angle;
        }
    }
    
    fn set_position(&mut self, pos: Offset) { self.position = Vec2::new(pos.x, pos.y); }
    fn get_position(&self) -> Offset { Offset::new(self.position.x, self.position.y) }
    fn get_size(&self) -> Size { self.size }
    fn intrinsic_width(&self, _height: f32) -> Option<f32> { Some(self.size.width) }
    fn intrinsic_height(&self, _width: f32) -> Option<f32> { Some(self.size.height) }
}

// =============================================================================
// SPARKLINE
// =============================================================================

pub struct Sparkline {
    data: Vec<f64>,
    position: Vec2,
    size: Size,
    color: Vec4,
}

impl Sparkline {
    pub fn new(data: Vec<f64>) -> Self {
        Self { data, position: Vec2::ZERO, size: Size::new(80.0, 24.0), color: Vec4::new(0.4, 0.8, 0.6, 1.0) }
    }
    
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.size = Size::new(width, height);
        self
    }
    
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }
}

impl Widget for Sparkline {
    fn layout(&mut self, _origin: Vec2, _available: Vec2) -> Vec2 {
        Vec2::new(self.size.width, self.size.height)
    }
    
    fn handle_event(&mut self, _event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool { false }
    fn update(&mut self, _dt: f32) {}
    
    fn render(&self, renderer: &mut GlassRenderer) {
        if self.data.is_empty() { return; }
        let pos = self.position;
        let w = self.size.width;
        let h = self.size.height;
        
        let min = self.data.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = self.data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let range = (max - min).max(0.001);
        let step = w / (self.data.len() - 1).max(1) as f32;
        
        for i in 0..self.data.len().saturating_sub(1) {
            let v1 = self.data[i];
            let v2 = self.data[i + 1];
            let x1 = pos.x + i as f32 * step;
            let x2 = pos.x + (i + 1) as f32 * step;
            let y1 = pos.y + h - ((v1 - min) / range) as f32 * h;
            let y2 = pos.y + h - ((v2 - min) / range) as f32 * h;
            let len = ((x2-x1).powi(2) + (y2-y1).powi(2)).sqrt();
            renderer.draw_rect(Vec2::new((x1+x2)/2.0 - len/2.0, (y1+y2)/2.0 - 1.0), Vec2::new(len, 2.0), self.color);
        }
    }
    
    fn set_position(&mut self, pos: Offset) { self.position = Vec2::new(pos.x, pos.y); }
    fn get_position(&self) -> Offset { Offset::new(self.position.x, self.position.y) }
    fn get_size(&self) -> Size { self.size }
    fn intrinsic_width(&self, _height: f32) -> Option<f32> { Some(self.size.width) }
    fn intrinsic_height(&self, _width: f32) -> Option<f32> { Some(self.size.height) }
}
