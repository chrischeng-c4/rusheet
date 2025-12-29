# API Overview

Rusheet provides a singleton API instance for all operations.

```typescript
import { rusheet } from 'rusheet';

await rusheet.init();
```

## API Categories

| Category | Description |
|----------|-------------|
| [Core API](./core) | Cell operations, formatting, sheets |
| [Events](./events) | Change, selection, edit callbacks |
| [Types](./types) | TypeScript interfaces |

## Quick Reference

```typescript
// Cell Operations
rusheet.setCellValue(row, col, value)
rusheet.getCellData(row, col)
rusheet.setCellFormat(row, col, format)
rusheet.clearRange(startRow, startCol, endRow, endCol)

// Events
rusheet.onChange(callback)
rusheet.onSelectionChange(callback)
rusheet.onCellEdit(callback)

// Batch Operations
rusheet.setData(data[][])
rusheet.getData(startRow, endRow, startCol, endCol)

// History
rusheet.undo()
rusheet.redo()

// Serialization
rusheet.serialize()
rusheet.deserialize(json)
```
