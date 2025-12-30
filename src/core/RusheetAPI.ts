import { emitter } from './EventEmitter';
import * as WasmBridge from './WasmBridge';
import type { CellData, CellFormat } from '../types';
import type {
  FormatChangeEvent,
  SheetAddEvent,
  SheetDeleteEvent,
  SheetRenameEvent,
  ActiveSheetChangeEvent,
  UndoEvent,
  RedoEvent,
  EventSource,
  RowsInsertEvent,
  RowsDeleteEvent,
  ColsInsertEvent,
  ColsDeleteEvent
} from '../types/events';

export interface CellChangeEvent {
  row: number;
  col: number;
  oldValue: string | null;
  newValue: string | null;
  source: 'user' | 'api' | 'undo' | 'redo';
}

export interface SelectionChangeEvent {
  row: number;
  col: number;
  previousRow: number;
  previousCol: number;
}

export interface CellEditEvent {
  row: number;
  col: number;
  value: string;
  phase: 'start' | 'change' | 'end' | 'cancel';
}

export class RusheetAPI {
  private static instance: RusheetAPI;
  private initialized = false;
  private currentSelection = { row: 0, col: 0 };
  private currentSheetIndex = 0;
  private sheetNames: string[] = ['Sheet1'];

  private constructor() {}

  static getInstance(): RusheetAPI {
    if (!RusheetAPI.instance) {
      RusheetAPI.instance = new RusheetAPI();
    }
    return RusheetAPI.instance;
  }

  // Initialization
  async init(): Promise<void> {
    if (this.initialized) return;
    await WasmBridge.initWasm();
    this.initialized = true;
  }

  // Event subscription
  onChange(callback: (event: CellChangeEvent) => void): () => void {
    return emitter.on('change', callback);
  }

  onSelectionChange(callback: (event: SelectionChangeEvent) => void): () => void {
    return emitter.on('selectionChange', callback);
  }

  onCellEdit(callback: (event: CellEditEvent) => void): () => void {
    return emitter.on('cellEdit', callback);
  }

  onFormatChange(callback: (event: FormatChangeEvent) => void): () => void {
    return emitter.on('formatChange', callback);
  }

  onSheetAdd(callback: (event: SheetAddEvent) => void): () => void {
    return emitter.on('sheetAdd', callback);
  }

  onSheetDelete(callback: (event: SheetDeleteEvent) => void): () => void {
    return emitter.on('sheetDelete', callback);
  }

  onSheetRename(callback: (event: SheetRenameEvent) => void): () => void {
    return emitter.on('sheetRename', callback);
  }

  onActiveSheetChange(callback: (event: ActiveSheetChangeEvent) => void): () => void {
    return emitter.on('activeSheetChange', callback);
  }

  onUndo(callback: (event: UndoEvent) => void): () => void {
    return emitter.on('undo', callback);
  }

  onRedo(callback: (event: RedoEvent) => void): () => void {
    return emitter.on('redo', callback);
  }

  onRowsInsert(callback: (event: RowsInsertEvent) => void): () => void {
    return emitter.on('rowsInsert', callback);
  }

  onRowsDelete(callback: (event: RowsDeleteEvent) => void): () => void {
    return emitter.on('rowsDelete', callback);
  }

  onColsInsert(callback: (event: ColsInsertEvent) => void): () => void {
    return emitter.on('colsInsert', callback);
  }

  onColsDelete(callback: (event: ColsDeleteEvent) => void): () => void {
    return emitter.on('colsDelete', callback);
  }

  // Cell operations (wrap WasmBridge and emit events)
  setCellValue(row: number, col: number, value: string, source: 'user' | 'api' = 'api'): [number, number][] {
    const oldData = WasmBridge.getCellData(row, col);
    const oldValue = oldData?.value ?? null;
    const affected = WasmBridge.setCellValue(row, col, value);

    emitter.emit<CellChangeEvent>('change', {
      row, col, oldValue, newValue: value, source
    });

    return affected;
  }

