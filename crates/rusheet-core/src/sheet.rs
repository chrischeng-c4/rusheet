use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::cell::{Cell, CellContent, CellValue};
use crate::range::CellCoord;

/// Default row height in pixels
pub const DEFAULT_ROW_HEIGHT: f64 = 21.0;
/// Default column width in pixels
pub const DEFAULT_COL_WIDTH: f64 = 100.0;

/// A single spreadsheet sheet with sparse storage for cells
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sheet {
    /// Sheet name (displayed in tab)
    pub name: String,
    /// Sparse storage for cells - only non-empty cells are stored
    #[serde(default)]
    cells: HashMap<(u32, u32), Cell>,
    /// Custom row heights (row index -> height in pixels)
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub row_heights: HashMap<u32, f64>,
    /// Custom column widths (column index -> width in pixels)
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub col_widths: HashMap<u32, f64>,
    /// Default height for rows without custom height
    #[serde(default = "default_row_height")]
    pub default_row_height: f64,
    /// Default width for columns without custom width
    #[serde(default = "default_col_width")]
    pub default_col_width: f64,
    /// Number of frozen rows (scroll lock)
    #[serde(default)]
    pub frozen_rows: u32,
    /// Number of frozen columns (scroll lock)
    #[serde(default)]
    pub frozen_cols: u32,
}

fn default_row_height() -> f64 {
    DEFAULT_ROW_HEIGHT
}

fn default_col_width() -> f64 {
    DEFAULT_COL_WIDTH
}

impl Sheet {
    /// Maximum number of rows (Excel compatibility)
    pub const MAX_ROWS: u32 = 1_048_576;
    /// Maximum number of columns (Column XFD)
    pub const MAX_COLS: u32 = 16_384;

    /// Create a new empty sheet with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            cells: HashMap::new(),
            row_heights: HashMap::new(),
            col_widths: HashMap::new(),
            default_row_height: DEFAULT_ROW_HEIGHT,
            default_col_width: DEFAULT_COL_WIDTH,
            frozen_rows: 0,
            frozen_cols: 0,
        }
    }

    /// Get a reference to a cell at the given coordinate
    pub fn get_cell(&self, coord: CellCoord) -> Option<&Cell> {
        self.cells.get(&(coord.row, coord.col))
    }

    /// Get a mutable reference to a cell, creating it if it doesn't exist
    pub fn get_cell_mut(&mut self, coord: CellCoord) -> &mut Cell {
        self.cells
            .entry((coord.row, coord.col))
            .or_insert_with(Cell::default)
    }

    /// Set a cell at the given coordinate
    pub fn set_cell(&mut self, coord: CellCoord, cell: Cell) {
        if cell.is_empty() {
            // Remove empty cells to save memory
            self.cells.remove(&(coord.row, coord.col));
        } else {
            self.cells.insert((coord.row, coord.col), cell);
        }
    }

    /// Set the value of a cell (parses input to determine type)
    pub fn set_cell_value(&mut self, coord: CellCoord, value: &str) {
        let content = parse_cell_input(value);
        let cell = self.get_cell_mut(coord);
        cell.content = content;

        // Clean up if cell became empty
        if cell.is_empty() {
            self.cells.remove(&(coord.row, coord.col));
        }
    }

    /// Remove a cell (make it empty)
    pub fn remove_cell(&mut self, coord: CellCoord) {
        self.cells.remove(&(coord.row, coord.col));
    }

    /// Get the computed value of a cell (returns Empty for non-existent cells)
    pub fn get_cell_value(&self, coord: CellCoord) -> &CellValue {
        self.get_cell(coord)
            .map(|c| c.computed_value())
            .unwrap_or(&CellValue::Empty)
    }

    /// Get the row height for a specific row
    pub fn get_row_height(&self, row: u32) -> f64 {
        *self
            .row_heights
            .get(&row)
            .unwrap_or(&self.default_row_height)
    }

    /// Set the row height for a specific row
    pub fn set_row_height(&mut self, row: u32, height: f64) {
        if (height - self.default_row_height).abs() < 0.01 {
            self.row_heights.remove(&row);
        } else {
            self.row_heights.insert(row, height);
        }
    }

    /// Get the column width for a specific column
    pub fn get_col_width(&self, col: u32) -> f64 {
        *self.col_widths.get(&col).unwrap_or(&self.default_col_width)
    }

    /// Set the column width for a specific column
    pub fn set_col_width(&mut self, col: u32, width: f64) {
        if (width - self.default_col_width).abs() < 0.01 {
            self.col_widths.remove(&col);
        } else {
            self.col_widths.insert(col, width);
        }
    }

    /// Get the number of non-empty cells
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// Check if the sheet is empty
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Get all non-empty cell coordinates
    pub fn non_empty_coords(&self) -> impl Iterator<Item = CellCoord> + '_ {
        self.cells
            .keys()
            .map(|(row, col)| CellCoord::new(*row, *col))
    }

    /// Get all cells in a range
    pub fn get_cells_in_range(
        &self,
        start: CellCoord,
        end: CellCoord,
    ) -> impl Iterator<Item = (CellCoord, &Cell)> {
        let min_row = start.row.min(end.row);
        let max_row = start.row.max(end.row);
        let min_col = start.col.min(end.col);
        let max_col = start.col.max(end.col);

        self.cells.iter().filter_map(move |((row, col), cell)| {
            if *row >= min_row && *row <= max_row && *col >= min_col && *col <= max_col {
                Some((CellCoord::new(*row, *col), cell))
            } else {
                None
            }
        })
    }

    /// Get the bounding box of non-empty cells
    pub fn used_range(&self) -> Option<(CellCoord, CellCoord)> {
        if self.cells.is_empty() {
            return None;
        }

        let mut min_row = u32::MAX;
        let mut max_row = 0;
        let mut min_col = u32::MAX;
        let mut max_col = 0;

        for (row, col) in self.cells.keys() {
            min_row = min_row.min(*row);
            max_row = max_row.max(*row);
            min_col = min_col.min(*col);
            max_col = max_col.max(*col);
        }

        Some((
            CellCoord::new(min_row, min_col),
            CellCoord::new(max_row, max_col),
        ))
    }

    /// Calculate Y position of a row (sum of heights of all rows above)
    pub fn row_y_position(&self, row: u32) -> f64 {
        let mut y = 0.0;
        for r in 0..row {
            y += self.get_row_height(r);
        }
        y
    }

    /// Calculate X position of a column (sum of widths of all columns before)
    pub fn col_x_position(&self, col: u32) -> f64 {
        let mut x = 0.0;
        for c in 0..col {
            x += self.get_col_width(c);
        }
        x
    }

    /// Find row at given Y position
    pub fn row_at_y(&self, y: f64) -> u32 {
        let mut current_y = 0.0;
        let mut row = 0;

        while current_y < y && row < Self::MAX_ROWS {
            current_y += self.get_row_height(row);
            if current_y > y {
                break;
            }
            row += 1;
        }

        row
    }

    /// Find column at given X position
    pub fn col_at_x(&self, x: f64) -> u32 {
        let mut current_x = 0.0;
        let mut col = 0;

        while current_x < x && col < Self::MAX_COLS {
            current_x += self.get_col_width(col);
            if current_x > x {
                break;
            }
            col += 1;
        }

        col
    }
}

