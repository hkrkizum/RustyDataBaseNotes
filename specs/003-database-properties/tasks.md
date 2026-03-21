# Tasks: プロパティシステムとデータベース概念の導入

**Input**: Design documents from `/specs/003-database-properties/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/

**Tests**: Tests are MANDATORY. Each user story must start with failing tests or an
equivalent executable verification task before implementation tasks appear.

**Organization**: Tasks are grouped by user story so each story can be implemented,
tested, and reviewed independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel if they touch different files and have no dependency
- **[Story]**: Which user story this task belongs to, for example `US1`
- Include exact file paths in every task description

## Path Conventions

- **Frontend**: `src/` for React components and hooks
- **Backend**: `src-tauri/src/` for Rust domain, infrastructure, and IPC layers
- **Migrations**: `src-tauri/migrations/` for SQLite schema changes

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create directory structure and migration files required by the feature

- [ ] T001 Create feature directories: `src-tauri/src/domain/database/`, `src-tauri/src/domain/property/`, `src/features/database/`
- [ ] T002 [P] Create SQLite migration files per data-model.md schema: `src-tauri/migrations/0003_create_databases.sql`, `src-tauri/migrations/0004_create_properties.sql`, `src-tauri/migrations/0005_add_page_database_id_and_property_values.sql`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Domain entities, repository traits, error types, DTO definitions, and module registration — MUST exist before any user story

**⚠️ CRITICAL**: No user story work begins until this phase is complete

- [ ] T003 [P] Implement Database entity (Database, DatabaseId, DatabaseTitle) with validation (title: 1–255 chars after trim, UUIDv7) and `new` / `from_stored` constructors in `src-tauri/src/domain/database/entity.rs`
- [ ] T004 [P] Implement DatabaseError enum (TitleEmpty, TitleTooLong, NotFound) in `src-tauri/src/domain/database/error.rs`
- [ ] T005 [P] Implement Property entity (Property, PropertyId, PropertyName, PropertyType enum, PropertyConfig enum with serde internally tagged, DateMode, SelectOption, SelectOptionId) with validation (name: 1–100 chars after trim, position ≥ 0, options ≤ 100, option value: 1–100 chars unique) in `src-tauri/src/domain/property/entity.rs`
- [ ] T006 [P] Implement PropertyError enum (NameEmpty, NameTooLong, DuplicateName, InvalidType, TooManyProperties, NotFound, InvalidConfig, TooManyOptions, OptionValueEmpty, DuplicateOptionValue) in `src-tauri/src/domain/property/error.rs`
- [ ] T007 [P] Implement PropertyValue entity (PropertyValue, PropertyValueId) with type-specific validation (Number: finite only, -0.0→0.0 normalization; Select: option ID exists; type mismatch check) and PropertyValueError enum in `src-tauri/src/domain/property/entity.rs` and `src-tauri/src/domain/property/error.rs`
- [ ] T008 [P] Define DatabaseRepository trait (create, find_by_id, find_all, update_title, delete) in `src-tauri/src/domain/database/repository.rs`
- [ ] T009 [P] Define PropertyRepository trait (create, find_by_database_id, find_by_id, update_name, update_config, update_positions, delete, count_by_database_id, next_position) and PropertyValueRepository trait (upsert, find_by_page_and_property, find_by_page_id, find_by_property_id, delete_by_page_and_property, delete_by_page_and_database, reset_select_option, find_all_for_database) in `src-tauri/src/domain/property/repository.rs`
- [ ] T010 Extend Page entity with `database_id: Option<DatabaseId>` field, update `new` and `from_stored` constructors in `src-tauri/src/domain/page/entity.rs`. Also update existing SqlxPageRepository's SQL queries and row mapping to include `database_id` column in `src-tauri/src/infrastructure/persistence/page_repository.rs`
- [ ] T011 Create `mod.rs` for database and property modules, register `pub mod database` and `pub mod property` in `src-tauri/src/domain/mod.rs`
- [ ] T012 [P] Add Rust DTO types (DatabaseDto, PropertyDto, PropertyValueDto, PropertyConfigDto, PropertyTypeDto, SelectOptionDto, PropertyValueInputDto, TableRowDto, TableDataDto) and extend PageDto with `database_id` in `src-tauri/src/ipc/dto.rs`
- [ ] T013 [P] Add 17 error kind extensions per contracts/ipc-commands.md (databaseNotFound, propertyNameEmpty, duplicatePropertyName, invalidConfig, invalidNumber, invalidDate, invalidSelectOption, typeMismatch, pageNotInDatabase, pageAlreadyInDatabase, propertyValueNotFound, etc.) in `src-tauri/src/ipc/error.rs`
- [ ] T014 [P] Create TypeScript type definitions (DatabaseDto, PropertyDto, PropertyValueDto, PropertyConfigDto as discriminated union matching serde tagged format, PropertyTypeDto, SelectOptionDto, PropertyValueInputDto, TableRowDto, TableDataDto, CommandError) in `src/features/database/types.ts`
- [ ] T015 [P] Update PageDto TypeScript type with `databaseId: string | null` field in `src/features/pages/types.ts`
- [ ] T016 Run foundational verification: `cargo make check` to confirm compilation passes

**Checkpoint**: Foundation is ready and user stories can proceed

---

## Phase 3: User Story 1 - データベースの作成 (Priority: P1) 🎯 MVP

**Goal**: ユーザーがデータベースを作成し，統合リストに表示され，空のテーブルビューを確認できる

**Independent Test**: データベースを作成してタイトルが表示され，空のテーブルビューが表示される。再起動後も保持される

### Tests for User Story 1

- [ ] T017 [P] [US1] Add failing domain tests for Database::new (valid title, empty title rejection, title > 255 chars rejection, title trimming) in `src-tauri/src/domain/database/entity.rs`
- [ ] T018 [P] [US1] Add failing repository tests for SqlxDatabaseRepository::create, find_by_id, find_all (created_at DESC order) in `src-tauri/src/infrastructure/persistence/database_repository.rs`

### Implementation for User Story 1

- [ ] T019 [P] [US1] Implement SqlxDatabaseRepository (create, find_by_id, find_all) in `src-tauri/src/infrastructure/persistence/database_repository.rs`, register in `src-tauri/src/infrastructure/persistence/mod.rs`
- [ ] T020 [US1] Implement create_database, list_databases, get_database IPC commands in `src-tauri/src/ipc/database_commands.rs`, register module in `src-tauri/src/ipc/mod.rs` and add handlers to invoke_handler in `src-tauri/src/lib.rs`
- [ ] T021 [P] [US1] Create useDatabase hook (createDatabase, listDatabases, getDatabase) with Tauri invoke calls in `src/features/database/useDatabase.ts`
- [ ] T022 [US1] Implement DatabaseListView (replaces existing PageList — unified list showing both pages and databases with type-distinguishing icons, click page to open editor, click database to open table view) in `src/features/database/DatabaseListView.tsx`
- [ ] T023 [US1] Implement TableView shell component (database title header, empty state with page-add prompt) in `src/features/database/TableView.tsx`
- [ ] T024 [US1] Integrate database views into App.tsx navigation (list view ↔ table view ↔ editor transitions) in `src/App.tsx`
- [ ] T025 [US1] Add `///` documentation for all public items in `src-tauri/src/domain/database/` and run `cargo make qa-rs`

