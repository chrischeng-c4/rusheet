use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::cell::{Cell, CellContent, CellValue};
use crate::chunk::ChunkedGrid;
use crate::range::CellCoord;
use crate::spatial::SpatialIndex;

/// Default row height in pixels
pub const DEFAULT_ROW_HEIGHT: f64 = 24.0;
/// Default column width in pixels
pub const DEFAULT_COL_WIDTH: f64 = 100.0;

/// A single spreadsheet sheet with sparse storage for cells
#[derive(Debug, Clone, Serialize)]
pub struct Sheet {
    /// Sheet name (displayed in tab)
    pub name: String,
    /// Sparse storage for cells using chunked grid - only non-empty cells are stored
    #[serde(default, with = "chunked_grid_serde")]
    cells: ChunkedGrid<Cell>,
    /// Custom row heights (row index -> height in pixels) - kept for serialization
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub row_heights: HashMap<u32, f64>,
    /// Custom column widths (column index -> width in pixels) - kept for serialization
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
    /// Spatial index for O(log N) position lookups (rebuilt on deserialize)
    #[serde(skip, default = "SpatialIndex::new")]
    spatial: SpatialIndex,
}

fn default_row_height() -> f64 {
    DEFAULT_ROW_HEIGHT
}

fn default_col_width() -> f64 {
    DEFAULT_COL_WIDTH
}

/// Custom serialization for ChunkedGrid to maintain backward compatibility
mod chunked_grid_serde {
    use super::*;
    use serde::ser::SerializeMap;
    use serde::{de, Deserializer, Serializer};
    use std::fmt;

    pub fn serialize<S>(grid: &ChunkedGrid<Cell>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as a map with stringified tuple keys "row,col" for JSON compatibility
        let mut map = serializer.serialize_map(Some(grid.len()))?;
        for ((row, col), cell) in grid.iter() {
            let key = format!("{},{}", row, col);
            map.serialize_entry(&key, cell)?;
        }
        map.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<ChunkedGrid<Cell>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GridVisitor;

        impl<'de> de::Visitor<'de> for GridVisitor {
            type Value = ChunkedGrid<Cell>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map with coordinate keys")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut grid = ChunkedGrid::new();

                while let Some(key) = map.next_key::<String>()? {
                    let cell: Cell = map.next_value()?;

                    // Parse "row,col" format
                    let parts: Vec<&str> = key.split(',').collect();
                    if parts.len() == 2 {
                        if let (Ok(row), Ok(col)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                            grid.insert(row as usize, col as usize, cell);
                        }
                    }
                }

                Ok(grid)
            }
        }

