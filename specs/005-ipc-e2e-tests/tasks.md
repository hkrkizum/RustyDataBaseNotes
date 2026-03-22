# Tasks: IPC テストおよび E2E テストの追加

**Input**: Design documents from `/specs/005-ipc-e2e-tests/`
**Prerequisites**: plan.md (required), spec.md (required), research.md,
data-model.md, contracts/test-helpers.md, quickstart.md

**Tests**: This feature's deliverable IS tests. US1/US2 tasks are IPC test
implementation; US3 tasks are E2E test implementation. Each task produces
executable test code verified by `cargo make test` or `cargo make e2e`.

**Organization**: Tasks are grouped by user story so each story can be
implemented, tested, and reviewed independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel if they touch different files and have no dependency
- **[Story]**: Which user story this task belongs to (US1, US2, US3)
- Include exact file paths in every task description

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create directory structures and install dependencies for IPC and E2E tests

- [ ] T001 Create IPC test module directory `src-tauri/src/ipc/tests/` with `mod.rs` that declares submodules (helpers, 6 domain test files)
- [ ] T002 [P] Create E2E test directory `e2e/` with `package.json` (WebDriverIO, @wdio/cli, @wdio/local-runner, @wdio/mocha-framework, @wdio/spec-reporter, typescript, ts-node, better-sqlite3, @types/better-sqlite3) and `tsconfig.json`
- [ ] T003 [P] Register `tests` submodule in `src-tauri/src/ipc/mod.rs` under `#[cfg(test)]`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Extract inner functions from all command handlers, build test helpers,
and add app-level E2E support. MUST complete before any user story work.

**⚠️ CRITICAL**: No user story work begins until this phase is complete

### Inner Function Extraction (Production Code)

- [ ] T004 [P] Extract `_inner` functions from `src-tauri/src/ipc/database_commands.rs` (5 commands: create/list/get/update/delete_database) per R-001 pattern
- [ ] T005 [P] Extract `_inner` functions from `src-tauri/src/ipc/page_commands.rs` (5 commands: create/list/get/update/delete_page) per R-001 pattern
- [ ] T006 [P] Extract `_inner` functions from `src-tauri/src/ipc/editor_commands.rs` (8 commands: open/close/add_block/edit_block_content/move_block_up/move_block_down/remove_block/save_editor) per R-001 pattern
- [ ] T007 [P] Extract `_inner` functions from `src-tauri/src/ipc/property_commands.rs` (9 commands: add/list/update_name/update_config/reorder/delete_property, reset_select_option, set/clear_property_value) per R-001 pattern
- [ ] T008 [P] Extract `_inner` functions from `src-tauri/src/ipc/table_commands.rs` (5 commands: add_page_to_database/add_existing_page/list_standalone_pages/remove_page_from_database/get_table_data) per R-001 pattern
- [ ] T009 [P] Extract `_inner` functions from `src-tauri/src/ipc/view_commands.rs` (6 commands: get/reset_view, update_sort/filter/group_conditions, toggle_group_collapsed) per R-001 pattern

### Test Infrastructure

- [ ] T010 Implement `TempDbGuard` struct and `setup_test_state()` function in `src-tauri/src/ipc/tests/helpers.rs` per R-002 pattern and contracts/test-helpers.md
- [ ] T011 [P] Add `RDBN_DB_PATH` environment variable support in app database initialization (`src-tauri/src/`) for E2E database isolation per FR-010
- [ ] T012 [P] Add `cargo make e2e` task to `Makefile.toml` per R-006 pattern (build → tauri-driver start → wdio run → tauri-driver stop → cleanup)
- [ ] T012a [P] Verify `flake.nix` devshell includes E2E runtime dependencies: `python3`（better-sqlite3 ネイティブビルド用），`xvfb-run`（WSLg 無効環境での headless E2E 実行用，`xorg.xvfb` パッケージ）。不足があれば `flake.nix` の `projectPackagesFor` に追加する

**Checkpoint**: All 38 inner functions extracted, test helpers compile, `cargo make check` passes

---

## Phase 3: User Story 1 — IPC コマンドハンドラの正常系テスト (Priority: P1) 🎯 MVP

**Goal**: All 38 IPC commands have passing normal-case tests that verify CRUD operations
return correct DTOs per data-model.md field specifications.

**Independent Test**: `TEST_FILTER="ipc::tests" cargo make test-filter` — all normal-case tests pass

### Implementation for User Story 1

