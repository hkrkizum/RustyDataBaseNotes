# Tasks: テーブルビュー操作拡張（ソート・フィルタ・グルーピング）

**Input**: Design documents from `/specs/004-table-view-operations/`
**Prerequisites**: plan.md (required), spec.md (required), research.md,
data-model.md, contracts/view-commands.md

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

**Purpose**: Establish module scaffolding and database schema for the View domain

- [X] T001 Create `src-tauri/src/domain/view/` directory with stub files: `mod.rs` (public re-exports), `entity.rs`, `repository.rs`, `error.rs`, `sort.rs`, `filter.rs`, `group.rs`
- [X] T002 Create migration `src-tauri/migrations/0006_create_views.sql` with views table, unique index on database_id, and `INSERT INTO views ... SELECT` for existing databases per data-model.md schema

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core View domain infrastructure that MUST exist before any user story lands

**⚠️ CRITICAL**: No user story work begins until this phase is complete

- [X] T003 [P] Implement ViewError enum with all variants (ViewNotFound, InvalidSortCondition, TooManySortConditions, InvalidFilterOperator, InvalidFilterValue, TooManyFilterConditions, PropertyNotFound, NoGroupCondition, DuplicateSortProperty) using thiserror in `src-tauri/src/domain/view/error.rs`
- [X] T004 [P] Implement View entity (ViewId, ViewName, ViewType, SortCondition, SortDirection, FilterCondition, FilterOperator, FilterValue, GroupCondition) with validation rules (name 1–100 chars, sort max 5, filter max 20, no duplicate sort property_id) and `#[cfg(test)]` unit tests in `src-tauri/src/domain/view/entity.rs`
- [X] T005 Define ViewRepository trait (find_by_database_id, save, update_sort_conditions, update_filter_conditions, update_group_condition, update_collapsed_groups, reset, remove_property_references) in `src-tauri/src/domain/view/repository.rs`
- [X] T006 Implement SqlxViewRepository with JSON serialization/deserialization for conditions columns and `#[cfg(test)]` CRUD tests (including FR-014: verify view is cascade-deleted when parent database is deleted) in `src-tauri/src/infrastructure/persistence/view_repository.rs`
- [X] T007 [P] Add ViewDto, SortConditionDto, FilterConditionDto, GroupConditionDto, GroupInfoDto with `#[serde(rename_all = "camelCase")]` to `src-tauri/src/ipc/dto.rs` and extend TableDataDto with `view: ViewDto` and `groups: Option<Vec<GroupInfoDto>>` fields
- [X] T008 [P] Add ViewError → CommandError kind mappings (viewNotFound, invalidSortCondition, tooManySortConditions, invalidFilterOperator, invalidFilterValue, tooManyFilterConditions, propertyNotFound, noGroupCondition, duplicateSortProperty) to `src-tauri/src/ipc/error.rs`
- [X] T009 [P] Add TypeScript types (ViewDto, SortConditionDto, FilterConditionDto, GroupConditionDto, GroupInfoDto, FilterOperatorDto, FilterValueDto) and extend TableDataDto with view and groups fields in `src/features/database/types.ts`
- [X] T010 Register `pub mod view` in `src-tauri/src/domain/mod.rs`, SqlxViewRepository in `src-tauri/src/infrastructure/persistence/mod.rs`, `pub mod view_commands` in `src-tauri/src/ipc/mod.rs`
- [X] T011 Implement get_view and reset_view IPC commands in `src-tauri/src/ipc/view_commands.rs` and register both in `src-tauri/src/lib.rs` generate_handler! macro
- [X] T012 Extend create_database to auto-create default View (name: "Table", view_type: Table, empty conditions) in `src-tauri/src/ipc/database_commands.rs` using SqlxViewRepository
- [X] T013 Run `cargo make sqlx-prepare` to reset dev.db with new migration and regenerate `.sqlx/` query cache

**Checkpoint**: Foundation is ready — View entity, repository, migration, DTOs, and basic commands are in place. User stories can proceed.

