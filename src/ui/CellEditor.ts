import GridRenderer from '../canvas/GridRenderer';
import * as WasmBridge from '../core/WasmBridge';
import { theme } from '../canvas/theme';
import { AutocompleteEngine } from './AutocompleteEngine';
import { AutocompleteUI } from './AutocompleteUI';

export default class CellEditor {
  private container: HTMLElement;
  private renderer: GridRenderer;
  private bridge: typeof WasmBridge;
  private formulaBar: HTMLInputElement;
  private textarea: HTMLTextAreaElement;
  private isEditing: boolean = false;
  private currentRow: number = -1;
  private currentCol: number = -1;
  private onCellChange?: (row: number, col: number) => void;
  private autocompleteEngine: AutocompleteEngine;
  private autocompleteUI: AutocompleteUI;
  private autocompleteEnabled: boolean = true;
  private autocompleteDebounce: number | null = null;

  constructor(
    container: HTMLElement,
    renderer: GridRenderer,
    bridge: typeof WasmBridge,
    formulaBar: HTMLInputElement
  ) {
    this.container = container;
    this.renderer = renderer;
    this.bridge = bridge;
    this.formulaBar = formulaBar;
    this.textarea = this.createTextarea();
    this.autocompleteEngine = new AutocompleteEngine();
    this.autocompleteUI = new AutocompleteUI(container);
    this.setupEventListeners();
  }

  /**
   * Create the textarea element for inline editing
   */
  private createTextarea(): HTMLTextAreaElement {
    const textarea = document.createElement('textarea');
    textarea.style.position = 'absolute';
    textarea.style.display = 'none';
    textarea.style.border = `${theme.activeCellBorderWidth}px solid ${theme.activeCellBorder}`;
    textarea.style.outline = 'none';
    textarea.style.resize = 'none';
    textarea.style.overflow = 'hidden';
    textarea.style.padding = `${theme.cellPadding}px`;
    textarea.style.margin = '0';
    textarea.style.font = theme.cellFont;
    textarea.style.color = theme.cellTextColor;
    textarea.style.backgroundColor = theme.backgroundColor;
    textarea.style.zIndex = '1000';
    textarea.style.boxSizing = 'border-box';

    this.container.appendChild(textarea);
    return textarea;
  }

  /**
   * Setup event listeners for the textarea and formula bar
   */
  private setupEventListeners(): void {
    // Textarea events
    this.textarea.addEventListener('keydown', this.handleTextareaKeydown.bind(this));
    this.textarea.addEventListener('input', this.handleTextareaInput.bind(this));
    this.textarea.addEventListener('blur', this.handleTextareaBlur.bind(this));

    // Formula bar events
    this.formulaBar.addEventListener('keydown', this.handleFormulaBarKeydown.bind(this));
    this.formulaBar.addEventListener('input', this.handleFormulaBarInput.bind(this));
    this.formulaBar.addEventListener('focus', this.handleFormulaBarFocus.bind(this));

    // Click outside to commit
    document.addEventListener('mousedown', this.handleDocumentClick.bind(this));
  }

