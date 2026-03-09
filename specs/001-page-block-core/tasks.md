---

description: "Task list for Page Block Core implementation"
---

# Tasks: Page Block Core

**Input**: Design documents from `/specs/001-page-block-core/`  
**Prerequisites**: [plan.md](/home/hikaru/Develop/RustyDataBaseNotes/specs/001-page-block-core/plan.md)，[spec.md](/home/hikaru/Develop/RustyDataBaseNotes/specs/001-page-block-core/spec.md)，[research.md](/home/hikaru/Develop/RustyDataBaseNotes/specs/001-page-block-core/research.md)，[data-model.md](/home/hikaru/Develop/RustyDataBaseNotes/specs/001-page-block-core/data-model.md)，[contracts/page-block-core-ipc.yaml](/home/hikaru/Develop/RustyDataBaseNotes/specs/001-page-block-core/contracts/page-block-core-ipc.yaml)，[quickstart.md](/home/hikaru/Develop/RustyDataBaseNotes/specs/001-page-block-core/quickstart.md)

**Tests**: Tests are MANDATORY。各 user story は，失敗するテストまたは実行可能検証を先に追加してから実装へ進む。  
**Organization**: Tasks are grouped by user story so each story can be implemented，tested，and reviewed independently。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 異なるファイルを触り，未完了タスクへの依存が無い場合に並列実行できる。
- **[Story]**: `US1`，`US2`，`US3` のように user story を示す。
- すべてのタスク記述には exact file path を含める。

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: feature を実装できる最小のプロジェクト骨格を作る。

- [ ] T001 Create frontend，backend，and test skeleton files in `src/app/App.tsx`，`src/features/page-block-core/PageEditor.tsx`，`src/features/page-block-core/usePageEditor.ts`，`src/lib/page-block-core/client.ts`，`src-tauri/src/main.rs`，and `tests/e2e/page-block-core.spec.ts`
- [ ] T002 Create workspace manifests and runtime config in `package.json`，`pnpm-workspace.yaml`，`tsconfig.json`，`src-tauri/Cargo.toml`，and `src-tauri/tauri.conf.json`
- [ ] T003 [P] Configure frontend and backend quality commands in `vitest.config.ts`，`playwright.config.ts`，`.eslintrc.cjs`，and `src-tauri/rustfmt.toml`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: すべての user story を支える共通基盤を先に固める。

**⚠️ CRITICAL**: この phase 完了前に user story 実装へ進まない。

- [ ] T004 Create the initial SQLite schema migration in `src-tauri/migrations/0001_page_block_core.sql`
- [ ] T005 [P] Define typed page and block DTOs plus invoke wrappers in `src/lib/page-block-core/contracts.ts` and `src-tauri/src/ipc/page_block.rs`
- [ ] T006 [P] Implement app-local-data path resolution and backup file helpers in `src-tauri/src/infrastructure/sqlite/paths.rs` and `src-tauri/src/infrastructure/sqlite/recovery.rs`
- [ ] T007 [P] Implement page aggregate validation and domain errors in `src-tauri/src/domain/page_block/mod.rs` and `src-tauri/src/domain/page_block/errors.rs`
- [ ] T008 [P] Lock down offline-only desktop capabilities in `src-tauri/capabilities/default.json` and `src-tauri/tauri.conf.json`
- [ ] T009 Add module-level bounded-context and `///` API documentation in `src-tauri/src/application/page_block/mod.rs` and `src-tauri/src/domain/page_block/mod.rs`

**Checkpoint**: SQLite schema，typed IPC，domain invariants，and offline boundaries are ready。

---

## Phase 3: User Story 1 - 初回ページで書き始める (Priority: P1) 🎯 MVP

**Goal**: 初回起動で空 page を自動生成し，title と plain text block を追加，編集，自動保存できるようにする。

**Independent Test**: 新規環境で起動し，空 page が表示され，そのまま 3 個以上の block を追加し，title または本文を編集して 500ms 以内に保存開始されることを確認する。

### Tests for User Story 1

- [ ] T010 [P] [US1] Add a failing frontend boot，add-block，and edit-autosave test in `src/features/page-block-core/PageEditor.test.tsx`
- [ ] T011 [P] [US1] Add a failing Rust integration test for page creation and first snapshot persistence in `src-tauri/tests/page_block_core_us1.rs`

