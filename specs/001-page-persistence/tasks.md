# Tasks: ページの永続化（最小縦断スライス）

**Input**: Design documents from `/specs/001-page-persistence/`
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

- **Frontend**: `src/` (React + TypeScript, Vite)
- **Backend**: `src-tauri/src/` (Rust, Tauri 2)
- **Migrations**: `src-tauri/migrations/`
- **Structure**: per plan.md Project Structure section

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish directory structure and dependencies required by all subsequent phases

- [ ] T001 Create backend module directories (`src-tauri/src/domain/page/`, `src-tauri/src/infrastructure/persistence/`, `src-tauri/src/ipc/`) and frontend directories (`src/features/pages/`, `src/components/toast/`) per plan.md Project Structure
- [ ] T002 Add Rust dependencies (sqlx 0.8 with runtime-tokio/sqlite/macros/migrate/chrono/uuid features, uuid 1 with v7/serde features, thiserror 2, chrono 0.4 with serde feature) to `src-tauri/Cargo.toml`
- [ ] T003 [P] Install frontend dependency `sonner` via `pnpm add sonner`
- [ ] T004 [P] Create `src-tauri/.env` with `DATABASE_URL=sqlite:dev.db` for sqlx compile-time query verification, and add `cargo:rerun-if-changed=migrations` to `src-tauri/build.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core domain model, error types, storage schema, database initialization, and typed boundaries that MUST exist before any user story work begins

**⚠️ CRITICAL**: No user story work begins until this phase is complete

- [ ] T005 Create migration file `src-tauri/migrations/0001_create_pages.sql` with `pages` table (id TEXT PK, title TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL) and `CREATE INDEX idx_pages_created_at ON pages (created_at DESC)`
- [ ] T006 [P] Implement `PageError` enum (TitleEmpty, TitleTooLong { len, max }, NotFound { id }) with `thiserror` derive and `Display` messages in `src-tauri/src/domain/page/error.rs`
- [ ] T007 [P] Implement `StorageError` enum (Sqlx(sqlx::Error), Migration(sqlx::migrate::MigrateError), DatabasePath(std::io::Error)) with `thiserror` derive in `src-tauri/src/infrastructure/persistence/error.rs`
- [ ] T008 Implement `PageId` (UUIDv7 newtype with `Uuid::now_v7()`, Display, FromStr, Serialize, Deserialize, serde transparent), `PageTitle` (String newtype with `TryFrom<String>` validating non-empty after trim and ≤255 chars, Display, Serialize, Deserialize), and `Page` struct (id, title, created_at, updated_at, constructor) in `src-tauri/src/domain/page/entity.rs`
- [ ] T009 [P] Define `PageRepository` async trait with method signatures for `create(&self, page: &Page)`, `find_by_id(&self, id: &PageId)`, `find_all(&self)`, `update_title(&self, id: &PageId, title: &PageTitle)`, `delete(&self, id: &PageId)` in `src-tauri/src/domain/page/repository.rs`
- [ ] T010 [P] Implement `CommandError` enum (Page(PageError), Storage(StorageError)) with `#[from]` conversions and custom `Serialize` impl producing `{ "kind": "<errorKind>", "message": "<msg>" }` JSON in `src-tauri/src/ipc/error.rs`
- [ ] T011 [P] Implement `PageDto` struct with `#[serde(rename_all = "camelCase")]` (id: String, title: String, created_at: String, updated_at: String) and `From<Page>` conversion in `src-tauri/src/ipc/dto.rs`
- [ ] T012 Implement `database::init_pool` function (create `SqlitePool` from path, set WAL mode via PRAGMA, run `sqlx::migrate!()`, return pool) in `src-tauri/src/infrastructure/persistence/database.rs`
- [ ] T013 Create all module declaration files: `src-tauri/src/domain/mod.rs`, `src-tauri/src/domain/page/mod.rs`, `src-tauri/src/infrastructure/mod.rs`, `src-tauri/src/infrastructure/persistence/mod.rs`, `src-tauri/src/ipc/mod.rs` with appropriate `pub mod` and `pub use` statements
- [ ] T014 Update `src-tauri/src/lib.rs`: add module declarations (domain, infrastructure, ipc), define `AppState { db: SqlitePool }`, initialize DB pool via `database::init_pool` in `app.setup()` hook using `tauri::async_runtime::block_on`, register `AppState` via `app.manage()`, replace existing `.expect()` calls with `Result`-based error propagation
- [ ] T015 [P] Define TypeScript types (`Page`, `CommandError`, `CreatePageArgs`, `UpdatePageTitleArgs`, `DeletePageArgs`, `GetPageArgs`) in `src/features/pages/types.ts` per contracts/ipc-commands.md
- [ ] T016 [P] Create `Toaster` wrapper component configuring sonner `<Toaster />` with app-appropriate defaults in `src/components/toast/Toaster.tsx`