- [ ] T013 [P] [US1] Write normal-case tests for 5 database commands (create→DTO fields, list→all returned, get→ID match, update→field change, delete→not found after) in `src-tauri/src/ipc/tests/database_commands_test.rs`
- [ ] T014 [P] [US1] Write normal-case tests for 5 page commands (create→DTO fields, list→all returned, get→ID match, update→title change, delete→not found after) in `src-tauri/src/ipc/tests/page_commands_test.rs`
- [ ] T015 [P] [US1] Write normal-case tests for 8 editor commands (open→EditorStateDto, add_block→block in state, edit_block_content→content updated, move_up/down→position changed, remove_block→block removed, save→is_dirty false, close→session removed) in `src-tauri/src/ipc/tests/editor_commands_test.rs`
- [ ] T016 [P] [US1] Write normal-case tests for 9 property commands (add→PropertyDto fields, list→all returned, update_name→name changed, update_config→config changed, reorder→positions updated, delete→removed, reset_select_option→option reset, set_value→PropertyValueDto, clear_value→value cleared) in `src-tauri/src/ipc/tests/property_commands_test.rs`
- [ ] T017 [P] [US1] Write normal-case tests for 5 table commands (add_page_to_database→page with database_id, add_existing_page→page linked, list_standalone_pages→unlinked pages only, remove_page_from_database→page unlinked, get_table_data→TableDataDto structure) in `src-tauri/src/ipc/tests/table_commands_test.rs`
- [ ] T018 [P] [US1] Write normal-case tests for 6 view commands (get→ViewDto fields, reset→default conditions, update_sort→sort applied, update_filter→filter applied, update_group→group applied, toggle_group_collapsed→collapsed state toggled) in `src-tauri/src/ipc/tests/view_commands_test.rs`
- [ ] T019 [US1] Run `cargo make test` and verify all 38 normal-case IPC tests pass. Record test count and execution time

**Checkpoint**: SC-001 met — all 38 IPC commands have passing normal-case tests

---

## Phase 4: User Story 2 — IPC コマンドハンドラの異常系・境界値テスト (Priority: P2)

**Goal**: All domains have error-case tests verifying CommandError variants per
data-model.md error mapping. Editor has a stateful flow test.

**Independent Test**: `TEST_FILTER="ipc::tests" cargo make test-filter` — all normal + error tests pass

### Implementation for User Story 2

- [ ] T020 [P] [US2] Write error tests for database commands (titleEmpty, databaseNotFound, cascade delete verifying pages→blocks, properties→property_values, views all removed) in `src-tauri/src/ipc/tests/database_commands_test.rs`
- [ ] T021 [P] [US2] Write error tests for page commands (titleEmpty, titleTooLong, notFound for get/update/delete) in `src-tauri/src/ipc/tests/page_commands_test.rs`
- [ ] T022 [P] [US2] Write error tests for editor commands (session not started for add/edit/move/remove/save, contentTooLong, blockNotFound, cannotMoveUp/Down, duplicate open returns existing session, operations after close return error) in `src-tauri/src/ipc/tests/editor_commands_test.rs`
- [ ] T023 [P] [US2] Write error tests for property commands (propertyNameEmpty, duplicatePropertyName, invalidNumber, typeMismatch for set_value) in `src-tauri/src/ipc/tests/property_commands_test.rs`
- [ ] T024 [P] [US2] Write error tests for table commands (databaseNotFound for add_page_to_database, notFound for non-existent page) in `src-tauri/src/ipc/tests/table_commands_test.rs`
- [ ] T025 [P] [US2] Write error tests for view commands (viewNotFound, invalidSortCondition referencing deleted property, verify view auto-excludes conditions referencing deleted properties) in `src-tauri/src/ipc/tests/view_commands_test.rs`
- [ ] T026 [US2] Write editor stateful flow test (open→add_block→edit_block_content→move_block_up→remove_block→save→close, verify each step returns correct EditorStateDto and save persists data) in `src-tauri/src/ipc/tests/editor_commands_test.rs`
- [ ] T027 [US2] Run `cargo make test` and verify all IPC tests (normal + error) pass. Record total test count

**Checkpoint**: SC-002 met — all domains have normal + error tests, editor has stateful flow test

---

## Phase 5: User Story 3 — E2E テストによるユーザーワークフロー検証 (Priority: P3)

**Goal**: 4 major workflows verified through actual UI automation with tauri-driver + WebDriverIO

**Independent Test**: `cargo make e2e` — all 4 E2E workflow scenarios pass

### E2E Infrastructure