### Implementation for User Story 1

- [ ] T012 [P] [US1] Implement editor session state and 500ms autosave scheduling in `src/features/page-block-core/usePageEditor.ts`
- [ ] T013 [P] [US1] Implement empty-page UI，title input，block list，and add-block action in `src/features/page-block-core/PageEditor.tsx`
- [ ] T014 [P] [US1] Implement Tauri invoke client for `load_page_core` and `persist_page_snapshot` in `src/lib/page-block-core/client.ts`
- [ ] T015 [US1] Implement initial load，auto-create-empty-page，and persist use cases in `src-tauri/src/application/page_block/service.rs`
- [ ] T016 [US1] Implement SQLite page snapshot repository for page create，block append，and text edits in `src-tauri/src/infrastructure/sqlite/page_store.rs`
- [ ] T017 [US1] Wire the feature into the desktop shell in `src/app/App.tsx` and `src-tauri/src/main.rs`
- [ ] T018 [US1] Add `///` docs and user-facing save status copy for empty-title fallback and autosave states in `src-tauri/src/ipc/page_block.rs` and `src/features/page-block-core/saveStatus.ts`
- [ ] T019 [US1] Record US1 QA steps and outcomes in `specs/001-page-block-core/quickstart.md`

**Parallel Example**: `T012` と `T014` は並列実行できる。その後に `T013` と `T015` を進め，最後に `T016`，`T017`，`T018` を統合する。

**Checkpoint**: User Story 1 は単独で動作し，初回起動から block 追加と autosave まで検証できる。

---

## Phase 4: User Story 2 - ブロック順序を整える (Priority: P2)

**Goal**: 同一 page 内で block を並び替え，表示順と保存順を一致させる。

**Independent Test**: 5 個以上の block を持つ page で 1 件を別位置へ移動し，再起動後も同じ順序が復元されることを確認する。

### Tests for User Story 2

- [ ] T020 [P] [US2] Add a failing frontend reorder interaction test in `src/features/page-block-core/PageEditor.reorder.test.tsx`
- [ ] T021 [P] [US2] Add a failing Rust integration test for reorder persistence and append-after-reorder behavior in `src-tauri/tests/page_block_core_us2.rs`

### Implementation for User Story 2

- [ ] T022 [P] [US2] Implement reorder helpers and optimistic ordering updates in `src/features/page-block-core/reorderBlocks.ts` and `src/features/page-block-core/PageEditor.tsx`
- [ ] T023 [P] [US2] Extend page aggregate reorder validation and application logic in `src-tauri/src/domain/page_block/mod.rs` and `src-tauri/src/application/page_block/service.rs`
- [ ] T024 [US2] Persist contiguous block positions atomically during reorder in `src-tauri/src/infrastructure/sqlite/page_store.rs`
- [ ] T025 [US2] Handle the `block_reordered` trigger in `src/lib/page-block-core/contracts.ts` and `src-tauri/src/ipc/page_block.rs`
- [ ] T026 [US2] Record US2 reorder QA steps and outcomes in `specs/001-page-block-core/quickstart.md`

**Parallel Example**: `T020` と `T021` を並列で先行し，`T022` と `T023` を契約固定後に並列実行できる。`T024` と `T025` はその後に統合する。

**Checkpoint**: User Story 2 は US1 の page 上で独立に検証でき，順序重複や欠落が発生しない。

---

## Phase 5: User Story 3 - 再起動後も内容を維持する (Priority: P3)

**Goal**: 再起動復元，保存失敗時の未保存保持，再試行，破損データ回復を実装する。

**Independent Test**: page 作成，block 追加，並び替え後に再起動して内容と順序が復元され，保存失敗時は未保存表示を維持しつつ，再起動後は最後の整合済み状態だけが戻ることを確認する。

### Tests for User Story 3

- [ ] T027 [P] [US3] Add a failing frontend recovery and unsaved-state test in `src/features/page-block-core/PageEditor.recovery.test.tsx`
- [ ] T028 [P] [US3] Add a failing Rust integration test for restart restore，save failure retry，and corruption fallback in `src-tauri/tests/page_block_core_us3.rs`

### Implementation for User Story 3