---

## Phase 3: User Story 1 — カラムによるソート (Priority: P1) 🎯 MVP

**Goal**: ユーザーがカラムヘッダーをクリックして行をソートできる。昇順→降順→解除のサイクル。プロパティ型別の正しいソート順序。

**Independent Test**: テーブルビューで各プロパティ型のカラムヘッダーをクリックし，行が値の昇順・降順で正しく並び替わることを確認する。

### Tests for User Story 1

- [ ] T014 [P] [US1] Write failing tests for sort logic covering Text (Unicode order), Number (f64 total_cmp), Date (date/datetime comparison), Select (position order), Checkbox (false < true), null positioning (ascending=末尾, descending=先頭), empty string=null treatment, and stable sort in `src-tauri/src/domain/view/sort.rs`
- [ ] T015 [P] [US1] Write failing test for update_sort_conditions IPC command covering single condition, validation errors (property not found, duplicate property), and empty conditions (clear sort) in `src-tauri/src/ipc/view_commands.rs`

### Implementation for User Story 1

- [ ] T016 [US1] Implement apply_sort function taking `&mut Vec<row data>` and `&[SortCondition]` with property-type-aware comparison, null handling, and stable sort guarantee (`sort_by` is stable in Rust) in `src-tauri/src/domain/view/sort.rs`
- [ ] T017 [US1] Implement update_sort_conditions IPC command with validation (max 5, no duplicate property_id, property existence check) in `src-tauri/src/ipc/view_commands.rs` and register in `src-tauri/src/lib.rs`
- [ ] T018 [US1] Extend get_table_data to load default view, apply sort conditions to rows using apply_sort, and include ViewDto in response in `src-tauri/src/ipc/table_commands.rs`
- [ ] T019 [P] [US1] Add sort direction indicator (▲▼) display and header-click sort cycle handler (none→asc→desc→none) to `src/features/database/TableHeader.tsx`
- [ ] T020 [P] [US1] Add updateSortConditions IPC invocation, header-click-to-single-sort logic, and view state tracking to `src/features/database/useTableData.ts`
- [ ] T021 [US1] Add sort toolbar button with active sort count badge to `src/features/database/TableView.tsx`

**Checkpoint**: User Story 1 is fully functional — single-column sort works for all property types via header click, with sort indicator.

---

## Phase 4: User Story 2 — 条件によるフィルタリング (Priority: P1)

**Goal**: ユーザーがプロパティ型に応じたフィルタ条件を追加し，条件に合致するページのみを表示できる。複数条件の AND 結合。

**Independent Test**: 各プロパティ型に対してフィルタ条件を設定し，条件に合致するページのみが表示されることを確認する。複数条件の AND 結合が正しく動作することを確認する。

### Tests for User Story 2

- [ ] T022 [P] [US2] Write failing tests for filter logic covering all 5 property types with all valid operators (Text: Equals/NotEquals/Contains/NotContains case-insensitive, Number: comparisons with f64 equality, Date: Equals minute-granularity/Before exclusive/After inclusive, Select: Is/IsNot including null in IsNot, Checkbox: IsChecked/IsUnchecked), IsEmpty/IsNotEmpty (null-only, not zero/empty-string), and multiple conditions AND in `src-tauri/src/domain/view/filter.rs`
- [ ] T023 [P] [US2] Write failing test for update_filter_conditions IPC command covering valid conditions, operator-type mismatch error, value-type mismatch error, max 20 limit, and property existence check in `src-tauri/src/ipc/view_commands.rs`

### Implementation for User Story 2

