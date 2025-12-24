import type { CellData, CellFormat } from '../types';

// Dynamic import for WASM module
let wasmModule: typeof import('../../pkg/rusheet_wasm') | null = null;
let engine: InstanceType<typeof import('../../pkg/rusheet_wasm').SpreadsheetEngine> | null = null;

export async function initWasm(): Promise<void> {
  if (wasmModule) return;

  try {
    wasmModule = await import('../../pkg/rusheet_wasm');
    await wasmModule.default();
    engine = new wasmModule.SpreadsheetEngine();
  } catch (error) {
    console.error('Failed to load WASM module:', error);
    throw error;
  }
}

function getEngine() {
  if (!engine) {
    throw new Error('WASM module not initialized. Call initWasm() first.');
  }
  return engine;
}

export function setCellValue(row: number, col: number, value: string): [number, number][] {
  const json = getEngine().setCellValue(row, col, value);
  return JSON.parse(json);
}

export function getCellData(row: number, col: number): CellData | null {
  const data = getEngine().getCellData(row, col);
  return data as CellData | null;
}

export function getViewportData(
  startRow: number,
  endRow: number,
  startCol: number,
  endCol: number
): CellData[] {
  const json = getEngine().getViewportData(startRow, endRow, startCol, endCol);
  return JSON.parse(json);
}

export function setCellFormat(row: number, col: number, format: CellFormat): boolean {
  return getEngine().setCellFormat(row, col, JSON.stringify(format));
}

export function setRangeFormat(
  startRow: number,
  startCol: number,
  endRow: number,
  endCol: number,
  format: CellFormat
): boolean {
  return getEngine().setRangeFormat(
    startRow,
    startCol,
    endRow,
    endCol,
    JSON.stringify(format)
  );
}

export function clearRange(
  startRow: number,
  startCol: number,
  endRow: number,
  endCol: number
): [number, number][] {
  const json = getEngine().clearRange(startRow, startCol, endRow, endCol);
  return JSON.parse(json);
}

export function undo(): [number, number][] {
  const json = getEngine().undo();
  return JSON.parse(json);
}

export function redo(): [number, number][] {
  const json = getEngine().redo();
  return JSON.parse(json);
}

export function canUndo(): boolean {
  return getEngine().canUndo();
}

export function canRedo(): boolean {
  return getEngine().canRedo();
}

export function addSheet(name: string): number {
  return getEngine().addSheet(name);
}

export function setActiveSheet(index: number): boolean {
  return getEngine().setActiveSheet(index);
}

export function getSheetNames(): string[] {
  const json = getEngine().getSheetNames();
  return JSON.parse(json);
}

export function getActiveSheetIndex(): number {
  return getEngine().getActiveSheetIndex();
}

export function renameSheet(index: number, name: string): boolean {
  return getEngine().renameSheet(index, name);
}

export function deleteSheet(index: number): boolean {
  return getEngine().deleteSheet(index);
}

export function setRowHeight(row: number, height: number): void {
  getEngine().setRowHeight(row, height);
}

export function setColWidth(col: number, width: number): void {
  getEngine().setColWidth(col, width);
}

export function getRowHeight(row: number): number {
  return getEngine().getRowHeight(row);
}

export function getColWidth(col: number): number {
  return getEngine().getColWidth(col);
}

export function serialize(): string {
  return getEngine().serialize();
}

export function deserialize(json: string): boolean {
  return getEngine().deserialize(json);
}

export function recalculateAll(): void {
  getEngine().recalculateAll();
}
