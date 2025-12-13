//! GlassUI Data Table Widget
//!
//! Tabular data display:
//! - Sortable columns
//! - Row selection
//! - Scrolling
//! - Cell rendering

use glam::{Vec2, Vec4};
use crate::renderer::GlassRenderer;
use crate::widget_id::WidgetId;
use crate::widgets::core::{Widget, get_theme};

// =============================================================================
// COLUMN
// =============================================================================

/// Sort direction
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
    None,
}

/// Table column definition
#[derive(Clone, Debug)]
pub struct GridColumn {
    pub id: String,
    pub label: String,
    pub width: f32,
    pub sortable: bool,
    pub sort_direction: SortDirection,
}

impl GridColumn {
    pub fn new(id: &str, label: &str, width: f32) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            width,
            sortable: true,
            sort_direction: SortDirection::None,
        }
    }
    
    pub fn not_sortable(mut self) -> Self {
        self.sortable = false;
        self
    }
}

// =============================================================================
// ROW AND CELL
// =============================================================================

/// A single cell value
#[derive(Clone, Debug)]
pub enum CellValue {
    Text(String),
    Number(f64),
    Bool(bool),
    Badge(String, Vec4),
}

impl CellValue {
    pub fn text(s: &str) -> Self {
        Self::Text(s.to_string())
    }
    
    pub fn number(n: f64) -> Self {
        Self::Number(n)
    }
    
    pub fn display(&self) -> String {
        match self {
            CellValue::Text(s) => s.clone(),
            CellValue::Number(n) => format!("{:.2}", n),
            CellValue::Bool(b) => if *b { "✓".to_string() } else { "✗".to_string() },
            CellValue::Badge(s, _) => s.clone(),
        }
    }
}

/// A table row of data
#[derive(Clone, Debug)]
pub struct GridRow {
    pub id: String,
    pub cells: Vec<CellValue>,
}

impl GridRow {
    pub fn new(id: &str, cells: Vec<CellValue>) -> Self {
        Self {
            id: id.to_string(),
            cells,
        }
    }
}

// =============================================================================
// DATA TABLE
// =============================================================================

/// Data table widget
pub struct DataTable {
    pub id: WidgetId,
    pub position: Vec2,
    pub size: Vec2,
    pub columns: Vec<GridColumn>,
    pub rows: Vec<GridRow>,
    pub selected_row: Option<usize>,
    pub hovered_row: Option<usize>,
    pub row_height: f32,
    pub header_height: f32,
    pub scroll_offset: f32,
    pub striped: bool,
    pub on_row_select: Option<Box<dyn FnMut(usize, &str)>>,
}

