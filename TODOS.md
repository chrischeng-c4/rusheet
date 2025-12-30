# RuSheet Development Roadmap

**Last Updated:** 2025-12-30

This document outlines all pending work, known issues, and future features for RuSheet.

**Legend**: `[ ]` pending | `[x]` done | `[!]` blocked | `[~]` in progress

---

## Current Priority Queue

### P0: Critical (Blocks Integration)

- [ ] **Implement Event/Callback System** - External systems cannot subscribe to cell changes
  - Required for: React integration, real-time sync, undo UI feedback
  - Suggested API: `engine.on('cellChanged', callback)`

- [ ] **Implement Row/Column Insert/Delete** - Basic spreadsheet operations missing
  - `insertRows(atRow, count)`, `deleteRows(atRow, count)`
  - `insertCols(atCol, count)`, `deleteCols(atCol, count)`
  - Must update formula references when rows/cols shift

- [ ] **Structured Error Handling** - API returns strings, not error objects
  - Create `RuSheetError { code, message, affectedCells }`
  - Consistent error codes across WASM boundary

### P1: High Priority (Production Readiness)

- [ ] **CSV Import/Export** - Standard data exchange format
  - Can implement in TypeScript layer using papaparse
  - Need WASM API to bulk-set cells efficiently

- [ ] **Fix WASM Loading in Node.js Tests** - 88 integration tests blocked
  - Error: `WebAssembly.instantiate(): Argument 0 must be a buffer source`
  - Options: Browser-only tests, WASM mock, or fix ArrayBuffer conversion
  - See "Testing Issues" section below for details

- [ ] **Cross-Sheet References** - `Sheet2!A1` syntax parsed but not evaluated
  - Parser supports `SheetRef`, evaluator returns `InvalidReference`
  - Implement in `rusheet-formula/src/evaluator.rs`

### P2: Medium Priority (Feature Completeness)

- [ ] **Advanced Lookup Functions**
  - VLOOKUP, HLOOKUP, INDEX, MATCH
  - SUMIF, COUNTIF, AVERAGEIF

- [ ] **Date/Time Functions**
  - DATE, TIME, NOW, TODAY
  - DATEVALUE, TIMEVALUE
  - Date formatting in cells

- [ ] **Conditional Formatting**
  - Rule-based cell styling
  - Data bars, color scales, icon sets

- [ ] **Data Validation**
  - Input validation rules per cell
  - Dropdown lists, number ranges, custom formulas

- [ ] **Cell Merging**
  - Merge/unmerge cell ranges
  - Merged cell rendering

### P3: Low Priority (Nice to Have)

- [ ] **XLSX Import/Export**
  - Use `calamine` (read) + `rust_xlsxwriter` (write)
  - Consider WASM bundle size tradeoff

- [ ] **Cell Comments/Notes**
  - Add/edit/delete comments
  - Comment indicators in cells

- [ ] **Find & Replace**
  - Search across sheets
  - Regex support

- [ ] **Named Ranges**
  - Create/edit named ranges
  - Use in formulas

- [ ] **Charts** (spec exists, not implemented)
  - Basic chart types (bar, line, pie)
  - Chart data range binding

---

## Testing Issues

### WASM Loading in Node.js Environment

**Status:** 🔴 Blocking 88 integration tests

**Problem:**
```
TypeError: WebAssembly.instantiate(): Argument 0 must be a buffer source
  at __wbg_load (pkg/rusheet_wasm.js:470:44)
```

**Root Cause:**
- WASM wrapper uses `fetch()` to load `.wasm` file
- Node.js test environment has no HTTP server
- `fetch()` polyfill loads from filesystem
- ArrayBuffer format mismatch when converting Node Buffer → WASM ArrayBuffer

**Attempted Solutions:**
```typescript
// Attempt 1: Polyfill fetch (failed)
globalThis.fetch = async (url) => {
  const buffer = readFileSync('pkg/rusheet_wasm_bg.wasm');
  return { arrayBuffer: async () => buffer.buffer.slice(...) };
};

// Attempt 2: Uint8Array conversion (failed)
const uint8Array = new Uint8Array(buffer);
const arrayBuffer = uint8Array.buffer;
```

**Recommended Solutions (choose one):**

1. **Browser-based Tests** (Recommended)
   - Use Vitest browser mode with Playwright
   - Config exists: `vite.config.browser.ts`
   - Run: `pnpm test:integration`

2. **Split Test Strategy**
   - Unit tests in Node (fast, no WASM)
   - Integration tests in browser (slower, real WASM)

3. **WASM Mock** (Not recommended)
   - JavaScript mock of WASM API
   - Defeats purpose of integration tests

### Current Test Coverage

| Type | Count | Status |
|------|-------|--------|
| Rust Tests | 331 | ✅ 100% passing |
| TS Unit Tests | ~32 | ✅ Passing |
| TS Integration | ~88 | ⚠️ WASM blocked |
| E2E (Playwright) | 2 files | ✅ Running |

---

## Completed Phases (Historical)

### Phase 0: Test Infrastructure ✅
- Vitest browser mode configured
- CI pipeline with GitHub Actions
- 20 unit tests passing

### Phase 1: Core Data Structure Refactor ✅
- 64x64 Morton-indexed chunks with bitvec
- All 251 Rust tests pass

### Phase 2: Zero-Copy Data Bridge ✅
- ViewportBuffer with pointer accessors
- `getViewportArrays()` API in TypeScript

### Phase 3: Offscreen Rendering ✅
- Web Worker with OffscreenCanvas
- RenderController implements IGridRenderer

### Phase 4: Formula Engine Hardening ✅
- Nom-based parser with 53 tests
- 24 built-in functions (SUM, IF, CONCATENATE, etc.)

---

## API Inventory

### Currently Implemented ✅

**Cell Operations:**
- `setCellValue(row, col, value)`
- `getCellData(row, col)`
- `clearRange(startRow, startCol, endRow, endCol)`

**Formatting:**
- `setCellFormat(row, col, format)`
- `setRangeFormat(startRow, startCol, endRow, endCol, format)`

**Sheets:**
- `addSheet(name)`, `deleteSheet(index)`, `renameSheet(index, name)`
- `setActiveSheet(index)`, `getSheetNames()`

**Viewport:**
- `getViewportData()`, `populateViewport()`
- Zero-copy pointer accessors

**History:**
- `undo()`, `redo()`, `canUndo()`, `canRedo()`

**Serialization:**
- `serialize()`, `deserialize(json)`

### Missing (See P0-P3 Above)

- Event subscription system
- Row/Column insert/delete
- CSV/XLSX import/export
- Advanced formula functions

---

## Architecture Notes

### Performance Optimizations
- **Morton encoding**: O(1) cell lookup in 64x64 chunks
- **Sparse storage**: bitvec + Option<T> array
- **Zero-copy viewport**: Direct memory access from JS
- **Formula caching**: Lazy evaluation with dependency tracking

### Module Dependencies
```
rusheet-wasm (WASM bindings)
  ├── rusheet-core (cells, sheets, formatting)
  ├── rusheet-formula (parser, evaluator)
  └── rusheet-history (undo/redo commands)
```

### Testing Principles
1. Test behavior, not implementation
2. Use real components in integration tests
3. Verify complete data flow (input → WASM → render → state)
4. Specification tests before implementation
