# Tasks: ブロックエディタ

**Input**: Design documents from `/specs/002-block-editor/`
**Prerequisites**: plan.md (required), spec.md (required), research.md,
data-model.md, contracts/ipc-commands.md, quickstart.md

**Tests**: Tests are MANDATORY. Each user story must start with failing tests or an
equivalent executable verification task before implementation tasks appear.

**Organization**: Tasks are grouped by user story so each story can be implemented,
tested, and reviewed independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel if they touch different files and have no dependency
- **[Story]**: Which user story this task belongs to, for example `US1`
- Include exact file paths in every task description

## Path Conventions

- **Frontend**: `src/` for TypeScript/React
- **Backend**: `src-tauri/src/` for Rust
- **Migrations**: `src-tauri/migrations/` for schema changes

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish only the minimum scaffolding required for the feature

- [ ] T001 Create backend module directories: `src-tauri/src/domain/block/`, `src-tauri/src/domain/editor/`
- [ ] T002 [P] Create frontend feature directory: `src/features/editor/`
- [ ] T003 [P] Add migration file `src-tauri/migrations/0002_create_blocks.sql` with `blocks` table, foreign key (`ON DELETE CASCADE`), and composite index `(page_id, position ASC)` per data-model.md schema

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core domain types, errors, repository trait, and IPC infrastructure that MUST exist before any user story lands

**⚠️ CRITICAL**: No user story work begins until this phase is complete

- [ ] T004 [P] Define `BlockId`, `BlockContent`, `BlockPosition` value objects and `Block` entity in `src-tauri/src/domain/block/entity.rs` — `BlockContent` validates 0–10,000 chars via `TryFrom<String>`, `BlockPosition` validates non-negative via `TryFrom<i64>`, `BlockId` wraps `Uuid` with `Uuid::now_v7()` factory
- [ ] T005 [P] Define `BlockError` enum (`ContentTooLong`, `InvalidPosition`, `NotFound`, `CannotMoveUp`, `CannotMoveDown`) in `src-tauri/src/domain/block/error.rs`
- [ ] T006 [P] Create `src-tauri/src/domain/block/mod.rs` re-exporting entity and error types
- [ ] T007 Implement `EditorSession` in `src-tauri/src/domain/editor/session.rs` — pure domain service with `new()`, `add_block()`, `edit_block_content()`, `move_block_up()`, `move_block_down()`, `remove_block()`, `blocks()`, `is_dirty()`, `mark_saved()`, `page_id()` per data-model.md
- [ ] T008 [P] Create `src-tauri/src/domain/editor/mod.rs` re-exporting `EditorSession`
- [ ] T009 Register new modules in `src-tauri/src/domain/mod.rs` — add `pub mod block; pub mod editor;`
- [ ] T010 [P] Define `BlockRepository` trait and `SqlxBlockRepository` implementation in `src-tauri/src/infrastructure/persistence/block_repository.rs` — `load_blocks(page_id)` returns `Vec<Block>` sorted by position, `save_all(page_id, blocks)` does delete-and-reinsert in transaction
- [ ] T011 [P] Add `PRAGMA foreign_keys = ON` to database initialization in `src-tauri/src/infrastructure/persistence/database.rs`
- [ ] T012 Register `block_repository` module in `src-tauri/src/infrastructure/persistence/mod.rs`
- [ ] T013 [P] Add `Block(BlockError)` variant to `CommandError` in `src-tauri/src/ipc/error.rs` with `From<BlockError>` impl, and extend Serialize impl with kind/message mappings per contracts/ipc-commands.md
- [ ] T014 [P] Add `EditorStateDto` and `BlockDto` structs with `serde(rename_all = "camelCase")` in `src-tauri/src/ipc/dto.rs`, with conversion from domain types
- [ ] T015 [P] Define TypeScript types `EditorState`, `Block`, and extend `CommandError` kinds in `src/features/editor/types.ts` per contracts/ipc-commands.md
- [ ] T016 Update `AppState` in `src-tauri/src/lib.rs` — add `sessions: tokio::sync::Mutex<HashMap<PageId, EditorSession>>` field and initialize it