**Checkpoint**: Foundation compiles (`cargo check`), migration runs, DB pool initializes, all types defined on both sides

---

## Phase 3: User Story 1 — ページを新規作成する (Priority: P1) 🎯 MVP

**Goal**: ユーザーがタイトルを指定してページを作成し、永続化できる

**Independent Test**: タイトルを指定してページを作成し、find_by_id で取得して全フィールドが正しいことを検証する

### Tests for User Story 1

- [ ] T017 [P] [US1] Write unit tests for `PageTitle` validation (empty string → TitleEmpty, 256-char string → TitleTooLong, valid string, 255-char boundary, whitespace-only → TitleEmpty) and `Page::new` construction (generates valid PageId, sets timestamps) as `#[cfg(test)] mod tests` in `src-tauri/src/domain/page/entity.rs`
- [ ] T018 [US1] Write integration test for `SqlxPageRepository::create` + `::find_by_id` using in-memory SQLite pool (`:memory:` with migration applied): verify created page round-trips correctly, verify duplicate id handling in `src-tauri/src/infrastructure/persistence/page_repository.rs` `#[cfg(test)] mod tests`

### Implementation for User Story 1

- [ ] T019 [US1] Implement `SqlxPageRepository` struct (holds `SqlitePool`) with `::new()`, `impl PageRepository for SqlxPageRepository` として `create` method (INSERT via `sqlx::query!`) and `find_by_id` method (SELECT by id via `sqlx::query_as!`, return `PageError::NotFound` if absent) を実装 in `src-tauri/src/infrastructure/persistence/page_repository.rs`
- [ ] T020 [US1] Implement `create_page` and `get_page` `#[tauri::command]` async handlers (extract `AppState` from `State`, construct domain types, call repository, convert to `PageDto`) in `src-tauri/src/ipc/page_commands.rs`, and register both commands in `invoke_handler` in `src-tauri/src/lib.rs`
- [ ] T021 [P] [US1] Implement `CreatePageForm` component (controlled title input, submit button, validation feedback, loading state, toast on success/error via sonner) in `src/features/pages/CreatePageForm.tsx` and `src/features/pages/CreatePageForm.module.css`
- [ ] T022 [US1] Implement initial `usePages` custom hook with `createPage` function (invoke `create_page` IPC, update local state, error handling) in `src/features/pages/usePages.ts`
- [ ] T023 [US1] Add `<Toaster />` to `src/App.tsx`, integrate `CreatePageForm`, verify create page flow compiles and builds (`pnpm build`)
- [ ] T024 [US1] Run `cargo make qa` to verify US1 backend tests pass and code quality gates (fmt, clippy, nextest, doc) are green

**Checkpoint**: User Story 1 is fully functional — page creation persists and can be retrieved by ID

---

## Phase 4: User Story 2 — ページ一覧を閲覧する (Priority: P1) 🎯 MVP

**Goal**: ユーザーがアプリを開いたとき、保存済みのすべてのページをタイトル・作成日時付きで一覧確認できる

**Independent Test**: 複数ページ作成後に `find_all` で全件取得し、created_at DESC 順であることを検証する

### Tests for User Story 2

- [ ] T025 [US2] Write integration test for `SqlxPageRepository::find_all` (empty list returns `Vec::new()`, multiple pages returned in created_at DESC order, timestamps formatted correctly) in `src-tauri/src/infrastructure/persistence/page_repository.rs` `#[cfg(test)] mod tests`

### Implementation for User Story 2

- [ ] T026 [US2] Implement `find_all` method (SELECT * FROM pages ORDER BY created_at DESC via `sqlx::query_as!`) on `SqlxPageRepository` in `src-tauri/src/infrastructure/persistence/page_repository.rs`
- [ ] T027 [US2] Implement `list_pages` `#[tauri::command]` async handler in `src-tauri/src/ipc/page_commands.rs`, register in `invoke_handler` in `src-tauri/src/lib.rs`
- [ ] T028 [P] [US2] Implement `PageListView` component (displays page list, empty state with guidance message prompting page creation) in `src/features/pages/PageListView.tsx` and `src/features/pages/PageListView.module.css`
- [ ] T029 [P] [US2] Implement `PageItem` component (read-only row displaying title and formatted created_at) in `src/features/pages/PageItem.tsx` and `src/features/pages/PageItem.module.css`
- [ ] T030 [US2] Add `listPages` and `refreshPages` functions to `usePages` hook, load pages on mount via `useEffect`, integrate `PageListView` as main view in `src/App.tsx`
- [ ] T031 [US2] Run `cargo make qa` to verify US1 + US2 tests pass and all quality gates are green