  /**
   * Handle textarea keydown events
   */
  private handleTextareaKeydown(event: KeyboardEvent): void {
    // Handle autocomplete shortcuts first
    if (this.autocompleteUI.isVisible()) {
      if (event.key === 'Tab') {
        event.preventDefault();
        this.acceptAutocomplete();
        return;
      }
      if (event.key === 'ArrowDown') {
        event.preventDefault();
        this.autocompleteUI.navigate(1);
        return;
      }
      if (event.key === 'ArrowUp') {
        event.preventDefault();
        this.autocompleteUI.navigate(-1);
        return;
      }
      if (event.key === 'Escape') {
        event.preventDefault();
        this.autocompleteUI.hide();
        return;
      }
    }

    // Existing key handling
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      this.commit();
      // Move down to next cell
      const activeCell = this.renderer.getActiveCell();
      this.renderer.setActiveCell(activeCell.row + 1, activeCell.col);
      if (this.onCellChange) {
        this.onCellChange(activeCell.row + 1, activeCell.col);
      }
    } else if (event.key === 'Tab') {
      event.preventDefault();
      this.commit();
      // Move right to next cell
      const activeCell = this.renderer.getActiveCell();
      const newCol = event.shiftKey ? activeCell.col - 1 : activeCell.col + 1;
      this.renderer.setActiveCell(activeCell.row, Math.max(0, newCol));
      if (this.onCellChange) {
        this.onCellChange(activeCell.row, Math.max(0, newCol));
      }
    } else if (event.key === 'Escape') {
      event.preventDefault();
      this.cancel();
    }
  }

  /**
   * Handle textarea input events
   */
  private handleTextareaInput(): void {
    // Sync with formula bar
    this.formulaBar.value = this.textarea.value;

    // Auto-resize textarea height
    this.textarea.style.height = 'auto';
    this.textarea.style.height = this.textarea.scrollHeight + 'px';

    // Trigger autocomplete with debounce
    if (this.autocompleteEnabled) {
      if (this.autocompleteDebounce !== null) {
        clearTimeout(this.autocompleteDebounce);
      }

      this.autocompleteDebounce = window.setTimeout(() => {
        this.showAutocomplete();
        this.autocompleteDebounce = null;
      }, 150);
    }
  }

  /**
   * Handle textarea blur events
   */
  private handleTextareaBlur(event: FocusEvent): void {
    // Don't commit if focus moved to formula bar
    const relatedTarget = event.relatedTarget as HTMLElement;
    if (relatedTarget === this.formulaBar) {
      return;
    }

    // Delay to allow click events to process
    setTimeout(() => {
      if (this.isEditing) {
        this.commit();
      }
    }, 100);
  }

  /**
   * Handle formula bar keydown events
   */
  private handleFormulaBarKeydown(event: KeyboardEvent): void {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      this.commit();
      // Move down to next cell
      const activeCell = this.renderer.getActiveCell();
      this.renderer.setActiveCell(activeCell.row + 1, activeCell.col);
      if (this.onCellChange) {
        this.onCellChange(activeCell.row + 1, activeCell.col);
      }
    } else if (event.key === 'Tab') {
      event.preventDefault();
      this.commit();
      // Move right to next cell
      const activeCell = this.renderer.getActiveCell();
      const newCol = event.shiftKey ? activeCell.col - 1 : activeCell.col + 1;
      this.renderer.setActiveCell(activeCell.row, Math.max(0, newCol));
      if (this.onCellChange) {
        this.onCellChange(activeCell.row, Math.max(0, newCol));
      }
    } else if (event.key === 'Escape') {
      event.preventDefault();
      this.cancel();
    }
  }

  /**
   * Handle formula bar input events
   */
  private handleFormulaBarInput(): void {
    if (this.isEditing) {
      // Sync with cell editor
      this.textarea.value = this.formulaBar.value;
      this.textarea.style.height = 'auto';
      this.textarea.style.height = this.textarea.scrollHeight + 'px';
    }
  }

  /**
   * Handle formula bar focus events
   */
  private handleFormulaBarFocus(): void {
    if (!this.isEditing) {
      const activeCell = this.renderer.getActiveCell();
      this.activate(activeCell.row, activeCell.col);
    }
  }

  /**
   * Handle document click events
   */
  private handleDocumentClick(event: MouseEvent): void {
    if (!this.isEditing) return;

    const target = event.target as HTMLElement;

    // Don't commit if clicking on textarea or formula bar
    if (target === this.textarea || target === this.formulaBar) {
      return;
    }

    // Don't commit if clicking inside container but might be on canvas
    // Let the canvas handle cell selection
    if (this.container.contains(target)) {
      this.commit();
    }
  }

  /**
   * Activate the cell editor at the specified position
   */
  public activate(row: number, col: number): void {
    this.currentRow = row;
    this.currentCol = col;
    this.isEditing = true;

    // Get current cell value
    const cellData = this.bridge.getCellData(row, col);
    const value = cellData?.formula || cellData?.value || '';

    // Set values
    this.textarea.value = value;
    this.formulaBar.value = value;

    // Position the textarea
    this.positionTextarea(row, col);

    // Show and focus the textarea
    this.textarea.style.display = 'block';
    this.textarea.focus();
    this.textarea.select();

    // Auto-resize
    this.textarea.style.height = 'auto';
    this.textarea.style.height = this.textarea.scrollHeight + 'px';
  }

  /**
   * Position the textarea over the cell
   */
  private positionTextarea(row: number, col: number): void {
    const pos = this.renderer.gridToScreen(row, col);
    const colWidth = this.bridge.getColWidth(col);
    const rowHeight = this.bridge.getRowHeight(row);

    this.textarea.style.left = `${pos.x}px`;
    this.textarea.style.top = `${pos.y}px`;
    this.textarea.style.width = `${colWidth}px`;
    this.textarea.style.minHeight = `${rowHeight}px`;
  }

  /**
   * Commit the current value and hide the editor
   */
  public commit(): void {
    if (!this.isEditing) return;

    const value = this.textarea.value;

    // Save value via bridge
    this.bridge.setCellValue(this.currentRow, this.currentCol, value);

    // Hide editor
    this.hide();
  }

  /**
   * Cancel editing and hide the editor
   */
  public cancel(): void {
    if (!this.isEditing) return;

    // Restore original value to formula bar
    const cellData = this.bridge.getCellData(this.currentRow, this.currentCol);
    const value = cellData?.formula || cellData?.value || '';
    this.formulaBar.value = value;

    // Hide editor
    this.hide();
  }

  /**
   * Hide the editor
   */
  private hide(): void {
    this.isEditing = false;
    this.textarea.style.display = 'none';
    this.textarea.value = '';
    this.currentRow = -1;
    this.currentCol = -1;
  }

  /**
   * Check if the editor is currently active
   */
  public isActive(): boolean {
    return this.isEditing;
  }

  /**
   * Set callback for cell change events
   */
  public setCellChangeCallback(callback: (row: number, col: number) => void): void {
    this.onCellChange = callback;
  }

  /**
   * Update the editor position (e.g., after scrolling)
   */
  public updatePosition(): void {
    if (this.isEditing) {
      this.positionTextarea(this.currentRow, this.currentCol);
    }
  }

  /**
   * Get the current editing position
   */
  public getCurrentPosition(): { row: number; col: number } | null {
    if (!this.isEditing) return null;
    return { row: this.currentRow, col: this.currentCol };
  }

  /**
   * Show autocomplete suggestions based on current input
   */
  private showAutocomplete(): void {
    if (!this.textarea) return;

    const text = this.textarea.value;
    const cursorPos = this.textarea.selectionStart || 0;
    const context = this.autocompleteEngine.createContext(text, cursorPos);

    if (!context.isFormula) {
      this.autocompleteUI.hide();
      return;
    }

    const suggestions = this.autocompleteEngine.getSuggestions(context);

    if (suggestions.length > 0) {
      // Calculate cursor position for dropdown placement
      const rect = this.textarea.getBoundingClientRect();
      this.autocompleteUI.show(suggestions, {
        x: rect.left,
        y: rect.bottom,
      });
    } else {
      this.autocompleteUI.hide();
    }
  }

  /**
   * Accept the currently selected autocomplete suggestion
   */
  private acceptAutocomplete(): void {
    const suggestion = this.autocompleteUI.getSelected();
    if (!suggestion || !this.textarea) return;

    const text = this.textarea.value;
    const cursorPos = this.textarea.selectionStart || 0;
    const context = this.autocompleteEngine.createContext(text, cursorPos);

    // Replace current token with suggestion
    const before = text.substring(0, context.tokenStartPos);
    const after = text.substring(cursorPos);
    const newText = before + suggestion.insertText + after;

    this.textarea.value = newText;
    this.formulaBar.value = newText;

    // Set cursor position after inserted text
    const newCursorPos = context.tokenStartPos + suggestion.insertText.length;
    this.textarea.setSelectionRange(newCursorPos, newCursorPos);

    this.autocompleteUI.hide();
    this.textarea.focus();
  }

  /**
   * Toggle autocomplete feature on or off
   */
  public toggleAutocomplete(enabled: boolean): void {
    this.autocompleteEnabled = enabled;
    if (!enabled) {
      this.autocompleteUI.hide();
    }
  }
}