**Checkpoint**: Foundation is ready — domain types, repository, errors, DTOs, and AppState all in place. User stories can proceed.

---

## Phase 3: User Story 1 — ページを開いてブロック一覧を表示する (Priority: P1) 🎯 MVP

**Goal**: ユーザーがページタイトルをクリックするとエディタ画面に切り替わり，ブロックが position 順に表示される。戻るボタンでページ一覧に復帰する。

**Independent Test**: ブロックを持つページと持たないページの両方を開き，画面切り替えと一覧への復帰が正しく動作することを確認する。

### Tests for User Story 1

- [ ] T017 [P] [US1] Add unit tests for `EditorSession::new()` — verify blocks loaded in position order, `is_dirty` starts false, empty blocks list accepted — in `src-tauri/src/domain/editor/session.rs` (`#[cfg(test)]` module)
- [ ] T018 [P] [US1] Add integration tests for `BlockRepository::load_blocks()` — in-memory SQLite, verify empty result and ordered result — in `src-tauri/src/infrastructure/persistence/block_repository.rs` (`#[cfg(test)]` module)

### Implementation for User Story 1

- [ ] T019 [US1] Implement `open_editor` IPC command in `src-tauri/src/ipc/editor_commands.rs` — load blocks via `BlockRepository`, create `EditorSession`, store in `AppState.sessions`, return `EditorStateDto`
- [ ] T020 [US1] Implement `close_editor` IPC command in `src-tauri/src/ipc/editor_commands.rs` — remove session from `AppState.sessions`, idempotent (no error if missing)
- [ ] T021 [US1] Register `editor_commands` module in `src-tauri/src/ipc/mod.rs` and register `open_editor`, `close_editor` commands in `invoke_handler` in `src-tauri/src/lib.rs`
- [ ] T022 [P] [US1] Implement `useEditor` hook in `src/features/editor/useEditor.ts` — `openEditor(pageId)`, `closeEditor(pageId)` IPC wrappers, manage `EditorState` and loading state
- [ ] T023 [P] [US1] Implement `EditorToolbar` component in `src/features/editor/EditorToolbar.tsx` and `src/features/editor/EditorToolbar.module.css` — back button, page title display, save button placeholder, dirty indicator placeholder
- [ ] T024 [P] [US1] Implement `BlockItem` component in `src/features/editor/BlockItem.tsx` and `src/features/editor/BlockItem.module.css` — display block content as read-only text (editing in US3), placeholder action buttons
- [ ] T025 [US1] Implement `BlockEditor` container in `src/features/editor/BlockEditor.tsx` and `src/features/editor/BlockEditor.module.css` — use `useEditor` hook, render `EditorToolbar` + block list via `BlockItem`, show empty state message when no blocks
- [ ] T026 [US1] Add view routing in `src/App.tsx` — `currentView` state (`{ type: 'list' } | { type: 'editor', pageId: string }`), render `PageListView` or `BlockEditor` based on state
- [ ] T027 [US1] Add `onPageClick` callback prop to `src/features/pages/PageListView.tsx` and wire title click in `src/features/pages/PageItem.tsx` to trigger editor navigation
- [ ] T028 [US1] Run `cargo make qa` and verify all tests pass

**Checkpoint**: User Story 1 is fully functional — pages can be opened in editor view and navigated back to list view.

---

## Phase 4: User Story 2 — テキストブロックを追加する (Priority: P1)

**Goal**: ユーザーがエディタ画面でブロック追加操作を行うと，空のテキストブロックが末尾に追加され，未保存状態になる。

**Independent Test**: エディタ画面でブロックを追加し，ブロックが末尾に表示され編集可能になることを確認する。

### Tests for User Story 2

- [ ] T029 [P] [US2] Add unit tests for `EditorSession::add_block()` — verify block appended at end with position = len, `is_dirty` becomes true, UUIDv7 assigned, empty content — in `src-tauri/src/domain/editor/session.rs`

### Implementation for User Story 2

