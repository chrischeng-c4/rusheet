export interface CellData {
  value: string | null;
  display_value: string;
  formula?: string;
  format: CellFormat;
  row: number;
  col: number;
}

export interface CellFormat {
  bold?: boolean;
  italic?: boolean;
  underline?: boolean;
  font_size?: number;
  text_color?: string;
  background_color?: string;
  horizontal_align?: 'left' | 'center' | 'right';
  vertical_align?: 'top' | 'middle' | 'bottom';
}

export interface Selection {
  startRow: number;
  startCol: number;
  endRow: number;
  endCol: number;
}

export interface Viewport {
  scrollX: number;
  scrollY: number;
  startRow: number;
  endRow: number;
  startCol: number;
  endCol: number;
  width: number;
  height: number;
}
