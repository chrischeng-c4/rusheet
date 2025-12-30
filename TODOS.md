# RuSheet Development Roadmap

**Last Updated:** 2025-12-30

This document outlines all pending work, known issues, and future features for RuSheet.

**Legend**: `[ ]` pending | `[x]` done | `[!]` blocked | `[~]` in progress

**Target Users:** 開發者（嵌入式）+ 企業用戶

---

## Google Sheets 差距分析

### 公式功能 (24 個 vs Google 400+)
| 缺少功能 | 優先級 |
|----------|--------|
| 跨工作表引用 `Sheet2!A1` | P1 | ✅ 已完成 |
| 陣列公式 `ARRAYFORMULA` | P2 |
| 命名範圍 | P2 |
| COUNTIF, SUMIF, AVERAGEIF | P1 | ✅ 已完成 |
| DATE, TODAY, NOW, DATEDIF | P1 |
| INDEX, MATCH, OFFSET | P2 |
| FIND, SEARCH, SUBSTITUTE | P2 |

### 資料功能
| 缺少功能 | 優先級 | 狀態 |
|----------|--------|------|
| 排序（單欄/多欄）| P1 | ✅ 已完成 |
| 合併儲存格 | P1 | ✅ 已完成 |
| 篩選/自動篩選 | P1 | ❌ |
| 條件格式 | P2 | ❌ |
| 資料驗證（下拉選單）| P2 | ❌ |
| 樞紐分析表 | P3 | ❌ |
| 圖表 | P3 | ❌ |

### 編輯功能
| 缺少功能 | 優先級 |
|----------|--------|
| 合併儲存格 | P1 |
| 尋找和取代 | P2 |
| 特殊貼上（僅值、轉置）| P2 |
| 自動填入 | P2 |

### 匯入/匯出
| 缺少功能 | 優先級 | 狀態 |
|----------|--------|------|
| CSV 匯入/匯出 | P0 | ✅ 已完成 |
| Excel 匯入 (.xlsx) | P1 | ✅ 已完成 |
| Excel 匯出 (.xlsx) | P2 | ✅ 已完成 |
| PDF 匯出 | P3 | ❌ |

### 協作功能
| 缺少功能 | 優先級 |
|----------|--------|
| 游標追蹤（顯示其他人位置）| P1 |
| 評論系統 | P2 |
| 版本歷史 | P2 |
| 權限控制（查看/編輯）| P2 |

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

### P0: Critical (Blocks Production Use)

- [x] **Event/Callback System** ✅ (2025-12-30)
- [x] **Row/Column Insert/Delete** ✅ (2025-12-30)
- [x] **Real-time Collaboration Server** ✅ (2025-12-30)

- [x] **CSV Import/Export** ✅ (2025-12-30)
  - `exportCSV()`, `importCSV()`, `downloadCSV()`, `importCSVFile()`
  - TypeScript 層用 papaparse 實現
  - 支援自訂分隔符、範圍匯出、位移匯入

- [ ] **Structured Error Handling**
  - Create `RuSheetError { code, message, affectedCells }`
  - Consistent error codes across WASM boundary

### P1: High Priority (核心功能補齊)

#### 公式功能
- [x] **Cross-Sheet References** ✅ (2025-12-30)
  - `Sheet2!A1`, `'Sheet Name'!A1:B5` 語法解析
  - `CrossSheetEvaluator` 支援跨工作表取值
  - `evaluate_formula_cross_sheet()` API
  - 84 Rust formula tests + 121 TypeScript tests passing

- [x] **Conditional Functions** ✅ (2025-12-30)
  - COUNTIF, SUMIF, AVERAGEIF
  - Criteria 解析：`>`, `<`, `>=`, `<=`, `<>`, `=` 及純值匹配
  - 支援可選的 sum_range/average_range 參數
  - 77 Rust tests passing

- [ ] **Date/Time Functions**
  - DATE, TIME, NOW, TODAY, DATEDIF
  - Date formatting in cells

#### 資料功能
- [x] **Sorting** ✅ (2025-12-30)
  - 單欄排序（升序/降序）
  - `sortRange()` API in Rust core, WASM, and TypeScript
  - Undo/redo 支援
  - 7 unit tests passing

- [x] **Cell Merging** ✅ (2025-12-30)
  - `mergeCells()`, `unmergeCells()` API in Rust core, WASM, TypeScript
  - `getMergedRanges()`, `getMergeInfo()`, `isMergedSlave()` query APIs
  - Undo/redo 支援 (MergeCellsCommand, UnmergeCellsCommand)
  - Canvas 渲染：合併區域背景、跳過 slave cells、選取區覆蓋整個合併範圍
  - 17 unit tests passing