/// Parse user input to determine cell content type
pub fn parse_cell_input(input: &str) -> CellContent {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return CellContent::Value(CellValue::Empty);
    }

    // Check for formula
    if trimmed.starts_with('=') {
        return CellContent::Formula {
            expression: trimmed.to_string(),
            cached_value: CellValue::Empty, // Will be calculated later
        };
    }

    // Check for boolean
    match trimmed.to_uppercase().as_str() {
        "TRUE" => return CellContent::Value(CellValue::Boolean(true)),
        "FALSE" => return CellContent::Value(CellValue::Boolean(false)),
        _ => {}
    }

    // Check for number
    if let Ok(num) = trimmed.parse::<f64>() {
        return CellContent::Value(CellValue::Number(num));
    }

    // Check for percentage (e.g., "50%")
    if trimmed.ends_with('%') {
        if let Ok(num) = trimmed[..trimmed.len() - 1].parse::<f64>() {
            return CellContent::Value(CellValue::Number(num / 100.0));
        }
    }

    // Default to text
    CellContent::Value(CellValue::Text(trimmed.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sheet_basic_operations() {
        let mut sheet = Sheet::new("Test");

        // Set a cell
        let coord = CellCoord::new(0, 0);
        sheet.set_cell(coord, Cell::number(42.0));

        // Get the cell
        let cell = sheet.get_cell(coord).unwrap();
        assert_eq!(cell.computed_value().as_number(), Some(42.0));

        // Remove the cell
        sheet.remove_cell(coord);
        assert!(sheet.get_cell(coord).is_none());
    }

    #[test]
    fn test_parse_cell_input() {
        // Number
        let content = parse_cell_input("42");
        assert!(matches!(content, CellContent::Value(CellValue::Number(n)) if n == 42.0));

        // Float
        let content = parse_cell_input("3.14");
        assert!(matches!(content, CellContent::Value(CellValue::Number(n)) if (n - 3.14).abs() < 0.001));

        // Boolean
        let content = parse_cell_input("TRUE");
        assert!(matches!(content, CellContent::Value(CellValue::Boolean(true))));

        // Formula
        let content = parse_cell_input("=SUM(A1:A10)");
        assert!(matches!(content, CellContent::Formula { expression, .. } if expression == "=SUM(A1:A10)"));

        // Text
        let content = parse_cell_input("Hello");
        assert!(matches!(content, CellContent::Value(CellValue::Text(s)) if s == "Hello"));

        // Percentage
        let content = parse_cell_input("50%");
        assert!(matches!(content, CellContent::Value(CellValue::Number(n)) if (n - 0.5).abs() < 0.001));
    }

    #[test]
    fn test_row_col_dimensions() {
        let mut sheet = Sheet::new("Test");

        // Default dimensions
        assert_eq!(sheet.get_row_height(0), DEFAULT_ROW_HEIGHT);
        assert_eq!(sheet.get_col_width(0), DEFAULT_COL_WIDTH);

        // Custom dimensions
        sheet.set_row_height(5, 30.0);
        sheet.set_col_width(3, 150.0);

        assert_eq!(sheet.get_row_height(5), 30.0);
        assert_eq!(sheet.get_col_width(3), 150.0);
    }

    #[test]
    fn test_used_range() {
        let mut sheet = Sheet::new("Test");

        assert!(sheet.used_range().is_none());

        sheet.set_cell(CellCoord::new(1, 1), Cell::number(1.0));
        sheet.set_cell(CellCoord::new(5, 10), Cell::number(2.0));

        let (start, end) = sheet.used_range().unwrap();
        assert_eq!(start, CellCoord::new(1, 1));
        assert_eq!(end, CellCoord::new(5, 10));
    }
}
