# Keyboard Shortcuts Specification

## Overview

This specification defines comprehensive keyboard shortcuts for RuSheet, covering navigation, selection, editing, clipboard operations, formatting, formulas, and view controls. Shortcuts are designed to match Excel and Google Sheets conventions while supporting platform-specific variations.

## Shortcut Categories

### Navigation

| Action | Windows/Linux | macOS | Description |
|--------|---------------|-------|-------------|
| Move up one cell | `↑` | `↑` | Move active cell up |
| Move down one cell | `↓` | `↓` | Move active cell down |
| Move left one cell | `←` | `←` | Move active cell left |
| Move right one cell | `→` | `→` | Move active cell right |
| Move to edge of data | `Ctrl + ↑/↓/←/→` | `Cmd + ↑/↓/←/→` | Jump to edge of continuous data |
| Move to cell A1 | `Ctrl + Home` | `Cmd + Home` | Jump to top-left cell |
| Move to last cell | `Ctrl + End` | `Cmd + End` | Jump to bottom-right used cell |
| Page up | `Page Up` | `Page Up` | Scroll up one screen |
| Page down | `Page Down` | `Page Down` | Scroll down one screen |
| Move to next sheet | `Ctrl + Page Down` | `Cmd + Page Down` | Switch to next worksheet |
| Move to previous sheet | `Ctrl + Page Up` | `Cmd + Page Up` | Switch to previous worksheet |
| Move right (Tab) | `Tab` | `Tab` | Move to next cell, confirm edit |
| Move left (Shift+Tab) | `Shift + Tab` | `Shift + Tab` | Move to previous cell |

### Selection

| Action | Windows/Linux | macOS | Description |
|--------|---------------|-------|-------------|
| Extend selection | `Shift + ↑/↓/←/→` | `Shift + ↑/↓/←/→` | Extend selection in direction |
| Extend to edge | `Ctrl + Shift + ↑/↓/←/→` | `Cmd + Shift + ↑/↓/←/→` | Extend to edge of data |
| Select all | `Ctrl + A` | `Cmd + A` | Select entire sheet |
| Select row | `Shift + Space` | `Shift + Space` | Select entire row |
| Select column | `Ctrl + Space` | `Ctrl + Space` | Select entire column |
| Select to beginning | `Shift + Home` | `Shift + Home` | Select from cursor to row start |
| Add to selection | `Ctrl + Click` | `Cmd + Click` | Add cell/range to selection |

### Editing

| Action | Windows/Linux | macOS | Description |
|--------|---------------|-------|-------------|
| Edit active cell | `F2` | `F2` | Enter edit mode |
| Enter edit mode | Type any character | Type any character | Replace cell content |
| Cancel edit | `Esc` | `Esc` | Cancel current edit |
| Confirm edit (down) | `Enter` | `Enter` | Confirm and move down |
| Confirm edit (right) | `Tab` | `Tab` | Confirm and move right |
| Delete content | `Delete` | `Delete` | Clear cell content |
| Clear content | `Backspace` | `Backspace` | Clear and enter edit mode |
| Fill down | `Ctrl + D` | `Cmd + D` | Copy top cell to selected range |
| Fill right | `Ctrl + R` | `Cmd + R` | Copy left cell to selected range |
| Insert current date | `Ctrl + ;` | `Cmd + ;` | Insert today's date |
| Insert current time | `Ctrl + Shift + ;` | `Cmd + Shift + ;` | Insert current time |
| New line in cell | `Alt + Enter` | `Option + Enter` | Insert line break in cell |

### Clipboard

| Action | Windows/Linux | macOS | Description |
|--------|---------------|-------|-------------|
| Cut | `Ctrl + X` | `Cmd + X` | Cut selection |
| Copy | `Ctrl + C` | `Cmd + C` | Copy selection |
| Paste | `Ctrl + V` | `Cmd + V` | Paste from clipboard |
| Paste special | `Ctrl + Shift + V` | `Cmd + Shift + V` | Paste with options |
| Paste values only | `Ctrl + Alt + V` | `Cmd + Option + V` | Paste only values |