**Checkpoint**: User Story 1 is fully functional and independently verifiable

---

## Phase 4: User Story 2 - データベースへのプロパティ定義 (Priority: P1)

**Goal**: データベースに5種類のプロパティ（列）を追加し，テーブルビューに列として表示される

**Independent Test**: 各型のプロパティを追加してテーブルビューの列ヘッダーに反映される。重複名は拒否される

### Tests for User Story 2

- [ ] T026 [P] [US2] Add failing domain tests for Property::new validation (name empty, name > 100 chars, valid with each PropertyType, PropertyConfig validation, property count limit 50) in `src-tauri/src/domain/property/entity.rs`
- [ ] T027 [P] [US2] Add failing repository tests for SqlxPropertyRepository::create, find_by_database_id (position ASC order), count_by_database_id, next_position in `src-tauri/src/infrastructure/persistence/property_repository.rs`

### Implementation for User Story 2

- [ ] T028 [P] [US2] Implement SqlxPropertyRepository (create, find_by_database_id, find_by_id, count_by_database_id, next_position) in `src-tauri/src/infrastructure/persistence/property_repository.rs`, register in `src-tauri/src/infrastructure/persistence/mod.rs`
- [ ] T029 [US2] Implement add_property, list_properties IPC commands with config validation (type-config consistency check, duplicate name check, count limit check) in `src-tauri/src/ipc/property_commands.rs`, register module in `src-tauri/src/ipc/mod.rs` and add handlers to `src-tauri/src/lib.rs`
- [ ] T030 [P] [US2] Create useTableData hook (addProperty, listProperties via `list_properties` IPC command) with Tauri invoke calls in `src/features/database/useTableData.ts`
- [ ] T031 [US2] Implement AddPropertyModal (name input, type selector with 5 types, config panel for select options / date mode) in `src/features/database/AddPropertyModal.tsx`
- [ ] T032 [US2] Implement TableHeader (property column headers from properties array, add-property button triggering AddPropertyModal) in `src/features/database/TableHeader.tsx`
- [ ] T033 [US2] Add `///` documentation for all public items in `src-tauri/src/domain/property/` and run `cargo make qa-rs`