  getCellData(row: number, col: number): CellData | null {
    return WasmBridge.getCellData(row, col);
  }

  setCellFormat(row: number, col: number, format: CellFormat, source: EventSource = 'api'): boolean {
    const success = WasmBridge.setCellFormat(row, col, format);
    if (success) {
      emitter.emit<FormatChangeEvent>('formatChange', {
        type: 'cell',
        startRow: row, startCol: col,
        endRow: row, endCol: col,
        format, source
      });
    }
    return success;
  }

  setRangeFormat(startRow: number, startCol: number, endRow: number, endCol: number, format: CellFormat, source: EventSource = 'api'): boolean {
    const success = WasmBridge.setRangeFormat(startRow, startCol, endRow, endCol, format);
    if (success) {
      emitter.emit<FormatChangeEvent>('formatChange', {
        type: 'range',
        startRow, startCol, endRow, endCol,
        format, source
      });
    }
    return success;
  }

  clearRange(startRow: number, startCol: number, endRow: number, endCol: number): [number, number][] {
    return WasmBridge.clearRange(startRow, startCol, endRow, endCol);
  }

  // Selection (emit events)
  setSelection(row: number, col: number): void {
    const previous = { ...this.currentSelection };
    this.currentSelection = { row, col };

    emitter.emit<SelectionChangeEvent>('selectionChange', {
      row, col,
      previousRow: previous.row,
      previousCol: previous.col
    });
  }

  getSelection(): { row: number; col: number } {
    return { ...this.currentSelection };
  }

  // Emit cell edit events (called by CellEditor)
  emitCellEdit(row: number, col: number, value: string, phase: CellEditEvent['phase']): void {
    emitter.emit<CellEditEvent>('cellEdit', { row, col, value, phase });
  }

  // Batch data loading
  setData(data: (string | number | null)[][]): void {
    for (let row = 0; row < data.length; row++) {
      for (let col = 0; col < data[row].length; col++) {
        const value = data[row][col];
        if (value !== null && value !== undefined && value !== '') {
          WasmBridge.setCellValue(row, col, String(value));
        }
      }
    }
    emitter.emit('dataLoaded', { rows: data.length });
  }

  // Get all data as 2D array
  getData(startRow = 0, endRow = 999, startCol = 0, endCol = 25): (string | null)[][] {
    const result: (string | null)[][] = [];
    for (let row = startRow; row <= endRow; row++) {
      const rowData: (string | null)[] = [];
      for (let col = startCol; col <= endCol; col++) {
        const cell = WasmBridge.getCellData(row, col);
        rowData.push(cell?.value ?? null);
      }
      result.push(rowData);
    }
    return result;
  }

  // Undo/Redo (emit events)
  undo(): [number, number][] {
    const affected = WasmBridge.undo();

    // Emit undo event
    emitter.emit<UndoEvent>('undo', { affectedCells: affected });

    // Also emit change events for each cell
    affected.forEach(([row, col]) => {
      const newData = WasmBridge.getCellData(row, col);
      emitter.emit<CellChangeEvent>('change', {
        row, col,
        oldValue: null,
        newValue: newData?.value ?? null,
        source: 'undo'
      });
    });
    return affected;
  }

  redo(): [number, number][] {
    const affected = WasmBridge.redo();

    // Emit redo event
    emitter.emit<RedoEvent>('redo', { affectedCells: affected });

    // Also emit change events for each cell
    affected.forEach(([row, col]) => {
      const newData = WasmBridge.getCellData(row, col);
      emitter.emit<CellChangeEvent>('change', {
        row, col,
        oldValue: null,
        newValue: newData?.value ?? null,
        source: 'redo'
      });
    });
    return affected;
  }

  canUndo(): boolean { return WasmBridge.canUndo(); }
  canRedo(): boolean { return WasmBridge.canRedo(); }

  // Serialization
  serialize(): string { return WasmBridge.serialize(); }
  deserialize(json: string): boolean { return WasmBridge.deserialize(json); }

