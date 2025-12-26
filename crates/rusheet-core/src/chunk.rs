//! Chunked sparse storage for efficient spreadsheet grid operations.
//!
//! This module implements a two-level sparse data structure optimized for spreadsheet grids:
//! - Top level: HashMap of chunk coordinates
//! - Bottom level: HashMap of local cell coordinates within each 16x16 chunk
//!
//! This provides O(1) random access while efficiently handling sparse data with
//! good cache locality for range queries common in spreadsheet rendering.

use std::collections::HashMap;

/// Size of each chunk in both dimensions (16x16 cells per chunk).
pub const CHUNK_SIZE: usize = 16;

/// Coordinate of a chunk in the grid.
///
/// Represents which 16x16 block a cell belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    /// Which 16-row block (row / CHUNK_SIZE)
    pub block_row: usize,
    /// Which 16-column block (col / CHUNK_SIZE)
    pub block_col: usize,
}

impl ChunkCoord {
    /// Create a ChunkCoord from a cell's global coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusheet_core::ChunkCoord;
    ///
    /// let coord = ChunkCoord::from_cell(17, 33);
    /// assert_eq!(coord.block_row, 1);
    /// assert_eq!(coord.block_col, 2);
    /// ```
    pub fn from_cell(row: usize, col: usize) -> Self {
        Self {
            block_row: row / CHUNK_SIZE,
            block_col: col / CHUNK_SIZE,
        }
    }
}

/// A single 16x16 chunk of cells.
///
/// Stores cells using local coordinates (0-15) within the chunk.
#[derive(Clone, Debug)]
pub struct Chunk<T> {
    /// Cells within this chunk, keyed by local (row, col) coordinates.
    cells: HashMap<(u8, u8), T>,
}

impl<T> Chunk<T> {
    /// Create a new empty chunk.
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }

    /// Get a reference to a cell at the given local coordinates.
    ///
    /// # Arguments
    ///
    /// * `local_row` - Row within this chunk (0-15)
    /// * `local_col` - Column within this chunk (0-15)
    pub fn get(&self, local_row: u8, local_col: u8) -> Option<&T> {
        self.cells.get(&(local_row, local_col))
    }

    /// Get a mutable reference to a cell at the given local coordinates.
    ///
    /// # Arguments
    ///
    /// * `local_row` - Row within this chunk (0-15)
    /// * `local_col` - Column within this chunk (0-15)
    pub fn get_mut(&mut self, local_row: u8, local_col: u8) -> Option<&mut T> {
        self.cells.get_mut(&(local_row, local_col))
    }

    /// Insert a value at the given local coordinates.
    ///
    /// Returns the previous value if one existed.
    ///
    /// # Arguments
    ///
    /// * `local_row` - Row within this chunk (0-15)
    /// * `local_col` - Column within this chunk (0-15)
    /// * `value` - Value to insert
    pub fn insert(&mut self, local_row: u8, local_col: u8, value: T) -> Option<T> {
        self.cells.insert((local_row, local_col), value)
    }

    /// Remove a cell at the given local coordinates.
    ///
    /// Returns the value if one existed.
    ///
    /// # Arguments
    ///
    /// * `local_row` - Row within this chunk (0-15)
    /// * `local_col` - Column within this chunk (0-15)
    pub fn remove(&mut self, local_row: u8, local_col: u8) -> Option<T> {
        self.cells.remove(&(local_row, local_col))
    }

    /// Check if this chunk is empty.
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Get the number of cells in this chunk.
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Iterate over all cells in this chunk.
    ///
    /// Returns an iterator of ((local_row, local_col), &value) tuples.
    pub fn iter(&self) -> impl Iterator<Item = ((u8, u8), &T)> {
        self.cells.iter().map(|(k, v)| (*k, v))
    }

    /// Iterate mutably over all cells in this chunk.
    ///
    /// Returns an iterator of ((local_row, local_col), &mut value) tuples.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = ((u8, u8), &mut T)> {
        self.cells.iter_mut().map(|(k, v)| (*k, v))
    }
}

impl<T> Default for Chunk<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// A chunked sparse grid for efficient spreadsheet storage.
///
/// This data structure divides the grid into 16x16 chunks and only allocates
/// storage for chunks that contain data. Provides O(1) random access while
/// minimizing memory usage for sparse data.
#[derive(Clone, Debug)]
pub struct ChunkedGrid<T> {
    /// Map of chunk coordinates to chunks.
    chunks: HashMap<ChunkCoord, Chunk<T>>,
}