- [ ] T024 [US2] Implement apply_filters function taking rows and `&[FilterCondition]` with operator-type validation, case-insensitive text matching, date granularity handling, and AND combination in `src-tauri/src/domain/view/filter.rs`
- [ ] T025 [US2] Implement update_filter_conditions IPC command with validation (max 20, operator-type compatibility per data-model.md table, value-type check) in `src-tauri/src/ipc/view_commands.rs` and register in `src-tauri/src/lib.rs`
- [ ] T026 [US2] Extend get_table_data to apply filter conditions before sort in `src-tauri/src/ipc/table_commands.rs`
- [ ] T027 [P] [US2] Create FilterConditionRow component with property dropdown, type-aware operator dropdown (only showing valid operators per FR-004), value input (text/number/date/select picker), and delete button in `src/features/database/FilterConditionRow.tsx`
- [ ] T028 [P] [US2] Create FilterPanel component with condition list, add-condition button, and active filter count in `src/features/database/FilterPanel.tsx`
- [ ] T029 [US2] Add updateFilterConditions IPC invocation and filter state management to `src/features/database/useTableData.ts`
- [ ] T030 [US2] Extend TableView.tsx with filter toolbar button, active filter indicator, and empty-state message with "すべてのフィルタを解除" button (FR-010) in `src/features/database/TableView.tsx`
- [ ] T030a [US2] Write test verifying FR-011: add a new page while filter is active, confirm it does NOT appear in get_table_data results when it fails filter conditions, and DOES appear after clearing all filters in `src-tauri/src/ipc/view_commands.rs`

**Checkpoint**: User Stories 1 and 2 both work independently — sort and filter can be applied together (filter→sort pipeline).

---

## Phase 5: User Story 5 — ビュー設定の保持 (Priority: P1)

**Goal**: ソート・フィルタ・グルーピングの設定が自動保存され，再起動後も復元される。プロパティ削除時に孤立条件が自動除去される。設定の一括リセットが可能。

**Independent Test**: ソート・フィルタをそれぞれ設定した後にアプリを再起動し，設定がすべて復元されることを確認する。リセット操作でデフォルト状態に戻ることを確認する。プロパティ削除後に関連条件が消えることを確認する。

### Tests for User Story 5

- [ ] T031 [P] [US5] Write failing tests for full round-trip persistence: set sort (2 conditions) + filter (3 conditions) + group_condition + collapsed_groups (2 groups), reload view, verify all conditions restored in `src-tauri/src/infrastructure/persistence/view_repository.rs`
- [ ] T032 [P] [US5] Write failing tests for remove_property_references: property deletion removes sort/filter/group conditions referencing that property_id, property type change removes incompatible filter operators, select option deletion removes referencing filter conditions in `src-tauri/src/infrastructure/persistence/view_repository.rs`
- [ ] T033 [P] [US5] Write failing tests for JSON deserialization fallback: corrupt sort_conditions JSON returns default empty view instead of error (FR-015) in `src-tauri/src/infrastructure/persistence/view_repository.rs`

### Implementation for User Story 5

- [ ] T034 [US5] Implement remove_property_references (property deletion), remove_incompatible_filters (property type change), and remove_select_option_references (select option deletion) in SqlxViewRepository in `src-tauri/src/infrastructure/persistence/view_repository.rs`
- [ ] T035 [US5] Integrate remove_property_references into delete_property command in `src-tauri/src/ipc/property_commands.rs`, remove_incompatible_filters into update_property_config, and remove_select_option_references into reset_select_option
- [ ] T036 [US5] Implement JSON deserialization fallback in SqlxViewRepository::find_by_database_id — on serde error, reset to default conditions and log warning in `src-tauri/src/infrastructure/persistence/view_repository.rs`
- [ ] T037 [US5] Add resetView IPC invocation and "設定をリセット" button to toolbar in `src/features/database/useTableData.ts` and `src/features/database/TableView.tsx`

**Checkpoint**: View settings persist across restart. Property deletion auto-repairs view settings. Reset restores defaults.

---

## Phase 6: User Story 3 — 複数カラムソート (Priority: P2)

**Goal**: ユーザーがソート設定パネルから複数カラムの優先順位付きソートを設定できる。カラムヘッダークリックは単一ソートへの切り替え。

**Independent Test**: 2つ以上のソート条件を設定し，第1ソートキーが同値の行が第2ソートキーで正しく並び替わることを確認する。

