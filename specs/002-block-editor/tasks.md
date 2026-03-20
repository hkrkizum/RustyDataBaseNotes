# Tasks: ブロックエディタ

**Input**: Design documents from `/specs/002-block-editor/`
**Prerequisites**: plan.md (required), spec.md (required), research.md,
data-model.md, contracts/ipc-commands.md, quickstart.md

**Tests**: Tests are MANDATORY. Each task group must start with failing tests or an
equivalent executable verification task before implementation tasks appear.
This applies to ALL phases, including foundational infrastructure.

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

### Tests for Foundational Types

- [ ] T004 [P] Add unit tests for `BlockContent::try_from()` and `BlockPosition::try_from()` — verify 0 chars OK, 10,000 chars OK, 10,001 chars error for BlockContent; verify 0 OK, negative value error for BlockPosition — in `src-tauri/src/domain/block/entity.rs` (`#[cfg(test)]` module)

### Implementation for Foundational Types

- [ ] T005 Define `BlockId`, `BlockContent`, `BlockPosition` value objects and `Block` entity in `src-tauri/src/domain/block/entity.rs` — `BlockContent` validates 0–10,000 chars via `TryFrom<String>`, `BlockPosition` validates non-negative via `TryFrom<i64>`, `BlockId` wraps `Uuid` with `Uuid::now_v7()` factory（**depends on T004**: 同一ファイルのテストが先行する必要がある — Red-Green）
- [ ] T006 [P] Define `BlockError` enum (`ContentTooLong`, `InvalidPosition`, `NotFound`, `CannotMoveUp`, `CannotMoveDown`) in `src-tauri/src/domain/block/error.rs`
- [ ] T007 [P] Create `src-tauri/src/domain/block/mod.rs` re-exporting entity and error types
- [ ] T008 Create `EditorSession` struct skeleton in `src-tauri/src/domain/editor/session.rs` — struct definition with fields (`page_id: PageId`, `blocks: Vec<Block>`, `is_dirty: bool`). Operation methods (`add_block`, `edit_block_content`, `move_block_up`, `move_block_down`, `remove_block`) are NOT implemented here — they will be added per user story following Red-Green
- [ ] T009 [P] Create `src-tauri/src/domain/editor/mod.rs` re-exporting `EditorSession`
- [ ] T010 Register new modules in `src-tauri/src/domain/mod.rs` — add `pub mod block; pub mod editor;`
- [ ] T011 [P] Define `BlockRepository` trait and `SqlxBlockRepository` implementation in `src-tauri/src/infrastructure/persistence/block_repository.rs` — `load_blocks(page_id)` returns `Vec<Block>` sorted by position, `save_all(page_id, blocks)` does delete-and-reinsert in transaction
- [ ] T012 [P] Add `PRAGMA foreign_keys = ON` to database initialization in `src-tauri/src/infrastructure/persistence/database.rs`
- [ ] T013 Register `block_repository` module in `src-tauri/src/infrastructure/persistence/mod.rs`
- [ ] T014 [P] Add `Block(BlockError)` variant to `CommandError` in `src-tauri/src/ipc/error.rs` with `From<BlockError>` impl, and extend Serialize impl with kind/message mappings per contracts/ipc-commands.md
- [ ] T015 [P] Add `EditorStateDto` and `BlockDto` structs with `serde(rename_all = "camelCase")` in `src-tauri/src/ipc/dto.rs`, with conversion from domain types
- [ ] T016 [P] Define TypeScript types `EditorState`, `Block`, and extend `CommandError` kinds in `src/features/editor/types.ts` per contracts/ipc-commands.md
- [ ] T017 Update `AppState` in `src-tauri/src/lib.rs` — add `sessions: tokio::sync::Mutex<HashMap<PageId, EditorSession>>` field and initialize it

**Checkpoint**: Foundation is ready — domain types, repository, errors, DTOs, and AppState all in place. User stories can proceed.

---

## Phase 3: User Story 1 — ページを開いてブロック一覧を表示する (Priority: P1) 🎯 MVP

**Goal**: ユーザーがページタイトルをクリックするとエディタ画面に切り替わり，ブロックが position 順に表示される。戻るボタンでページ一覧に復帰する。

**Independent Test**: ブロックを持つページと持たないページの両方を開き，画面切り替えと一覧への復帰が正しく動作することを確認する。

### Tests for User Story 1

