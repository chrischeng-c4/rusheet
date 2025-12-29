# Events

Rusheet uses an event-driven architecture. All event subscriptions return an unsubscribe function.

## onChange

Fired when any cell value changes.

```typescript
onChange(callback: (event: CellChangeEvent) => void): () => void
```

**CellChangeEvent:**
```typescript
interface CellChangeEvent {
  row: number;
  col: number;
  oldValue: string | null;
  newValue: string | null;
  source: 'user' | 'api' | 'undo' | 'redo';
}
```

**Example:**
```typescript
const unsubscribe = rusheet.onChange((event) => {
  console.log(`Cell ${event.row},${event.col}: ${event.oldValue} → ${event.newValue}`);
  console.log(`Changed by: ${event.source}`);
});

// Later: stop listening
unsubscribe();
```

---

## onSelectionChange

Fired when the active cell selection changes.

```typescript
onSelectionChange(callback: (event: SelectionChangeEvent) => void): () => void
```

**SelectionChangeEvent:**
```typescript
interface SelectionChangeEvent {
  row: number;
  col: number;
  previousRow: number;
  previousCol: number;
}
```

---

## onCellEdit

Fired during the cell editing lifecycle.

```typescript
onCellEdit(callback: (event: CellEditEvent) => void): () => void
```

**CellEditEvent:**
```typescript
interface CellEditEvent {
  row: number;
  col: number;
  value: string;
  phase: 'start' | 'change' | 'end' | 'cancel';
}
```

**Lifecycle Phases:**
| Phase | Description |
|-------|-------------|
| `start` | User begins editing a cell |
| `change` | Value changes during editing |
| `end` | Editing completed (value committed) |
| `cancel` | Editing cancelled (Escape pressed) |

**Example:**
```typescript
rusheet.onCellEdit((event) => {
  if (event.phase === 'end') {
    console.log(`User finished editing ${event.row},${event.col}: ${event.value}`);
  }
});
```
