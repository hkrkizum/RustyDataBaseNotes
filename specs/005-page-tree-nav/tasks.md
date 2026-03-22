# Tasks: Page Tree Navigation

**Input**: Design documents from `/specs/005-page-tree-nav/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/ipc-commands.md, quickstart.md

**Tests**: Tests are MANDATORY. Each user story must start with failing tests or an
equivalent executable verification task before implementation tasks appear.

**Organization**: Tasks are grouped by user story so each story can be implemented,
tested, and reviewed independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel if they touch different files and have no dependency
- **[Story]**: Which user story this task belongs to, for example `US1`
- Include exact file paths in every task description

## Path Conventions

- **Desktop app**: `src/` for frontend, `src-tauri/src/` for Rust backend,
  `src-tauri/migrations/` for schema changes

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Install dependencies and configure build tooling for Tailwind CSS v4, shadcn/ui, D&D, and icons

- [x] T001 Install Tailwind CSS v4 and Vite plugin: `pnpm add tailwindcss @tailwindcss/vite`, add `tailwindcss()` plugin to `vite.config.ts`
- [x] T002 Configure path alias `@/*` → `./src/*` in `tsconfig.json` (baseUrl + paths) and `vite.config.ts` (resolve.alias)
- [x] T003 Initialize shadcn/ui: `pnpm dlx shadcn@latest init`, verify `components.json` and `src/lib/utils.ts` (cn() helper) are generated
- [x] T004 Install shadcn/ui components: `pnpm dlx shadcn@latest add sidebar collapsible button input dropdown-menu context-menu tooltip scroll-area` → files generated in `src/components/ui/`
- [x] T005 [P] Install D&D libraries: `pnpm add @atlaskit/pragmatic-drag-and-drop @atlaskit/pragmatic-drag-and-drop-hitbox @atlaskit/pragmatic-drag-and-drop-react-drop-indicator @atlaskit/pragmatic-drag-and-drop-auto-scroll`
- [x] T006 [P] Install icon library: `pnpm add lucide-react`
- [x] T007 Replace CSS entry point: update `src/index.css` with `@import "tailwindcss"` and `@custom-variant dark (&:where(.dark, .dark *))`, remove legacy directives if any

**Checkpoint**: `pnpm install` succeeds, `cargo make serve` starts without errors

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Schema changes, domain model extensions, repository updates, and IPC contract changes that all user stories depend on

**⚠️ CRITICAL**: No user story work begins until this phase is complete

- [x] T008 Create migration `src-tauri/migrations/0007_add_page_hierarchy.sql`: `ALTER TABLE pages ADD COLUMN parent_id TEXT REFERENCES pages(id) ON DELETE SET NULL; ALTER TABLE pages ADD COLUMN sort_order INTEGER NOT NULL DEFAULT 0; CREATE INDEX idx_pages_parent_id ON pages(parent_id);`
- [x] T009 Extend Page entity in `src-tauri/src/domain/page/entity.rs`: add `parent_id: Option<PageId>` and `sort_order: i64` fields; add `new_child(title, parent_id)` constructor; update `from_stored()` to accept parent_id and sort_order; add `parent_id()`, `sort_order()`, `is_standalone()`, `is_database_page()` accessors
- [x] T010 [P] Add new error variants in `src-tauri/src/domain/page/error.rs`: `CircularReference { page_id: String, target_parent_id: String }`, `MaxDepthExceeded { page_id: String, current_depth: usize, max_depth: usize }`, `DatabasePageCannotNest { page_id: String }` — all with `#[error(...)]` messages per data-model.md
- [x] T011 [P] Create PageHierarchyService in `src-tauri/src/domain/page/hierarchy.rs`: implement `validate_move()`, `validate_create_child()`, `ancestor_chain()`, `depth()`, `max_descendant_depth()` with `MAX_DEPTH: usize = 5`; export module in `src-tauri/src/domain/page/mod.rs`
- [x] T012 Add new methods to PageRepository trait in `src-tauri/src/domain/page/repository.rs`: `update_parent_id(page_id, parent_id) → Page`, `find_children(parent_id) → Vec<Page>`, `find_root_pages() → Vec<Page>` (parent_id IS NULL AND database_id IS NULL), `find_ancestors(page_id) → Vec<Page>` (recursive CTE), `bulk_update_parent_id(page_ids, new_parent_id) → ()`
- [x] T013 Implement new PageRepository methods in `src-tauri/src/infrastructure/persistence/page_repository.rs`: `update_parent_id` (UPDATE SET parent_id), `find_children` (WHERE parent_id = ?), `find_root_pages` (WHERE parent_id IS NULL AND database_id IS NULL), `find_ancestors` (recursive CTE with `depth < 10` safety limit), `bulk_update_parent_id` (batch UPDATE WHERE id IN)
- [x] T014 [P] Update IPC DTOs in `src-tauri/src/ipc/dto.rs`: extend `PageDto` with `parent_id: Option<String>` and `sort_order: i64`; add `SidebarItemDto { id, title, item_type: SidebarItemType, parent_id, database_id, created_at }` and `SidebarItemType` enum; remove `is_dirty` field from `EditorStateDto`; update `From<Page> for PageDto`
- [x] T015 [P] Add IPC error mappings in `src-tauri/src/ipc/error.rs`: map `PageError::CircularReference` → kind `"circularReference"`, `PageError::MaxDepthExceeded` → kind `"maxDepthExceeded"`, `PageError::DatabasePageCannotNest` → kind `"databasePageCannotNest"`
- [x] T016 Run `cargo make sqlx-prepare` to regenerate `.sqlx/` cache after migration and new queries

**Checkpoint**: `cargo make check` passes, all domain types compile, foundation ready for user stories

---

## Phase 3: User Story 1 — Unified Visual Design (Priority: P1) 🎯 MVP

**Goal**: Migrate all existing components from CSS Modules to Tailwind CSS + shadcn/ui, achieving a unified visual design system with OS-based dark/light theme support

**Independent Test**: All screens (editor, table view, page list) render with consistent button/input/spacing/color styles in both ライト/ダーク modes. Zero `*.module.css` files and zero CSS Modules imports remain.

### Tests for User Story 1

- [x] T017 [P] [US1] Write Vitest test verifying zero `*.module.css` files exist under `src/` and zero CSS Modules `import` statements in `src/**/*.{ts,tsx}` — file: `src/__tests__/design-system.test.ts`

### Implementation for User Story 1

- [x] T018 [US1] Set up ThemeProvider for OS-based dark/light theme in `src/App.tsx`: add theme provider with `defaultTheme="system"`, toggle `dark` class on `document.documentElement` based on `prefers-color-scheme`
- [x] T019 [P] [US1] Migrate editor components from CSS Modules to Tailwind in `src/features/editor/`: `BlockEditor.tsx` (BlockEditor.module.css), `EditorToolbar.tsx` (EditorToolbar.module.css), `BlockItem.tsx` (BlockItem.module.css), `UnsavedConfirmModal.tsx` (UnsavedConfirmModal.module.css) — replace `styles.xxx` class references with Tailwind utility classes and shadcn/ui components
- [x] T020 [P] [US1] Migrate pages components from CSS Modules to Tailwind in `src/features/pages/`: `PageListView.tsx` (PageListView.module.css), `PageItem.tsx` (PageItem.module.css), `CreatePageForm.tsx` (CreatePageForm.module.css), `DeleteConfirmModal.tsx` (DeleteConfirmModal.module.css)
- [x] T021 [P] [US1] Migrate database components from CSS Modules to Tailwind in `src/features/database/`: `DatabaseListView.tsx`, `TableView.tsx`, `TableHeader.tsx`, `TableRow.tsx`, `PropertyCell.tsx`, `GroupHeader.tsx`, `FilterPanel.tsx`, `PropertyConfigPanel.tsx`, `AddPageModal.tsx`, `AddPropertyModal.tsx` (10 corresponding .module.css files)
- [x] T022 [US1] Delete all 18 `*.module.css` files under `src/features/` and delete `src/App.css` — verify no remaining CSS Modules imports in any `.tsx` file
- [x] T023 [US1] Visual review: verify design consistency (color, spacing, border-radius) across editor, table view, and page list in both ライト/ダーク themes (6 patterns: 3 screens × 2 themes)
- [x] T024 [US1] Run `cargo make check-all` to verify no regressions after design system migration

**Checkpoint**: US1 complete — all screens use Tailwind/shadcn, zero CSS Modules remain, both themes work

---

## Phase 4: User Story 2 — Sidebar Navigation (Priority: P2)

**Goal**: Add a persistent left sidebar showing all standalone pages and databases with direct click navigation from any screen. Migrate editor to debounce-based auto-save, removing manual save UI and unsaved confirmation dialogs.

**Independent Test**: User creates 5 pages and 2 databases, navigates between all items via sidebar clicks without returning to a home/list view. Auto-save fires on content change, no save button visible.

### Tests for User Story 2

- [x] T025 [P] [US2] Write Rust integration test for `list_sidebar_items` command: verify it returns standalone pages (itemType: "page"), databases (itemType: "database"), and DB-owned pages (with databaseId set) — file: `src-tauri/tests/sidebar_test.rs` or inline tests in `src-tauri/src/ipc/page_commands.rs`
- [x] T026 [P] [US2] Write Vitest component tests for sidebar rendering: items display with correct icons, click navigation dispatches route change, empty state shows header + create button, active item has highlight background, inline rename on creation (label → input, Enter confirms, Esc cancels, empty string reverts to original) — file: `src/features/sidebar/__tests__/Sidebar.test.tsx`
- [x] T027 [P] [US2] Write Vitest tests for `useAutoSave` hook: debounce fires save_editor after 500ms idle, retry with exponential backoff (1s→2s→4s) on failure, toast.warning after 3 retries exhausted, permanent error (NotFound) skips retry, unmount triggers immediate flush — file: `src/hooks/__tests__/useAutoSave.test.ts`

### Implementation for User Story 2

- [x] T028 [US2] Implement `list_sidebar_items` IPC command in `src-tauri/src/ipc/page_commands.rs`: query all standalone pages + all databases + all DB-owned pages, map each to `SidebarItemDto` with appropriate `item_type`; register command in `src-tauri/src/lib.rs` Tauri builder
- [x] T029 [US2] Create `useLocalStorage` hook in `src/hooks/useLocalStorage.ts`: generic `get<T>/set<T>` with JSON serialize/deserialize, fallback to default value on parse failure or corruption
- [x] T030 [US2] Create `useSidebar` hook in `src/features/sidebar/useSidebar.ts`: fetch data via `list_sidebar_items` on mount, build `SidebarTreeNode[]` tree from flat `SidebarItemDto[]` using parentId/databaseId, manage expand/collapse state (`Record<string, boolean>`) in localStorage key `sidebar-expanded`, track active item ID, support optimistic updates with rollback on backend error
- [x] T031 [US2] Create `useAutoSave` hook in `src/hooks/useAutoSave.ts`: `scheduleSave(pageId)` with 500ms debounce via `useRef<number>` timer, invoke `save_editor` IPC, exponential backoff retry (1s→2s→4s, max 3 attempts), always retry with latest state (not stale snapshot), detect permanent errors (PageError::NotFound) and skip retry with immediate toast, `toast.warning("保存に失敗しました")` (5s auto-dismiss) on all retries exhausted, `useEffect` cleanup cancels timer and fires synchronous flush (best-effort), cancel in-progress retries on unmount/page change
- [x] T032 [US2] Create Sidebar container in `src/features/sidebar/Sidebar.tsx`: shadcn `Sidebar` + `SidebarHeader` (app title) + `SidebarContent` with `ScrollArea` for tree area + fixed header/create button outside scroll, fixed width (240–260px), sidebar visibility persisted in localStorage key `sidebar-visible`
- [x] T033 [P] [US2] Create SidebarTree in `src/features/sidebar/SidebarTree.tsx`: recursive rendering of `SidebarTreeNode[]`, `FileText` icon for pages, `Table2` icon for databases, `Collapsible` + `CollapsibleTrigger`/`CollapsibleContent` for nodes with children, sort items by `createdAt` DESC within each level
- [x] T034 [P] [US2] Create SidebarItem in `src/features/sidebar/SidebarItem.tsx`: `SidebarMenuButton` with click handler (page → editor route, database → table view route), active item background highlight via shadcn styling, `ChevronRight` icon rotation for expand/collapse toggle on parent items
- [x] T054 [US2] Implement inline rename in `src/features/sidebar/SidebarItem.tsx`: on rename trigger, replace label with `<input>` element, `Enter` key confirms and calls `update_page_title` IPC, `Esc` key cancels and restores original title, `blur` (focus-out) confirms, empty string on confirm reverts to original title without IPC call, `maxLength={255}` on input element. During inline editing, call `event.stopPropagation()` on `keydown` for `Cmd/Ctrl+B` to prevent sidebar toggle <!-- moved from US3; merged T056 keyboard shortcut conflict prevention -->
- [x] T035 [P] [US2] Create SidebarCreateButton in `src/features/sidebar/SidebarCreateButton.tsx`: shadcn `DropdownMenu` with "ページ" and "データベース" options, on create → invoke `create_page`/`create_database` IPC → auto-navigate to new item → update `last-opened-item` in localStorage → trigger inline rename in sidebar
- [ ] T036 [US2] Update `src/App.tsx` layout: wrap with `SidebarProvider`, add `Sidebar` + `SidebarInset` for main content area, CSS Flexbox layout where sidebar hidden → main content full width, add `Cmd/Ctrl+B` keyboard shortcut listener for sidebar toggle via `useSidebar` hook
- [ ] T037 [US2] Integrate auto-save in `src/features/editor/BlockEditor.tsx`: call `useAutoSave.scheduleSave` on all content changes (block add/delete/edit/reorder), remove manual save button click handler and Ctrl+S/Cmd+S save shortcut handler
- [ ] T038 [US2] Clean up `src/features/editor/EditorToolbar.tsx`: remove save button and unsaved indicator UI elements; add `Ctrl+S`/`Cmd+S` `preventDefault` to suppress browser default save dialog
- [ ] T039 [US2] Delete `src/features/editor/UnsavedConfirmModal.tsx` and remove all imports/references from `BlockEditor.tsx` and any navigation guard logic
- [ ] T040 [US2] Remove `isDirty` from frontend: update EditorStateDto TypeScript type definition to remove `isDirty` field, remove any UI elements or conditional logic depending on `isDirty` state across `src/features/editor/`
- [ ] T041 [US2] Implement last-opened-item restoration in `src/App.tsx` or `src/features/sidebar/useSidebar.ts`: on app startup, read `last-opened-item` from localStorage (`{ id: string, type: "page" | "database" }`), navigate to that item; on every navigation, update `last-opened-item`; if item deleted or localStorage corrupted, fallback to first root-level item; if no items exist, show empty state
- [ ] T042 [US2] Run `cargo make check-all` to verify no regressions

**Checkpoint**: US2 complete — sidebar shows all items, click navigation works from every screen, auto-save active, manual save and unsaved dialog removed

---

## Phase 5: User Story 3 — Page Hierarchy (Priority: P3)

**Goal**: Enable page nesting up to 5 levels with drag-and-drop reparenting, context menu for child creation/rename/delete, parent deletion promoting children, and tree expand/collapse persistence

**Independent Test**: User creates a parent page with 3 sub-pages. Folds parent to hide children, unfolds to show. Drags a sub-page under another sub-page for 3-level nesting. Deletes parent; children promoted to root. Circular and depth-limit moves are visually blocked.

### Tests for User Story 3

- [ ] T043 [P] [US3] Write Rust unit tests for `PageHierarchyService` in `src-tauri/src/domain/page/hierarchy.rs` (or `src-tauri/tests/hierarchy_unit_test.rs`): `validate_move` rejects circular reference, rejects self-reference (`page_id == new_parent_id`), rejects depth > 5, rejects DB page; `validate_create_child` rejects depth ≥ MAX_DEPTH, rejects DB page parent; `ancestor_chain` returns correct chain; `depth` returns correct level; `max_descendant_depth` calculates subtree depth
- [ ] T044 [P] [US3] Write Rust integration tests for hierarchy IPC commands in `src-tauri/tests/hierarchy_test.rs`: `create_child_page` success + MaxDepthExceeded + DatabasePageCannotNest; `move_page` success + CircularReference + root promotion (newParentId=null) + no-op (same parent); `delete_page` with child promotion to grandparent or root
- [ ] T045 [P] [US3] Write Vitest tests for D&D tree interactions in `src/features/sidebar/__tests__/SidebarDnD.test.tsx`: valid drop reparents page, invalid drop (circular/depth/DB page) shows blocked instruction, root drop zone promotes page to root level

### Implementation for User Story 3

- [ ] T046 [US3] Implement `create_child_page` IPC command in `src-tauri/src/ipc/page_commands.rs`: load parent page (NotFound check), verify not DB page (DatabasePageCannotNest), compute parent depth via `PageHierarchyService::depth()`, call `validate_create_child()`, create page via `Page::new_child(title, parent_id)`, save via repository; register in `src-tauri/src/lib.rs`
- [ ] T047 [US3] Implement `move_page` IPC command in `src-tauri/src/ipc/page_commands.rs`: load page (NotFound), verify not DB page, if newParentId is Some: load parent + verify not DB page + load ancestors via `find_ancestors` + compute `max_descendant_depth` + call `validate_move()`, update via `update_parent_id` in single transaction; if newParentId is null: just update parent_id to NULL; register in `src-tauri/src/lib.rs`
- [ ] T048 [US3] Extend `delete_page` in `src-tauri/src/ipc/page_commands.rs` with child promotion logic: in transaction, get page's `parent_id` (promotion target), `find_children(page_id)`, `bulk_update_parent_id(children, page.parent_id)`, then delete page (blocks CASCADE via existing FK)
- [ ] T049 [US3] Add drag-and-drop to `src/features/sidebar/SidebarItem.tsx`: attach `draggable()` from `@atlaskit/pragmatic-drag-and-drop/element/adapter` with `getInitialData()` storing `{ pageId, parentId, depth, itemType }`, attach `dropTargetForElements()` with `canDrop` filtering (reject DB pages, reject self), visual feedback for drag source
- [ ] T050 [US3] Implement tree-item hitbox in `src/features/sidebar/SidebarTree.tsx`: use `attachInstruction()` from `@atlaskit/pragmatic-drag-and-drop-hitbox/tree-item` with `currentLevel` and `indentPerLevel`, set `mode: "standard"` and `block: [blocked-conditions]`, render `DropIndicator` from `@atlaskit/pragmatic-drag-and-drop-react-drop-indicator/tree-item` for valid drops, show forbidden cursor for `instruction-blocked`
- [ ] T051 [US3] Add `monitorForElements` in `src/features/sidebar/SidebarTree.tsx`: on `onDrop`, extract instruction type — for `make-child`: call `move_page(pageId, newParentId=targetId)`, for `reorder-above`/`reorder-below`: call `move_page(pageId, newParentId=targetParentId)` (reorder within parent is future scope, just reparent), for root drop: call `move_page(pageId, newParentId=null)`; apply optimistic update, rollback + re-fetch `list_sidebar_items` on error
- [ ] T052 [US3] Add `autoScrollForElements()` from `@atlaskit/pragmatic-drag-and-drop-auto-scroll/element` to sidebar scroll container for auto-scroll when dragging near top/bottom edges — in `src/features/sidebar/SidebarTree.tsx`
- [ ] T053 [US3] Create `SidebarContextMenu` in `src/features/sidebar/SidebarContextMenu.tsx`: shadcn `ContextMenu` + `DropdownMenu`, three menu items: "子ページ作成" (hidden when item is at depth 5 or is a database, calls `create_child_page` → auto-navigate → inline rename), "名前変更" (triggers inline rename mode on SidebarItem), "削除" (shows confirm dialog via shadcn `AlertDialog`, message includes "子ページはルートレベルに昇格されます" when item has children, calls `delete_page` IPC). Add hover "..." button to `src/features/sidebar/SidebarItem.tsx` as `DropdownMenu` trigger + right-click `ContextMenu` trigger <!-- "..." button deferred from T034 (US2) -->
- [ ] T055 [US3] Auto-expand ancestors on startup restore in `src/features/sidebar/useSidebar.ts`: when last-opened-item is inside a collapsed subtree, walk the tree upward to find all ancestor IDs, set each ancestor to `expanded: true` in localStorage expanded state, then call `scrollIntoView({ behavior: "smooth", block: "nearest" })` on the target item DOM element
- [ ] T057 [US3] D&D / context menu exclusivity in `src/features/sidebar/SidebarContextMenu.tsx`: check `isDragging` state from D&D monitor, suppress context menu open (both right-click and "..." button) while any drag is in progress
- [ ] T058 [US3] Root-level drop zone in `src/features/sidebar/SidebarTree.tsx`: add a `dropTargetForElements()` on the sidebar empty area below all items, on drop call `move_page(pageId, newParentId=null)` for root promotion, show root-level drop indicator line
- [ ] T059 [US3] Run `cargo make check-all` to verify no regressions

**Checkpoint**: All user stories independently functional — hierarchy, D&D, context menu, inline rename, auto-save all work correctly

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Performance verification, documentation, and final quality assurance across all user stories

- [ ] T060 Verify performance targets with 500-page seed data: sidebar initial render ≤200ms, expand/collapse ≤50ms, sidebar click → screen transition ≤100ms, `list_sidebar_items` backend query total ≤50ms, hierarchy operations (create child / move / delete) ≤500ms (CC-003) — measure with React Profiler commit duration and manual timing
- [ ] T061 [P] Verify zero CSS Modules remain: confirm zero `*.module.css` files and zero CSS Modules `import` statements in entire `src/` directory via grep
- [ ] T062 [P] Add `///` documentation to all new public Rust items: IPC commands (`list_sidebar_items`, `create_child_page`, `move_page`), `PageHierarchyService` methods, new `PageError` variants, new `PageRepository` trait methods — include `# Errors` sections; verify `cargo doc --no-deps` passes cleanly
- [ ] T063 [P] Verify `save_editor` and `open_editor` in `src-tauri/src/ipc/editor_commands.rs` no longer serialize `is_dirty` in `EditorStateDto` response
- [ ] T064 Run full QA: `cargo make qa` (sqlx-prepare → fmt → clippy → test → doc-test → doc-check → fmt-ts → lint-ts → ts-check → test-ts) — all checks must pass