- [ ] T028 [US3] Create WebDriverIO configuration in `e2e/wdio.conf.ts` per contracts/test-helpers.md (tauri:options, mocha framework, spec reporter, 30s timeout)
- [ ] T029 [P] [US3] Implement E2E helpers (`clearDatabase()`, `waitForApp()`, `findByTestId()`) in `e2e/helpers/app.ts` per contracts/test-helpers.md
- [ ] T030 [P] [US3] Add `data-testid` attributes to React components needed for E2E selectors (sidebar page list, page title input, editor block elements, database table view, filter controls)

### E2E Workflow Tests

- [ ] T031 [P] [US3] Write page workflow E2E test (create page → input title → verify page appears in sidebar list) in `e2e/specs/page-workflow.spec.ts`
- [ ] T032 [P] [US3] Write database workflow E2E test (add record → set property value → verify table view shows record) in `e2e/specs/database-workflow.spec.ts`
- [ ] T033 [P] [US3] Write view workflow E2E test (set text equality filter → verify matching records shown, non-matching hidden) in `e2e/specs/view-workflow.spec.ts`
- [ ] T034 [P] [US3] Write editor workflow E2E test (add text block → edit → move → delete → save → reload and verify persistence) in `e2e/specs/editor-workflow.spec.ts`
- [ ] T035 [US3] Run `cargo make e2e` and verify all 4 E2E workflow tests pass. Record execution time

**Checkpoint**: SC-003 met — all 4 major workflows have passing E2E tests

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: QA gate integration, documentation, and final verification

- [ ] T036 Verify `cargo make qa` includes IPC tests and passes (SC-004: IPC tests in quality gate)
- [ ] T037 [P] Add `///` doc comments to all `pub(crate)` test helper functions and inner logic functions per Constitution VI
- [ ] T038 Run full QA and E2E: `cargo make qa && cargo make e2e` — all green

---

## Dependencies & Execution Order

### Phase Dependencies

```text
Phase 1 (Setup) ──→ Phase 2 (Foundational) ──┬──→ Phase 3 (US1: P1) ──→ Phase 4 (US2: P2) ──┐
                                               │                                                │
                                               └──→ Phase 5 (US3: P3) ────────────────────────┤
                                                                                                │
                                                                                                └──→ Phase 6 (Polish)
```

- **Setup (Phase 1)**: Starts immediately
- **Foundational (Phase 2)**: Depends on Setup, blocks all user stories
- **US1 (Phase 3)**: Depends on Foundational (inner functions + helpers). **MVP scope**
- **US2 (Phase 4)**: Depends on US1 (builds on normal-case test patterns)
- **US3 (Phase 5)**: Depends on Foundational (RDBN_DB_PATH + Makefile e2e task). Can run in parallel with US1/US2
- **Polish (Phase 6)**: Depends on all user stories complete

### Within Each User Story

- US1/US2: All 6 domain test files can be written in parallel (`[P]` tasks)
- US3: E2E infrastructure (T028-T030) must complete before workflow specs (T031-T034)
- QA verification tasks (T019, T027, T035) must run after their respective implementations

### Parallel Opportunities

| Parallel Group | Tasks | Condition |
|---------------|-------|-----------|
| Inner function extraction | T004–T009 | After Phase 1 complete |
| US1 domain tests | T013–T018 | After T004–T010 complete |
| US2 domain error tests | T020–T025 | After T019 (US1 verified) |
| US3 E2E infrastructure | T029–T030 | After T028 (wdio.conf.ts) |
| US3 E2E workflow specs | T031–T034 | After T029–T030 complete |
| US1 + US3 in parallel | Phase 3 + Phase 5 | Both depend only on Phase 2 |

---

## Implementation Strategy

### MVP First

1. Complete Phase 1 (Setup)
2. Complete Phase 2 (Foundational — inner function extraction + test helpers)
3. Complete Phase 3 (US1 — IPC normal-case tests for all 38 commands)
4. Validate with `cargo make test` before expanding scope

### Incremental Delivery

1. US1 delivers SC-001 (all 38 commands have normal-case tests)
2. US2 delivers SC-002 (error/boundary tests added)
3. US3 delivers SC-003 (E2E workflow tests)
4. Phase 6 delivers SC-004 (QA gate integration verified)
5. Re-run `cargo make qa` after each story

---

## Notes

- Inner function extraction (T004–T009) modifies production code but does NOT change public API — existing `#[tauri::command]` signatures are unchanged
- IPC tests use `#[cfg(test)]` and compile only in test mode — no production binary impact
- E2E tests are in separate `e2e/` directory with their own Node.js dependencies
- `cargo make e2e` is NOT included in `cargo make qa` — run separately before merge
- Test code permits `unwrap()`, `expect()`, `assert!()` per Constitution VII exception