  // Sheet management (pass through)
  addSheet(name: string, source: EventSource = 'api'): number {
    const index = WasmBridge.addSheet(name);
    this.sheetNames = WasmBridge.getSheetNames();
    emitter.emit<SheetAddEvent>('sheetAdd', { index, name, source });
    return index;
  }

  setActiveSheet(index: number, source: EventSource = 'api'): boolean {
    const previousIndex = this.currentSheetIndex;
    const previousName = this.sheetNames[previousIndex] ?? 'Sheet1';
    const success = WasmBridge.setActiveSheet(index);
    if (success) {
      this.currentSheetIndex = index;
      this.sheetNames = WasmBridge.getSheetNames();
      const newName = this.sheetNames[index] ?? 'Sheet1';
      emitter.emit<ActiveSheetChangeEvent>('activeSheetChange', {
        previousIndex, newIndex: index, previousName, newName, source
      });
    }
    return success;
  }

  getSheetNames(): string[] { return WasmBridge.getSheetNames(); }
  getActiveSheetIndex(): number { return WasmBridge.getActiveSheetIndex(); }

  renameSheet(index: number, newName: string, source: EventSource = 'api'): boolean {
    const oldName = this.sheetNames[index] ?? '';
    const success = WasmBridge.renameSheet(index, newName);
    if (success) {
      this.sheetNames = WasmBridge.getSheetNames();
      emitter.emit<SheetRenameEvent>('sheetRename', { index, oldName, newName, source });
    }
    return success;
  }

  deleteSheet(index: number, source: EventSource = 'api'): boolean {
    const name = this.sheetNames[index] ?? '';
    const success = WasmBridge.deleteSheet(index);
    if (success) {
      this.sheetNames = WasmBridge.getSheetNames();
      emitter.emit<SheetDeleteEvent>('sheetDelete', { index, name, source });
    }
    return success;
  }

  // Row/Col sizing (pass through)
  setRowHeight(row: number, height: number): void { WasmBridge.setRowHeight(row, height); }
  setColWidth(col: number, width: number): void { WasmBridge.setColWidth(col, width); }
  getRowHeight(row: number): number { return WasmBridge.getRowHeight(row); }
  getColWidth(col: number): number { return WasmBridge.getColWidth(col); }

  // Row/Column Insert/Delete
  insertRows(atRow: number, count: number, source: EventSource = 'api'): [number, number][] {
    const affected = WasmBridge.insertRows(atRow, count);
    emitter.emit<RowsInsertEvent>('rowsInsert', { atRow, count, affectedCells: affected, source });
    return affected;
  }

  deleteRows(atRow: number, count: number, source: EventSource = 'api'): [number, number][] {
    const affected = WasmBridge.deleteRows(atRow, count);
    emitter.emit<RowsDeleteEvent>('rowsDelete', { atRow, count, affectedCells: affected, source });
    return affected;
  }

  insertCols(atCol: number, count: number, source: EventSource = 'api'): [number, number][] {
    const affected = WasmBridge.insertCols(atCol, count);
    emitter.emit<ColsInsertEvent>('colsInsert', { atCol, count, affectedCells: affected, source });
    return affected;
  }

  deleteCols(atCol: number, count: number, source: EventSource = 'api'): [number, number][] {
    const affected = WasmBridge.deleteCols(atCol, count);
    emitter.emit<ColsDeleteEvent>('colsDelete', { atCol, count, affectedCells: affected, source });
    return affected;
  }

  // Viewport (pass through)
  getViewportData(startRow: number, endRow: number, startCol: number, endCol: number) {
    return WasmBridge.getViewportData(startRow, endRow, startCol, endCol);
  }
  getViewportArrays(startRow: number, endRow: number, startCol: number, endCol: number) {
    return WasmBridge.getViewportArrays(startRow, endRow, startCol, endCol);
  }

  // Cleanup
  destroy(): void {
    emitter.removeAllListeners();
    this.initialized = false;
  }
}

// Export singleton
export const rusheet = RusheetAPI.getInstance();