- [ ] T018 [P] [US1] Add unit tests for `EditorSession::new()` — verify blocks loaded in position order, `is_dirty` starts false, empty blocks list accepted — in `src-tauri/src/domain/editor/session.rs` (`#[cfg(test)]` module)
- [ ] T019 [P] [US1] Add integration tests for `BlockRepository::load_blocks()` — in-memory SQLite, verify empty result and ordered result — in `src-tauri/src/infrastructure/persistence/block_repository.rs` (`#[cfg(test)]` module)

### Implementation for User Story 1

- [ ] T020 [US1] Implement `EditorSession::new()`, `blocks()`, `is_dirty()`, `mark_saved()`, `page_id()` in `src-tauri/src/domain/editor/session.rs` — constructor loads blocks, accessors return session state
- [ ] T021 [US1] Implement `open_editor` IPC command in `src-tauri/src/ipc/editor_commands.rs` — load blocks via `BlockRepository`, create `EditorSession`, store in `AppState.sessions`, return `EditorStateDto`
- [ ] T022 [US1] Implement `close_editor` IPC command in `src-tauri/src/ipc/editor_commands.rs` — remove session from `AppState.sessions`, idempotent (no error if missing)
- [ ] T023 [US1] Register `editor_commands` module in `src-tauri/src/ipc/mod.rs` and register `open_editor`, `close_editor` commands in `invoke_handler` in `src-tauri/src/lib.rs`
- [ ] T024 [P] [US1] Implement `useEditor` hook in `src/features/editor/useEditor.ts` — `openEditor(pageId)`, `closeEditor(pageId)` IPC wrappers, manage `EditorState` and loading state
- [ ] T025 [P] [US1] Implement `EditorToolbar` component in `src/features/editor/EditorToolbar.tsx` and `src/features/editor/EditorToolbar.module.css` — back button, page title display, save button placeholder, dirty indicator placeholder
- [ ] T026 [P] [US1] Implement `BlockItem` component in `src/features/editor/BlockItem.tsx` and `src/features/editor/BlockItem.module.css` — display block content as read-only text (editing in US3), placeholder action buttons
- [ ] T027 [US1] Implement `BlockEditor` container in `src/features/editor/BlockEditor.tsx` and `src/features/editor/BlockEditor.module.css` — use `useEditor` hook, render `EditorToolbar` + block list via `BlockItem`, show empty state message when no blocks
- [ ] T028 [US1] Add view routing in `src/App.tsx` — `currentView` state (`{ type: 'list' } | { type: 'editor', pageId: string }`), render `PageListView` or `BlockEditor` based on state
- [ ] T029 [US1] Add `onPageClick` callback prop to `src/features/pages/PageListView.tsx` and wire title click in `src/features/pages/PageItem.tsx` to trigger editor navigation
- [ ] T030 [US1] Run `cargo make qa` and verify all tests pass

**Checkpoint**: User Story 1 is fully functional — pages can be opened in editor view and navigated back to list view.

---

## Phase 4: User Story 2 — テキストブロックを追加する (Priority: P1)

**Goal**: ユーザーがエディタ画面でブロック追加操作を行うと，空のテキストブロックが末尾に追加され，未保存状態になる。

**Independent Test**: エディタ画面でブロックを追加し，ブロックが末尾に表示され編集可能になることを確認する。

### Tests for User Story 2

- [ ] T031 [P] [US2] Add unit tests for `EditorSession::add_block()` — verify block appended at end with position = len, `is_dirty` becomes true, UUIDv7 assigned, empty content — in `src-tauri/src/domain/editor/session.rs`

### Implementation for User Story 2

- [ ] T032 [US2] Implement `EditorSession::add_block()` in `src-tauri/src/domain/editor/session.rs` — append empty text block at end, generate UUIDv7, set `is_dirty = true`, return `&Block`
- [ ] T033 [US2] Implement `add_block` IPC command in `src-tauri/src/ipc/editor_commands.rs` — get session, call `add_block()`, return `EditorStateDto`
- [ ] T034 [US2] Register `add_block` command in `invoke_handler` in `src-tauri/src/lib.rs`
- [ ] T035 [US2] Add `addBlock(pageId)` to `useEditor` hook in `src/features/editor/useEditor.ts`
- [ ] T036 [US2] Add "ブロック追加" button to `BlockEditor.tsx` — call `addBlock`, auto-focus new block (last element), hide empty state when blocks exist
- [ ] T037 [US2] Run `cargo make qa` and verify all tests pass

**Checkpoint**: User Story 2 is fully functional — blocks can be added to pages.

---

## Phase 5: User Story 3 — ブロックの内容を編集する (Priority: P1)

**Goal**: ユーザーがブロックのテキスト領域に入力すると内容がリアルタイムに反映され，`onBlur` でバックエンドに同期される。