**Checkpoint**: User Stories 1 and 2 both work independently — pages can be created and listed with persistence across restarts

---

## Phase 5: User Story 3 — ページのタイトルを更新する (Priority: P2)

**Goal**: ユーザーが一覧画面上でページタイトルをインライン編集し、変更が永続化される

**Independent Test**: 既存ページのタイトルを変更し、find_by_id で updated_at が更新され新タイトルが返ることを検証する

### Tests for User Story 3

- [ ] T032 [US3] Write integration test for `SqlxPageRepository::update_title` (success with updated_at change, NotFound for non-existent id, TitleEmpty for empty title, TitleTooLong for 256+ chars) in `src-tauri/src/infrastructure/persistence/page_repository.rs` `#[cfg(test)] mod tests`

### Implementation for User Story 3

- [ ] T033 [US3] Implement `update_title` method (UPDATE title and updated_at WHERE id, return `PageError::NotFound` if 0 rows affected, re-fetch and return updated `Page`) on `SqlxPageRepository` in `src-tauri/src/infrastructure/persistence/page_repository.rs`
- [ ] T034 [US3] Implement `update_page_title` `#[tauri::command]` async handler in `src-tauri/src/ipc/page_commands.rs`, register in `invoke_handler` in `src-tauri/src/lib.rs`
- [ ] T035 [US3] Add inline editing mode to `PageItem` (click title → text input, Enter/blur to confirm, Escape to cancel, validation error toast) in `src/features/pages/PageItem.tsx` and `src/features/pages/PageItem.module.css`
- [ ] T036 [US3] Add `updatePageTitle` function to `usePages` hook (invoke `update_page_title` IPC, update local state, toast on error) in `src/features/pages/usePages.ts`
- [ ] T037 [US3] Run `cargo make qa` to verify US1 + US2 + US3 tests pass and all quality gates are green

**Checkpoint**: User Story 3 is fully functional — titles can be edited inline with persistence and validation

---

## Phase 6: User Story 4 — ページを削除する (Priority: P2)

**Goal**: ユーザーが不要なページを確認ダイアログ経由で削除し、永続化層からも除去される

**Independent Test**: ページ削除後に find_by_id で NotFound が返り、find_all にも含まれないことを検証する

### Tests for User Story 4

- [ ] T038 [US4] Write integration test for `SqlxPageRepository::delete` (success removes page, NotFound for non-existent id, deleted page absent from find_all) in `src-tauri/src/infrastructure/persistence/page_repository.rs` `#[cfg(test)] mod tests`

### Implementation for User Story 4

- [ ] T039 [US4] Implement `delete` method (DELETE WHERE id, return `PageError::NotFound` if 0 rows affected) on `SqlxPageRepository` in `src-tauri/src/infrastructure/persistence/page_repository.rs`
- [ ] T040 [US4] Implement `delete_page` `#[tauri::command]` async handler in `src-tauri/src/ipc/page_commands.rs`, register in `invoke_handler` in `src-tauri/src/lib.rs`
- [ ] T041 [P] [US4] Implement `DeleteConfirmModal` component (modal dialog with confirm/cancel buttons, page title display) in `src/features/pages/DeleteConfirmModal.tsx` and `src/features/pages/DeleteConfirmModal.module.css`
- [ ] T042 [US4] Add delete button to `PageItem`, wire `DeleteConfirmModal` (open on click, confirm triggers delete), add `deletePage` function to `usePages` hook in `src/features/pages/PageItem.tsx` and `src/features/pages/usePages.ts`
- [ ] T043 [US4] Run `cargo make qa` to verify all US tests pass and quality gates are green

**Checkpoint**: All CRUD operations are functional — create, list, update, delete with full persistence

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Performance validation, offline build support, edge case verification, final QA

- [ ] T044 [P] Run `cargo sqlx prepare` in `src-tauri/` to generate `.sqlx/` offline query metadata directory for CI builds
- [ ] T045 [P] Verify performance targets: create a test inserting 1,000 pages and confirm `find_all` completes in <1s, `create` in <1s in `src-tauri/src/infrastructure/persistence/page_repository.rs` `#[cfg(test)]`
- [ ] T046 Verify edge cases in `src-tauri/src/infrastructure/persistence/page_repository.rs` `#[cfg(test)]`:
  - (a) タイトル境界値: 255 文字（有効），256 文字（`TitleTooLong` で拒否），空白のみ（`TitleEmpty` で拒否）
  - (b) 同時書き込み: 2 つの `tokio::spawn` タスクから同時に `create` を実行し，両方が成功し `find_all` で 2 件取得できることを検証
  - (c) マイグレーション失敗復旧: 不正な SQL を含むマイグレーションファイルを追加した状態で `init_pool` を呼び出し，エラーが返ること，および不正マイグレーションを除去後に再度 `init_pool` が成功することを検証
  - (d) WAL モード整合性: ページ作成直後にプール接続を強制ドロップし，新しいプールで再接続して作成済みページが取得できることを検証
