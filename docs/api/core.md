# Core API

## Cell Operations

### setCellValue

Sets a cell's value. Supports plain text, numbers, and formulas.

```typescript
setCellValue(row: number, col: number, value: string, source?: 'user' | 'api'): [number, number][]
```

**Parameters:**
- `row` - Row index (0-based)
- `col` - Column index (0-based)
- `value` - Cell value (prefix with `=` for formulas)
- `source` - Change source for event tracking (default: `'api'`)

**Returns:** Array of affected cell coordinates

**Example:**
```typescript
rusheet.setCellValue(0, 0, 'Hello');
rusheet.setCellValue(0, 1, '100');
rusheet.setCellValue(0, 2, '=B1*2');
```

### getCellData

Retrieves complete cell data including value, display value, and format.

```typescript
getCellData(row: number, col: number): CellData | null
```

**Returns:** `CellData` object or `null` if cell is empty

### setCellFormat

Applies formatting to a cell.

```typescript
setCellFormat(row: number, col: number, format: CellFormat): boolean
```

**Example:**
```typescript
rusheet.setCellFormat(0, 0, {
  bold: true,
  text_color: '#ff0000',
  horizontal_align: 'center'
});
```

### setRangeFormat

Applies formatting to a range of cells.

```typescript
setRangeFormat(startRow: number, startCol: number, endRow: number, endCol: number, format: CellFormat): boolean
```

### clearRange

Clears all cells in a range.

```typescript
clearRange(startRow: number, startCol: number, endRow: number, endCol: number): [number, number][]
```

---

## Batch Operations

### setData

Loads a 2D array of data into the spreadsheet.

```typescript
setData(data: (string | number | null)[][]): void
```

**Example:**
```typescript
rusheet.setData([
  ['Name', 'Age', 'Score'],
  ['Alice', 25, 95],
  ['Bob', 30, 88],
]);
```

### getData

Retrieves data as a 2D array.

```typescript
getData(startRow?: number, endRow?: number, startCol?: number, endCol?: number): (string | null)[][]
```

---

## History

### undo / redo

```typescript
undo(): [number, number][]
redo(): [number, number][]
canUndo(): boolean
canRedo(): boolean
```

---

## Serialization

### serialize / deserialize

```typescript
serialize(): string
deserialize(json: string): boolean
```

**Example:**
```typescript
// Save
const json = rusheet.serialize();
localStorage.setItem('spreadsheet', json);

// Load
const saved = localStorage.getItem('spreadsheet');
if (saved) rusheet.deserialize(saved);
```

---

## Sheet Management

```typescript
addSheet(name: string): number
setActiveSheet(index: number): boolean
getSheetNames(): string[]
getActiveSheetIndex(): number
renameSheet(index: number, name: string): boolean
deleteSheet(index: number): boolean
```

---

## Row/Column Sizing

```typescript
setRowHeight(row: number, height: number): void
setColWidth(col: number, width: number): void
getRowHeight(row: number): number
getColWidth(col: number): number
```