- [ ] T030 [US2] Implement `add_block` IPC command in `src-tauri/src/ipc/editor_commands.rs` — get session, call `add_block()`, return `EditorStateDto`
- [ ] T031 [US2] Register `add_block` command in `invoke_handler` in `src-tauri/src/lib.rs`
- [ ] T032 [US2] Add `addBlock(pageId)` to `useEditor` hook in `src/features/editor/useEditor.ts`
- [ ] T033 [US2] Add "ブロック追加" button to `BlockEditor.tsx` — call `addBlock`, auto-focus new block (last element), hide empty state when blocks exist
- [ ] T034 [US2] Run `cargo make qa` and verify all tests pass

**Checkpoint**: User Story 2 is fully functional — blocks can be added to pages.

---

## Phase 5: User Story 3 — ブロックの内容を編集する (Priority: P1)

**Goal**: ユーザーがブロックのテキスト領域に入力すると内容がリアルタイムに反映され，`onBlur` でバックエンドに同期される。

**Independent Test**: 既存ブロックの内容を変更し，画面上で反映されること，`onBlur` でバックエンドに同期されることを確認する。

### Tests for User Story 3

- [ ] T035 [P] [US3] Add unit tests for `EditorSession::edit_block_content()` — verify content updated, `is_dirty` true, `ContentTooLong` error at 10,001 chars, `NotFound` error for invalid ID, empty content accepted — in `src-tauri/src/domain/editor/session.rs`
- [ ] T036 [P] [US3] Add unit tests for `BlockContent::try_from()` — verify 0 chars OK, 10,000 chars OK, 10,001 chars error — in `src-tauri/src/domain/block/entity.rs`

### Implementation for User Story 3

- [ ] T037 [US3] Implement `edit_block_content` IPC command in `src-tauri/src/ipc/editor_commands.rs` — get session, call `edit_block_content()`, return `EditorStateDto`
- [ ] T038 [US3] Register `edit_block_content` command in `invoke_handler` in `src-tauri/src/lib.rs`
- [ ] T039 [US3] Add `editBlockContent(pageId, blockId, content)` to `useEditor` hook in `src/features/editor/useEditor.ts`
- [ ] T040 [US3] Update `BlockItem` component in `src/features/editor/BlockItem.tsx` — replace read-only display with `<textarea>`, local state for input buffer, `onBlur` triggers `editBlockContent` IPC, `maxLength={10000}` hint
- [ ] T041 [US3] Run `cargo make qa` and verify all tests pass

**Checkpoint**: User Story 3 is fully functional — block content can be edited in-place.

---

## Phase 6: User Story 6 — 編集内容を明示的に保存する (Priority: P1)

**Goal**: ユーザーが保存ボタンまたは Ctrl+S で全ブロックを一括永続化する。保存後に再度開いたとき内容と順序が再現される。

**Independent Test**: ブロック追加・編集後に保存し，ページを閉じて再度開き，内容と順序が維持されていることを確認する。

### Tests for User Story 6

- [ ] T042 [P] [US6] Add unit tests for `EditorSession::mark_saved()` — verify `is_dirty` becomes false — in `src-tauri/src/domain/editor/session.rs`
- [ ] T043 [P] [US6] Add integration tests for `BlockRepository::save_all()` — in-memory SQLite, verify delete-and-reinsert in transaction, `created_at` preserved, `updated_at` refreshed, position normalized — in `src-tauri/src/infrastructure/persistence/block_repository.rs`

### Implementation for User Story 6

- [ ] T044 [US6] Implement `save_editor` IPC command in `src-tauri/src/ipc/editor_commands.rs` — skip save if not dirty, call `BlockRepository::save_all()`, then `mark_saved()`, return `EditorStateDto` with `isDirty: false`
- [ ] T045 [US6] Register `save_editor` command in `invoke_handler` in `src-tauri/src/lib.rs`
- [ ] T046 [US6] Add `saveEditor(pageId)` to `useEditor` hook in `src/features/editor/useEditor.ts` — sync focused block content before save, handle saving state
- [ ] T047 [US6] Wire save button in `EditorToolbar.tsx` — call `saveEditor`, show success toast (sonner), show error toast on failure (FR-014)
- [ ] T048 [US6] Add Ctrl+S keyboard shortcut in `useEditor.ts` — `useEffect` with `keydown` listener, sync focused block then save, `e.preventDefault()` to suppress browser save dialog
- [ ] T049 [US6] Run `cargo make qa` and verify all tests pass