---

## Dependencies & Execution Order

### Phase Dependencies

```
Phase 1: Setup ──────────────────────────────┐
Phase 2: Foundational (depends on Setup) ────┤
                                             ├─→ Phase 3: US1 (depends on Phase 1+2)
                                             │       └─→ Phase 4: US2 (depends on US1)
                                             │              └─→ Phase 5: US3 (depends on US2)
                                             │                     └─→ Phase 6: Polish
```

### User Story Dependencies

- **US1 → US2**: Sidebar UI built with shadcn/ui components established in US1
- **US2 → US3**: Tree hierarchy and D&D extend US2's flat sidebar
- US1, US2, US3 are **sequential** (not parallelizable between stories)

### Within Each User Story

- Tests MUST be written first and MUST fail before implementation
- Frontend and backend tasks can run in parallel when contracts are stable
- QA checkpoint is required for story completion

### Parallel Opportunities

| Phase | Parallel Tasks | Reason |
|-------|---------------|--------|
| Setup | T005, T006 | Independent npm packages |
| Foundational | T010, T011 (domain); T014, T015 (IPC) | Different files, no shared state |
| US1 | T019, T020, T021 | Different feature directories |
| US2 tests | T025, T026, T027 | Backend / frontend / hook — separate files |
| US2 impl | T033, T034, T035 | Independent sidebar sub-components; T054 sequential after T034 |
| US3 tests | T043, T044, T045 | Unit / integration / frontend — separate files |
| Polish | T061, T062, T063 | Independent verification tasks |