**Checkpoint**: User Stories 1 and 2 both work independently

---

## Phase 5: User Story 3 - データベース内でのページ追加 (Priority: P1)

**Goal**: テーブルビューから新規ページ作成と既存スタンドアロンページの追加ができる

**Independent Test**: 新規ページ追加と既存ページ追加をそれぞれ実行し，テーブルの行として表示される

### Tests for User Story 3

- [ ] T034 [P] [US3] Add failing repository tests for SqlxPageRepository::set_database_id, find_standalone_pages (database_id IS NULL filter) in `src-tauri/src/infrastructure/persistence/page_repository.rs`
- [ ] T035 [P] [US3] Add failing IPC tests for add_page_to_database, add_existing_page_to_database (pageAlreadyInDatabase rejection), list_standalone_pages in `src-tauri/src/ipc/table_commands.rs`

### Implementation for User Story 3

- [ ] T036 [US3] Extend PageRepository trait with set_database_id and find_standalone_pages methods in `src-tauri/src/domain/page/repository.rs`
- [ ] T037 [US3] Implement set_database_id, find_standalone_pages in SqlxPageRepository in `src-tauri/src/infrastructure/persistence/page_repository.rs`
- [ ] T038 [US3] Implement add_page_to_database, add_existing_page_to_database, list_standalone_pages IPC commands in `src-tauri/src/ipc/table_commands.rs`, register module in `src-tauri/src/ipc/mod.rs` and add handlers to `src-tauri/src/lib.rs`
- [ ] T039 [P] [US3] Add addPageToDatabase, addExistingPageToDatabase, listStandalonePages to useTableData hook in `src/features/database/useTableData.ts`
- [ ] T040 [US3] Implement AddPageModal (list standalone pages, select to add to database) in `src/features/database/AddPageModal.tsx`
- [ ] T041 [US3] Add new-page creation row and existing-page-add button to TableView in `src/features/database/TableView.tsx`
- [ ] T041a [US3] Add `///` documentation for table_commands and Page database_id extension public items, run `cargo make qa-rs`

**Checkpoint**: User Story 3 is functional — pages can be added to databases

---

## Phase 6: User Story 4 - プロパティ値の編集 (Priority: P1)

**Goal**: テーブルビュー上で各プロパティ型の値をインライン編集し，即時保存される

**Independent Test**: 5種類すべてのプロパティ型で値を入力し，再起動後も保持される

### Tests for User Story 4

- [ ] T042 [P] [US4] Add failing domain tests for PropertyValue validation (Number: NaN/Infinity rejection, -0.0→0.0 normalization; Select: invalid option ID rejection; type mismatch between PropertyType and value) in `src-tauri/src/domain/property/entity.rs`
- [ ] T043 [P] [US4] Add failing repository tests for SqlxPropertyValueRepository::upsert (insert and update), find_by_page_and_property, find_by_page_id in `src-tauri/src/infrastructure/persistence/property_value_repository.rs`

### Implementation for User Story 4

