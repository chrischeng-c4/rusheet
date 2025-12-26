export interface CellPosition {
  row: number;
  col: number;
}

export interface CellRange {
  startRow: number;
  endRow: number;
  startCol: number;
  endCol: number;
}

export interface GridTheme {
  headerBackground: string;
  headerFont: string;
  headerTextColor: string;
  cellFont: string;
  cellTextColor: string;
  gridLineColor: string;
  activeCellBorder: string;
  selectionBorderWidth: number;
  defaultColWidth: number;
  defaultRowHeight: number;
  headerHeight: number;
  headerWidth: number;
  cellPadding: number;
  selectionColor: string;
}

export const defaultTheme: GridTheme = {
  headerBackground: '#f3f4f6',
  headerFont: 'bold 12px Arial',
  headerTextColor: '#374151',
  cellFont: '13px Arial',
  cellTextColor: '#111827',
  gridLineColor: '#e5e7eb',
  activeCellBorder: '#2563eb',
  selectionBorderWidth: 2,
  defaultColWidth: 100,
  defaultRowHeight: 24,
  headerHeight: 24,
  headerWidth: 50,
  cellPadding: 4,
  selectionColor: 'rgba(37, 99, 235, 0.1)',
};

export interface ViewportState {
  scrollX: number;
  scrollY: number;
  width: number;
  height: number;
}
