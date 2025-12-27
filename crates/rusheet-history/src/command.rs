use rusheet_core::{Cell, CellContent, CellCoord, CellFormat, CellValue, Sheet};

/// Type alias for boxed commands
pub type CommandBox = Box<dyn Command>;

/// Trait for undoable commands
pub trait Command: std::fmt::Debug + Send + Sync {
    /// Execute the command, returning affected cell coordinates
    fn execute(&mut self, sheet: &mut Sheet) -> Vec<CellCoord>;

    /// Undo the command, returning affected cell coordinates
    fn undo(&mut self, sheet: &mut Sheet) -> Vec<CellCoord>;

    /// Get a description of this command (for UI display)
    fn description(&self) -> &str;

    /// Try to merge with another command (for typing sequences)
    /// Returns true if merge was successful
    fn merge(&mut self, _other: &dyn Command) -> bool {
        false
    }

    /// Check if this command can be merged with another
    fn can_merge(&self, _other: &dyn Command) -> bool {
        false
    }
}

/// Set a single cell's value
#[derive(Debug)]
pub struct SetCellValueCommand {
    coord: CellCoord,
    new_content: CellContent,
    old_content: Option<CellContent>,
}

impl SetCellValueCommand {
    pub fn new(coord: CellCoord, new_content: CellContent) -> Self {
        Self {
            coord,
            new_content,
            old_content: None,
        }
    }

    pub fn from_value(coord: CellCoord, value: CellValue) -> Self {
        Self::new(coord, CellContent::Value(value))
    }

    pub fn from_input(coord: CellCoord, input: &str) -> Self {
        let content = rusheet_core::sheet::parse_cell_input(input);
        Self::new(coord, content)
    }
}

impl Command for SetCellValueCommand {
    fn execute(&mut self, sheet: &mut Sheet) -> Vec<CellCoord> {
        // Capture old state for undo
        self.old_content = sheet.get_cell(self.coord).map(|c| c.content.clone());

        // Apply new value
        let cell = sheet.get_cell_mut(self.coord);
        cell.content = self.new_content.clone();

        vec![self.coord]
    }

    fn undo(&mut self, sheet: &mut Sheet) -> Vec<CellCoord> {
        let cell = sheet.get_cell_mut(self.coord);
        cell.content = self
            .old_content
            .clone()
            .unwrap_or(CellContent::Value(CellValue::Empty));

        vec![self.coord]
    }

    fn description(&self) -> &str {
        "Set cell value"
    }

}

/// Set cell format
#[derive(Debug)]
pub struct SetCellFormatCommand {
    coord: CellCoord,
    new_format: CellFormat,
    old_format: Option<CellFormat>,
}

impl SetCellFormatCommand {
    pub fn new(coord: CellCoord, new_format: CellFormat) -> Self {
        Self {
            coord,
            new_format,
            old_format: None,
        }
    }
}

impl Command for SetCellFormatCommand {
    fn execute(&mut self, sheet: &mut Sheet) -> Vec<CellCoord> {
        // Capture old format first (immutable borrow)
        self.old_format = sheet.get_cell(self.coord).map(|c| c.format.clone());

        // Then apply new format (mutable borrow)
        let cell = sheet.get_cell_mut(self.coord);
        cell.format = self.new_format.clone();

        vec![self.coord]
    }

    fn undo(&mut self, sheet: &mut Sheet) -> Vec<CellCoord> {
        let cell = sheet.get_cell_mut(self.coord);
        cell.format = self.old_format.clone().unwrap_or_default();

        vec![self.coord]
    }

    fn description(&self) -> &str {
        "Set cell format"
    }
}

/// Set formatting for a range of cells
#[derive(Debug)]
pub struct SetRangeFormatCommand {
    start: CellCoord,
    end: CellCoord,
    new_format: CellFormat,
    old_formats: Vec<(CellCoord, CellFormat)>,
}

impl SetRangeFormatCommand {
    pub fn new(start: CellCoord, end: CellCoord, new_format: CellFormat) -> Self {
        Self {
            start,
            end,
            new_format,
            old_formats: Vec::new(),
        }
    }
}

impl Command for SetRangeFormatCommand {
    fn execute(&mut self, sheet: &mut Sheet) -> Vec<CellCoord> {
        let mut affected = Vec::new();
        self.old_formats.clear();

        let min_row = self.start.row.min(self.end.row);
        let max_row = self.start.row.max(self.end.row);
        let min_col = self.start.col.min(self.end.col);
        let max_col = self.start.col.max(self.end.col);

        for row in min_row..=max_row {
            for col in min_col..=max_col {
                let coord = CellCoord::new(row, col);

                // Capture old format (immutable borrow)
                let old_format = sheet.get_cell(coord)
                    .map(|c| c.format.clone())
                    .unwrap_or_default();
                self.old_formats.push((coord, old_format));

                // Apply new format (mutable borrow)
                let cell = sheet.get_cell_mut(coord);
                cell.format = self.new_format.clone();

                affected.push(coord);
            }
        }

        affected
    }

    fn undo(&mut self, sheet: &mut Sheet) -> Vec<CellCoord> {
        let mut affected = Vec::new();

        for (coord, format) in &self.old_formats {
            let cell = sheet.get_cell_mut(*coord);
            cell.format = format.clone();
            affected.push(*coord);
        }

        affected
    }

    fn description(&self) -> &str {
        "Set range format"
    }
}

