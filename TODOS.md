# RuSheet Development Roadmap

**Last Updated:** 2025-12-30

This document outlines all pending work, known issues, and future features for RuSheet.

**Legend**: `[ ]` pending | `[x]` done | `[!]` blocked | `[~]` in progress

---

## Open-Source Readiness Checklist

### Documentation & Packaging (Critical)

- [x] **README.md** - Project overview, quick start, badges ✅ (2025-12-30)
  - Installation instructions (npm, cargo)
  - Basic usage examples
  - Links to documentation
  - React component example

- [x] **LICENSE file** - MIT license file in repository root ✅ (2025-12-30)

- [x] **package.json metadata** - Complete npm package info ✅ (2025-12-30)
  - [x] `description` field
  - [x] `author` field
  - [x] `repository` URL
  - [x] `homepage` (docs URL)
  - [x] `bugs` (GitHub issues URL)
  - [x] `keywords` (spreadsheet, wasm, rust, formula, etc.)

- [x] **CONTRIBUTING.md** - Contribution guidelines ✅ (2025-12-30)
  - Development setup
  - Code style
  - PR process
  - Issue templates

- [ ] **CHANGELOG.md** - Release notes history (skipped - rapid development phase)
  - Follow Keep a Changelog format
  - Semantic versioning

- [ ] **GitHub Templates**
  - [ ] ISSUE_TEMPLATE (bug report, feature request)
  - [ ] PULL_REQUEST_TEMPLATE
  - [ ] CODE_OF_CONDUCT.md

### SDK & Integration

- [ ] **Headless API** - Server-side usage without DOM
  - Node.js compatible WASM loading
  - No canvas dependency for data operations
  - Use case: Server-side formula calculation

- [x] **React Component Wrapper** - `<RuSheet />` component ✅ (2025-12-30)
  - Props: `initialData`, `onChange`, `onSelectionChange`, `collaboration`, etc.
  - `RuSheetRef` API for imperative control (getCellData, setCellValue, etc.)
  - `useRuSheet()` hook for easier ref management
  - Example: `examples/react-basic.tsx`

- [ ] **Vue Component Wrapper** - `<RuSheet />` component
  - Similar API to React wrapper

- [ ] **REST API Client SDK** - TypeScript client for rusheet-server
  - Workbook CRUD operations
  - WebSocket connection helper
  - Type-safe API calls

- [ ] **Storybook / Playground**
  - Interactive component demos
  - API exploration

- [ ] **CodeSandbox / StackBlitz Examples**
  - One-click runnable examples
  - Different framework integrations

### Package Publishing

- [ ] **Publish to npm** - `rusheet` package
  - Build pipeline for ESM/CJS/UMD
  - Type definitions included
  - README on npm page

- [ ] **Publish to crates.io**
  - [ ] `rusheet-core`
  - [ ] `rusheet-formula`
  - [ ] `rusheet-history`
  - [ ] `rusheet-wasm` (if useful standalone)

---

## Current Priority Queue

### P0: Critical (Blocks Integration)

- [x] **Implement Event/Callback System** ✅ (2025-12-30)
  - Added: `onFormatChange`, `onSheetAdd/Delete/Rename`, `onActiveSheetChange`, `onUndo/Redo`
  - Integrated `rusheet` API into main.ts and CellEditor
  - Library export via `src/index.ts`
  - 11 unit tests added

- [x] **Implement Row/Column Insert/Delete** ✅ (2025-12-30)
  - `insertRows(atRow, count)`, `deleteRows(atRow, count)`
  - `insertCols(atCol, count)`, `deleteCols(atCol, count)`
  - Formula references auto-update (respects $absolute refs)
  - Full undo/redo support with history commands
  - TypeScript events: `onRowsInsert`, `onRowsDelete`, `onColsInsert`, `onColsDelete`

- [x] **Real-time Collaboration Server** ✅ (2025-12-30)
  - Rust backend with Axum (rusheet-server crate)
  - REST API for workbook CRUD
  - WebSocket endpoint with yrs (Rust Yjs)
  - PostgreSQL persistence
  - Frontend Yjs integration
  - User presence and cursor awareness

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

- [ ] **XLSX Import/Export** - Excel compatibility
  - Use `calamine` (read) + `rust_xlsxwriter` (write)
  - Consider WASM bundle size tradeoff
  - Critical for enterprise adoption

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

- [ ] **Named Ranges**
  - Create/edit named ranges
  - Use in formulas