### Formatting

| Action | Windows/Linux | macOS | Description |
|--------|---------------|-------|-------------|
| Bold | `Ctrl + B` | `Cmd + B` | Toggle bold |
| Italic | `Ctrl + I` | `Cmd + I` | Toggle italic |
| Underline | `Ctrl + U` | `Cmd + U` | Toggle underline |
| Strikethrough | `Ctrl + Shift + X` | `Cmd + Shift + X` | Toggle strikethrough |
| Format as currency | `Ctrl + Shift + $` | `Cmd + Shift + $` | Apply currency format |
| Format as percentage | `Ctrl + Shift + %` | `Cmd + Shift + %` | Apply percentage format |
| Format as number | `Ctrl + Shift + !` | `Cmd + Shift + !` | Apply number format |
| Format as date | `Ctrl + Shift + #` | `Cmd + Shift + #` | Apply date format |
| Format as time | `Ctrl + Shift + @` | `Cmd + Shift + @` | Apply time format |
| Increase decimals | `Ctrl + Shift + >` | `Cmd + Shift + >` | Add decimal place |
| Decrease decimals | `Ctrl + Shift + <` | `Cmd + Shift + <` | Remove decimal place |

### Formulas

| Action | Windows/Linux | macOS | Description |
|--------|---------------|-------|-------------|
| Start formula | `=` | `=` | Begin formula entry |
| AutoSum | `Alt + =` | `Option + =` | Insert SUM formula |
| Toggle formula view | `Ctrl + ~` | `Cmd + ~` | Show/hide formulas |
| Recalculate all | `Ctrl + Shift + F9` | `Cmd + Shift + F9` | Force recalculation |
| Evaluate formula | `F9` | `F9` | Evaluate selected part |
| Toggle absolute ref | `F4` | `F4` | Cycle A1 → $A$1 → A$1 → $A1 |
| Insert function | `Shift + F3` | `Shift + F3` | Open function picker |

### Rows and Columns

| Action | Windows/Linux | macOS | Description |
|--------|---------------|-------|-------------|
| Insert row | `Ctrl + Shift + +` (with row selected) | `Cmd + Shift + +` | Insert row above |
| Insert column | `Ctrl + Shift + +` (with col selected) | `Cmd + Shift + +` | Insert column left |
| Delete row | `Ctrl + -` (with row selected) | `Cmd + -` | Delete selected rows |
| Delete column | `Ctrl + -` (with col selected) | `Cmd + -` | Delete selected columns |
| Hide rows | `Ctrl + 9` | `Cmd + 9` | Hide selected rows |
| Hide columns | `Ctrl + 0` | `Cmd + 0` | Hide selected columns |
| Unhide rows | `Ctrl + Shift + 9` | `Cmd + Shift + 9` | Unhide rows |
| Unhide columns | `Ctrl + Shift + 0` | `Cmd + Shift + 0` | Unhide columns |

### View

| Action | Windows/Linux | macOS | Description |
|--------|---------------|-------|-------------|
| Zoom in | `Ctrl + +` | `Cmd + +` | Increase zoom |
| Zoom out | `Ctrl + -` | `Cmd + -` | Decrease zoom |
| Zoom 100% | `Ctrl + 0` | `Cmd + 0` | Reset zoom to 100% |
| Full screen | `F11` | `Cmd + Ctrl + F` | Toggle full screen |
| Freeze panes | `Alt + W, F` | `Option + W, F` | Freeze rows/columns |

### File Operations

| Action | Windows/Linux | macOS | Description |
|--------|---------------|-------|-------------|
| New workbook | `Ctrl + N` | `Cmd + N` | Create new workbook |
| Open | `Ctrl + O` | `Cmd + O` | Open file dialog |
| Save | `Ctrl + S` | `Cmd + S` | Save current file |
| Save as | `Ctrl + Shift + S` | `Cmd + Shift + S` | Save with new name |
| Print | `Ctrl + P` | `Cmd + P` | Open print dialog |
| Close | `Ctrl + W` | `Cmd + W` | Close current workbook |

