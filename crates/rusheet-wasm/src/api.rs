use rusheet_core::{CellContent, CellCoord, CellError, CellFormat, CellValue, HorizontalAlign, VerticalAlign, Workbook};
use rusheet_formula::{extract_references, DependencyGraph};
use rusheet_history::{
    ClearRangeCommand, HistoryManager, MergeCellsCommand, SetCellFormatCommand, SetCellValueCommand,
    SetRangeFormatCommand, InsertRowsCommand, DeleteRowsCommand, InsertColsCommand, DeleteColsCommand,
    SortRangeCommand, UnmergeCellsCommand,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

use crate::viewport::{pack_format, ViewportBuffer};

/// Main spreadsheet engine exposed to JavaScript
#[wasm_bindgen]
pub struct SpreadsheetEngine {
    workbook: Workbook,
    dep_graph: DependencyGraph,
    history: HistoryManager,
    /// Reusable buffer for viewport data (zero-copy optimization)
    viewport_buffer: ViewportBuffer,
}

/// Cell data for JavaScript
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct CellFormatData {
    #[serde(skip_serializing_if = "is_false")]
    pub bold: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub italic: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub underline: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fontSize")]
    pub font_size: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "textColor")]
    pub text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backgroundColor")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "horizontalAlign")]
    pub horizontal_align: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "verticalAlign")]
    pub vertical_align: Option<String>,
}

fn is_false(b: &bool) -> bool {
    !*b
}

/// Merge range data for JavaScript
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergeRangeData {
    pub start_row: u32,
    pub start_col: u32,
    pub end_row: u32,
    pub end_col: u32,
}