- [ ] **Find & Replace**
  - Search across sheets
  - Regex support

### P3: Low Priority (Nice to Have)

- [ ] **Cell Comments/Notes**
  - Add/edit/delete comments
  - Comment indicators in cells

- [ ] **Charts** (spec exists, not implemented)
  - Basic chart types (bar, line, pie)
  - Chart data range binding
  - Consider Chart.js or D3 integration

- [ ] **Print/PDF Export**
  - Print preview
  - PDF generation

- [ ] **Keyboard Shortcuts Documentation**
  - Comprehensive shortcut list
  - Customizable shortcuts

---

## Security & Authentication (Collaboration)

### Authentication

- [ ] **User Authentication**
  - JWT token-based auth
  - OAuth2 providers (Google, GitHub)
  - Session management

- [ ] **Workbook Permissions**
  - Owner, Editor, Viewer roles
  - Per-workbook access control
  - Public/private workbooks

- [ ] **Share Links**
  - Generate shareable URLs
  - Link expiration
  - Password protection

### Security

- [ ] **Rate Limiting**
  - API rate limits
  - WebSocket connection limits

- [ ] **Input Sanitization**
  - Prevent XSS in cell content
  - Formula injection protection

- [ ] **Audit Logging**
  - Track changes per user
  - Access logs

---

## Accessibility & i18n

### Accessibility (a11y)

- [ ] **Keyboard Navigation**
  - Full keyboard support documented
  - Focus indicators
  - Skip links

- [ ] **Screen Reader Support**
  - ARIA labels
  - Live regions for updates
  - Accessible grid pattern

- [ ] **High Contrast Mode**
  - Support system preferences
  - Custom high contrast theme

### Internationalization (i18n)

- [ ] **Locale Support**
  - Number formatting (decimal separator)
  - Date formatting
  - Currency formatting

- [ ] **RTL Support**
  - Right-to-left text direction
  - Mirrored UI

- [ ] **Translation Framework**
  - Externalized strings
  - Translation files

---

## Developer Experience

### Tooling

- [ ] **CI/CD Pipeline Improvements**
  - Automated releases
  - npm/crates.io publishing
  - Changelog generation

- [ ] **Pre-commit Hooks**
  - Lint on commit
  - Format on commit
  - Type check

### Documentation Site (VitePress) ✅

- [x] API reference
- [x] Getting started guide
- [x] Architecture overview
- [ ] More examples and tutorials
- [ ] Versioned docs

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
| Rust Tests | 394 | ✅ 100% passing |
| TS Unit Tests | 62 | ✅ Passing |
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

### Phase 5: Event System & Row/Col Operations ✅
- Complete event/callback system
- Row/column insert/delete with undo/redo

### Phase 6: Collaboration Server ✅
- rusheet-server crate with Axum
- Real-time sync with Yjs/yrs
- PostgreSQL persistence
- DevContainer setup

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

**Row/Column Operations:** ✅
- `insertRows(atRow, count)`, `deleteRows(atRow, count)`
- `insertCols(atCol, count)`, `deleteCols(atCol, count)`

**Events:** ✅
- `onChange`, `onSelectionChange`, `onCellEdit`
- `onFormatChange`, `onSheetAdd/Delete/Rename`, `onActiveSheetChange`
- `onUndo`, `onRedo`
- `onRowsInsert`, `onRowsDelete`, `onColsInsert`, `onColsDelete`

**Collaboration Server API:** ✅
- `GET/POST /api/workbooks` - List/create workbooks
- `GET/PUT/DELETE /api/workbooks/{id}` - Workbook CRUD
- `GET/PUT /api/workbooks/{id}/content` - Content storage
- `WS /ws/{workbook_id}` - Real-time collaboration

### Missing (See Priority Queue Above)

- CSV/XLSX import/export
- Advanced formula functions
- Headless API
- Vue component wrapper

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

rusheet-server (Collaboration backend)
  ├── rusheet-core
  ├── axum (HTTP/WebSocket)
  ├── yrs (CRDT)
  └── sqlx (PostgreSQL)

Frontend
  ├── rusheet-wasm (via pkg/)
  ├── yjs + y-websocket (collaboration)
  └── Canvas rendering
```

### Testing Principles
1. Test behavior, not implementation
2. Use real components in integration tests
3. Verify complete data flow (input → WASM → render → state)
4. Specification tests before implementation