        deserializer.deserialize_map(GridVisitor)
    }
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
            cells: ChunkedGrid::new(),
            row_heights: HashMap::new(),
            col_widths: HashMap::new(),
            default_row_height: DEFAULT_ROW_HEIGHT,
            default_col_width: DEFAULT_COL_WIDTH,
            frozen_rows: 0,
            frozen_cols: 0,
            spatial: SpatialIndex::new(),
        }
    }

    /// Get a reference to a cell at the given coordinate
    pub fn get_cell(&self, coord: CellCoord) -> Option<&Cell> {
        self.cells.get(coord.row as usize, coord.col as usize)
    }

    /// Get a mutable reference to a cell, creating it if it doesn't exist
    pub fn get_cell_mut(&mut self, coord: CellCoord) -> &mut Cell {
        let row = coord.row as usize;
        let col = coord.col as usize;

        // Check if cell exists, if not insert default
        if self.cells.get(row, col).is_none() {
            self.cells.insert(row, col, Cell::default());
        }

        // Now we can safely get mutable reference
        self.cells.get_mut(row, col).expect("Cell was just inserted")
    }

    /// Set a cell at the given coordinate
    pub fn set_cell(&mut self, coord: CellCoord, cell: Cell) {
        let row = coord.row as usize;
        let col = coord.col as usize;

        if cell.is_empty() {
            // Remove empty cells to save memory
            self.cells.remove(row, col);
        } else {
            self.cells.insert(row, col, cell);
        }
    }

    /// Set the value of a cell (parses input to determine type)
    pub fn set_cell_value(&mut self, coord: CellCoord, value: &str) {
        let content = parse_cell_input(value);
        let cell = self.get_cell_mut(coord);
        cell.content = content;

        // Clean up if cell became empty
        if cell.is_empty() {
            self.cells.remove(coord.row as usize, coord.col as usize);
        }
    }

    /// Remove a cell (make it empty)
    pub fn remove_cell(&mut self, coord: CellCoord) {
        self.cells.remove(coord.row as usize, coord.col as usize);
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
            // Reset to default in spatial index
            self.spatial.set_row_height(row as usize, self.default_row_height);
        } else {
            self.row_heights.insert(row, height);
            // Update spatial index
            self.spatial.set_row_height(row as usize, height);
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
            // Reset to default in spatial index
            self.spatial.set_col_width(col as usize, self.default_col_width);
        } else {
            self.col_widths.insert(col, width);
            // Update spatial index
            self.spatial.set_col_width(col as usize, width);
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
            .iter()
            .map(|((row, col), _)| CellCoord::new(row as u32, col as u32))
    }

    /// Get all cells in a range
    pub fn get_cells_in_range(
        &self,
        start: CellCoord,
        end: CellCoord,
    ) -> Vec<(CellCoord, &Cell)> {
        let min_row = start.row.min(end.row) as usize;
        let max_row = start.row.max(end.row) as usize;
        let min_col = start.col.min(end.col) as usize;
        let max_col = start.col.max(end.col) as usize;

        self.cells
            .cells_in_range(min_row, min_col, max_row, max_col)
            .into_iter()
            .map(|((row, col), cell)| (CellCoord::new(row as u32, col as u32), cell))
            .collect()
    }

    /// Get the bounding box of non-empty cells
    pub fn used_range(&self) -> Option<(CellCoord, CellCoord)> {
        if self.cells.is_empty() {
            return None;
        }

        let mut min_row = usize::MAX;
        let mut max_row = 0;
        let mut min_col = usize::MAX;
        let mut max_col = 0;

        for ((row, col), _) in self.cells.iter() {
            min_row = min_row.min(row);
            max_row = max_row.max(row);
            min_col = min_col.min(col);
            max_col = max_col.max(col);
        }

        Some((
            CellCoord::new(min_row as u32, min_col as u32),
            CellCoord::new(max_row as u32, max_col as u32),
        ))
    }

    /// Calculate Y position of a row (sum of heights of all rows above)
    pub fn row_y_position(&self, row: u32) -> f64 {
        self.spatial.get_row_offset(row as usize)
    }

    /// Calculate X position of a column (sum of widths of all columns before)
    pub fn col_x_position(&self, col: u32) -> f64 {
        self.spatial.get_col_offset(col as usize)
    }

    /// Find row at given Y position
    pub fn row_at_y(&self, y: f64) -> u32 {
        self.spatial.find_row_at_offset(y) as u32
    }

    /// Find column at given X position
    pub fn col_at_x(&self, x: f64) -> u32 {
        self.spatial.find_col_at_offset(x) as u32
    }

    /// Rebuild the spatial index from row_heights and col_widths
    /// Called after deserialization to reconstruct the runtime index
    fn rebuild_spatial_index(&mut self) {
        self.spatial = SpatialIndex::new();

        // Apply custom row heights
        for (&row, &height) in &self.row_heights {
            self.spatial.set_row_height(row as usize, height);
        }

        // Apply custom column widths
        for (&col, &width) in &self.col_widths {
            self.spatial.set_col_width(col as usize, width);
        }
    }
}