- [ ] T047 Run full QA suite: `cargo make qa` (fmt + clippy + nextest + doc) and `pnpm build`, confirm zero warnings and all tests green
- [ ] T048 DB ファイル検証・起動時エラー報告: `database::init_pool` に DB ファイルパスの事前検証を追加。DB ファイルが破損（SQLite ヘッダ不正）または親ディレクトリが存在しない場合に，`StorageError` を返しフロントエンドに構造化エラーとして伝達する。`src-tauri/src/infrastructure/persistence/database.rs` に検証ロジックを実装し，対応するテストを `#[cfg(test)]` に追加

---

## Dependencies & Execution Order

### Phase Dependencies

```
Phase 1: Setup ──► Phase 2: Foundational ──► Phase 3: US1 (P1) ──► Phase 4: US2 (P1) ──┬──► Phase 5: US3 (P2) ──┬──► Phase 7: Polish
                                                                                         │                        │
                                                                                         └──► Phase 6: US4 (P2) ──┘
```

- **Setup** (Phase 1): Starts immediately
- **Foundational** (Phase 2): Depends on Setup, blocks ALL user stories
- **US1** (Phase 3): Depends on Foundational — **must complete before US2** (US2 displays what US1 creates)
- **US2** (Phase 4): Depends on US1 — **must complete before US3/US4**（US3 の T035 と US4 の T042 は US2 の T029 で作成される `PageItem.tsx` を編集するため）
- **US3** (Phase 5): Depends on US2 — update path の実装
- **US4** (Phase 6): Depends on US2 — delete path の実装。**US3 と並行実行可能**（異なる CRUD 操作・異なる UI コンポーネントを扱う）
- **Polish** (Phase 7): Starts after ALL user stories are complete

### Within Each User Story

1. Tests MUST be written first and MUST fail before implementation
2. Backend repository → IPC command → Frontend component (sequential)
3. Frontend components marked [P] can be built in parallel with backend when contracts are stable
4. QA task is the final gate for each story

### Parallel Opportunities

| Tasks | Can Parallelize? | Reason |
|---|---|---|
| T006 + T007 | ✅ Yes | Different error files, no shared dependency |
| T009 + T010 + T011 | ✅ Yes | Different files (repository trait, IPC error, DTO) |
| T015 + T016 | ✅ Yes | Independent frontend files |
| T017 + T018 | ✅ Yes (T017 only) | Unit tests (T017) independent of integration tests (T018 depends on repository) |
| T021 + backend work | ✅ Yes | Frontend component can be built with types from Phase 2 |
| T028 + T029 | ✅ Yes | Different frontend components |
| US3 + US4 | ✅ Yes (after US2) | Touch different CRUD operations and different UI components, but both depend on PageItem.tsx (T029) |
| T041 + backend work | ✅ Yes | Modal component independent of delete implementation |

---

## Implementation Strategy

### MVP First

1. Complete Setup (Phase 1)
2. Complete Foundational (Phase 2)
3. Complete User Story 1 — ページを新規作成する (Phase 3)
4. Complete User Story 2 — ページ一覧を閲覧する (Phase 4)
5. **Validate**: App can create and list pages with persistence — this is the minimum useful product

### Incremental Delivery

1. Ship US1 + US2 (P1 stories) as the first functional increment
2. Run `cargo make qa` after each story — proceed only when green
3. Add US3 (title update) — expand CRUD capability
4. Add US4 (delete) — complete CRUD
5. Polish phase validates performance, edge cases, and offline build support

### Suggested MVP Scope

- **Minimum**: Phase 1 + Phase 2 + Phase 3 (US1) + Phase 4 (US2)
- **Full scope**: All phases including US3, US4, and Polish

---

## Notes

- All Rust code must avoid `unwrap()`, `expect()`, `panic!()`, `unsafe` per Clippy lints in Cargo.toml
- All errors propagate via `Result` — use `?` operator throughout
- Public Rust API requires `///` doc comments (English)
- IPC command names match Rust function names (Tauri default): `create_page`, `list_pages`, `get_page`, `update_page_title`, `delete_page`
- SQLite TEXT dates use ISO 8601 format — chrono handles parsing/formatting
- `SqlitePool` is Send + Sync + Clone — no Mutex needed for Tauri State
- Frontend uses CSS Modules (`.module.css`) — Vite supports this without configuration