### Find and Replace

| Action | Windows/Linux | macOS | Description |
|--------|---------------|-------|-------------|
| Find | `Ctrl + F` | `Cmd + F` | Open find dialog |
| Replace | `Ctrl + H` | `Cmd + H` | Open replace dialog |
| Find next | `F3` or `Ctrl + G` | `Cmd + G` | Find next occurrence |
| Find previous | `Shift + F3` or `Ctrl + Shift + G` | `Cmd + Shift + G` | Find previous occurrence |

### Undo/Redo

| Action | Windows/Linux | macOS | Description |
|--------|---------------|-------|-------------|
| Undo | `Ctrl + Z` | `Cmd + Z` | Undo last action |
| Redo | `Ctrl + Y` or `Ctrl + Shift + Z` | `Cmd + Shift + Z` | Redo last undone action |

## Implementation Architecture

### Keyboard Event Handler

```typescript
// frontend/src/keyboard/shortcuts.ts

export type KeyboardShortcut = {
  key: string;
  ctrlKey?: boolean;
  shiftKey?: boolean;
  altKey?: boolean;
  metaKey?: boolean;
  action: string;
  preventDefault?: boolean;
};

export class ShortcutManager {
  private shortcuts: Map<string, KeyboardShortcut[]> = new Map();
  private platform: 'mac' | 'windows' | 'linux';

  constructor() {
    this.platform = this.detectPlatform();
    this.registerDefaultShortcuts();
  }

  private detectPlatform(): 'mac' | 'windows' | 'linux' {
    const ua = navigator.userAgent.toLowerCase();
    if (ua.indexOf('mac') !== -1) return 'mac';
    if (ua.indexOf('linux') !== -1) return 'linux';
    return 'windows';
  }

  register(shortcut: KeyboardShortcut): void {
    const key = this.getShortcutKey(shortcut);

    if (!this.shortcuts.has(key)) {
      this.shortcuts.set(key, []);
    }

    this.shortcuts.get(key)!.push(shortcut);
  }

  private getShortcutKey(shortcut: KeyboardShortcut): string {
    const parts: string[] = [];

    if (shortcut.ctrlKey) parts.push('ctrl');
    if (shortcut.shiftKey) parts.push('shift');
    if (shortcut.altKey) parts.push('alt');
    if (shortcut.metaKey) parts.push('meta');
    parts.push(shortcut.key.toLowerCase());

    return parts.join('+');
  }

  handleKeyDown(event: KeyboardEvent): boolean {
    const key = this.eventToShortcutKey(event);
    const shortcuts = this.shortcuts.get(key);

    if (shortcuts && shortcuts.length > 0) {
      const shortcut = shortcuts[0]; // Take first matching shortcut

      if (shortcut.preventDefault !== false) {
        event.preventDefault();
      }

      this.executeAction(shortcut.action);
      return true;
    }

    return false;
  }

  private eventToShortcutKey(event: KeyboardEvent): string {
    const parts: string[] = [];

    // On Mac, Cmd key is used instead of Ctrl
    if (this.platform === 'mac') {
      if (event.metaKey) parts.push('ctrl'); // Map Cmd to ctrl
    } else {
      if (event.ctrlKey) parts.push('ctrl');
    }

    if (event.shiftKey) parts.push('shift');
    if (event.altKey) parts.push('alt');

    parts.push(event.key.toLowerCase());

    return parts.join('+');
  }

  private executeAction(action: string): void {
    // Dispatch custom event with action
    window.dispatchEvent(new CustomEvent('shortcut-action', {
      detail: { action },
    }));
  }

  private registerDefaultShortcuts(): void {
    // Navigation
    this.register({ key: 'ArrowUp', action: 'navigate-up' });
    this.register({ key: 'ArrowDown', action: 'navigate-down' });
    this.register({ key: 'ArrowLeft', action: 'navigate-left' });
    this.register({ key: 'ArrowRight', action: 'navigate-right' });

    this.register({ key: 'ArrowUp', ctrlKey: true, action: 'navigate-edge-up' });
    this.register({ key: 'ArrowDown', ctrlKey: true, action: 'navigate-edge-down' });
    this.register({ key: 'ArrowLeft', ctrlKey: true, action: 'navigate-edge-left' });
    this.register({ key: 'ArrowRight', ctrlKey: true, action: 'navigate-edge-right' });

    this.register({ key: 'Home', ctrlKey: true, action: 'navigate-home' });
    this.register({ key: 'End', ctrlKey: true, action: 'navigate-end' });

    // Selection
    this.register({ key: 'ArrowUp', shiftKey: true, action: 'select-up' });
    this.register({ key: 'ArrowDown', shiftKey: true, action: 'select-down' });
    this.register({ key: 'ArrowLeft', shiftKey: true, action: 'select-left' });
    this.register({ key: 'ArrowRight', shiftKey: true, action: 'select-right' });

    this.register({ key: 'a', ctrlKey: true, action: 'select-all' });
    this.register({ key: ' ', shiftKey: true, action: 'select-row' });
    this.register({ key: ' ', ctrlKey: true, action: 'select-column' });

    // Editing
    this.register({ key: 'F2', action: 'edit-cell' });
    this.register({ key: 'Escape', action: 'cancel-edit' });
    this.register({ key: 'Enter', action: 'confirm-edit-down' });
    this.register({ key: 'Tab', action: 'confirm-edit-right' });
    this.register({ key: 'Delete', action: 'delete-content' });

    this.register({ key: 'd', ctrlKey: true, action: 'fill-down' });
    this.register({ key: 'r', ctrlKey: true, action: 'fill-right' });

    // Clipboard
    this.register({ key: 'x', ctrlKey: true, action: 'cut' });
    this.register({ key: 'c', ctrlKey: true, action: 'copy' });
    this.register({ key: 'v', ctrlKey: true, action: 'paste' });

    // Formatting
    this.register({ key: 'b', ctrlKey: true, action: 'format-bold' });
    this.register({ key: 'i', ctrlKey: true, action: 'format-italic' });
    this.register({ key: 'u', ctrlKey: true, action: 'format-underline' });

    // Formulas
    this.register({ key: '=', altKey: true, action: 'insert-sum' });
    this.register({ key: 'F4', action: 'toggle-absolute-ref' });

    // Undo/Redo
    this.register({ key: 'z', ctrlKey: true, action: 'undo' });
    this.register({ key: 'y', ctrlKey: true, action: 'redo' });

    // File
    this.register({ key: 's', ctrlKey: true, action: 'save' });
    this.register({ key: 'o', ctrlKey: true, action: 'open' });
    this.register({ key: 'n', ctrlKey: true, action: 'new' });

    // Find
    this.register({ key: 'f', ctrlKey: true, action: 'find' });
    this.register({ key: 'h', ctrlKey: true, action: 'replace' });
  }

  getShortcutsForAction(action: string): KeyboardShortcut[] {
    const result: KeyboardShortcut[] = [];

    for (const shortcuts of this.shortcuts.values()) {
      for (const shortcut of shortcuts) {
        if (shortcut.action === action) {
          result.push(shortcut);
        }
      }
    }

    return result;
  }

  getShortcutDisplay(shortcut: KeyboardShortcut): string {
    const parts: string[] = [];

    if (this.platform === 'mac') {
      if (shortcut.ctrlKey) parts.push('⌘');
      if (shortcut.shiftKey) parts.push('⇧');
      if (shortcut.altKey) parts.push('⌥');
    } else {
      if (shortcut.ctrlKey) parts.push('Ctrl');
      if (shortcut.shiftKey) parts.push('Shift');
      if (shortcut.altKey) parts.push('Alt');
    }

    parts.push(this.getKeyDisplay(shortcut.key));

    return parts.join('+');
  }

  private getKeyDisplay(key: string): string {
    const keyMap: Record<string, string> = {
      'ArrowUp': '↑',
      'ArrowDown': '↓',
      'ArrowLeft': '←',
      'ArrowRight': '→',
      ' ': 'Space',
    };

    return keyMap[key] || key.toUpperCase();
  }
}
```