### Tests for User Story 3

- [ ] T038 [P] [US3] Write failing tests for multi-column sort: primary key ties broken by secondary key, 3+ sort conditions chaining, and sort condition reordering in `src-tauri/src/domain/view/sort.rs`

### Implementation for User Story 3

- [ ] T039 [US3] Verify and extend apply_sort to handle chained multi-key comparison (Ordering::then_with pattern) if not already complete in `src-tauri/src/domain/view/sort.rs`
- [ ] T040 [P] [US3] Create SortPanel component with add-condition button (property + direction selectors), per-condition delete button, and up/down reorder buttons in `src/features/database/SortPanel.tsx`
- [ ] T041 [US3] Update header-click behavior: if multi-sort active, clear all and switch to clicked column's single sort in `src/features/database/TableHeader.tsx`
- [ ] T042 [US3] Add SortPanel toggle to toolbar and integrate with updateSortConditions in `src/features/database/useTableData.ts` and `src/features/database/TableView.tsx`

**Checkpoint**: Multi-column sort works via SortPanel. Header click resets to single sort.

---

## Phase 7: User Story 4 — プロパティによるグルーピング (Priority: P2)

**Goal**: ユーザーが1つのプロパティを選択してグルーピングし，各グループのヘッダー（値・件数）表示と折りたたみ・展開ができる。

**Independent Test**: セレクト型プロパティでグルーピングを適用し，選択肢ごとのグループが表示され，折りたたみ・展開が動作することを確認する。

### Tests for User Story 4

- [ ] T043 [P] [US4] Write failing tests for grouping logic: Text (case-insensitive exact match), Number (f64 identity), Date (day/minute granularity), Select (option value match, definition order), Checkbox (true/false groups), null→"未設定" group at end, empty group exclusion, and group ordering rules per FR-006 in `src-tauri/src/domain/view/group.rs`
- [ ] T044 [P] [US4] Write failing tests for update_group_condition (set/clear, collapsed_groups cleared on property change) and toggle_group_collapsed (add/remove from set, null for "未設定" group) IPC commands in `src-tauri/src/ipc/view_commands.rs`

### Implementation for User Story 4

- [ ] T045 [US4] Implement group_rows function with type-aware grouping keys, group ordering (Select: position, Checkbox: checked→unchecked, Text/Number/Date: ascending, null last), and GroupInfo computation in `src-tauri/src/domain/view/group.rs`
- [ ] T046 [US4] Implement update_group_condition (with collapsed_groups clear on change) and toggle_group_collapsed (HashSet toggle, null support) IPC commands in `src-tauri/src/ipc/view_commands.rs` and register in `src-tauri/src/lib.rs`
- [ ] T047 [US4] Extend get_table_data to apply grouping after filter+sort: compute GroupInfoDto list, reorder rows by group, exclude collapsed group rows from response in `src-tauri/src/ipc/table_commands.rs`
- [ ] T048 [P] [US4] Create GroupPanel component with property selector dropdown and "グルーピング解除" button in `src/features/database/GroupPanel.tsx`
- [ ] T049 [P] [US4] Create GroupHeader component displaying group value (or "未設定"), count badge, and collapse/expand toggle with aria-expanded in `src/features/database/GroupHeader.tsx`
- [ ] T050 [US4] Add updateGroupCondition and toggleGroupCollapsed IPC invocations to `src/features/database/useTableData.ts`
- [ ] T051 [US4] Extend TableView.tsx with grouping toolbar button, GroupHeader rendering between row groups, and panel toggle (mutually exclusive with sort/filter panels) in `src/features/database/TableView.tsx`

**Checkpoint**: All planned user stories are independently functional. Grouping works with sort and filter (filter→sort→group pipeline).

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Work that spans multiple user stories