**Independent Test**: 既存ブロックの内容を変更し，画面上で反映されること，`onBlur` でバックエンドに同期されることを確認する。

### Tests for User Story 3

- [ ] T038 [P] [US3] Add unit tests for `EditorSession::edit_block_content()` — verify content updated, `is_dirty` true, `ContentTooLong` error at 10,001 chars, `NotFound` error for invalid ID, empty content accepted — in `src-tauri/src/domain/editor/session.rs`
- [ ] T039 [P] [US3] Add unit tests for `BlockContent::try_from()` with multi-byte Unicode — verify BMP 外文字（絵文字，サロゲートペア等）が `chars().count()` で正しくカウントされること（基本的な境界テスト 10,000/10,001 chars は T004 で実施済み） — in `src-tauri/src/domain/block/entity.rs`

### Implementation for User Story 3

- [ ] T040 [US3] Implement `EditorSession::edit_block_content()` in `src-tauri/src/domain/editor/session.rs` — find block by ID, update content via `BlockContent::try_from()`, set `is_dirty = true`
- [ ] T041 [US3] Implement `edit_block_content` IPC command in `src-tauri/src/ipc/editor_commands.rs` — get session, call `edit_block_content()`, return `EditorStateDto`
- [ ] T042 [US3] Register `edit_block_content` command in `invoke_handler` in `src-tauri/src/lib.rs`
- [ ] T043 [US3] Add `editBlockContent(pageId, blockId, content)` to `useEditor` hook in `src/features/editor/useEditor.ts`
- [ ] T044 [US3] Update `BlockItem` component in `src/features/editor/BlockItem.tsx` — replace read-only display with `<textarea>`, local state for input buffer, `onBlur` triggers `editBlockContent` IPC. `maxLength={10000}` は UX ヒントとして使用（バックエンドの `BlockContent` バリデーションが権威的基準。HTML `maxLength` は UTF-16 コードユニット数でカウントするため，BMP 外文字で Rust の `chars().count()` と乖離する可能性がある）。**エラー時リカバリ**: `editBlockContent` が `contentTooLong` エラーを返した場合，返却された `EditorState` のブロック内容で textarea のローカル state を上書きし，エラー toast を表示する（textarea とバックエンド状態の乖離を防止）
- [ ] T045 [US3] Run `cargo make qa` and verify all tests pass

**Checkpoint**: User Story 3 is fully functional — block content can be edited in-place.

---

## Phase 6: User Story 6 — 編集内容を明示的に保存する (Priority: P1)

**Goal**: ユーザーが保存ボタンまたは Ctrl+S で全ブロックを一括永続化する。保存後に再度開いたとき内容と順序が再現される。

**⚠️ Dependency**: Phase 5（US3: edit_block_content）に依存する — 保存前にフォーカス中ブロックの内容を `edit_block_content` で同期する必要があるため

**Independent Test**: ブロック追加・編集後に保存し，ページを閉じて再度開き，内容と順序が維持されていることを確認する。

### Tests for User Story 6

- [ ] T046 [P] [US6] Add unit tests for `EditorSession::mark_saved()` — verify `is_dirty` becomes false — in `src-tauri/src/domain/editor/session.rs`
- [ ] T047 [P] [US6] Add integration tests for `BlockRepository::save_all()` — in-memory SQLite, verify delete-and-reinsert in transaction, `created_at` preserved, `updated_at` refreshed, position normalized — in `src-tauri/src/infrastructure/persistence/block_repository.rs`

### Implementation for User Story 6

- [ ] T048 [US6] Implement `save_editor` IPC command in `src-tauri/src/ipc/editor_commands.rs` — skip save if not dirty, call `BlockRepository::save_all()`, then `mark_saved()`, return `EditorStateDto` with `isDirty: false`
- [ ] T049 [US6] Register `save_editor` command in `invoke_handler` in `src-tauri/src/lib.rs`
- [ ] T050 [US6] Add `saveEditor(pageId)` to `useEditor` hook in `src/features/editor/useEditor.ts` — sync focused block content before save, handle saving state
- [ ] T051 [US6] Wire save button in `EditorToolbar.tsx` — call `saveEditor`, show success toast (sonner), show error toast on failure (FR-014)
- [ ] T052 [US6] Add Ctrl+S keyboard shortcut in `useEditor.ts` — `useEffect` with `keydown` listener, sync focused block then save, `e.preventDefault()` to suppress browser save dialog
- [ ] T053 [US6] Run `cargo make qa` and verify all tests pass

**Checkpoint**: User Story 6 is fully functional — blocks persist across editor sessions.

---

## Phase 7: User Story 4 — ブロックを並び替える (Priority: P2)