- [ ] T044 [P] [US4] Implement SqlxPropertyValueRepository (upsert, find_by_page_and_property, find_by_page_id, find_by_property_id, delete_by_page_and_property) in `src-tauri/src/infrastructure/persistence/property_value_repository.rs`, register in `src-tauri/src/infrastructure/persistence/mod.rs`
- [ ] T045 [US4] Implement set_property_value (with pageNotInDatabase check, type-specific validation) and clear_property_value (no-op if not exists) IPC commands in `src-tauri/src/ipc/property_commands.rs`, add handlers to `src-tauri/src/lib.rs`
- [ ] T046 [P] [US4] Add setPropertyValue, clearPropertyValue to useTableData hook in `src/features/database/useTableData.ts`
- [ ] T047 [US4] Implement PropertyCell component (type-specific inline editors: text input, number input with NaN/Infinity client-side guard, date/datetime picker, select dropdown from options, checkbox toggle with immediate save) in `src/features/database/PropertyCell.tsx`
- [ ] T048 [US4] Add `///` documentation for PropertyValue-related public items and run `cargo make qa-rs`

**Checkpoint**: All property types can be edited inline and values persist

---

## Phase 7: User Story 5 - テーブルビューでのページ一覧表示 (Priority: P1)

**Goal**: データベースのページをテーブル形式で完全表示し，タイトルクリックでエディタ，ダブルクリックでインライン編集

**Independent Test**: 複数ページ×複数プロパティのテーブルが正しく表示され，タイトルクリック/ダブルクリックが機能する

### Tests for User Story 5

- [ ] T049 [P] [US5] Add failing repository tests for SqlxPropertyValueRepository::find_all_for_database (bulk fetch joining property_values with properties) in `src-tauri/src/infrastructure/persistence/property_value_repository.rs`
- [ ] T050 [P] [US5] Add failing IPC tests for get_table_data (aggregated TableDataDto with database, properties, rows with values) in `src-tauri/src/ipc/table_commands.rs`

### Implementation for User Story 5

- [ ] T051 [US5] Implement find_all_for_database in SqlxPropertyValueRepository in `src-tauri/src/infrastructure/persistence/property_value_repository.rs`
- [ ] T052 [US5] Implement get_table_data IPC command (fetches database, properties, pages with database_id, all property values; assembles into TableDataDto) in `src-tauri/src/ipc/table_commands.rs`, add handler to `src-tauri/src/lib.rs`
- [ ] T053 [US5] Implement TableRow component (title cell: click→navigate to editor, double-click→inline title edit with immediate save; property value cells using PropertyCell) in `src/features/database/TableRow.tsx`
- [ ] T054 [US5] Complete TableView with full data rendering (useTableData with get_table_data, TableHeader + TableRow grid, empty state for 0 properties showing title-only column, loading state) in `src/features/database/TableView.tsx`
- [ ] T055 [US5] Run full P1 integration verification: `cargo make qa`

**Checkpoint**: Full table view is functional with all P1 features

---

## Phase 8: User Story 6 - プロパティスキーマの編集 (Priority: P2)

**Goal**: プロパティの名前変更・並び替え・削除ができ，セレクト選択肢の追加・削除が管理できる

**Independent Test**: 名前変更→列ヘッダー更新，並び替え→列順変更，削除→列と値の消滅，選択肢削除→該当値リセット

### Tests for User Story 6

- [ ] T056 [P] [US6] Add failing repository tests for SqlxPropertyRepository::update_name (duplicate check), update_config, update_positions, delete (CASCADE verification) in `src-tauri/src/infrastructure/persistence/property_repository.rs`
- [ ] T057 [P] [US6] Add failing repository tests for SqlxPropertyValueRepository::reset_select_option (NULL reset for deleted option ID) in `src-tauri/src/infrastructure/persistence/property_value_repository.rs`
- [ ] T058 [P] [US6] Add failing IPC tests for update_property_name, update_property_config (select option add/delete with value reset), reorder_properties (full list required), delete_property in `src-tauri/src/ipc/property_commands.rs`

### Implementation for User Story 6