---

## Implementation Strategy

### MVP First

1. Complete Setup (Phase 1)
2. Complete Foundational (Phase 2)
3. Complete **User Story 1 — Unified Visual Design**
4. Validate US1 independently: all screens render correctly in both themes, zero CSS Modules

### Incremental Delivery

1. **US1** delivers design system — visual consistency, dark mode support
2. **US2** delivers sidebar — direct navigation, auto-save, no more "back to list"
3. **US3** delivers hierarchy — tree organization, D&D, context menu
4. Run `cargo make qa` after each story to ensure no regressions

### Suggested MVP Scope

US1 alone delivers immediate value (consistent design, dark/light theme). US2 adds the core navigation improvement. US3 adds advanced organization features. Ship US1 first, expand incrementally.

---

## Notes

- All 18 CSS Module files must be migrated and deleted in US1
- Auto-save migration is in US2 (motivated by sidebar navigation removing the need for unsaved confirmation)
- Inline rename (T054) is in US2 (required for US2 acceptance scenario #5: create-and-rename flow; T056 merged into T054)
- `PageHierarchyService` is pure domain logic with no repository dependency; IPC command handlers orchestrate data loading and pass `&[Page]` to the service
- D&D uses `@atlaskit/pragmatic-drag-and-drop-hitbox/tree-item` for native depth validation
- `sort_order` column is added in the migration but used only with `DEFAULT 0`; manual reordering within same parent is future scope
- Same-parent reordering via D&D (reorder-above/reorder-below) is future scope; current D&D only reparents
