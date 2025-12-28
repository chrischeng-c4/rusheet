import * as WasmBridge from './WasmBridge';

export class PersistenceManager {
  private static readonly STORAGE_KEY = 'rusheet_workbook';
  private static readonly AUTO_SAVE_DELAY_MS = 1000;
  private saveTimeout: number | null = null;

  /**
   * Load workbook from localStorage
   * Returns true if data was loaded successfully
   */
  public load(): boolean {
    try {
      const json = localStorage.getItem(PersistenceManager.STORAGE_KEY);
      if (!json) {
        console.log('[Persistence] No saved data found');
        return false;
      }

      console.log('[Persistence] Loading workbook from localStorage...');
      const success = WasmBridge.deserialize(json);

      if (success) {
        console.log('[Persistence] Workbook loaded successfully');
      } else {
        console.error('[Persistence] Failed to deserialize workbook data');
      }

      return success;
    } catch (error) {
      console.error('[Persistence] Error loading workbook:', error);
      return false;
    }
  }

  /**
   * Save workbook to localStorage immediately
   */
  public save(): boolean {
    try {
      const json = WasmBridge.serialize();
      localStorage.setItem(PersistenceManager.STORAGE_KEY, json);
      console.log('[Persistence] Workbook saved to localStorage');
      return true;
    } catch (error) {
      console.error('[Persistence] Error saving workbook:', error);
      return false;
    }
  }

  /**
   * Schedule auto-save with debounce
   * Cancels previous pending save if called again within delay
   */
  public scheduleSave(): void {
    // Clear existing timeout
    if (this.saveTimeout !== null) {
      clearTimeout(this.saveTimeout);
    }

    // Schedule new save
    this.saveTimeout = window.setTimeout(() => {
      this.save();
      this.saveTimeout = null;
    }, PersistenceManager.AUTO_SAVE_DELAY_MS);
  }

  /**
   * Clear saved data from localStorage
   */
  public clear(): boolean {
    try {
      localStorage.removeItem(PersistenceManager.STORAGE_KEY);
      console.log('[Persistence] Cleared saved workbook data');
      return true;
    } catch (error) {
      console.error('[Persistence] Error clearing workbook:', error);
      return false;
    }
  }

  /**
   * Export workbook as downloadable file
   */
  public exportToFile(filename: string = 'workbook.json'): void {
    try {
      const json = WasmBridge.serialize();
      const blob = new Blob([json], { type: 'application/json' });
      const url = URL.createObjectURL(blob);

      const link = document.createElement('a');
      link.href = url;
      link.download = filename;
      link.click();

      URL.revokeObjectURL(url);
      console.log('[Persistence] Exported workbook to file');
    } catch (error) {
      console.error('[Persistence] Error exporting workbook:', error);
    }
  }

  /**
   * Import workbook from file
   */
  public importFromFile(file: File, onComplete: (success: boolean) => void): void {
    const reader = new FileReader();

    reader.onload = (e) => {
      try {
        const json = e.target?.result as string;
        const success = WasmBridge.deserialize(json);

        if (success) {
          console.log('[Persistence] Imported workbook from file');
          // Save to localStorage after successful import
          this.save();
        } else {
          console.error('[Persistence] Failed to import workbook');
        }

        onComplete(success);
      } catch (error) {
        console.error('[Persistence] Error importing workbook:', error);
        onComplete(false);
      }
    };

    reader.onerror = () => {
      console.error('[Persistence] Error reading file');
      onComplete(false);
    };

    reader.readAsText(file);
  }
}