- [ ] T052 [P] Add `///` documentation (summary + Examples + Errors sections) to all public items in `src-tauri/src/domain/view/` module files and verify `cargo doc --no-deps` passes
- [ ] T053 [P] Add accessibility attributes: aria-sort on sortable column headers in `src/features/database/TableHeader.tsx`, aria-expanded on group headers in `src/features/database/GroupHeader.tsx`, keyboard navigation (Tab/Enter/Escape) on all panel components
- [ ] T054 [P] Verify CC-003 performance targets (500ms for 100 pages × 10 properties, 2s for 1,000 pages) for sort, filter, and grouping operations with test data
- [ ] T055 Run full QA: `cargo make qa` (sqlx-prepare → fmt-check → clippy → lint-ts → ts-check → test → test-ts)

---

## Dependencies & Execution Order

### Phase Dependencies

```
Phase 1 (Setup)
  └─► Phase 2 (Foundational)
        └─► Phase 3 (US1: Sort) ──────────┐
        └─► Phase 4 (US2: Filter) ────────┤
              └─► Phase 5 (US5: Persist) ──┤
        Phase 3 + Phase 4 ─► Phase 6 (US3) ┤
        Phase 3 + Phase 4 ─► Phase 7 (US4) ┤
              All US ─────────► Phase 8 ────┘
```

- **Setup** (Phase 1): Starts immediately
- **Foundational** (Phase 2): Depends on Setup, blocks all user stories
- **US1 Sort** (Phase 3): Depends on Foundational. Can run in parallel with US2
- **US2 Filter** (Phase 4): Depends on Foundational. Can run in parallel with US1
- **US5 Persistence** (Phase 5): Depends on US1 + US2 (needs sort/filter to test persistence)
- **US3 Multi-sort** (Phase 6): Depends on US1 (extends sort UI)
- **US4 Grouping** (Phase 7): Depends on Foundational. Can start after US1+US2 for get_table_data pipeline
- **Polish** (Phase 8): Starts after all user stories complete

### Within Each User Story

- Tests MUST be written first and MUST fail before implementation
- Frontend and backend tasks can run in parallel when marked [P] and touching different files
- IPC command implementation must complete before frontend integration
- QA checkpoint required for story completion

### Parallel Opportunities

- **Phase 2**: T003, T004 in parallel; T007, T008, T009 in parallel
- **Phase 3 + 4**: US1 and US2 can run in parallel (different files: sort.rs vs filter.rs, SortPanel vs FilterPanel)
- **Phase 3**: T014+T015 (tests) in parallel; T019+T020 (frontend) in parallel
- **Phase 4**: T022+T023 (tests) in parallel; T027+T028 (frontend) in parallel
- **Phase 5**: T031+T032+T033 (all tests) in parallel
- **Phase 6**: T040 (frontend) in parallel with T038 (test)
- **Phase 7**: T043+T044 (tests) in parallel; T048+T049 (frontend) in parallel
- **Phase 8**: T052, T053, T054 all in parallel

---

## Implementation Strategy

### MVP First

1. Complete Setup (Phase 1)
2. Complete Foundational (Phase 2)
3. Complete User Story 1 — Single Column Sort (Phase 3)
4. Validate US1 independently: header click sorts all 5 property types correctly

### Incremental Delivery

1. Ship US1 (Sort) first — single most valuable table operation
2. Add US2 (Filter) — second most valuable, independent of sort UI
3. Add US5 (Persistence + Auto-repair) — ensures data integrity for shipped features
4. Add US3 (Multi-sort) — extends sort with panel UI
5. Add US4 (Grouping) — most complex, builds on stable sort+filter pipeline
6. Re-run `cargo make qa` after each story

---

## Notes

- All Rust domain logic in `src-tauri/src/domain/view/` must have `///` doc comments on public items
- No `unsafe`, `unwrap()`, `expect()`, or `panic!()` in non-test code
- Filter→Sort→Group is the fixed pipeline order in get_table_data
- Frontend panels are mutually exclusive (only one open at a time)
- All conditions are sent as complete arrays (replace, not incremental update)
- Collapsed group rows are excluded from get_table_data response (not sent to frontend)