impl DataTable {
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(),
            position: Vec2::ZERO,
            size: Vec2::new(500.0, 300.0),
            columns: Vec::new(),
            rows: Vec::new(),
            selected_row: None,
            hovered_row: None,
            row_height: 32.0,
            header_height: 36.0,
            scroll_offset: 0.0,
            striped: true,
            on_row_select: None,
        }
    }
    
    /// Add a column
    pub fn add_column(&mut self, column: GridColumn) {
        self.columns.push(column);
    }
    
    /// Add a row
    pub fn add_row(&mut self, row: GridRow) {
        self.rows.push(row);
    }
    
    /// Set columns
    pub fn with_columns(mut self, columns: Vec<GridColumn>) -> Self {
        self.columns = columns;
        self
    }
    
    /// Set rows
    pub fn with_rows(mut self, rows: Vec<GridRow>) -> Self {
        self.rows = rows;
        self
    }
    
    /// Sort by column
    pub fn sort_by(&mut self, column_id: &str) {
        if let Some(col_idx) = self.columns.iter().position(|c| c.id == column_id) {
            let col = &mut self.columns[col_idx];
            if !col.sortable { return; }
            
            // Toggle direction
            col.sort_direction = match col.sort_direction {
                SortDirection::None | SortDirection::Descending => SortDirection::Ascending,
                SortDirection::Ascending => SortDirection::Descending,
            };
            
            // Clear other columns
            for (i, c) in self.columns.iter_mut().enumerate() {
                if i != col_idx {
                    c.sort_direction = SortDirection::None;
                }
            }
            
            // Sort rows
            let dir = self.columns[col_idx].sort_direction;
            self.rows.sort_by(|a, b| {
                let cell_a = a.cells.get(col_idx);
                let cell_b = b.cells.get(col_idx);
                
                let ordering = match (cell_a, cell_b) {
                    (Some(CellValue::Number(na)), Some(CellValue::Number(nb))) => {
                        na.partial_cmp(nb).unwrap_or(std::cmp::Ordering::Equal)
                    },
                    (Some(a), Some(b)) => a.display().cmp(&b.display()),
                    _ => std::cmp::Ordering::Equal,
                };
                
                if dir == SortDirection::Descending {
                    ordering.reverse()
                } else {
                    ordering
                }
            });
        }
    }
    
    /// Row at y position
    fn row_at_y(&self, y: f32) -> Option<usize> {
        let content_y = y - self.position.y - self.header_height + self.scroll_offset;
        if content_y < 0.0 { return None; }
        let row = (content_y / self.row_height) as usize;
        if row < self.rows.len() { Some(row) } else { None }
    }
    
    /// Create sample data
    pub fn sample() -> Self {
        let mut table = Self::new();
        table.columns = vec![
            GridColumn::new("name", "Name", 150.0),
            GridColumn::new("status", "Status", 100.0),
            GridColumn::new("progress", "Progress", 80.0),
            GridColumn::new("updated", "Updated", 120.0),
        ];
        table.rows = vec![
            GridRow::new("1", vec![
                CellValue::text("Dashboard"),
                CellValue::Badge("Active".to_string(), Vec4::new(0.3, 0.8, 0.4, 1.0)),
                CellValue::number(85.0),
                CellValue::text("2m ago"),
            ]),
            GridRow::new("2", vec![
                CellValue::text("API Server"),
                CellValue::Badge("Running".to_string(), Vec4::new(0.3, 0.6, 0.9, 1.0)),
                CellValue::number(100.0),
                CellValue::text("5m ago"),
            ]),
            GridRow::new("3", vec![
                CellValue::text("Database"),
                CellValue::Badge("Warning".to_string(), Vec4::new(0.9, 0.7, 0.2, 1.0)),
                CellValue::number(62.0),
                CellValue::text("1h ago"),
            ]),
        ];
        table
    }
}

impl Default for DataTable {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for DataTable {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = max_size;
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        // Update hover
        self.hovered_row = None;
        if mouse_pos.x >= self.position.x && mouse_pos.x <= self.position.x + self.size.x &&
           mouse_pos.y > self.position.y + self.header_height {
            self.hovered_row = self.row_at_y(mouse_pos.y);
        }
        
        // Handle click
        if let winit::event::Event::WindowEvent { 
            event: winit::event::WindowEvent::MouseInput { 
                state: winit::event::ElementState::Pressed,
                button: winit::event::MouseButton::Left,
                ..
            }, .. 
        } = event {
            // Check header click (sort)
            if mouse_pos.y >= self.position.y && mouse_pos.y <= self.position.y + self.header_height {
                let mut x = self.position.x;
                for col in &self.columns {
                    if mouse_pos.x >= x && mouse_pos.x <= x + col.width {
                        self.sort_by(&col.id.clone());
                        return true;
                    }
                    x += col.width;
                }
            }
            
            // Check row click
            if let Some(row) = self.hovered_row {
                self.selected_row = Some(row);
                if let Some(callback) = &mut self.on_row_select {
                    callback(row, &self.rows[row].id);
                }
                return true;
            }
        }
        
        // Handle scroll
        if let winit::event::Event::WindowEvent { 
            event: winit::event::WindowEvent::MouseWheel { delta, .. }, 
            .. 
        } = event {
            let scroll = match delta {
                winit::event::MouseScrollDelta::LineDelta(_, y) => *y * 30.0,
                winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
            };
            let max_scroll = (self.rows.len() as f32 * self.row_height - (self.size.y - self.header_height)).max(0.0);
            self.scroll_offset = (self.scroll_offset - scroll).clamp(0.0, max_scroll);
            return true;
        }
        
        false
    }

    fn update(&mut self, _dt: f32) {}

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();
        