### Action Handler Integration

```typescript
// frontend/src/keyboard/action-handler.ts

export class ShortcutActionHandler {
  constructor(
    private engine: WasmEngineWrapper,
    private selection: SelectionManager,
    private navigator: KeyboardNavigator
  ) {
    this.registerActionHandlers();
  }

  private registerActionHandlers(): void {
    window.addEventListener('shortcut-action', (e: Event) => {
      const customEvent = e as CustomEvent;
      const { action } = customEvent.detail;

      this.handleAction(action);
    });
  }

  private handleAction(action: string): void {
    switch (action) {
      // Navigation
      case 'navigate-up':
        this.navigator.navigate(this.selection, 'up', false);
        break;
      case 'navigate-down':
        this.navigator.navigate(this.selection, 'down', false);
        break;
      case 'navigate-left':
        this.navigator.navigate(this.selection, 'left', false);
        break;
      case 'navigate-right':
        this.navigator.navigate(this.selection, 'right', false);
        break;

      // Selection
      case 'select-up':
        this.navigator.navigate(this.selection, 'up', true);
        break;
      case 'select-down':
        this.navigator.navigate(this.selection, 'down', true);
        break;
      case 'select-all':
        this.selection.selectAll();
        break;

      // Editing
      case 'edit-cell':
        this.selection.enterEditMode();
        break;
      case 'cancel-edit':
        this.selection.cancelEdit();
        break;

      // Clipboard
      case 'copy':
        this.handleCopy();
        break;
      case 'paste':
        this.handlePaste();
        break;

      // Undo/Redo
      case 'undo':
        this.engine.undo();
        break;
      case 'redo':
        this.engine.redo();
        break;

      default:
        console.warn('Unhandled shortcut action:', action);
    }
  }

  private handleCopy(): void {
    const cells = this.selection.getSelectedCells();
    const data = this.engine.getCellValues(cells);

    // Copy to clipboard
    navigator.clipboard.writeText(JSON.stringify(data));
  }

  private handlePaste(): void {
    navigator.clipboard.readText().then((text) => {
      try {
        const data = JSON.parse(text);
        this.engine.setCellValues(data);
      } catch (e) {
        // Plain text paste
        const activeCell = this.selection.getActiveCell();
        this.engine.setCellValue(activeCell.row, activeCell.col, text);
      }
    });
  }
}
```

