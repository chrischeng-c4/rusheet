use std::collections::HashSet;

/// Binary Indexed Tree (Fenwick Tree) for efficient prefix sum queries.
///
/// This data structure supports O(log N) updates and prefix sum queries,
/// which is essential for quickly finding row/column positions in a spreadsheet
/// where cells can have variable heights/widths.
#[derive(Debug, Clone)]
pub struct FenwickTree {
    /// Internal tree storage (1-indexed internally)
    tree: Vec<f64>,
}

impl FenwickTree {
    /// Creates a new Fenwick Tree with the given size.
    ///
    /// # Arguments
    /// * `size` - The number of elements the tree should support
    pub fn new(size: usize) -> Self {
        Self {
            tree: vec![0.0; size + 1],
        }
    }

    /// Updates the value at the given index by adding delta.
    ///
    /// # Arguments
    /// * `index` - The 0-based index to update
    /// * `delta` - The amount to add to the current value
    ///
    /// Time complexity: O(log N)
    pub fn update(&mut self, index: usize, delta: f64) {
        let mut idx = index + 1; // Convert to 1-indexed
        while idx < self.tree.len() {
            self.tree[idx] += delta;
            idx += idx & idx.wrapping_neg(); // Add least significant bit
        }
    }

    /// Computes the prefix sum from index 0 to the given index (inclusive).
    ///
    /// # Arguments
    /// * `index` - The 0-based index to compute prefix sum up to
    ///
    /// Returns the sum of all values from 0 to index (inclusive).
    ///
    /// Time complexity: O(log N)
    pub fn prefix_sum(&self, index: usize) -> f64 {
        // Clamp index to valid range to prevent out of bounds access
        let safe_index = index.min(self.capacity().saturating_sub(1));
        let mut sum = 0.0;
        let mut idx = safe_index + 1; // Convert to 1-indexed
        while idx > 0 {
            sum += self.tree[idx];
            idx -= idx & idx.wrapping_neg(); // Remove least significant bit
        }
        sum
    }