/// Clear cell content (but keep format)
#[derive(Debug)]
pub struct ClearCellCommand {
    coord: CellCoord,
    old_content: Option<CellContent>,
}

impl ClearCellCommand {
    pub fn new(coord: CellCoord) -> Self {
        Self {
            coord,
            old_content: None,
        }
    }
}

impl Command for ClearCellCommand {
    fn execute(&mut self, sheet: &mut Sheet) -> Vec<CellCoord> {
        self.old_content = sheet.get_cell(self.coord).map(|c| c.content.clone());

        let cell = sheet.get_cell_mut(self.coord);
        cell.content = CellContent::Value(CellValue::Empty);

        vec![self.coord]
    }

    fn undo(&mut self, sheet: &mut Sheet) -> Vec<CellCoord> {
        if let Some(content) = &self.old_content {
            let cell = sheet.get_cell_mut(self.coord);
            cell.content = content.clone();
        }

        vec![self.coord]
    }

    fn description(&self) -> &str {
        "Clear cell"
    }
}

/// Clear a range of cells
#[derive(Debug)]
pub struct ClearRangeCommand {
    start: CellCoord,
    end: CellCoord,
    old_cells: Vec<(CellCoord, Cell)>,
}

impl ClearRangeCommand {
    pub fn new(start: CellCoord, end: CellCoord) -> Self {
        Self {
            start,
            end,
            old_cells: Vec::new(),
        }
    }
}

impl Command for ClearRangeCommand {
    fn execute(&mut self, sheet: &mut Sheet) -> Vec<CellCoord> {
        let mut affected = Vec::new();
        self.old_cells.clear();

        let min_row = self.start.row.min(self.end.row);
        let max_row = self.start.row.max(self.end.row);
        let min_col = self.start.col.min(self.end.col);
        let max_col = self.start.col.max(self.end.col);

        for row in min_row..=max_row {
            for col in min_col..=max_col {
                let coord = CellCoord::new(row, col);

                // Capture old cell
                if let Some(cell) = sheet.get_cell(coord) {
                    self.old_cells.push((coord, cell.clone()));
                }

                // Clear the cell
                sheet.remove_cell(coord);
                affected.push(coord);
            }
        }

        affected
    }

    fn undo(&mut self, sheet: &mut Sheet) -> Vec<CellCoord> {
        let mut affected = Vec::new();

        for (coord, cell) in &self.old_cells {
            sheet.set_cell(*coord, cell.clone());
            affected.push(*coord);
        }

        affected
    }

    fn description(&self) -> &str {
        "Clear range"
    }
}

/// Composite command for batch operations
#[derive(Debug)]
pub struct CompositeCommand {
    commands: Vec<CommandBox>,
    description: String,
}

impl CompositeCommand {
    pub fn new(commands: Vec<CommandBox>, description: impl Into<String>) -> Self {
        Self {
            commands,
            description: description.into(),
        }
    }
}

impl Command for CompositeCommand {
    fn execute(&mut self, sheet: &mut Sheet) -> Vec<CellCoord> {
        let mut affected = Vec::new();
        for cmd in &mut self.commands {
            affected.extend(cmd.execute(sheet));
        }
        affected
    }

    fn undo(&mut self, sheet: &mut Sheet) -> Vec<CellCoord> {
        let mut affected = Vec::new();
        // Undo in reverse order
        for cmd in self.commands.iter_mut().rev() {
            affected.extend(cmd.undo(sheet));
        }
        affected
    }

    fn description(&self) -> &str {
        &self.description
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_cell_value_command() {
        let mut sheet = Sheet::new("Test");
        let coord = CellCoord::new(0, 0);

        let mut cmd = SetCellValueCommand::from_value(coord, CellValue::Number(42.0));

        // Execute
        let affected = cmd.execute(&mut sheet);
        assert_eq!(affected, vec![coord]);
        assert_eq!(
            sheet.get_cell_value(coord).as_number(),
            Some(42.0)
        );

        // Undo
        let affected = cmd.undo(&mut sheet);
        assert_eq!(affected, vec![coord]);
        assert!(sheet.get_cell_value(coord).is_empty());
    }

    #[test]
    fn test_clear_range_command() {
        let mut sheet = Sheet::new("Test");

        // Set up some cells
        sheet.set_cell(CellCoord::new(0, 0), Cell::number(1.0));
        sheet.set_cell(CellCoord::new(0, 1), Cell::number(2.0));
        sheet.set_cell(CellCoord::new(1, 0), Cell::number(3.0));

        let mut cmd = ClearRangeCommand::new(CellCoord::new(0, 0), CellCoord::new(1, 1));

        // Execute
        cmd.execute(&mut sheet);
        assert!(sheet.get_cell(CellCoord::new(0, 0)).is_none());
        assert!(sheet.get_cell(CellCoord::new(0, 1)).is_none());
        assert!(sheet.get_cell(CellCoord::new(1, 0)).is_none());

        // Undo
        cmd.undo(&mut sheet);
        assert_eq!(
            sheet.get_cell_value(CellCoord::new(0, 0)).as_number(),
            Some(1.0)
        );
        assert_eq!(
            sheet.get_cell_value(CellCoord::new(0, 1)).as_number(),
            Some(2.0)
        );
        assert_eq!(
            sheet.get_cell_value(CellCoord::new(1, 0)).as_number(),
            Some(3.0)
        );
    }
}