## Accessibility Considerations

- All shortcuts should work with screen readers
- Provide visual feedback for shortcut actions
- Support custom shortcut configuration for users with disabilities
- Ensure keyboard-only navigation is fully functional
- Provide shortcut help overlay (Ctrl+/ or Cmd+/)

## Testing

```typescript
// frontend/tests/keyboard-shortcuts.test.ts

import { ShortcutManager } from '../src/keyboard/shortcuts';

describe('ShortcutManager', () => {
  let manager: ShortcutManager;

  beforeEach(() => {
    manager = new ShortcutManager();
  });

  it('should detect platform correctly', () => {
    expect(manager['platform']).toMatch(/mac|windows|linux/);
  });

  it('should handle Ctrl+C for copy', () => {
    const event = new KeyboardEvent('keydown', {
      key: 'c',
      ctrlKey: true,
    });

    const handled = manager.handleKeyDown(event);
    expect(handled).toBe(true);
  });

  it('should convert shortcut to display format', () => {
    const shortcut = { key: 'c', ctrlKey: true, action: 'copy' };
    const display = manager.getShortcutDisplay(shortcut);

    if (manager['platform'] === 'mac') {
      expect(display).toBe('⌘+C');
    } else {
      expect(display).toBe('Ctrl+C');
    }
  });
});
```

## References

- [Excel Keyboard Shortcuts](https://support.microsoft.com/en-us/office/keyboard-shortcuts-in-excel-1798d9d5-842a-42b8-9c99-9b7213f0040f)
- [Google Sheets Shortcuts](https://support.google.com/docs/answer/181110)
- [KeyboardEvent - MDN](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent)
- [Accessibility Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