- [ ] T059 [US6] Implement SqlxPropertyRepository::update_name, update_config, update_positions, delete in `src-tauri/src/infrastructure/persistence/property_repository.rs`
- [ ] T060 [US6] Implement SqlxPropertyValueRepository::reset_select_option in `src-tauri/src/infrastructure/persistence/property_value_repository.rs`
- [ ] T061 [US6] Implement update_property_name, update_property_config (with select option delete → value reset in transaction), reorder_properties (validate full property ID list), delete_property IPC commands in `src-tauri/src/ipc/property_commands.rs`, add handlers to `src-tauri/src/lib.rs`
- [ ] T061a [P] [US6] Add updatePropertyName, updatePropertyConfig, reorderProperties, deleteProperty to useTableData hook in `src/features/database/useTableData.ts`
- [ ] T062 [P] [US6] Implement PropertyConfigPanel (rename input, select option list management with add/delete, date mode toggle, delete property with confirmation dialog) in `src/features/database/PropertyConfigPanel.tsx`
- [ ] T063 [US6] Integrate PropertyConfigPanel into TableHeader (column header click/popover opens config panel, drag-based or button-based reorder) in `src/features/database/TableHeader.tsx`
- [ ] T063a [US6] Add `///` documentation for property schema editing public items and run `cargo make qa-rs`

**Checkpoint**: Property schema can be fully managed

---

## Phase 9: User Story 7 - データベースの管理 (Priority: P2)

**Goal**: データベースのタイトル変更と削除ができる。削除時にページは保持される

**Independent Test**: タイトル変更後に一覧とテーブルヘッダーに反映。削除後にページがページ一覧に残存する

### Tests for User Story 7

- [ ] T064 [P] [US7] Add failing repository tests for SqlxDatabaseRepository::update_title, delete (verify: properties and property_values are cascade-deleted, pages.database_id is set to NULL, pages and blocks are preserved) in `src-tauri/src/infrastructure/persistence/database_repository.rs`
- [ ] T065 [P] [US7] Add failing IPC tests for update_database_title, delete_database in `src-tauri/src/ipc/database_commands.rs`

### Implementation for User Story 7

- [ ] T066 [US7] Implement SqlxDatabaseRepository::update_title, delete in `src-tauri/src/infrastructure/persistence/database_repository.rs`
- [ ] T067 [US7] Implement update_database_title, delete_database IPC commands in `src-tauri/src/ipc/database_commands.rs`, add handlers to `src-tauri/src/lib.rs`
- [ ] T067a [P] [US7] Add updateDatabaseTitle, deleteDatabase to useDatabase hook in `src/features/database/useDatabase.ts`
- [ ] T068 [US7] Add database title inline editing to TableView header and delete button with confirmation dialog (warning about property/value deletion, page preservation message) in `src/features/database/TableView.tsx`
- [ ] T069 [US7] Add database delete option to DatabaseListView item context menu in `src/features/database/DatabaseListView.tsx`
- [ ] T069a [US7] Add `///` documentation for database management public items and run `cargo make qa-rs`

**Checkpoint**: Database lifecycle (create, rename, delete) is complete

---

## Phase 10: User Story 8 - テーブルビューからのページ削除 (Priority: P2)

**Goal**: ページをデータベースから除外（ページ保持）または完全削除（ページ+ブロック削除）できる

**Independent Test**: 除外後にページ一覧に残存しプロパティ値は消滅。完全削除後にページ自体が消滅

### Tests for User Story 8

- [ ] T070 [P] [US8] Add failing repository tests for SqlxPropertyValueRepository::delete_by_page_and_database (removes only property values for that page in that database) in `src-tauri/src/infrastructure/persistence/property_value_repository.rs`
- [ ] T071 [P] [US8] Add failing IPC tests for remove_page_from_database (page.database_id → NULL, property values deleted, page preserved; idempotent for standalone pages) in `src-tauri/src/ipc/table_commands.rs`

### Implementation for User Story 8

- [ ] T072 [US8] Implement SqlxPropertyValueRepository::delete_by_page_and_database in `src-tauri/src/infrastructure/persistence/property_value_repository.rs`
- [ ] T073 [US8] Implement remove_page_from_database IPC command (transaction: set page.database_id = NULL + delete property values for page) in `src-tauri/src/ipc/table_commands.rs`, add handler to `src-tauri/src/lib.rs`
- [ ] T074 [US8] Add row context menu to TableRow with "データベースから除外" and "完全に削除" options (full delete reuses existing delete_page command) in `src/features/database/TableRow.tsx`
- [ ] T075 [US8] Add confirmation dialogs for exclude (property values will be lost) and full delete (page and blocks will be deleted) in `src/features/database/TableView.tsx`
- [ ] T075a [US8] Add `///` documentation for page removal public items and run `cargo make qa-rs`