        // Background
        renderer.draw_rounded_rect(self.position, self.size, Vec4::new(0.06, 0.06, 0.08, 0.9), 8.0);
        
        // Header
        renderer.draw_rounded_rect(
            self.position,
            Vec2::new(self.size.x, self.header_height),
            Vec4::new(0.1, 0.1, 0.12, 0.95),
            8.0
        );
        
        let mut x = self.position.x;
        for col in &self.columns {
            // Header text
            renderer.draw_text(&col.label, Vec2::new(x + 12.0, self.position.y + 10.0), 13.0, theme.text);
            
            // Sort indicator
            if col.sortable {
                let indicator = match col.sort_direction {
                    SortDirection::Ascending => "↑",
                    SortDirection::Descending => "↓",
                    SortDirection::None => "",
                };
                if !indicator.is_empty() {
                    renderer.draw_text(indicator, Vec2::new(x + col.width - 20.0, self.position.y + 10.0), 12.0, theme.primary);
                }
            }
            
            x += col.width;
        }
        
        // Rows
        let content_y = self.position.y + self.header_height;
        for (i, row) in self.rows.iter().enumerate() {
            let row_y = content_y + i as f32 * self.row_height - self.scroll_offset;
            
            // Skip if outside visible area
            if row_y + self.row_height < content_y || row_y > self.position.y + self.size.y {
                continue;
            }
            
            // Striped background
            if self.striped && i % 2 == 1 {
                renderer.draw_rounded_rect(
                    Vec2::new(self.position.x + 4.0, row_y),
                    Vec2::new(self.size.x - 8.0, self.row_height),
                    Vec4::new(1.0, 1.0, 1.0, 0.03),
                    4.0
                );
            }
            
            // Selection
            if self.selected_row == Some(i) {
                renderer.draw_rounded_rect(
                    Vec2::new(self.position.x + 4.0, row_y),
                    Vec2::new(self.size.x - 8.0, self.row_height),
                    Vec4::new(theme.primary.x, theme.primary.y, theme.primary.z, 0.25),
                    4.0
                );
            } else if self.hovered_row == Some(i) {
                renderer.draw_rounded_rect(
                    Vec2::new(self.position.x + 4.0, row_y),
                    Vec2::new(self.size.x - 8.0, self.row_height),
                    Vec4::new(1.0, 1.0, 1.0, 0.08),
                    4.0
                );
            }
            
            // Cells
            let mut cell_x = self.position.x;
            for (j, cell) in row.cells.iter().enumerate() {
                let col_width = self.columns.get(j).map(|c| c.width).unwrap_or(100.0);
                
                match cell {
                    CellValue::Badge(text, color) => {
                        // Badge background
                        let badge_w = text.len() as f32 * 7.0 + 12.0;
                        renderer.draw_rounded_rect(
                            Vec2::new(cell_x + 8.0, row_y + 6.0),
                            Vec2::new(badge_w, 20.0),
                            *color,
                            10.0
                        );
                        renderer.draw_text(text, Vec2::new(cell_x + 14.0, row_y + 8.0), 11.0, theme.text);
                    },
                    CellValue::Bool(b) => {
                        let icon = if *b { "✓" } else { "✗" };
                        let color = if *b { Vec4::new(0.3, 0.8, 0.4, 1.0) } else { Vec4::new(0.9, 0.3, 0.3, 1.0) };
                        renderer.draw_text(icon, Vec2::new(cell_x + 12.0, row_y + 8.0), 14.0, color);
                    },
                    _ => {
                        renderer.draw_text(&cell.display(), Vec2::new(cell_x + 12.0, row_y + 8.0), 12.0, theme.text_secondary);
                    }
                }
                
                cell_x += col_width;
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
    fn test_data_table() {
        let table = DataTable::sample();
        assert_eq!(table.columns.len(), 4);
        assert_eq!(table.rows.len(), 3);
    }
    
    #[test]
    fn test_sort() {
        let mut table = DataTable::sample();
        table.sort_by("progress");
        
        // Should be sorted ascending by progress
        if let CellValue::Number(n) = &table.rows[0].cells[2] {
            assert_eq!(*n, 62.0);  // Lowest first
        }
    }
}