- [ ] T029 [P] [US3] Implement recovery notice，dirty banner，and retry-save UI state in `src/features/page-block-core/recoveryNotice.ts` and `src/features/page-block-core/PageEditor.tsx`
- [ ] T030 [P] [US3] Extend editor orchestration for restart restore，failed-save retention，and retry scheduling in `src/features/page-block-core/usePageEditor.ts`
- [ ] T031 [P] [US3] Implement backup copy，corruption isolation，and last-consistent-state recovery in `src-tauri/src/infrastructure/sqlite/recovery.rs` and `src-tauri/src/infrastructure/sqlite/page_store.rs`
- [ ] T032 [US3] Implement startup fallback，revision conflict handling，and retryable persistence errors in `src-tauri/src/application/page_block/service.rs` and `src-tauri/src/ipc/page_block.rs`
- [ ] T033 [US3] Add `///` docs and user-facing recovery error copy in `src-tauri/src/ipc/page_block.rs` and `src/features/page-block-core/saveStatus.ts`
- [ ] T034 [US3] Record US3 restart，failure，retry，and corruption QA outcomes in `specs/001-page-block-core/quickstart.md`

**Parallel Example**: `T027` と `T028` を並列で起こし，`T029` と `T031` を別レイヤーで並列実装できる。`T030` と `T032` は統合点として後続に置く。

**Checkpoint**: User Story 3 は整合済み復元と障害回復を単独で示せる。

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: story 横断の品質調整と最終検証を行う。

- [ ] T035 [P] Update developer run instructions and local-first guarantees in `README.md`
- [ ] T036 Simplify temporary scaffolding and tighten exports in `src/features/page-block-core/index.ts` and `src-tauri/src/application/page_block/mod.rs`
- [ ] T037 Verify the 200-block performance target and document the measurements in `specs/001-page-block-core/quickstart.md`
- [ ] T038 [P] Add cross-story migration，backup，and end-to-end regression coverage in `tests/e2e/page-block-core.spec.ts` and `src-tauri/tests/page_block_core_migration.rs`
- [ ] T039 Run full QA and record the command results in `specs/001-page-block-core/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- Setup: すぐ開始できる。
- Foundational: Setup に依存し，すべての user story をブロックする。
- User Story 1: Foundational 完了後に開始する。MVP の最小出荷単位。
- User Story 2: Foundational と US1 の page/block 作成フローに依存する。
- User Story 3: Foundational と US1 の保存フローに依存し，reorder 復元を含める場合は US2 完了後に進める。
- Polish: 対象 user story 完了後に実施する。

### Dependency Graph

`Setup -> Foundational -> US1 -> US2 -> US3 -> Polish`

### Within Each User Story

- テストタスクを先に書き，失敗を確認してから実装へ進む。
- frontend と Rust backend は，契約が固定した後は `[P]` タスクを並列に進められる。
- 永続化と recovery の実装が完了するまで，story を完了扱いにしない。
- `quickstart.md` への QA 記録を各 story の完了条件に含める。

### Parallel Opportunities

- Setup では `T003` を `T001`，`T002` と並列で進められる。
- Foundational では `T005`，`T006`，`T007`，`T008` が並列候補。
- US1 では `T010`，`T011`，`T012`，`T014` が契約固定後に並列候補。
- US2 では `T020` と `T021`，`T022` と `T023` が並列候補。
- US3 では `T027` と `T028`，`T029` と `T031` が並列候補。
- Polish では `T035` と `T038` が並列候補。

---

## Implementation Strategy

### MVP First

1. Phase 1 Setup を完了する。
2. Phase 2 Foundational を完了する。
3. Phase 3 US1 を完了し，独立テストを通す。
4. US1 が green のまま維持できることを確認してから US2 以降へ進む。

### Incremental Delivery

1. 単一 page の起動，追加，編集，自動保存を最初に出荷する。
2. 次に reorder を追加し，位置不変条件を固定する。
3. 最後に recovery と restart persistence を加え，ローカル完結性を完成させる。

---

## Notes

- すべてのタスクは checklist 形式，Task ID，必要な `[P]`，必要な `[USn]`，exact file path を含む。
- テストを optional として扱わない。
- `unsafe`，`unwrap()`，`expect()`，`panic!()` を導入するタスクは作らない。
- backup，migration，recovery を polish へ先送りせず，story 完了条件に含める。