- [ ] **Filtering / AutoFilter**
  - 自動篩選下拉選單
  - 多條件篩選

#### 匯入匯出
- [x] **XLSX Import/Export** ✅ (2025-12-30)
  - `exportXLSX()`, `importXLSX()`, `downloadXLSX()`, `importXLSXFile()`
  - TypeScript 層用 SheetJS (xlsx) 實現
  - 支援多工作表選擇、範圍匯出

#### 協作功能
- [ ] **Cursor Tracking** 🔥
  - 顯示其他協作者的游標位置
  - 用戶顏色標識

### P2: Medium Priority (Feature Completeness)

#### 公式
- [ ] **Advanced Lookup Functions**
  - INDEX, MATCH, OFFSET, INDIRECT

- [ ] **Array Formulas**
  - ARRAYFORMULA 支援
  - 動態陣列溢出

- [ ] **Named Ranges**
  - 創建/編輯命名範圍
  - 在公式中使用

- [ ] **Text Functions**
  - FIND, SEARCH, SUBSTITUTE, TEXT

#### 資料
- [ ] **Conditional Formatting**
  - 規則型儲存格樣式
  - 資料條、色階、圖示集

- [ ] **Data Validation**
  - 下拉選單、數字範圍
  - 自訂公式驗證

#### 編輯
- [ ] **Find & Replace**
  - 跨工作表搜尋
  - 正則表達式支援

- [ ] **Special Paste**
  - 僅貼上值、僅格式
  - 轉置貼上

- [ ] **AutoFill**
  - 拖曳填充
  - 序列識別（日期、數字）

#### 協作
- [ ] **Comments System**
  - 儲存格評論、回覆
  - 評論指示器

- [ ] **Version History**
  - 查看歷史版本
  - 回滾功能

- [ ] **Permission Control**
  - 查看/編輯權限
  - 工作表保護

#### 匯出
- [x] **XLSX Export** ✅ (2025-12-30)
  - TypeScript 層用 SheetJS (xlsx) 實現（非 Rust）
  - 基本格式匯出

### P3: Low Priority (Nice to Have)

- [ ] **Pivot Tables**
  - 基本樞紐分析功能
  - 分組、彙總

- [ ] **Charts**
  - 基本圖表類型（柱狀、折線、圓餅）
  - Chart.js 或 D3 整合

- [ ] **Print/PDF Export**
  - 列印預覽
  - PDF 生成

- [ ] **Plugin System**
  - 擴充機制
  - 自訂函數

### Testing ✅

- [x] **Fix WASM Loading in Node.js Tests** ✅ (2025-12-30)
  - Fixed missing dependencies (papaparse, xlsx)
  - Rebuilt WASM module with getMergedRanges
  - Updated Vitest 4 config (poolOptions → singleFork)
  - 121 unit tests + 131 integration tests passing

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

## Testing

### Current Test Coverage ✅

| Type | Count | Status |
|------|-------|--------|
| Rust Tests | 394 | ✅ 100% passing |
| TS Unit Tests | 121 | ✅ Passing |
| TS Integration | 131 | ✅ Passing (1 flaky) |
| E2E (Playwright) | 2 files | ✅ Running |

### WASM Loading in Node.js Environment ✅ RESOLVED

**Status:** 🟢 Fixed (2025-12-30)

**Resolution:**
The issue was not actually with WASM loading mechanism, but with:
1. Missing npm dependencies (`papaparse`, `xlsx`) not installed
2. WASM module needed rebuild to include new `getMergedRanges` function
3. Vitest 4 config deprecation (`poolOptions` → top-level `singleFork`)

**Working Setup (in `src/__tests__/setup.ts`):**
- Custom `fetch()` override loads WASM from filesystem
- `WebAssembly.instantiateStreaming` disabled to force fallback path
- Canvas 2D context mock for Node.js environment

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
- `onSortRange`

**Import/Export:** ✅ (2025-12-30)
- CSV: `exportCSV()`, `importCSV()`, `downloadCSV()`, `importCSVFile()`
- XLSX: `exportXLSX()`, `importXLSX()`, `downloadXLSX()`, `importXLSXFile()`, `getXLSXSheetNames()`

**Sorting:** ✅ (2025-12-30)
- `sortRange(startRow, endRow, startCol, endCol, sortCol, ascending)`

**Cell Merging:** ✅ (2025-12-30)
- `mergeCells(startRow, startCol, endRow, endCol)`
- `unmergeCells(row, col)`
- `getMergedRanges()`, `getMergeInfo(row, col)`, `isMergedSlave(row, col)`

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