// Custom Deserialize implementation to rebuild spatial index after deserialization
impl<'de> Deserialize<'de> for Sheet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Helper struct with same fields for deserialization
        #[derive(Deserialize)]
        struct SheetHelper {
            name: String,
            #[serde(default, with = "chunked_grid_serde")]
            cells: ChunkedGrid<Cell>,
            #[serde(default)]
            row_heights: HashMap<u32, f64>,
            #[serde(default)]
            col_widths: HashMap<u32, f64>,
            #[serde(default = "default_row_height")]
            default_row_height: f64,
            #[serde(default = "default_col_width")]
            default_col_width: f64,
            #[serde(default)]
            frozen_rows: u32,
            #[serde(default)]
            frozen_cols: u32,
        }

        let helper = SheetHelper::deserialize(deserializer)?;

        let mut sheet = Sheet {
            name: helper.name,
            cells: helper.cells,
            row_heights: helper.row_heights,
            col_widths: helper.col_widths,
            default_row_height: helper.default_row_height,
            default_col_width: helper.default_col_width,
            frozen_rows: helper.frozen_rows,
            frozen_cols: helper.frozen_cols,
            spatial: SpatialIndex::new(),
        };

        // Rebuild the spatial index from the deserialized data
        sheet.rebuild_spatial_index();

        Ok(sheet)
    }
}