    /// Finds the index where the cumulative sum would include the given offset.
    ///
    /// Uses binary search to find the index where the offset falls within that element's range.
    /// For example, if we have values [10, 20, 30] with cumulative sums [10, 30, 60],
    /// - offset 0-10 returns index 0 (within first element's range [0, 10])
    /// - offset 11-30 returns index 1 (within second element's range (10, 30])
    /// - offset 31-60 returns index 2 (within third element's range (30, 60])
    /// - offset > 60 returns index 2 (clamps to last valid index)
    ///
    /// # Arguments
    /// * `offset` - The target offset value
    ///
    /// Returns the 0-based index where the offset falls.
    ///
    /// Time complexity: O(log² N)
    pub fn find_index_for_offset(&self, offset: f64) -> usize {
        if offset < 0.0 {
            return 0;
        }

        let capacity = self.capacity();
        if capacity == 0 {
            return 0;
        }

        // First, find the last non-zero element by finding where the cumulative sum stops increasing
        let max_sum = self.prefix_sum(capacity - 1);

        // If offset is beyond the max sum, return the last index that contributes to the sum
        if offset >= max_sum {
            // Find the last index with a non-zero contribution
            for i in (0..capacity).rev() {
                let curr_sum = self.prefix_sum(i);
                let prev_sum = if i > 0 { self.prefix_sum(i - 1) } else { 0.0 };
                if curr_sum > prev_sum {
                    return i;
                }
            }
            return capacity - 1;
        }

        // Find the smallest index i where prefix_sum(i) >= offset
        let mut left = 0;
        let mut right = capacity - 1;

        while left < right {
            let mid = left + (right - left) / 2;

            if self.prefix_sum(mid) < offset {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }

    /// Returns the capacity of the Fenwick Tree.
    pub fn capacity(&self) -> usize {
        self.tree.len().saturating_sub(1)
    }

    /// Grows the tree to the new size, preserving existing values.
    ///
    /// # Arguments
    /// * `new_size` - The new capacity for the tree
    pub fn grow(&mut self, new_size: usize) {
        let old_capacity = self.capacity();
        if new_size > old_capacity {
            // Extract current values
            let mut values: Vec<f64> = Vec::with_capacity(new_size);
            for i in 0..old_capacity {
                let curr = self.prefix_sum(i);
                let prev = if i > 0 { self.prefix_sum(i - 1) } else { 0.0 };
                values.push(curr - prev);
            }

            // Extend with zeros
            values.resize(new_size, 0.0);

            // Rebuild tree
            self.tree = vec![0.0; new_size + 1];
            for (i, &value) in values.iter().enumerate() {
                if value != 0.0 {
                    self.update(i, value);
                }
            }
        }
    }
}

/// Spatial index for efficient row and column position lookups.
///
/// Uses Fenwick Trees to provide O(log N) lookups for:
/// - Finding which row/column a pixel offset falls into
/// - Getting the pixel offset for a given row/column
/// - Updating row heights and column widths
#[derive(Debug, Clone)]
pub struct SpatialIndex {
    /// Fenwick tree for cumulative row heights
    row_heights: FenwickTree,
    /// Fenwick tree for cumulative column widths
    col_widths: FenwickTree,
    /// Actual row sizes (indexed by row number)
    row_sizes: Vec<f64>,
    /// Actual column sizes (indexed by column number)
    col_sizes: Vec<f64>,
    /// Set of hidden row indices
    hidden_rows: HashSet<usize>,
    /// Set of hidden column indices
    hidden_cols: HashSet<usize>,
    /// Default height for new rows
    default_row_height: f64,
    /// Default width for new columns
    default_col_width: f64,
}

impl Default for SpatialIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl SpatialIndex {
    /// Default row height in pixels
    pub const DEFAULT_ROW_HEIGHT: f64 = 21.0;
    /// Default column width in pixels
    pub const DEFAULT_COL_WIDTH: f64 = 100.0;
    /// Initial row capacity
    pub const INITIAL_ROWS: usize = 1000;
    /// Initial column capacity
    pub const INITIAL_COLS: usize = 26;

    /// Creates a new SpatialIndex with default dimensions.
    ///
    /// Initializes with 1000 rows and 26 columns, using default sizes.
    pub fn new() -> Self {
        let mut index = Self {
            row_heights: FenwickTree::new(Self::INITIAL_ROWS),
            col_widths: FenwickTree::new(Self::INITIAL_COLS),
            row_sizes: vec![Self::DEFAULT_ROW_HEIGHT; Self::INITIAL_ROWS],
            col_sizes: vec![Self::DEFAULT_COL_WIDTH; Self::INITIAL_COLS],
            hidden_rows: HashSet::new(),
            hidden_cols: HashSet::new(),
            default_row_height: Self::DEFAULT_ROW_HEIGHT,
            default_col_width: Self::DEFAULT_COL_WIDTH,
        };

        // Initialize the Fenwick trees with default values
        for i in 0..Self::INITIAL_ROWS {
            index.row_heights.update(i, Self::DEFAULT_ROW_HEIGHT);
        }
        for i in 0..Self::INITIAL_COLS {
            index.col_widths.update(i, Self::DEFAULT_COL_WIDTH);
        }

        index
    }

    /// Finds which row contains the given y-offset.
    ///
    /// # Arguments
    /// * `y` - The vertical pixel offset from the top
    ///
    /// Returns the 0-based row index.
    ///
    /// Time complexity: O(log N)
    pub fn find_row_at_offset(&self, y: f64) -> usize {
        self.row_heights.find_index_for_offset(y)
    }

    /// Finds which column contains the given x-offset.
    ///
    /// # Arguments
    /// * `x` - The horizontal pixel offset from the left
    ///
    /// Returns the 0-based column index.
    ///
    /// Time complexity: O(log N)
    pub fn find_col_at_offset(&self, x: f64) -> usize {
        self.col_widths.find_index_for_offset(x)
    }

    /// Gets the y-offset (top edge) of the given row.
    ///
    /// # Arguments
    /// * `row` - The 0-based row index
    ///
    /// Returns the pixel offset from the top of the spreadsheet.
    ///
    /// Time complexity: O(log N)
    pub fn get_row_offset(&self, row: usize) -> f64 {
        if row == 0 {
            0.0
        } else {
            self.row_heights.prefix_sum(row - 1)
        }
    }

    /// Gets the x-offset (left edge) of the given column.
    ///
    /// # Arguments
    /// * `col` - The 0-based column index
    ///
    /// Returns the pixel offset from the left of the spreadsheet.
    ///
    /// Time complexity: O(log N)
    pub fn get_col_offset(&self, col: usize) -> f64 {
        if col == 0 {
            0.0
        } else {
            self.col_widths.prefix_sum(col - 1)
        }
    }

    /// Sets the height of a specific row.
    ///
    /// # Arguments
    /// * `row` - The 0-based row index
    /// * `height` - The new height in pixels
    ///
    /// Time complexity: O(log N)
    pub fn set_row_height(&mut self, row: usize, height: f64) {
        self.ensure_capacity(row + 1, 0);

        let old_height = self.row_sizes[row];
        let delta = height - old_height;

        self.row_sizes[row] = height;
        self.row_heights.update(row, delta);
    }

    /// Sets the width of a specific column.
    ///
    /// # Arguments
    /// * `col` - The 0-based column index
    /// * `width` - The new width in pixels
    ///
    /// Time complexity: O(log N)
    pub fn set_col_width(&mut self, col: usize, width: f64) {
        self.ensure_capacity(0, col + 1);

        let old_width = self.col_sizes[col];
        let delta = width - old_width;

        self.col_sizes[col] = width;
        self.col_widths.update(col, delta);
    }

    /// Gets the height of a specific row.
    ///
    /// # Arguments
    /// * `row` - The 0-based row index
    ///
    /// Returns the height in pixels, or the default height if the row doesn't exist.
    pub fn get_row_height(&self, row: usize) -> f64 {
        self.row_sizes.get(row).copied().unwrap_or(self.default_row_height)
    }

    /// Gets the width of a specific column.
    ///
    /// # Arguments
    /// * `col` - The 0-based column index
    ///
    /// Returns the width in pixels, or the default width if the column doesn't exist.
    pub fn get_col_width(&self, col: usize) -> f64 {
        self.col_sizes.get(col).copied().unwrap_or(self.default_col_width)
    }

    /// Hides the specified row.
    ///
    /// # Arguments
    /// * `row` - The 0-based row index to hide
    pub fn hide_row(&mut self, row: usize) {
        if !self.hidden_rows.contains(&row) {
            self.hidden_rows.insert(row);
            let height = self.get_row_height(row);
            self.row_heights.update(row, -height);
        }
    }

    /// Unhides the specified row.
    ///
    /// # Arguments
    /// * `row` - The 0-based row index to unhide
    pub fn unhide_row(&mut self, row: usize) {
        if self.hidden_rows.remove(&row) {
            let height = self.get_row_height(row);
            self.row_heights.update(row, height);
        }
    }

    /// Hides the specified column.
    ///
    /// # Arguments
    /// * `col` - The 0-based column index to hide
    pub fn hide_col(&mut self, col: usize) {
        if !self.hidden_cols.contains(&col) {
            self.hidden_cols.insert(col);
            let width = self.get_col_width(col);
            self.col_widths.update(col, -width);
        }
    }

    /// Unhides the specified column.
    ///
    /// # Arguments
    /// * `col` - The 0-based column index to unhide
    pub fn unhide_col(&mut self, col: usize) {
        if self.hidden_cols.remove(&col) {
            let width = self.get_col_width(col);
            self.col_widths.update(col, width);
        }
    }

    /// Ensures the spatial index has capacity for at least the given number of rows and columns.
    ///
    /// # Arguments
    /// * `rows` - Minimum number of rows needed
    /// * `cols` - Minimum number of columns needed
    pub fn ensure_capacity(&mut self, rows: usize, cols: usize) {
        // Grow rows if needed
        if rows > self.row_sizes.len() {
            let old_len = self.row_sizes.len();
            self.row_sizes.resize(rows, self.default_row_height);
            self.row_heights.grow(rows);

            // Initialize new rows with default height
            for i in old_len..rows {
                self.row_heights.update(i, self.default_row_height);
            }
        }

        // Grow columns if needed
        if cols > self.col_sizes.len() {
            let old_len = self.col_sizes.len();
            self.col_sizes.resize(cols, self.default_col_width);
            self.col_widths.grow(cols);

            // Initialize new columns with default width
            for i in old_len..cols {
                self.col_widths.update(i, self.default_col_width);
            }
        }
    }

    /// Returns whether the given row is hidden.
    pub fn is_row_hidden(&self, row: usize) -> bool {
        self.hidden_rows.contains(&row)
    }

    /// Returns whether the given column is hidden.
    pub fn is_col_hidden(&self, col: usize) -> bool {
        self.hidden_cols.contains(&col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fenwick_tree_basic() {
        let mut tree = FenwickTree::new(10);

        // Update some values
        tree.update(0, 5.0);
        tree.update(1, 3.0);
        tree.update(2, 7.0);
        tree.update(3, 2.0);

        // Test prefix sums
        assert_eq!(tree.prefix_sum(0), 5.0);
        assert_eq!(tree.prefix_sum(1), 8.0);
        assert_eq!(tree.prefix_sum(2), 15.0);
        assert_eq!(tree.prefix_sum(3), 17.0);
    }

    #[test]
    fn test_fenwick_tree_updates() {
        let mut tree = FenwickTree::new(5);

        tree.update(0, 10.0);
        tree.update(1, 10.0);
        tree.update(2, 10.0);

        assert_eq!(tree.prefix_sum(2), 30.0);

        // Update with delta
        tree.update(1, 5.0); // Add 5 more
        assert_eq!(tree.prefix_sum(2), 35.0);

        // Update with negative delta
        tree.update(0, -3.0);
        assert_eq!(tree.prefix_sum(2), 32.0);
    }

    #[test]
    fn test_fenwick_tree_find_index() {
        let mut tree = FenwickTree::new(10);

        // Create a tree with values: [10, 20, 30, 40, 50]
        tree.update(0, 10.0);
        tree.update(1, 20.0);
        tree.update(2, 30.0);
        tree.update(3, 40.0);
        tree.update(4, 50.0);

        // Cumulative: [10, 30, 60, 100, 150]
        assert_eq!(tree.find_index_for_offset(0.0), 0);
        assert_eq!(tree.find_index_for_offset(5.0), 0);
        assert_eq!(tree.find_index_for_offset(10.0), 0);
        assert_eq!(tree.find_index_for_offset(15.0), 1);
        assert_eq!(tree.find_index_for_offset(30.0), 1);
        assert_eq!(tree.find_index_for_offset(35.0), 2);
        assert_eq!(tree.find_index_for_offset(60.0), 2);
        assert_eq!(tree.find_index_for_offset(65.0), 3);
        assert_eq!(tree.find_index_for_offset(100.0), 3);
        assert_eq!(tree.find_index_for_offset(150.0), 4);
        assert_eq!(tree.find_index_for_offset(200.0), 4);
    }

    #[test]
    fn test_spatial_index_new() {
        let index = SpatialIndex::new();

        // Check initial capacity
        assert_eq!(index.row_sizes.len(), SpatialIndex::INITIAL_ROWS);
        assert_eq!(index.col_sizes.len(), SpatialIndex::INITIAL_COLS);

        // Check default values
        assert_eq!(index.get_row_height(0), SpatialIndex::DEFAULT_ROW_HEIGHT);
        assert_eq!(index.get_col_width(0), SpatialIndex::DEFAULT_COL_WIDTH);
    }

    #[test]
    fn test_spatial_index_row_operations() {
        let mut index = SpatialIndex::new();

        // Test default row offsets
        assert_eq!(index.get_row_offset(0), 0.0);
        assert_eq!(index.get_row_offset(1), 21.0);
        assert_eq!(index.get_row_offset(2), 42.0);

        // Set custom row height
        index.set_row_height(1, 50.0);
        assert_eq!(index.get_row_height(1), 50.0);
        assert_eq!(index.get_row_offset(2), 71.0); // 21 + 50

        // Test find_row_at_offset
        assert_eq!(index.find_row_at_offset(0.0), 0);
        assert_eq!(index.find_row_at_offset(20.0), 0);
        assert_eq!(index.find_row_at_offset(21.0), 0);
        assert_eq!(index.find_row_at_offset(30.0), 1);
        assert_eq!(index.find_row_at_offset(71.0), 1);
        assert_eq!(index.find_row_at_offset(80.0), 2);
    }

    #[test]
    fn test_spatial_index_col_operations() {
        let mut index = SpatialIndex::new();

        // Test default column offsets
        assert_eq!(index.get_col_offset(0), 0.0);
        assert_eq!(index.get_col_offset(1), 100.0);
        assert_eq!(index.get_col_offset(2), 200.0);

        // Set custom column width
        index.set_col_width(1, 150.0);
        assert_eq!(index.get_col_width(1), 150.0);
        assert_eq!(index.get_col_offset(2), 250.0); // 100 + 150

        // Test find_col_at_offset
        assert_eq!(index.find_col_at_offset(0.0), 0);
        assert_eq!(index.find_col_at_offset(50.0), 0);
        assert_eq!(index.find_col_at_offset(100.0), 0);
        assert_eq!(index.find_col_at_offset(150.0), 1);
        assert_eq!(index.find_col_at_offset(250.0), 1);
        assert_eq!(index.find_col_at_offset(300.0), 2);
    }

    #[test]
    fn test_spatial_index_hide_row() {
        let mut index = SpatialIndex::new();

        // Initially all rows visible
        assert!(!index.is_row_hidden(1));

        // Hide row 1
        index.hide_row(1);
        assert!(index.is_row_hidden(1));

        // Row 2 should now start where row 1 used to start
        assert_eq!(index.get_row_offset(2), 21.0); // Only row 0 counts

        // Unhide row 1
        index.unhide_row(1);
        assert!(!index.is_row_hidden(1));
        assert_eq!(index.get_row_offset(2), 42.0); // Rows 0 and 1 count
    }

    #[test]
    fn test_spatial_index_hide_col() {
        let mut index = SpatialIndex::new();

        // Initially all columns visible
        assert!(!index.is_col_hidden(1));

        // Hide column 1
        index.hide_col(1);
        assert!(index.is_col_hidden(1));

        // Column 2 should now start where column 1 used to start
        assert_eq!(index.get_col_offset(2), 100.0); // Only column 0 counts

        // Unhide column 1
        index.unhide_col(1);
        assert!(!index.is_col_hidden(1));
        assert_eq!(index.get_col_offset(2), 200.0); // Columns 0 and 1 count
    }

    #[test]
    fn test_spatial_index_ensure_capacity() {
        let mut index = SpatialIndex::new();

        // Ensure capacity for more rows and columns
        index.ensure_capacity(2000, 50);

        assert_eq!(index.row_sizes.len(), 2000);
        assert_eq!(index.col_sizes.len(), 50);

        // New rows/columns should have default sizes
        assert_eq!(index.get_row_height(1500), SpatialIndex::DEFAULT_ROW_HEIGHT);
        assert_eq!(index.get_col_width(40), SpatialIndex::DEFAULT_COL_WIDTH);
    }

    #[test]
    fn test_spatial_index_find_with_custom_sizes() {
        let mut index = SpatialIndex::new();

        // Set varying row heights
        index.set_row_height(0, 30.0);
        index.set_row_height(1, 40.0);
        index.set_row_height(2, 50.0);

        // Cumulative: [30, 70, 120, ...]
        assert_eq!(index.find_row_at_offset(0.0), 0);
        assert_eq!(index.find_row_at_offset(29.0), 0);
        assert_eq!(index.find_row_at_offset(30.0), 0);
        assert_eq!(index.find_row_at_offset(31.0), 1);
        assert_eq!(index.find_row_at_offset(69.0), 1);
        assert_eq!(index.find_row_at_offset(70.0), 1);
        assert_eq!(index.find_row_at_offset(71.0), 2);
        assert_eq!(index.find_row_at_offset(119.0), 2);
        assert_eq!(index.find_row_at_offset(120.0), 2);
        assert_eq!(index.find_row_at_offset(121.0), 3);
    }

    #[test]
    fn test_spatial_index_multiple_hides() {
        let mut index = SpatialIndex::new();

        // Set custom heights for rows 0-3
        index.set_row_height(0, 20.0);
        index.set_row_height(1, 30.0);
        index.set_row_height(2, 40.0);
        index.set_row_height(3, 50.0);

        // Hide rows 1 and 2
        index.hide_row(1);
        index.hide_row(2);

        // Offset calculation should skip hidden rows
        assert_eq!(index.get_row_offset(0), 0.0);
        assert_eq!(index.get_row_offset(1), 20.0);
        assert_eq!(index.get_row_offset(2), 20.0); // Hidden rows don't contribute
        assert_eq!(index.get_row_offset(3), 20.0);
        assert_eq!(index.get_row_offset(4), 70.0); // 20 + 50 (rows 0 and 3)
    }

    #[test]
    fn test_fenwick_tree_grow() {
        let mut tree = FenwickTree::new(5);

        tree.update(0, 10.0);
        tree.update(1, 20.0);

        assert_eq!(tree.capacity(), 5);
        assert_eq!(tree.prefix_sum(1), 30.0);

        // Grow the tree
        tree.grow(10);

        assert_eq!(tree.capacity(), 10);
        assert_eq!(tree.prefix_sum(1), 30.0); // Values preserved

        // Can now update higher indices
        tree.update(7, 5.0);
        assert_eq!(tree.prefix_sum(7), 35.0);
    }
}