**Goal**: ユーザーがブロックの上移動・下移動ボタンで順序を変更できる。先頭/末尾のボタンは無効化される。

**Independent Test**: 複数ブロックで上下移動を行い，順序が正しく入れ替わること，境界でボタンが無効化されることを確認する。

### Tests for User Story 4

- [ ] T054 [P] [US4] Add unit tests for `EditorSession::move_block_up()` and `move_block_down()` — verify swap, `CannotMoveUp` at top, `CannotMoveDown` at bottom, `NotFound` for invalid ID, `is_dirty` true — in `src-tauri/src/domain/editor/session.rs`

### Implementation for User Story 4

- [ ] T055 [US4] Implement `EditorSession::move_block_up()` and `move_block_down()` in `src-tauri/src/domain/editor/session.rs` — swap with adjacent block, update positions, return error at boundaries
- [ ] T056 [US4] Implement `move_block_up` and `move_block_down` IPC commands in `src-tauri/src/ipc/editor_commands.rs`
- [ ] T057 [US4] Register `move_block_up`, `move_block_down` commands in `invoke_handler` in `src-tauri/src/lib.rs`
- [ ] T058 [US4] Add `moveBlockUp(pageId, blockId)` and `moveBlockDown(pageId, blockId)` to `useEditor` hook in `src/features/editor/useEditor.ts`
- [ ] T059 [US4] Add up/down move buttons to `BlockItem` component in `src/features/editor/BlockItem.tsx` — disable up button for first block (position === 0), disable down button for last block (position === blocks.length - 1)
- [ ] T060 [US4] Run `cargo make qa` and verify all tests pass

**Checkpoint**: User Story 4 is fully functional — blocks can be reordered.

---

## Phase 8: User Story 5 — ブロックを削除する (Priority: P2)

**Goal**: ユーザーがブロックの削除ボタンをクリックするとブロックが除去され，残りの順序が維持される。

**Independent Test**: ブロックを削除し画面から消えること，残りのブロックの順序が維持されることを確認する。

### Tests for User Story 5

- [ ] T061 [P] [US5] Add unit tests for `EditorSession::remove_block()` — verify removal, position renumbered, `NotFound` for invalid ID, last block removal returns empty, `is_dirty` true — in `src-tauri/src/domain/editor/session.rs`

### Implementation for User Story 5

- [ ] T062 [US5] Implement `EditorSession::remove_block()` in `src-tauri/src/domain/editor/session.rs` — remove block by ID, renumber positions from 0, set `is_dirty = true`
- [ ] T063 [US5] Implement `remove_block` IPC command in `src-tauri/src/ipc/editor_commands.rs`
- [ ] T064 [US5] Register `remove_block` command in `invoke_handler` in `src-tauri/src/lib.rs`
- [ ] T065 [US5] Add `removeBlock(pageId, blockId)` to `useEditor` hook in `src/features/editor/useEditor.ts`
- [ ] T066 [US5] Add delete button to `BlockItem` component in `src/features/editor/BlockItem.tsx` — call `removeBlock`, show empty state in `BlockEditor.tsx` when all blocks removed
- [ ] T067 [US5] Run `cargo make qa` and verify all tests pass

**Checkpoint**: User Story 5 is fully functional — blocks can be deleted.

---

## Phase 9: User Story 7 — 未保存状態を視覚的に確認する (Priority: P2)

**Goal**: 未保存インジケータの表示・非表示，未保存状態での画面離脱時に確認ダイアログが表示される。

**Independent Test**: ブロック変更で未保存インジケータ表示，保存後に消える，未保存時の離脱で確認ダイアログが表示されることを確認する。

### Tests for User Story 7

- [ ] T068 [P] [US7] Add unit tests for `EditorSession::is_dirty()` state transitions — starts false, becomes true after any mutation, returns to false after `mark_saved()` — in `src-tauri/src/domain/editor/session.rs` (may already be covered; ensure comprehensive coverage)

### Implementation for User Story 7

- [ ] T069 [US7] Wire dirty indicator in `EditorToolbar.tsx` — show/hide based on `isDirty` from `EditorState`
- [ ] T070 [US7] Implement `UnsavedConfirmModal` component in `src/features/editor/UnsavedConfirmModal.tsx` and `src/features/editor/UnsavedConfirmModal.module.css` — "未保存の変更があります。破棄しますか？" message, cancel and discard buttons
- [ ] T071 [US7] Wire unsaved confirm dialog in `BlockEditor.tsx` — when back button clicked and `isDirty`, show `UnsavedConfirmModal`; on discard call `closeEditor` and navigate to list; on cancel stay in editor
- [ ] T072 [US7] Run `cargo make qa` and verify all tests pass

