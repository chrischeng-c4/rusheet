use rusheet_core::{CellContent, CellCoord, CellFormat, CellValue, Workbook};
use rusheet_formula::{evaluate_formula, extract_references, DependencyGraph};
use rusheet_history::{
    ClearRangeCommand, HistoryManager, SetCellFormatCommand, SetCellValueCommand,
    SetRangeFormatCommand,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

/// Main spreadsheet engine exposed to JavaScript
#[wasm_bindgen]
pub struct SpreadsheetEngine {
    workbook: Workbook,
    dep_graph: DependencyGraph,
    history: HistoryManager,
}

/// Cell data for JavaScript
#[derive(Serialize, Deserialize)]
pub struct CellData {
    pub value: Option<String>,
    pub display_value: String,
    pub formula: Option<String>,
    pub format: CellFormatData,
    pub row: u32,
    pub col: u32,
}

/// Cell format data for JavaScript
#[derive(Serialize, Deserialize, Default)]
pub struct CellFormatData {
    #[serde(skip_serializing_if = "is_false")]
    pub bold: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub italic: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub underline: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_align: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_align: Option<String>,
}

fn is_false(b: &bool) -> bool {
    !*b
}

impl From<&CellFormat> for CellFormatData {
    fn from(format: &CellFormat) -> Self {
        use rusheet_core::{HorizontalAlign, VerticalAlign};

        CellFormatData {
            bold: format.bold,
            italic: format.italic,
            underline: format.underline,
            font_size: format.font_size,
            text_color: format.text_color.map(|c| c.to_hex()),
            background_color: format.background_color.map(|c| c.to_hex()),
            horizontal_align: match format.horizontal_align {
                HorizontalAlign::Left => None,
                HorizontalAlign::Center => Some("center".to_string()),
                HorizontalAlign::Right => Some("right".to_string()),
            },
            vertical_align: match format.vertical_align {
                VerticalAlign::Middle => None,
                VerticalAlign::Top => Some("top".to_string()),
                VerticalAlign::Bottom => Some("bottom".to_string()),
            },
        }
    }
}

#[wasm_bindgen]
impl SpreadsheetEngine {
    /// Create a new spreadsheet engine
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            workbook: Workbook::new("Untitled"),
            dep_graph: DependencyGraph::new(),
            history: HistoryManager::new(100),
        }
    }

    /// Set cell value (handles both plain values and formulas)
    /// Returns JSON array of affected cell coordinates for re-render
    #[wasm_bindgen(js_name = setCellValue)]
    pub fn set_cell_value(&mut self, row: u32, col: u32, value: &str) -> String {
        let coord = CellCoord::new(row, col);
        let cmd = Box::new(SetCellValueCommand::from_input(coord, value));

        // Execute command
        let mut affected = self.history.execute(cmd, self.workbook.active_sheet_mut());

        // Update dependency graph and recalculate if this is a formula
        if value.starts_with('=') {
            let refs = extract_references(value);
            let deps: HashSet<_> = refs.into_iter().map(|(r, c)| (r, c)).collect();
            self.dep_graph.set_dependencies((row, col), deps);

            // Recalculate this cell and dependents
            if let Ok(order) = self.dep_graph.get_recalc_order((row, col)) {
                for (r, c) in order {
                    self.recalculate_cell(r, c);
                    affected.push(CellCoord::new(r, c));
                }
            }
        } else {
            // Clear dependencies for non-formula cells
            self.dep_graph.remove_cell((row, col));

            // Recalculate cells that depend on this one
            if let Ok(order) = self.dep_graph.get_recalc_order((row, col)) {
                for (r, c) in order {
                    if (r, c) != (row, col) {
                        self.recalculate_cell(r, c);
                        affected.push(CellCoord::new(r, c));
                    }
                }
            }
        }

        // Return affected cells as JSON
        let coords: Vec<[u32; 2]> = affected.iter().map(|c| [c.row, c.col]).collect();
        serde_json::to_string(&coords).unwrap_or_else(|_| "[]".to_string())
    }

    /// Recalculate a single cell's formula
    fn recalculate_cell(&mut self, row: u32, col: u32) {
        let coord = CellCoord::new(row, col);
        let sheet = self.workbook.active_sheet();

        if let Some(cell) = sheet.get_cell(coord) {
            if let CellContent::Formula { expression, .. } = &cell.content {
                let expression = expression.clone();

                // Create closure to get cell values
                let sheet = self.workbook.active_sheet();
                let result = evaluate_formula(&expression, |r, c| {
                    sheet
                        .get_cell(CellCoord::new(r, c))
                        .map(|c| c.computed_value().clone())
                        .unwrap_or(CellValue::Empty)
                });

                // Update cached value
                let sheet = self.workbook.active_sheet_mut();
                if let Some(cell) = sheet.get_cell(coord) {
                    let new_content = CellContent::Formula {
                        expression,
                        cached_value: result,
                    };
                    let mut new_cell = cell.clone();
                    new_cell.content = new_content;
                    sheet.set_cell(coord, new_cell);
                }
            }
        }
    }

    /// Get cell data for rendering
    #[wasm_bindgen(js_name = getCellData)]
    pub fn get_cell_data(&self, row: u32, col: u32) -> JsValue {
        let coord = CellCoord::new(row, col);
        let sheet = self.workbook.active_sheet();

        let data = if let Some(cell) = sheet.get_cell(coord) {
            CellData {
                value: Some(cell.computed_value().as_text()),
                display_value: cell.content.display_value(),
                formula: cell.content.formula_expression().map(String::from),
                format: CellFormatData::from(&cell.format),
                row,
                col,
            }
        } else {
            CellData {
                value: None,
                display_value: String::new(),
                formula: None,
                format: CellFormatData::default(),
                row,
                col,
            }
        };

        serde_wasm_bindgen::to_value(&data).unwrap_or(JsValue::NULL)
    }

    /// Get visible cells for viewport (virtual scrolling)
    #[wasm_bindgen(js_name = getViewportData)]
    pub fn get_viewport_data(
        &self,
        start_row: u32,
        end_row: u32,
        start_col: u32,
        end_col: u32,
    ) -> String {
        let sheet = self.workbook.active_sheet();
        let mut cells: Vec<CellData> = Vec::new();

        for row in start_row..=end_row {
            for col in start_col..=end_col {
                let coord = CellCoord::new(row, col);
                if let Some(cell) = sheet.get_cell(coord) {
                    cells.push(CellData {
                        value: Some(cell.computed_value().as_text()),
                        display_value: cell.content.display_value(),
                        formula: cell.content.formula_expression().map(String::from),
                        format: CellFormatData::from(&cell.format),
                        row,
                        col,
                    });
                }
            }
        }

        serde_json::to_string(&cells).unwrap_or_else(|_| "[]".to_string())
    }

    /// Set cell format
    #[wasm_bindgen(js_name = setCellFormat)]
    pub fn set_cell_format(&mut self, row: u32, col: u32, format_json: &str) -> bool {
        let format_data: CellFormatData = match serde_json::from_str(format_json) {
            Ok(f) => f,
            Err(_) => return false,
        };

        let coord = CellCoord::new(row, col);
        let format = cell_format_from_data(&format_data);

        let cmd = Box::new(SetCellFormatCommand::new(coord, format));
        self.history.execute(cmd, self.workbook.active_sheet_mut());

        true
    }

    /// Apply format to range
    #[wasm_bindgen(js_name = setRangeFormat)]
    pub fn set_range_format(
        &mut self,
        start_row: u32,
        start_col: u32,
        end_row: u32,
        end_col: u32,
        format_json: &str,
    ) -> bool {
        let format_data: CellFormatData = match serde_json::from_str(format_json) {
            Ok(f) => f,
            Err(_) => return false,
        };

        let start = CellCoord::new(start_row, start_col);
        let end = CellCoord::new(end_row, end_col);
        let format = cell_format_from_data(&format_data);

        let cmd = Box::new(SetRangeFormatCommand::new(start, end, format));
        self.history.execute(cmd, self.workbook.active_sheet_mut());

        true
    }

    /// Clear a range of cells
    #[wasm_bindgen(js_name = clearRange)]
    pub fn clear_range(
        &mut self,
        start_row: u32,
        start_col: u32,
        end_row: u32,
        end_col: u32,
    ) -> String {
        let start = CellCoord::new(start_row, start_col);
        let end = CellCoord::new(end_row, end_col);

        let cmd = Box::new(ClearRangeCommand::new(start, end));
        let affected = self.history.execute(cmd, self.workbook.active_sheet_mut());

        // Clear dependencies and recalculate dependents
        let mut all_affected: Vec<CellCoord> = affected.clone();
        for coord in &affected {
            self.dep_graph.remove_cell((coord.row, coord.col));

            if let Ok(order) = self.dep_graph.get_recalc_order((coord.row, coord.col)) {
                for (r, c) in order {
                    self.recalculate_cell(r, c);
                    all_affected.push(CellCoord::new(r, c));
                }
            }
        }

        let coords: Vec<[u32; 2]> = all_affected.iter().map(|c| [c.row, c.col]).collect();
        serde_json::to_string(&coords).unwrap_or_else(|_| "[]".to_string())
    }

    // --- Undo/Redo ---

    /// Undo the last command
    #[wasm_bindgen]
    pub fn undo(&mut self) -> String {
        if let Some(affected) = self.history.undo(self.workbook.active_sheet_mut()) {
            // Recalculate all affected cells
            for coord in &affected {
                if let Ok(order) = self.dep_graph.get_recalc_order((coord.row, coord.col)) {
                    for (r, c) in order {
                        self.recalculate_cell(r, c);
                    }
                }
            }

            let coords: Vec<[u32; 2]> = affected.iter().map(|c| [c.row, c.col]).collect();
            serde_json::to_string(&coords).unwrap_or_else(|_| "[]".to_string())
        } else {
            "[]".to_string()
        }
    }

    /// Redo the last undone command
    #[wasm_bindgen]
    pub fn redo(&mut self) -> String {
        if let Some(affected) = self.history.redo(self.workbook.active_sheet_mut()) {
            // Recalculate all affected cells
            for coord in &affected {
                if let Ok(order) = self.dep_graph.get_recalc_order((coord.row, coord.col)) {
                    for (r, c) in order {
                        self.recalculate_cell(r, c);
                    }
                }
            }

            let coords: Vec<[u32; 2]> = affected.iter().map(|c| [c.row, c.col]).collect();
            serde_json::to_string(&coords).unwrap_or_else(|_| "[]".to_string())
        } else {
            "[]".to_string()
        }
    }

    #[wasm_bindgen(js_name = canUndo)]
    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    #[wasm_bindgen(js_name = canRedo)]
    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }

    // --- Sheet Management ---

    /// Add a new sheet
    #[wasm_bindgen(js_name = addSheet)]
    pub fn add_sheet(&mut self, name: &str) -> usize {
        self.workbook.add_sheet(name)
    }

    /// Set active sheet by index
    #[wasm_bindgen(js_name = setActiveSheet)]
    pub fn set_active_sheet(&mut self, index: usize) -> bool {
        self.workbook.set_active_sheet(index)
    }

    /// Get all sheet names as JSON array
    #[wasm_bindgen(js_name = getSheetNames)]
    pub fn get_sheet_names(&self) -> String {
        let names = self.workbook.sheet_names();
        serde_json::to_string(&names).unwrap_or_else(|_| "[]".to_string())
    }

    /// Get active sheet index
    #[wasm_bindgen(js_name = getActiveSheetIndex)]
    pub fn get_active_sheet_index(&self) -> usize {
        self.workbook.active_sheet_index
    }

    /// Rename a sheet
    #[wasm_bindgen(js_name = renameSheet)]
    pub fn rename_sheet(&mut self, index: usize, name: &str) -> bool {
        self.workbook.rename_sheet(index, name)
    }

    /// Delete a sheet
    #[wasm_bindgen(js_name = deleteSheet)]
    pub fn delete_sheet(&mut self, index: usize) -> bool {
        self.workbook.remove_sheet(index).is_some()
    }

    // --- Row/Column sizing ---

    #[wasm_bindgen(js_name = setRowHeight)]
    pub fn set_row_height(&mut self, row: u32, height: f64) {
        self.workbook.active_sheet_mut().set_row_height(row, height);
    }

    #[wasm_bindgen(js_name = setColWidth)]
    pub fn set_col_width(&mut self, col: u32, width: f64) {
        self.workbook.active_sheet_mut().set_col_width(col, width);
    }

    #[wasm_bindgen(js_name = getRowHeight)]
    pub fn get_row_height(&self, row: u32) -> f64 {
        self.workbook.active_sheet().get_row_height(row)
    }

    #[wasm_bindgen(js_name = getColWidth)]
    pub fn get_col_width(&self, col: u32) -> f64 {
        self.workbook.active_sheet().get_col_width(col)
    }

    // --- Serialization ---

    /// Serialize workbook to JSON
    #[wasm_bindgen]
    pub fn serialize(&self) -> String {
        self.workbook
            .to_json()
            .unwrap_or_else(|_| "{}".to_string())
    }

    /// Deserialize workbook from JSON
    #[wasm_bindgen]
    pub fn deserialize(&mut self, json: &str) -> bool {
        match Workbook::from_json(json) {
            Ok(wb) => {
                self.workbook = wb;
                self.rebuild_dependency_graph();
                self.history.clear();
                true
            }
            Err(_) => false,
        }
    }

    /// Rebuild dependency graph from current workbook state
    fn rebuild_dependency_graph(&mut self) {
        self.dep_graph.clear();

        for sheet in &self.workbook.sheets {
            for coord in sheet.non_empty_coords() {
                if let Some(cell) = sheet.get_cell(coord) {
                    if let Some(expression) = cell.content.formula_expression() {
                        let refs = extract_references(expression);
                        let deps: HashSet<_> = refs.into_iter().collect();
                        self.dep_graph.set_dependencies((coord.row, coord.col), deps);
                    }
                }
            }
        }
    }

    /// Recalculate all formulas in the workbook
    #[wasm_bindgen(js_name = recalculateAll)]
    pub fn recalculate_all(&mut self) {
        let sheet = self.workbook.active_sheet();
        let coords: Vec<CellCoord> = sheet.non_empty_coords().collect();

        for coord in coords {
            self.recalculate_cell(coord.row, coord.col);
        }
    }

    /// Get total dimensions of the spreadsheet
    #[wasm_bindgen(js_name = getDimensions)]
    pub fn get_dimensions(&self) -> String {
        let sheet = self.workbook.active_sheet();
        
        // Use a reasonable limit for scrollable area for now
        let max_rows = 2000;
        let max_cols = 100;
        
        let width = sheet.col_x_position(max_cols);
        let height = sheet.row_y_position(max_rows);

        serde_json::to_string(&serde_json::json!({
            "width": width,
            "height": height,
            "rows": max_rows,
            "cols": max_cols
        })).unwrap_or_else(|_| "{}".to_string())
    }

    /// Get cell coordinates from pixel coordinates
    #[wasm_bindgen(js_name = getCellFromPixel)]
    pub fn get_cell_from_pixel(&self, x: f64, y: f64) -> Vec<u32> {
        let sheet = self.workbook.active_sheet();
        let row = sheet.row_at_y(y);
        let col = sheet.col_at_x(x);
        vec![row, col]
    }
}

impl Default for SpreadsheetEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert CellFormatData to CellFormat
fn cell_format_from_data(data: &CellFormatData) -> CellFormat {
    use rusheet_core::{Color, HorizontalAlign, VerticalAlign};

    let mut format = CellFormat::default();
    format.bold = data.bold;
    format.italic = data.italic;
    format.underline = data.underline;
    format.font_size = data.font_size;

    if let Some(ref color) = data.text_color {
        format.text_color = Color::from_hex(color);
    }
    if let Some(ref color) = data.background_color {
        format.background_color = Color::from_hex(color);
    }

    if let Some(ref align) = data.horizontal_align {
        format.horizontal_align = match align.as_str() {
            "center" => HorizontalAlign::Center,
            "right" => HorizontalAlign::Right,
            _ => HorizontalAlign::Left,
        };
    }

    if let Some(ref align) = data.vertical_align {
        format.vertical_align = match align.as_str() {
            "top" => VerticalAlign::Top,
            "bottom" => VerticalAlign::Bottom,
            _ => VerticalAlign::Middle,
        };
    }

    format
}