/// Parse user input to determine cell content type
pub fn parse_cell_input(input: &str) -> CellContent {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return CellContent::Value {
            value: CellValue::Empty,
            original_input: None,
        };
    }

    // Formulas: store expression
    if trimmed.starts_with('=') {
        return CellContent::Formula {
            expression: trimmed.to_string(),
            cached_value: CellValue::Empty,
        };
    }

    // Store original input for all value types
    let original = trimmed.to_string();

    // Boolean
    match trimmed.to_uppercase().as_str() {
        "TRUE" => return CellContent::Value {
            value: CellValue::Boolean(true),
            original_input: Some(original),
        },
        "FALSE" => return CellContent::Value {
            value: CellValue::Boolean(false),
            original_input: Some(original),
        },
        _ => {}
    }

    // Number
    if let Ok(num) = trimmed.parse::<f64>() {
        return CellContent::Value {
            value: CellValue::Number(num),
            original_input: Some(original),
        };
    }

    // Percentage
    if trimmed.ends_with('%') {
        if let Ok(num) = trimmed[..trimmed.len() - 1].parse::<f64>() {
            return CellContent::Value {
                value: CellValue::Number(num / 100.0),
                original_input: Some(original),
            };
        }
    }

    // Text
    CellContent::Value {
        value: CellValue::Text(trimmed.to_string()),
        original_input: Some(original),
    }
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
        assert!(matches!(content, CellContent::Value { value: CellValue::Number(n), .. } if n == 42.0));

        // Float
        let content = parse_cell_input("3.14");
        assert!(matches!(content, CellContent::Value { value: CellValue::Number(n), .. } if (n - 3.14).abs() < 0.001));

        // Boolean
        let content = parse_cell_input("TRUE");
        assert!(matches!(content, CellContent::Value { value: CellValue::Boolean(true), .. }));

        // Formula
        let content = parse_cell_input("=SUM(A1:A10)");
        assert!(matches!(content, CellContent::Formula { expression, .. } if expression == "=SUM(A1:A10)"));

        // Text
        let content = parse_cell_input("Hello");
        assert!(matches!(content, CellContent::Value { value: CellValue::Text(s), .. } if s == "Hello"));

        // Percentage
        let content = parse_cell_input("50%");
        assert!(matches!(content, CellContent::Value { value: CellValue::Number(n), .. } if (n - 0.5).abs() < 0.001));
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

    #[test]
    fn test_spatial_index_performance() {
        let mut sheet = Sheet::new("Test");

        // Set custom row heights
        sheet.set_row_height(0, 30.0);
        sheet.set_row_height(1, 40.0);
        sheet.set_row_height(2, 50.0);

        // Test row position lookups (O(log N) with spatial index)
        assert_eq!(sheet.row_y_position(0), 0.0);
        assert_eq!(sheet.row_y_position(1), 30.0);
        assert_eq!(sheet.row_y_position(2), 70.0);
        assert_eq!(sheet.row_y_position(3), 120.0);

        // Test finding row at Y position
        assert_eq!(sheet.row_at_y(0.0), 0);
        assert_eq!(sheet.row_at_y(29.0), 0);
        assert_eq!(sheet.row_at_y(30.0), 0);
        assert_eq!(sheet.row_at_y(31.0), 1);
        assert_eq!(sheet.row_at_y(69.0), 1);
        assert_eq!(sheet.row_at_y(70.0), 1);
        assert_eq!(sheet.row_at_y(71.0), 2);

        // Set custom column widths
        sheet.set_col_width(0, 100.0);
        sheet.set_col_width(1, 150.0);
        sheet.set_col_width(2, 200.0);

        // Test column position lookups
        assert_eq!(sheet.col_x_position(0), 0.0);
        assert_eq!(sheet.col_x_position(1), 100.0);
        assert_eq!(sheet.col_x_position(2), 250.0);
        assert_eq!(sheet.col_x_position(3), 450.0);

        // Test finding column at X position
        assert_eq!(sheet.col_at_x(0.0), 0);
        assert_eq!(sheet.col_at_x(99.0), 0);
        assert_eq!(sheet.col_at_x(100.0), 0);
        assert_eq!(sheet.col_at_x(101.0), 1);
        assert_eq!(sheet.col_at_x(249.0), 1);
        assert_eq!(sheet.col_at_x(250.0), 1);
        assert_eq!(sheet.col_at_x(251.0), 2);
    }

    #[test]
    fn test_chunked_grid_integration() {
        let mut sheet = Sheet::new("Test");

        // Insert cells in different chunks
        for row in 0..100 {
            for col in 0..100 {
                if (row + col) % 10 == 0 {
                    sheet.set_cell(CellCoord::new(row, col), Cell::number(row as f64 + col as f64));
                }
            }
        }

        // Verify cell count
        let expected_count = (0..100)
            .flat_map(|r| (0..100).map(move |c| (r, c)))
            .filter(|(r, c)| (r + c) % 10 == 0)
            .count();
        assert_eq!(sheet.cell_count(), expected_count);

        // Test range queries (should be efficient with chunked storage)
        let cells = sheet.get_cells_in_range(CellCoord::new(10, 10), CellCoord::new(30, 30));
        assert!(!cells.is_empty());

        // Verify cell retrieval
        assert_eq!(
            sheet.get_cell(CellCoord::new(10, 10)).unwrap().computed_value().as_number(),
            Some(20.0)
        );

        // Test removal
        sheet.remove_cell(CellCoord::new(10, 10));
        assert!(sheet.get_cell(CellCoord::new(10, 10)).is_none());
    }

    #[test]
    fn test_spatial_index_rebuild() {
        // Test that spatial index is properly rebuilt with custom dimensions
        let mut sheet = Sheet::new("Test");

        sheet.set_row_height(0, 30.0);
        sheet.set_row_height(5, 50.0);
        sheet.set_col_width(0, 150.0);
        sheet.set_col_width(3, 200.0);

        // Verify spatial index works correctly with custom dimensions
        assert_eq!(sheet.row_y_position(0), 0.0);
        assert_eq!(sheet.row_y_position(1), 30.0);
        assert_eq!(sheet.row_y_position(5), 30.0 + 24.0 * 4.0); // 30 + 4 default rows
        assert_eq!(sheet.row_y_position(6), 30.0 + 24.0 * 4.0 + 50.0);

        assert_eq!(sheet.col_x_position(0), 0.0);
        assert_eq!(sheet.col_x_position(1), 150.0);
        assert_eq!(sheet.col_x_position(3), 150.0 + 100.0 * 2.0); // 150 + 2 default cols
        assert_eq!(sheet.col_x_position(4), 150.0 + 100.0 * 2.0 + 200.0);

        // Test row/col finding
        assert_eq!(sheet.row_at_y(0.0), 0);
        assert_eq!(sheet.row_at_y(30.0), 0);
        assert_eq!(sheet.row_at_y(31.0), 1);

        assert_eq!(sheet.col_at_x(0.0), 0);
        assert_eq!(sheet.col_at_x(150.0), 0);
        assert_eq!(sheet.col_at_x(151.0), 1);
    }
}