**Checkpoint**: User Story 7 is fully functional — unsaved state is visible and protected.

---

## Phase 10: Polish & Cross-Cutting Concerns

**Purpose**: Work that spans multiple user stories

- [ ] T073 Verify `PRAGMA foreign_keys = ON` works — add integration test in `src-tauri/src/infrastructure/persistence/block_repository.rs` that deletes a page and confirms its blocks are cascade-deleted
- [ ] T074 [P] Verify performance: create 1,000 blocks in-memory, save, reload — confirm both operations complete in <1s — add as benchmark or integration test in `src-tauri/src/infrastructure/persistence/block_repository.rs`
- [ ] T075 Run full QA: `cargo make qa` — formatting, lint, tests, doc-tests, TypeScript checks all pass
- [ ] T076 Verify all 7 user story acceptance scenarios can be manually exercised via `cargo make serve`. Includes: (a) 外部通信の不在確認 — ネットワークモニタリングまたは `grep -r "fetch\|XMLHttpRequest\|navigator\.sendBeacon" src/` で外部通信コードがないことを検証する（FR-010）, (b) 全操作フロー（ページを開く → ブロック追加 → 編集 → 保存 → 再度開く）が 30 秒以内に完了することを計測する（SC-001）

---

## Dependencies & Execution Order

### Phase Dependencies

```
Phase 1: Setup ─────────────────────────────────────► immediate
Phase 2: Foundational ──────────────────────────────► depends on Phase 1
Phase 3: US1 (open/close editor) ───────────────────► depends on Phase 2
Phase 4: US2 (add block) ──────────────────────────► depends on Phase 3
Phase 5: US3 (edit content) ────────────────────────► depends on Phase 3
Phase 6: US6 (save) ───────────────────────────────► depends on Phase 3 + Phase 5
Phase 7: US4 (reorder) ────────────────────────────► depends on Phase 3
Phase 8: US5 (delete) ─────────────────────────────► depends on Phase 3
Phase 9: US7 (unsaved indicator + confirm) ─────────► depends on Phase 6
Phase 10: Polish ───────────────────────────────────► depends on all above
```

### Parallel Opportunities

- **Phase 2**: T004, T005, T006, T007, T009 in parallel (different files). T011, T012, T014, T015, T016 in parallel (different files).
- **Phase 3**: T024, T025, T026 frontend tasks in parallel after IPC commands registered.
- **Phase 5 + Phase 6**: US3 と US6 は同時実施を推奨する。Phase 6（保存）は Phase 5（edit_block_content）に依存するため，US3 の IPC コマンド登録後に US6 の実装を開始する。
- **Phases 4, 7, 8**: US2, US4, US5 can run in parallel after US1 completes (independent user stories touching same files — serialize if conflict).

---

## Implementation Strategy

### MVP First

1. Complete Phase 1 (Setup)
2. Complete Phase 2 (Foundational)
3. Complete Phase 3 (US1: open/close editor)
4. Complete Phase 5 (US3: edit content) + Phase 6 (US6: save) を同時に実施 — Phase 6 は edit_block_content（Phase 5）に依存するため，Phase 5 の IPC 登録後に Phase 6 の実装に進む
5. Complete Phase 4 (US2: add block) — Phase 5 と並行可能
6. Validate end-to-end: add → edit → save → reopen flow

### Incremental Delivery

1. Ship US1 first — editor can open and display blocks
2. Add US2 + US3 + US6 together — full create/edit/save cycle (functional MVP). **US3 と US6 は同時実施が必須**（保存前のフォーカス同期に edit_block_content が必要）
3. Add US4 + US5 — reorder and delete (enriched editing)
4. Add US7 — unsaved protection (polish)
5. Re-run `cargo make qa` after each story

---

## Notes

- All Rust code must avoid `unwrap()`, `expect()`, `panic!()`, `unsafe` per constitution
- Every IPC command returns `Result<..., CommandError>` with proper error serialization
- `EditorSession` is pure domain logic — no DB dependency, fully testable without infrastructure
- **Red-Green-Refactor**: EditorSession の各操作メソッドは，対応するユーザーストーリーのテストタスクの後に実装する（Phase 2 ではスケルトンのみ）
- Frontend is a thin UI layer — no business logic, state comes from backend `EditorState`
- `onBlur` sync strategy: textarea local state → `edit_block_content` IPC on blur → before save, sync focused block explicitly
- `maxLength` は UX ヒントであり，バックエンドの `BlockContent` バリデーション（`chars().count()` ベース）が権威的基準
