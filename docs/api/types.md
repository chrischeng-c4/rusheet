# Types

## CellData

Complete cell information returned by `getCellData()`.

```typescript
interface CellData {
  value: string | null;      // Original input value
  display_value: string;     // Formatted/calculated display string
  formula?: string;          // Formula expression (if applicable)
  format: CellFormat;        // Cell formatting
  row: number;               // Row index (0-based)
  col: number;               // Column index (0-based)
}
```

---

## CellFormat

Cell styling properties for `setCellFormat()`.

```typescript
interface CellFormat {
  bold?: boolean;
  italic?: boolean;
  underline?: boolean;
  font_size?: number;
  text_color?: string;           // Hex color, e.g., "#ff0000"
  background_color?: string;     // Hex color
  horizontal_align?: 'left' | 'center' | 'right';
  vertical_align?: 'top' | 'middle' | 'bottom';
}
```

---

## Event Types

```typescript
interface CellChangeEvent {
  row: number;
  col: number;
  oldValue: string | null;
  newValue: string | null;
  source: 'user' | 'api' | 'undo' | 'redo';
}

interface SelectionChangeEvent {
  row: number;
  col: number;
  previousRow: number;
  previousCol: number;
}

interface CellEditEvent {
  row: number;
  col: number;
  value: string;
  phase: 'start' | 'change' | 'end' | 'cancel';
}
```