**Checkpoint**: User Story 6 is fully functional — blocks persist across editor sessions.

---

## Phase 7: User Story 4 — ブロックを並び替える (Priority: P2)

**Goal**: ユーザーがブロックの上移動・下移動ボタンで順序を変更できる。先頭/末尾のボタンは無効化される。

**Independent Test**: 複数ブロックで上下移動を行い，順序が正しく入れ替わること，境界でボタンが無効化されることを確認する。

### Tests for User Story 4

- [ ] T050 [P] [US4] Add unit tests for `EditorSession::move_block_up()` and `move_block_down()` — verify swap, `CannotMoveUp` at top, `CannotMoveDown` at bottom, `NotFound` for invalid ID, `is_dirty` true — in `src-tauri/src/domain/editor/session.rs`

### Implementation for User Story 4

- [ ] T051 [US4] Implement `move_block_up` and `move_block_down` IPC commands in `src-tauri/src/ipc/editor_commands.rs`
- [ ] T052 [US4] Register `move_block_up`, `move_block_down` commands in `invoke_handler` in `src-tauri/src/lib.rs`
- [ ] T053 [US4] Add `moveBlockUp(pageId, blockId)` and `moveBlockDown(pageId, blockId)` to `useEditor` hook in `src/features/editor/useEditor.ts`
- [ ] T054 [US4] Add up/down move buttons to `BlockItem` component in `src/features/editor/BlockItem.tsx` — disable up button for first block (position === 0), disable down button for last block (position === blocks.length - 1)
- [ ] T055 [US4] Run `cargo make qa` and verify all tests pass

**Checkpoint**: User Story 4 is fully functional — blocks can be reordered.

---

## Phase 8: User Story 5 — ブロックを削除する (Priority: P2)

**Goal**: ユーザーがブロックの削除ボタンをクリックするとブロックが除去され，残りの順序が維持される。

**Independent Test**: ブロックを削除し画面から消えること，残りのブロックの順序が維持されることを確認する。

### Tests for User Story 5

- [ ] T056 [P] [US5] Add unit tests for `EditorSession::remove_block()` — verify removal, position renumbered, `NotFound` for invalid ID, last block removal returns empty, `is_dirty` true — in `src-tauri/src/domain/editor/session.rs`

### Implementation for User Story 5

- [ ] T057 [US5] Implement `remove_block` IPC command in `src-tauri/src/ipc/editor_commands.rs`
- [ ] T058 [US5] Register `remove_block` command in `invoke_handler` in `src-tauri/src/lib.rs`
- [ ] T059 [US5] Add `removeBlock(pageId, blockId)` to `useEditor` hook in `src/features/editor/useEditor.ts`
- [ ] T060 [US5] Add delete button to `BlockItem` component in `src/features/editor/BlockItem.tsx` — call `removeBlock`, show empty state in `BlockEditor.tsx` when all blocks removed
- [ ] T061 [US5] Run `cargo make qa` and verify all tests pass

**Checkpoint**: User Story 5 is fully functional — blocks can be deleted.

---

## Phase 9: User Story 7 — 未保存状態を視覚的に確認する (Priority: P2)

**Goal**: 未保存インジケータの表示・非表示，未保存状態での画面離脱時に確認ダイアログが表示される。

**Independent Test**: ブロック変更で未保存インジケータ表示，保存後に消える，未保存時の離脱で確認ダイアログが表示されることを確認する。

### Tests for User Story 7

- [ ] T062 [P] [US7] Add unit tests for `EditorSession::is_dirty()` state transitions — starts false, becomes true after any mutation, returns to false after `mark_saved()` — in `src-tauri/src/domain/editor/session.rs` (may already be covered; ensure comprehensive coverage)

### Implementation for User Story 7