impl<T: Clone> ChunkedGrid<T> {
    /// Create a new empty chunked grid.
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    /// Get a reference to a cell at the given global coordinates.
    ///
    /// # Arguments
    ///
    /// * `row` - Global row coordinate
    /// * `col` - Global column coordinate
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        let chunk_coord = ChunkCoord::from_cell(row, col);
        let (local_row, local_col) = to_local_coords(row, col);
        self.chunks.get(&chunk_coord)?.get(local_row, local_col)
    }

    /// Get a mutable reference to a cell at the given global coordinates.
    ///
    /// # Arguments
    ///
    /// * `row` - Global row coordinate
    /// * `col` - Global column coordinate
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        let chunk_coord = ChunkCoord::from_cell(row, col);
        let (local_row, local_col) = to_local_coords(row, col);
        self.chunks.get_mut(&chunk_coord)?.get_mut(local_row, local_col)
    }

    /// Insert a value at the given global coordinates.
    ///
    /// Creates a new chunk if necessary. Returns the previous value if one existed.
    ///
    /// # Arguments
    ///
    /// * `row` - Global row coordinate
    /// * `col` - Global column coordinate
    /// * `value` - Value to insert
    pub fn insert(&mut self, row: usize, col: usize, value: T) -> Option<T> {
        let chunk_coord = ChunkCoord::from_cell(row, col);
        let (local_row, local_col) = to_local_coords(row, col);
        let chunk = self.chunks.entry(chunk_coord).or_insert_with(Chunk::new);
        chunk.insert(local_row, local_col, value)
    }

    /// Remove a cell at the given global coordinates.
    ///
    /// Removes the chunk if it becomes empty. Returns the value if one existed.
    ///
    /// # Arguments
    ///
    /// * `row` - Global row coordinate
    /// * `col` - Global column coordinate
    pub fn remove(&mut self, row: usize, col: usize) -> Option<T> {
        let chunk_coord = ChunkCoord::from_cell(row, col);
        let (local_row, local_col) = to_local_coords(row, col);

        let chunk = self.chunks.get_mut(&chunk_coord)?;
        let value = chunk.remove(local_row, local_col);

        // Clean up empty chunk
        if chunk.is_empty() {
            self.chunks.remove(&chunk_coord);
        }

        value
    }

    /// Check if a cell exists at the given global coordinates.
    ///
    /// # Arguments
    ///
    /// * `row` - Global row coordinate
    /// * `col` - Global column coordinate
    pub fn contains(&self, row: usize, col: usize) -> bool {
        self.get(row, col).is_some()
    }

    /// Get the total number of cells across all chunks.
    pub fn len(&self) -> usize {
        self.chunks.values().map(|chunk| chunk.len()).sum()
    }

    /// Check if the grid is empty.
    pub fn is_empty(&self) -> bool {
        self.chunks.is_empty()
    }

    /// Get a reference to a chunk containing the given cell.
    ///
    /// Useful for batch operations or rendering.
    ///
    /// # Arguments
    ///
    /// * `row` - Global row coordinate
    /// * `col` - Global column coordinate
    pub fn get_chunk(&self, row: usize, col: usize) -> Option<&Chunk<T>> {
        let chunk_coord = ChunkCoord::from_cell(row, col);
        self.chunks.get(&chunk_coord)
    }

    /// Get or create a mutable reference to a chunk containing the given cell.
    ///
    /// Creates the chunk if it doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `row` - Global row coordinate
    /// * `col` - Global column coordinate
    pub fn get_or_create_chunk(&mut self, row: usize, col: usize) -> &mut Chunk<T> {
        let chunk_coord = ChunkCoord::from_cell(row, col);
        self.chunks.entry(chunk_coord).or_insert_with(Chunk::new)
    }

    /// Iterate over all cells in the grid.
    ///
    /// Returns an iterator of ((row, col), &value) tuples with global coordinates.
    pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.chunks.iter().flat_map(|(chunk_coord, chunk)| {
            chunk.iter().map(move |((local_row, local_col), value)| {
                let (row, col) = to_global_coords(chunk_coord, local_row, local_col);
                ((row, col), value)
            })
        })
    }

    /// Iterate mutably over all cells in the grid.
    ///
    /// Returns an iterator of ((row, col), &mut value) tuples with global coordinates.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut T)> {
        self.chunks.iter_mut().flat_map(|(chunk_coord, chunk)| {
            let chunk_coord = *chunk_coord;
            chunk.iter_mut().map(move |((local_row, local_col), value)| {
                let (row, col) = to_global_coords(&chunk_coord, local_row, local_col);
                ((row, col), value)
            })
        })
    }

    /// Get all cells within a given range.
    ///
    /// Efficiently queries only the chunks that overlap with the range.
    /// Useful for rendering visible cells.
    ///
    /// # Arguments
    ///
    /// * `start_row` - Starting row (inclusive)
    /// * `start_col` - Starting column (inclusive)
    /// * `end_row` - Ending row (inclusive)
    /// * `end_col` - Ending column (inclusive)
    pub fn cells_in_range(
        &self,
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize,
    ) -> Vec<((usize, usize), &T)> {
        let start_chunk = ChunkCoord::from_cell(start_row, start_col);
        let end_chunk = ChunkCoord::from_cell(end_row, end_col);

        let mut result = Vec::new();

        for block_row in start_chunk.block_row..=end_chunk.block_row {
            for block_col in start_chunk.block_col..=end_chunk.block_col {
                let chunk_coord = ChunkCoord { block_row, block_col };

                if let Some(chunk) = self.chunks.get(&chunk_coord) {
                    for ((local_row, local_col), value) in chunk.iter() {
                        let (row, col) = to_global_coords(&chunk_coord, local_row, local_col);

                        if row >= start_row && row <= end_row
                            && col >= start_col && col <= end_col {
                            result.push(((row, col), value));
                        }
                    }
                }
            }
        }

        result
    }
}