**Checkpoint**: Page removal (exclude and full delete) works correctly

---

## Phase 11: Polish & Cross-Cutting Concerns

**Purpose**: Cross-story integration verification, performance, edge cases, final QA

- [ ] T076 Verify `PRAGMA foreign_keys = ON` is set at app startup in `src-tauri/src/infrastructure/persistence/database.rs` (required for CASCADE behavior)
- [ ] T077 [P] Add integration tests for data integrity: orphaned property values cannot exist after property/page/database deletion in `src-tauri/src/infrastructure/persistence/`
- [ ] T078 [P] Verify performance targets: table view with 100 pages × 10 properties loads ≤ 1s, inline edit save completes ≤ 500ms (manual verification or benchmark test)
- [ ] T079 [P] Verify edge cases: 1-page-1-database constraint enforcement, empty database table view, property count limit 50 rejection, select option limit 100 rejection, empty title rejection for both database and property
- [ ] T080 Run full QA suite: `cargo make qa` (fmt-rs, clippy, test, doc-test, doc-check, fmt-ts, lint-ts, ts-check, test-ts)
- [ ] T081 [P] Verify SC-001: end-to-end workflow (database creation → property definition → page addition → property value input) completes within 3 minutes by a first-time user. Manual walkthrough on dev machine

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Starts immediately
- **Foundational (Phase 2)**: Depends on Setup, blocks all user stories
- **US1 (Phase 3)**: Depends on Foundational — **MVP milestone**
- **US2 (Phase 4)**: Depends on US1 (database must exist for properties)
- **US3 (Phase 5)**: Depends on US1 (database must exist for page addition); **can run in parallel with US2**
- **US4 (Phase 6)**: Depends on US2 + US3 (properties and pages in database must exist)
- **US5 (Phase 7)**: Depends on US1–US4 (full table data needed for complete view)
- **US6 (Phase 8)**: Depends on US2 (properties must exist to edit schema); can start after US5
- **US7 (Phase 9)**: Depends on US1 (database must exist to manage); can start after US5
- **US8 (Phase 10)**: Depends on US3 (pages must be in database to remove); can start after US5
- **Polish (Phase 11)**: Depends on all user stories

### Within Each User Story

- Tests MUST be written first and MUST fail before implementation
- Frontend and backend tasks can run in parallel only when IPC contracts are implemented
- Documentation and QA tasks are required for story completion

### Parallel Opportunities

- Tasks marked `[P]` can run in parallel within the same phase
- **US2 and US3 can run in parallel** after US1 completes (different files, no shared state)
- **US6, US7, US8 can run in parallel** after US5 completes (independent feature additions)
- Domain tests, repository tests, and frontend components target different files

---

## Implementation Strategy

### MVP First

1. Complete Setup (Phase 1)
2. Complete Foundational (Phase 2)
3. Complete User Story 1 — データベースの作成
4. Validate US1 independently before expanding scope

### Incremental Delivery

1. Ship P1 stories: US1 → (US2 ∥ US3) → US4 → US5
2. Re-run `cargo make qa` after each story
3. Add P2 stories (US6 ∥ US7 ∥ US8) after all P1 stories are green
4. Polish phase verifies cross-cutting concerns and performance targets

---

## Notes

- All domain entities use UUIDv7 (time-ordered) for IDs
- `PropertyConfig` uses serde internally tagged enum (`#[serde(tag = "type")]`)
- Select option values are stored by ID (not display name) for rename stability
- `PropertyValue` for checkbox defaults to `false`; all other types default to no record (NULL)
- `pages.database_id` ON DELETE SET NULL preserves pages when database is deleted
- `property_values` FK ON DELETE CASCADE handles cleanup for both page and property deletion
- Frontend error messages are generated from `kind` field; `message` is debug-only English
- `clear_property_value` deletes the PropertyValue row (not setting columns to NULL)
- `-0.0` is normalized to `0.0` for number type values
- Empty string `""` is a valid text value; use `clear_property_value` for unset state