- [ ] T063 [US7] Wire dirty indicator in `EditorToolbar.tsx` — show/hide based on `isDirty` from `EditorState`
- [ ] T064 [US7] Implement `UnsavedConfirmModal` component in `src/features/editor/UnsavedConfirmModal.tsx` and `src/features/editor/UnsavedConfirmModal.module.css` — "未保存の変更があります。破棄しますか？" message, cancel and discard buttons
- [ ] T065 [US7] Wire unsaved confirm dialog in `BlockEditor.tsx` — when back button clicked and `isDirty`, show `UnsavedConfirmModal`; on discard call `closeEditor` and navigate to list; on cancel stay in editor
- [ ] T066 [US7] Run `cargo make qa` and verify all tests pass

**Checkpoint**: User Story 7 is fully functional — unsaved state is visible and protected.

---

## Phase 10: Polish & Cross-Cutting Concerns

**Purpose**: Work that spans multiple user stories

- [ ] T067 Verify `PRAGMA foreign_keys = ON` works — add integration test in `src-tauri/src/infrastructure/persistence/block_repository.rs` that deletes a page and confirms its blocks are cascade-deleted
- [ ] T068 [P] Verify performance: create 1,000 blocks in-memory, save, reload — confirm both operations complete in <1s — add as benchmark or integration test in `src-tauri/src/infrastructure/persistence/block_repository.rs`
- [ ] T069 Run full QA: `cargo make qa` — formatting, lint, tests, doc-tests, TypeScript checks all pass
- [ ] T070 Verify all 7 user story acceptance scenarios can be manually exercised via `cargo make serve`

---

## Dependencies & Execution Order

### Phase Dependencies

```
Phase 1: Setup ─────────────────────────────────────► immediate
Phase 2: Foundational ──────────────────────────────► depends on Phase 1
Phase 3: US1 (open/close editor) ───────────────────► depends on Phase 2
Phase 4: US2 (add block) ──────────────────────────► depends on Phase 3
Phase 5: US3 (edit content) ────────────────────────► depends on Phase 3
Phase 6: US6 (save) ───────────────────────────────► depends on Phase 3
Phase 7: US4 (reorder) ────────────────────────────► depends on Phase 3
Phase 8: US5 (delete) ─────────────────────────────► depends on Phase 3
Phase 9: US7 (unsaved indicator + confirm) ─────────► depends on Phase 6
Phase 10: Polish ───────────────────────────────────► depends on all above
```

### Parallel Opportunities

- **Phase 2**: T004, T005, T006, T008 in parallel (different files). T010, T011, T013, T014, T015 in parallel (different files).
- **Phase 3**: T022, T023, T024 frontend tasks in parallel after IPC commands registered.
- **Phases 4, 5, 7, 8**: US2, US4, US5 can run in parallel after US1 completes (independent user stories touching same files — serialize if conflict).
- **Phase 6**: US6 can run in parallel with US4/US5 after US1 completes.

---

## Implementation Strategy

### MVP First

1. Complete Phase 1 (Setup)
2. Complete Phase 2 (Foundational)
3. Complete Phase 3 (US1: open/close editor)
4. Complete Phase 4 (US2: add block) + Phase 5 (US3: edit content) + Phase 6 (US6: save)
5. Validate end-to-end: add → edit → save → reopen flow

### Incremental Delivery

1. Ship US1 first — editor can open and display blocks
2. Add US2 + US3 + US6 together — full create/edit/save cycle (functional MVP)
3. Add US4 + US5 — reorder and delete (enriched editing)
4. Add US7 — unsaved protection (polish)
5. Re-run `cargo make qa` after each story

---

## Notes

- All Rust code must avoid `unwrap()`, `expect()`, `panic!()`, `unsafe` per constitution
- Every IPC command returns `Result<..., CommandError>` with proper error serialization
- `EditorSession` is pure domain logic — no DB dependency, fully testable without infrastructure
- Frontend is a thin UI layer — no business logic, state comes from backend `EditorState`
- `onBlur` sync strategy: textarea local state → `edit_block_content` IPC on blur → before save, sync focused block explicitly