impl<T: Clone> Default for ChunkedGrid<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert global coordinates to local coordinates within a chunk.
///
/// # Arguments
///
/// * `row` - Global row coordinate
/// * `col` - Global column coordinate
///
/// # Returns
///
/// A tuple of (local_row, local_col) within the chunk (0-15 for each).
pub fn to_local_coords(row: usize, col: usize) -> (u8, u8) {
    let local_row = (row % CHUNK_SIZE) as u8;
    let local_col = (col % CHUNK_SIZE) as u8;
    (local_row, local_col)
}

/// Convert chunk coordinate and local coordinates to global coordinates.
///
/// # Arguments
///
/// * `chunk` - The chunk coordinate
/// * `local_row` - Row within the chunk (0-15)
/// * `local_col` - Column within the chunk (0-15)
///
/// # Returns
///
/// A tuple of (global_row, global_col).
pub fn to_global_coords(chunk: &ChunkCoord, local_row: u8, local_col: u8) -> (usize, usize) {
    let row = chunk.block_row * CHUNK_SIZE + local_row as usize;
    let col = chunk.block_col * CHUNK_SIZE + local_col as usize;
    (row, col)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_coord_from_cell() {
        assert_eq!(
            ChunkCoord::from_cell(0, 0),
            ChunkCoord { block_row: 0, block_col: 0 }
        );
        assert_eq!(
            ChunkCoord::from_cell(15, 15),
            ChunkCoord { block_row: 0, block_col: 0 }
        );
        assert_eq!(
            ChunkCoord::from_cell(16, 16),
            ChunkCoord { block_row: 1, block_col: 1 }
        );
        assert_eq!(
            ChunkCoord::from_cell(17, 33),
            ChunkCoord { block_row: 1, block_col: 2 }
        );
    }

    #[test]
    fn test_to_local_coords() {
        assert_eq!(to_local_coords(0, 0), (0, 0));
        assert_eq!(to_local_coords(15, 15), (15, 15));
        assert_eq!(to_local_coords(16, 16), (0, 0));
        assert_eq!(to_local_coords(17, 33), (1, 1));
        assert_eq!(to_local_coords(31, 47), (15, 15));
    }

    #[test]
    fn test_to_global_coords() {
        let chunk = ChunkCoord { block_row: 0, block_col: 0 };
        assert_eq!(to_global_coords(&chunk, 0, 0), (0, 0));
        assert_eq!(to_global_coords(&chunk, 15, 15), (15, 15));

        let chunk = ChunkCoord { block_row: 1, block_col: 2 };
        assert_eq!(to_global_coords(&chunk, 0, 0), (16, 32));
        assert_eq!(to_global_coords(&chunk, 1, 1), (17, 33));
        assert_eq!(to_global_coords(&chunk, 15, 15), (31, 47));
    }

    #[test]
    fn test_chunk_basic_operations() {
        let mut chunk = Chunk::new();
        assert!(chunk.is_empty());
        assert_eq!(chunk.len(), 0);

        // Insert
        assert_eq!(chunk.insert(0, 0, "A1"), None);
        assert_eq!(chunk.len(), 1);
        assert!(!chunk.is_empty());

        // Get
        assert_eq!(chunk.get(0, 0), Some(&"A1"));
        assert_eq!(chunk.get(1, 1), None);

        // Insert duplicate
        assert_eq!(chunk.insert(0, 0, "A1_updated"), Some("A1"));
        assert_eq!(chunk.len(), 1);

        // Get mut
        if let Some(val) = chunk.get_mut(0, 0) {
            *val = "A1_mutated";
        }
        assert_eq!(chunk.get(0, 0), Some(&"A1_mutated"));

        // Remove
        assert_eq!(chunk.remove(0, 0), Some("A1_mutated"));
        assert_eq!(chunk.len(), 0);
        assert!(chunk.is_empty());
        assert_eq!(chunk.remove(0, 0), None);
    }

    #[test]
    fn test_chunk_iteration() {
        let mut chunk = Chunk::new();
        chunk.insert(0, 0, 1);
        chunk.insert(5, 10, 2);
        chunk.insert(15, 15, 3);

        let mut items: Vec<_> = chunk.iter().collect();
        items.sort_by_key(|((r, c), _)| (*r, *c));

        assert_eq!(items.len(), 3);
        assert_eq!(items[0], ((0, 0), &1));
        assert_eq!(items[1], ((5, 10), &2));
        assert_eq!(items[2], ((15, 15), &3));
    }

    #[test]
    fn test_chunked_grid_basic_operations() {
        let mut grid = ChunkedGrid::new();
        assert!(grid.is_empty());
        assert_eq!(grid.len(), 0);

        // Insert
        assert_eq!(grid.insert(0, 0, "A1"), None);
        assert_eq!(grid.len(), 1);
        assert!(!grid.is_empty());
        assert!(grid.contains(0, 0));

        // Get
        assert_eq!(grid.get(0, 0), Some(&"A1"));
        assert_eq!(grid.get(1, 1), None);
        assert!(!grid.contains(1, 1));

        // Insert in different chunk
        assert_eq!(grid.insert(17, 33, "B2"), None);
        assert_eq!(grid.len(), 2);

        // Get mut
        if let Some(val) = grid.get_mut(0, 0) {
            *val = "A1_updated";
        }
        assert_eq!(grid.get(0, 0), Some(&"A1_updated"));

        // Remove
        assert_eq!(grid.remove(0, 0), Some("A1_updated"));
        assert_eq!(grid.len(), 1);
        assert!(!grid.contains(0, 0));
    }

    #[test]
    fn test_chunked_grid_chunk_creation() {
        let mut grid = ChunkedGrid::new();

        // Insert in different chunks
        grid.insert(0, 0, 1);      // Chunk (0, 0)
        grid.insert(16, 0, 2);     // Chunk (1, 0)
        grid.insert(0, 16, 3);     // Chunk (0, 1)
        grid.insert(16, 16, 4);    // Chunk (1, 1)

        assert_eq!(grid.chunks.len(), 4);
        assert_eq!(grid.len(), 4);

        assert_eq!(grid.get(0, 0), Some(&1));
        assert_eq!(grid.get(16, 0), Some(&2));
        assert_eq!(grid.get(0, 16), Some(&3));
        assert_eq!(grid.get(16, 16), Some(&4));
    }

    #[test]
    fn test_chunked_grid_chunk_cleanup() {
        let mut grid = ChunkedGrid::new();

        // Insert multiple cells in same chunk
        grid.insert(0, 0, 1);
        grid.insert(0, 1, 2);
        grid.insert(1, 0, 3);

        assert_eq!(grid.chunks.len(), 1);
        assert_eq!(grid.len(), 3);

        // Remove cells one by one
        grid.remove(0, 0);
        assert_eq!(grid.chunks.len(), 1); // Chunk still exists
        assert_eq!(grid.len(), 2);

        grid.remove(0, 1);
        assert_eq!(grid.chunks.len(), 1); // Chunk still exists
        assert_eq!(grid.len(), 1);

        // Remove last cell - chunk should be cleaned up
        grid.remove(1, 0);
        assert_eq!(grid.chunks.len(), 0); // Chunk removed
        assert_eq!(grid.len(), 0);
        assert!(grid.is_empty());
    }

    #[test]
    fn test_chunked_grid_iteration() {
        let mut grid = ChunkedGrid::new();
        grid.insert(0, 0, 1);
        grid.insert(17, 33, 2);
        grid.insert(100, 200, 3);

        let mut items: Vec<_> = grid.iter().map(|(coord, val)| (coord, *val)).collect();
        items.sort_by_key(|((r, c), _)| (*r, *c));

        assert_eq!(items.len(), 3);
        assert_eq!(items[0], ((0, 0), 1));
        assert_eq!(items[1], ((17, 33), 2));
        assert_eq!(items[2], ((100, 200), 3));
    }

    #[test]
    fn test_chunked_grid_iter_mut() {
        let mut grid = ChunkedGrid::new();
        grid.insert(0, 0, 1);
        grid.insert(17, 33, 2);
        grid.insert(100, 200, 3);

        // Multiply all values by 10
        for (_, value) in grid.iter_mut() {
            *value *= 10;
        }

        assert_eq!(grid.get(0, 0), Some(&10));
        assert_eq!(grid.get(17, 33), Some(&20));
        assert_eq!(grid.get(100, 200), Some(&30));
    }

    #[test]
    fn test_cells_in_range() {
        let mut grid = ChunkedGrid::new();

        // Insert cells in various positions
        grid.insert(0, 0, "A1");
        grid.insert(5, 5, "F6");
        grid.insert(10, 10, "K11");
        grid.insert(15, 15, "P16");
        grid.insert(20, 20, "U21");
        grid.insert(50, 50, "AY51");

        // Query range (0, 0) to (15, 15)
        let mut cells = grid.cells_in_range(0, 0, 15, 15);
        cells.sort_by_key(|((r, c), _)| (*r, *c));

        assert_eq!(cells.len(), 4);
        assert_eq!(cells[0], ((0, 0), &"A1"));
        assert_eq!(cells[1], ((5, 5), &"F6"));
        assert_eq!(cells[2], ((10, 10), &"K11"));
        assert_eq!(cells[3], ((15, 15), &"P16"));

        // Query range (10, 10) to (25, 25)
        let mut cells = grid.cells_in_range(10, 10, 25, 25);
        cells.sort_by_key(|((r, c), _)| (*r, *c));

        assert_eq!(cells.len(), 3);
        assert_eq!(cells[0], ((10, 10), &"K11"));
        assert_eq!(cells[1], ((15, 15), &"P16"));
        assert_eq!(cells[2], ((20, 20), &"U21"));

        // Query range that includes nothing
        let cells = grid.cells_in_range(30, 30, 40, 40);
        assert_eq!(cells.len(), 0);
    }

    #[test]
    fn test_cells_in_range_across_chunks() {
        let mut grid = ChunkedGrid::new();

        // Insert cells across multiple chunks
        for row in 0..64 {
            for col in 0..64 {
                if (row + col) % 10 == 0 {
                    grid.insert(row, col, row * 1000 + col);
                }
            }
        }

        // Query a range that spans multiple chunks
        let cells = grid.cells_in_range(10, 10, 30, 30);

        // Verify all cells in range are returned
        for ((row, col), value) in &cells {
            assert!(*row >= 10 && *row <= 30);
            assert!(*col >= 10 && *col <= 30);
            assert_eq!(**value, row * 1000 + col);
        }

        // Verify the count is correct
        let expected_count = (10..=30)
            .flat_map(|r| (10..=30).map(move |c| (r, c)))
            .filter(|(r, c)| (r + c) % 10 == 0)
            .count();
        assert_eq!(cells.len(), expected_count);
    }

    #[test]
    fn test_get_chunk() {
        let mut grid = ChunkedGrid::new();
        grid.insert(0, 0, 1);
        grid.insert(5, 5, 2);

        let chunk = grid.get_chunk(0, 0);
        assert!(chunk.is_some());
        assert_eq!(chunk.unwrap().len(), 2);

        let chunk = grid.get_chunk(16, 16);
        assert!(chunk.is_none());
    }

    #[test]
    fn test_get_or_create_chunk() {
        let mut grid = ChunkedGrid::new();

        // Get or create a chunk
        let chunk = grid.get_or_create_chunk(0, 0);
        chunk.insert(0, 0, 42);

        assert_eq!(grid.get(0, 0), Some(&42));
        assert_eq!(grid.chunks.len(), 1);

        // Get existing chunk
        let chunk = grid.get_or_create_chunk(5, 5);
        chunk.insert(5, 5, 99);

        assert_eq!(grid.get(5, 5), Some(&99));
        assert_eq!(grid.chunks.len(), 1); // Still same chunk
    }
}