/// Merge info for a cell
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergeInfo {
    pub master_row: u32,
    pub master_col: u32,
    pub row_span: u32,
    pub col_span: u32,
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
            viewport_buffer: ViewportBuffer::with_capacity(1000),
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

            // Immediately evaluate the formula we just set
            self.recalculate_cell(row, col);

            // Then recalculate dependents
            if let Ok(order) = self.dep_graph.get_recalc_order((row, col)) {
                for (r, c) in order {
                    if (r, c) != (row, col) {
                        // Don't recalculate the formula cell twice
                        self.recalculate_cell(r, c);
                        affected.push(CellCoord::new(r, c));
                    }
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
        let current_sheet_name = self.workbook.active_sheet().name.to_string();

        let expression = {
            let sheet = self.workbook.active_sheet();
            if let Some(cell) = sheet.get_cell(coord) {
                if let CellContent::Formula { expression, .. } = &cell.content {
                    Some(expression.clone())
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some(expression) = expression {
            // Log formula evaluation for debugging (only in WASM target)
            #[cfg(all(debug_assertions, target_arch = "wasm32"))]
            web_sys::console::log_1(&format!(
                "[Formula] Evaluating cell ({}, {}): {}",
                row, col, expression
            ).into());

            // Create closure to get cell values from any sheet
            let result = rusheet_formula::evaluate_formula_cross_sheet(
                &expression,
                Some(&current_sheet_name),
                |sheet_name, r, c| {
                    let sheet = if let Some(name) = sheet_name {
                        match self.workbook.get_sheet_by_name(name) {
                            Some(s) => s,
                            None => return CellValue::Error(CellError::InvalidReference),
                        }
                    } else {
                        self.workbook.active_sheet()
                    };
                    sheet
                        .get_cell(CellCoord::new(r, c))
                        .map(|c| c.computed_value().clone())
                        .unwrap_or(CellValue::Empty)
                },
            );

            // Log result (only in WASM target)
            #[cfg(all(debug_assertions, target_arch = "wasm32"))]
            web_sys::console::log_1(&format!(
                "[Formula] Result for ({}, {}): {:?}",
                row, col, result
            ).into());

            // Check for errors and log them (only in WASM target)
            #[cfg(target_arch = "wasm32")]
            if matches!(result, CellValue::Error(_)) {
                web_sys::console::error_1(&format!(
                    "[Formula Error] Cell ({}, {}) formula '{}' failed: {:?}",
                    row, col, expression, result
                ).into());
            }

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

    /// Get cell data for rendering
    #[wasm_bindgen(js_name = getCellData)]
    pub fn get_cell_data(&self, row: u32, col: u32) -> JsValue {
        let coord = CellCoord::new(row, col);
        let sheet = self.workbook.active_sheet();

        let data = if let Some(cell) = sheet.get_cell(coord) {
            CellData {
                value: Some(cell.content.original_input()),
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
                        value: Some(cell.content.original_input()),
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
            Err(e) => {
                web_sys::console::log_1(&format!(
                    "Error deserializing format for cell ({}, {}): {}",
                    row, col, e
                ).into());
                return false;
            }
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
            Err(e) => {
                web_sys::console::log_1(&format!(
                    "Error deserializing range format ({},{}) to ({},{}): {}",
                    start_row, start_col, end_row, end_col, e
                ).into());
                return false;
            }
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

    /// Clear undo/redo history
    #[wasm_bindgen(js_name = clearHistory)]
    pub fn clear_history(&mut self) {
        self.history = HistoryManager::new(100);
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

    // --- Row/Column Insert/Delete ---

    /// Insert rows at the given position
    /// Returns JSON array of affected cell coordinates
    #[wasm_bindgen(js_name = insertRows)]
    pub fn insert_rows(&mut self, at_row: u32, count: u32) -> String {
        let cmd = Box::new(InsertRowsCommand::new(at_row, count));
        let affected = self.history.execute(cmd, self.workbook.active_sheet_mut());

        // Rebuild dependency graph since cell references changed
        self.rebuild_dependency_graph();

        // Recalculate all formulas
        self.recalculate_all();

        let coords: Vec<[u32; 2]> = affected.iter().map(|c| [c.row, c.col]).collect();
        serde_json::to_string(&coords).unwrap_or_else(|_| "[]".to_string())
    }

    /// Delete rows at the given position
    /// Returns JSON array of affected cell coordinates
    #[wasm_bindgen(js_name = deleteRows)]
    pub fn delete_rows(&mut self, at_row: u32, count: u32) -> String {
        let cmd = Box::new(DeleteRowsCommand::new(at_row, count));
        let affected = self.history.execute(cmd, self.workbook.active_sheet_mut());

        // Rebuild dependency graph since cell references changed
        self.rebuild_dependency_graph();

        // Recalculate all formulas
        self.recalculate_all();

        let coords: Vec<[u32; 2]> = affected.iter().map(|c| [c.row, c.col]).collect();
        serde_json::to_string(&coords).unwrap_or_else(|_| "[]".to_string())
    }

    /// Insert columns at the given position
    /// Returns JSON array of affected cell coordinates
    #[wasm_bindgen(js_name = insertCols)]
    pub fn insert_cols(&mut self, at_col: u32, count: u32) -> String {
        let cmd = Box::new(InsertColsCommand::new(at_col, count));
        let affected = self.history.execute(cmd, self.workbook.active_sheet_mut());

        // Rebuild dependency graph since cell references changed
        self.rebuild_dependency_graph();

        // Recalculate all formulas
        self.recalculate_all();

        let coords: Vec<[u32; 2]> = affected.iter().map(|c| [c.row, c.col]).collect();
        serde_json::to_string(&coords).unwrap_or_else(|_| "[]".to_string())
    }

    /// Delete columns at the given position
    /// Returns JSON array of affected cell coordinates
    #[wasm_bindgen(js_name = deleteCols)]
    pub fn delete_cols(&mut self, at_col: u32, count: u32) -> String {
        let cmd = Box::new(DeleteColsCommand::new(at_col, count));
        let affected = self.history.execute(cmd, self.workbook.active_sheet_mut());

        // Rebuild dependency graph since cell references changed
        self.rebuild_dependency_graph();

        // Recalculate all formulas
        self.recalculate_all();

        let coords: Vec<[u32; 2]> = affected.iter().map(|c| [c.row, c.col]).collect();
        serde_json::to_string(&coords).unwrap_or_else(|_| "[]".to_string())
    }

    /// Sort a range of rows by a specific column
    /// Returns JSON array of affected cell coordinates
    #[wasm_bindgen(js_name = sortRange)]
    pub fn sort_range(
        &mut self,
        start_row: u32,
        end_row: u32,
        start_col: u32,
        end_col: u32,
        sort_col: u32,
        ascending: bool,
    ) -> String {
        let cmd = Box::new(SortRangeCommand::new(
            start_row, end_row, start_col, end_col, sort_col, ascending,
        ));
        let affected = self.history.execute(cmd, self.workbook.active_sheet_mut());

        // Recalculate formulas in the sorted range
        self.recalculate_all();

        let coords: Vec<[u32; 2]> = affected.iter().map(|c| [c.row, c.col]).collect();
        serde_json::to_string(&coords).unwrap_or_else(|_| "[]".to_string())
    }

    // --- Cell Merging ---

    /// Merge cells in a range. Returns JSON array of affected cell coordinates.
    #[wasm_bindgen(js_name = mergeCells)]
    pub fn merge_cells(
        &mut self,
        start_row: u32,
        start_col: u32,
        end_row: u32,
        end_col: u32,
    ) -> String {
        let cmd = Box::new(MergeCellsCommand::from_coords(start_row, start_col, end_row, end_col));
        let affected = self.history.execute(cmd, self.workbook.active_sheet_mut());

        let coords: Vec<[u32; 2]> = affected.iter().map(|c| [c.row, c.col]).collect();
        serde_json::to_string(&coords).unwrap_or_else(|_| "[]".to_string())
    }

    /// Unmerge cells at a position. Returns JSON array of affected cell coordinates.
    #[wasm_bindgen(js_name = unmergeCells)]
    pub fn unmerge_cells(&mut self, row: u32, col: u32) -> String {
        let coord = CellCoord::new(row, col);
        let cmd = Box::new(UnmergeCellsCommand::from_coord(coord));
        let affected = self.history.execute(cmd, self.workbook.active_sheet_mut());

        let coords: Vec<[u32; 2]> = affected.iter().map(|c| [c.row, c.col]).collect();
        serde_json::to_string(&coords).unwrap_or_else(|_| "[]".to_string())
    }

    /// Get merged ranges as JSON array of objects with start/end coordinates.
    #[wasm_bindgen(js_name = getMergedRanges)]
    pub fn get_merged_ranges(&self) -> String {
        let sheet = self.workbook.active_sheet();
        let ranges: Vec<MergeRangeData> = sheet
            .get_merged_ranges()
            .iter()
            .map(|r| MergeRangeData {
                start_row: r.start.row,
                start_col: r.start.col,
                end_row: r.end.row,
                end_col: r.end.col,
            })
            .collect();

        serde_json::to_string(&ranges).unwrap_or_else(|_| "[]".to_string())
    }

    /// Check if a cell is part of a merged range (but not the master cell).
    #[wasm_bindgen(js_name = isMergedSlave)]
    pub fn is_merged_slave(&self, row: u32, col: u32) -> bool {
        let coord = CellCoord::new(row, col);
        self.workbook.active_sheet().is_merged_slave(coord)
    }

    /// Get the merge info for a cell (returns null if not merged).
    #[wasm_bindgen(js_name = getMergeInfo)]
    pub fn get_merge_info(&self, row: u32, col: u32) -> JsValue {
        let coord = CellCoord::new(row, col);
        let sheet = self.workbook.active_sheet();

        if let Some(range) = sheet.get_merge_at(coord) {
            let info = MergeInfo {
                master_row: range.start.row,
                master_col: range.start.col,
                row_span: range.row_span(),
                col_span: range.col_span(),
            };
            serde_wasm_bindgen::to_value(&info).unwrap_or(JsValue::NULL)
        } else {
            JsValue::NULL
        }
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

    // =========================================================================
    // Zero-Copy Viewport API
    // =========================================================================

    /// Populate the internal viewport buffer with cells in the given range.
    /// Call this before accessing the viewport arrays.
    #[wasm_bindgen(js_name = populateViewport)]
    pub fn populate_viewport(&mut self, start_row: u32, end_row: u32, start_col: u32, end_col: u32) {
        self.viewport_buffer.clear();
        let sheet = self.workbook.active_sheet();

        for row in start_row..=end_row {
            for col in start_col..=end_col {
                let coord = CellCoord::new(row, col);
                if let Some(cell) = sheet.get_cell(coord) {
                    // Extract numeric value (NaN for non-numeric)
                    let numeric_value = match &cell.content {
                        CellContent::Value { value: CellValue::Number(n), .. } => *n,
                        CellContent::Formula { cached_value: CellValue::Number(n), .. } => *n,
                        _ => f64::NAN,
                    };

                    // Pack format flags
                    let h_align = match cell.format.horizontal_align {
                        HorizontalAlign::Left => 0,
                        HorizontalAlign::Center => 1,
                        HorizontalAlign::Right => 2,
                    };
                    let v_align = match cell.format.vertical_align {
                        VerticalAlign::Middle => 0,
                        VerticalAlign::Top => 1,
                        VerticalAlign::Bottom => 2,
                    };
                    let format_flags = pack_format(
                        cell.format.bold,
                        cell.format.italic,
                        cell.format.underline,
                        cell.format.font_size,
                        h_align,
                        v_align,
                    );

                    self.viewport_buffer.push(
                        row,
                        col,
                        numeric_value,
                        format_flags,
                        cell.content.display_value(),
                    );
                }
            }
        }
    }

    /// Get the number of cells in the viewport buffer
    #[wasm_bindgen(js_name = getViewportLen)]
    pub fn get_viewport_len(&self) -> usize {
        self.viewport_buffer.len()
    }

    /// Get pointer to viewport row indices (Uint32Array)
    #[wasm_bindgen(js_name = getViewportRowsPtr)]
    pub fn get_viewport_rows_ptr(&self) -> *const u32 {
        self.viewport_buffer.rows.as_ptr()
    }

    /// Get pointer to viewport column indices (Uint32Array)
    #[wasm_bindgen(js_name = getViewportColsPtr)]
    pub fn get_viewport_cols_ptr(&self) -> *const u32 {
        self.viewport_buffer.cols.as_ptr()
    }

    /// Get pointer to viewport numeric values (Float64Array)
    #[wasm_bindgen(js_name = getViewportValuesPtr)]
    pub fn get_viewport_values_ptr(&self) -> *const f64 {
        self.viewport_buffer.values.as_ptr()
    }

    /// Get pointer to viewport format flags (Uint32Array)
    #[wasm_bindgen(js_name = getViewportFormatsPtr)]
    pub fn get_viewport_formats_ptr(&self) -> *const u32 {
        self.viewport_buffer.formats.as_ptr()
    }

    /// Get viewport display values as JSON (strings still need serialization)
    #[wasm_bindgen(js_name = getViewportDisplayValues)]
    pub fn get_viewport_display_values(&self) -> String {
        serde_json::to_string(&self.viewport_buffer.display_values)
            .unwrap_or_else(|_| "[]".to_string())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_format_data_deserialization() {
        // Test camelCase JSON from frontend
        let json = r##"{
            "bold": true,
            "italic": false,
            "fontSize": 12,
            "textColor": "#ff0000",
            "backgroundColor": "#00ff00",
            "horizontalAlign": "center",
            "verticalAlign": "top"
        }"##;

        let result: Result<CellFormatData, _> = serde_json::from_str(json);
        assert!(result.is_ok(), "Failed to deserialize camelCase format data");

        let format = result.unwrap();
        assert_eq!(format.bold, true);
        assert_eq!(format.font_size, Some(12));
        assert_eq!(format.text_color, Some("#ff0000".to_string()));
        assert_eq!(format.background_color, Some("#00ff00".to_string()));
        assert_eq!(format.horizontal_align, Some("center".to_string()));
        assert_eq!(format.vertical_align, Some("top".to_string()));
    }

    #[test]
    fn test_cell_format_data_round_trip() {
        let format = CellFormatData {
            bold: true,
            italic: false,
            underline: false,
            font_size: Some(14),
            text_color: Some("#ff0000".to_string()),
            background_color: Some("#00ff00".to_string()),
            horizontal_align: Some("right".to_string()),
            vertical_align: Some("bottom".to_string()),
        };

        let json = serde_json::to_string(&format).unwrap();
        let deserialized: CellFormatData = serde_json::from_str(&json).unwrap();

        assert_eq!(format.bold, deserialized.bold);
        assert_eq!(format.font_size, deserialized.font_size);
        assert_eq!(format.text_color, deserialized.text_color);
        assert_eq!(format.background_color, deserialized.background_color);
        assert_eq!(format.horizontal_align, deserialized.horizontal_align);
        assert_eq!(format.vertical_align, deserialized.vertical_align);
    }

    #[test]
    fn test_cell_format_data_partial_fields() {
        // Test that partial fields deserialize correctly
        let json = r##"{
            "bold": true,
            "textColor": "#ff0000"
        }"##;

        let format: CellFormatData = serde_json::from_str(json).unwrap();
        assert_eq!(format.bold, true);
        assert_eq!(format.italic, false); // default
        assert_eq!(format.text_color, Some("#ff0000".to_string()));
        assert_eq!(format.font_size, None); // not provided
    }
}

#[cfg(test)]
mod bug_fixes {
    use rusheet_core::{CellCoord, Sheet};

    #[test]
    fn test_bug_1_3_number_preservation() {
        // Test at the core level without WASM
        let mut sheet = Sheet::new("Test");

        // Enter "10" in B2 (row=1, col=1)
        let coord = CellCoord::new(1, 1);
        let content = rusheet_core::sheet::parse_cell_input("10");
        let cell = rusheet_core::Cell::new(content);
        sheet.set_cell(coord, cell);

        // Verify original_input returns "10"
        let cell = sheet.get_cell(coord).unwrap();
        let original = cell.content.original_input();
        assert_eq!(original, "10",
                   "Bug #1-3: Number '10' should be preserved as original input");
        assert_eq!(cell.content.display_value(), "10");

        // Enter formula "=B2" in B3
        let coord_b3 = CellCoord::new(1, 2);
        let formula_content = rusheet_core::sheet::parse_cell_input("=B2");
        let formula_cell = rusheet_core::Cell::new(formula_content);
        sheet.set_cell(coord_b3, formula_cell);

        // Re-check B2 - should STILL be "10", not corrupted
        let cell_again = sheet.get_cell(coord).unwrap();
        let original_again = cell_again.content.original_input();
        assert_eq!(original_again, "10",
                   "Bug #1-3: Re-reading B2 should still return '10', not 'B2C2'");
    }

    #[test]
    fn test_original_input_preserved_for_all_types() {
        // Test at the core level without WASM
        let mut sheet = Sheet::new("Test");

        // Test number
        let content = rusheet_core::sheet::parse_cell_input("42");
        sheet.set_cell(CellCoord::new(0, 0), rusheet_core::Cell::new(content));
        let cell = sheet.get_cell(CellCoord::new(0, 0)).unwrap();
        assert_eq!(cell.content.original_input(), "42");

        // Test percentage
        let content = rusheet_core::sheet::parse_cell_input("50%");
        sheet.set_cell(CellCoord::new(0, 1), rusheet_core::Cell::new(content));
        let cell = sheet.get_cell(CellCoord::new(0, 1)).unwrap();
        assert_eq!(cell.content.original_input(), "50%");
        assert_eq!(cell.content.display_value(), "0.5");

        // Test boolean
        let content = rusheet_core::sheet::parse_cell_input("TRUE");
        sheet.set_cell(CellCoord::new(0, 2), rusheet_core::Cell::new(content));
        let cell = sheet.get_cell(CellCoord::new(0, 2)).unwrap();
        assert_eq!(cell.content.original_input(), "TRUE");

        // Test text
        let content = rusheet_core::sheet::parse_cell_input("Hello");
        sheet.set_cell(CellCoord::new(0, 3), rusheet_core::Cell::new(content));
        let cell = sheet.get_cell(CellCoord::new(0, 3)).unwrap();
        assert_eq!(cell.content.original_input(), "Hello");

        // Test formula
        let content = rusheet_core::sheet::parse_cell_input("=A1+B1");
        sheet.set_cell(CellCoord::new(0, 4), rusheet_core::Cell::new(content));
        let cell = sheet.get_cell(CellCoord::new(0, 4)).unwrap();
        assert_eq!(cell.content.original_input(), "=A1+B1");
    }

    // Helper function to get cell data
    fn get_cell_as_data(engine: &super::SpreadsheetEngine, row: u32, col: u32) -> super::CellData {
        let coord = CellCoord::new(row, col);
        let sheet = engine.workbook.active_sheet();

        if let Some(cell) = sheet.get_cell(coord) {
            super::CellData {
                value: Some(cell.content.original_input()),
                display_value: cell.content.display_value(),
                formula: cell.content.formula_expression().map(String::from),
                format: super::CellFormatData::from(&cell.format),
                row,
                col,
            }
        } else {
            super::CellData {
                value: None,
                display_value: String::new(),
                formula: None,
                format: super::CellFormatData::default(),
                row,
                col,
            }
        }
    }

    #[test]
    fn test_bug_4_formula_evaluation() {
        let mut engine = super::SpreadsheetEngine::new();

        // Test case from bug report: "=5+3" should display "8"
        engine.set_cell_value(0, 0, "=5+3");

        let data = get_cell_as_data(&engine, 0, 0);

        assert_eq!(data.formula, Some("=5+3".to_string()), "Formula should be stored");
        assert_eq!(data.display_value, "8", "Bug #4: '=5+3' should evaluate to '8', not empty");
        assert_ne!(data.display_value, "", "Display value should not be empty");
    }

    #[test]
    fn test_formula_with_cell_references() {
        let mut engine = super::SpreadsheetEngine::new();

        // Set up: A1=10, A2=20
        engine.set_cell_value(0, 0, "10");
        engine.set_cell_value(1, 0, "20");

        // Formula: A3=A1+A2
        engine.set_cell_value(2, 0, "=A1+A2");

        let data = get_cell_as_data(&engine, 2, 0);
        assert_eq!(data.formula, Some("=A1+A2".to_string()));
        assert_eq!(data.display_value, "30", "A1+A2 should equal 30");
    }

    #[test]
    fn test_formula_multiplication_division() {
        let mut engine = super::SpreadsheetEngine::new();

        // Test multiplication
        engine.set_cell_value(0, 0, "=6*7");
        let data = get_cell_as_data(&engine, 0, 0);
        assert_eq!(data.display_value, "42", "6*7 should equal 42");

        // Test division
        engine.set_cell_value(0, 1, "=100/4");
        let data = get_cell_as_data(&engine, 0, 1);
        assert_eq!(data.display_value, "25", "100/4 should equal 25");

        // Test combined operations
        engine.set_cell_value(0, 2, "=(5+3)*2");
        let data = get_cell_as_data(&engine, 0, 2);
        assert_eq!(data.display_value, "16", "(5+3)*2 should equal 16");
    }

    #[test]
    fn test_formula_recalculation_on_dependency_change() {
        let mut engine = super::SpreadsheetEngine::new();

        // Set up: A1=5, B1=A1*2
        engine.set_cell_value(0, 0, "5");
        engine.set_cell_value(0, 1, "=A1*2");

        let data = get_cell_as_data(&engine, 0, 1);
        assert_eq!(data.display_value, "10", "Initial: A1*2 = 5*2 = 10");

        // Change A1 to 10
        engine.set_cell_value(0, 0, "10");

        // B1 should automatically recalculate
        let data = get_cell_as_data(&engine, 0, 1);
        assert_eq!(data.display_value, "20", "After change: A1*2 = 10*2 = 20");
    }

    #[test]
    fn test_bug_7_persistence() {
        let mut engine = super::SpreadsheetEngine::new();

        // User enters data
        engine.set_cell_value(0, 0, "Hello");
        engine.set_cell_value(1, 0, "World");
        engine.set_cell_value(2, 0, "=A1&\" \"&A2");  // Concatenation formula

        // Simulate page refresh: serialize and deserialize
        let json = engine.serialize();
        assert!(!json.is_empty(), "Serialized JSON should not be empty");

        let mut new_engine = super::SpreadsheetEngine::new();
        let success = new_engine.deserialize(&json);

        assert!(success, "Bug #7: Data should persist after refresh");

        // Verify data is still there
        let data1 = get_cell_as_data(&new_engine, 0, 0);
        assert_eq!(data1.value, Some("Hello".to_string()),
                   "Bug #7: 'Hello' should persist");

        let data2 = get_cell_as_data(&new_engine, 1, 0);
        assert_eq!(data2.value, Some("World".to_string()),
                   "Bug #7: 'World' should persist");

        // Verify formula persists and evaluates
        let data3 = get_cell_as_data(&new_engine, 2, 0);
        assert_eq!(data3.formula, Some("=A1&\" \"&A2".to_string()),
                   "Formula should persist");
    }

    #[test]
    fn test_serialization_preserves_formatting() {
        use serde_json::json;

        let mut engine = super::SpreadsheetEngine::new();

        // Set cell value
        engine.set_cell_value(0, 0, "Formatted");

        // Apply formatting
        let format = json!({
            "bold": true,
            "textColor": "#ff0000",
            "backgroundColor": "#ffff00"
        }).to_string();
        engine.set_cell_format(0, 0, &format);

        // Serialize and deserialize
        let json = engine.serialize();
        let mut new_engine = super::SpreadsheetEngine::new();
        new_engine.deserialize(&json);

        // Verify formatting persists
        let data = get_cell_as_data(&new_engine, 0, 0);
        assert_eq!(data.format.bold, true);
        assert_eq!(data.format.text_color, Some("#ff0000".to_string()));
        assert_eq!(data.format.background_color, Some("#ffff00".to_string()));
    }
}
