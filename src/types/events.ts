import type { CellFormat } from './index';

// Event source type
export type EventSource = 'user' | 'api' | 'undo' | 'redo' | 'system';

// Cell change event (existing, adding timestamp)
export interface CellChangeEvent {
  row: number;
  col: number;
  oldValue: string | null;
  newValue: string | null;
  source: EventSource;
}

// Selection change event (existing)
export interface SelectionChangeEvent {
  row: number;
  col: number;
  previousRow: number;
  previousCol: number;
}

// Cell edit event (existing)
export interface CellEditEvent {
  row: number;
  col: number;
  value: string;
  phase: 'start' | 'change' | 'end' | 'cancel';
}

// Format change event (NEW)
export interface FormatChangeEvent {
  type: 'cell' | 'range';
  startRow: number;
  startCol: number;
  endRow: number;
  endCol: number;
  format: CellFormat;
  source: EventSource;
}

// Sheet events (NEW)
export interface SheetAddEvent {
  index: number;
  name: string;
  source: EventSource;
}

export interface SheetDeleteEvent {
  index: number;
  name: string;
  source: EventSource;
}

export interface SheetRenameEvent {
  index: number;
  oldName: string;
  newName: string;
  source: EventSource;
}

export interface ActiveSheetChangeEvent {
  previousIndex: number;
  newIndex: number;
  previousName: string;
  newName: string;
  source: EventSource;
}

// Undo/Redo events (NEW)
export interface UndoEvent {
  affectedCells: [number, number][];
}

export interface RedoEvent {
  affectedCells: [number, number][];
}

// Data loaded event
export interface DataLoadedEvent {
  rows: number;
}
