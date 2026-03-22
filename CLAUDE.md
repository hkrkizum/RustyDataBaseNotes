# CLAUDE.md

プロジェクトの原則，技術標準，ワークフロー，ガバナンスは
[`.specify/memory/constitution.md`](.specify/memory/constitution.md) に定義されている。
作業開始前に必ず参照すること。

## Task Runner (cargo-make)

タスク定義は `Makefile.toml`。すべて `cargo make <task>` で実行する。

### 頻用タスク

| タスク | 内容 |
|--------|------|
| `serve` | Tauri 開発サーバー（HMR） |
| `fmt` | Rust + TypeScript を一括フォーマット |
| `check-all` | テスト抜き静的チェック（fmt → clippy → lint-ts → ts-check） |
| `qa` | フル QA（`sqlx-prepare` → Rust QA → TypeScript QA を順次実行） |
| `test-rs` | Rust テスト（`TEST_FILTER=<pattern>` で絞り込み可） |
| `test-ts` | TypeScript テスト（Vitest） |
| `sqlx-prepare` | dev.db リセット＋ `.sqlx/` キャッシュ再生成 |
| `dev-db-reset` | 開発用 SQLite DB を削除（再起動で再作成） |

サブタスク・カバレッジ等の全タスクは `Makefile.toml` を参照。

### 環境変数

| 変数 | デフォルト | 用途 |
|------|-----------|------|
| `RUST_WORKSPACE` | `src-tauri` | Rust ワークスペースパス |
| `TEST_BUILD_JOBS` | `8` | テストビルド並列度（OOM 防止） |
| `QA_CHECK_ONLY` | 未設定 | `1` で fmt を check-only に切替 |
| `TEST_FILTER` | 未設定 | `test-rs` のテスト名フィルタ |

### Git Hooks

`.githooks/` にフックを管理。`pre-commit` で `check-all`，`pre-merge-commit`（main マージ時）で `QA_CHECK_ONLY=1 qa` を実行。

## Recent Changes
- 005-page-tree-nav: Added Rust 2024 (edition = "2024", toolchain 1.94.0), TypeScript ~5.8.3
- 004-table-view-operations: Added Rust 2024 (edition = "2024", toolchain 1.94.0), TypeScript ~5.8.3 + Tauri 2, React 19, sqlx 0.8 (SQLite), uuid 1 (v7), chrono 0.4, thiserror 2, serde 1, serde_json 1, Sonner (toast), Biome (lint/format)
- 003-database-properties: Added Rust 2024 (edition = "2024")，TypeScript ~5.8.3 + Tauri 2，React 19，sqlx 0.8 (SQLite)，uuid 1 (v7)，chrono 0.4，thiserror 2，serde 1，Sonner (toast)，Biome (lint/format)

## Active Technologies
- Rust 2024 (edition = "2024", toolchain 1.94.0), TypeScript ~5.8.3 + Tauri 2, React 19, sqlx 0.8 (SQLite), uuid 1 (v7), chrono 0.4, thiserror 2, serde 1, serde_json 1, Sonner (toast), Biome (lint/format) (005-page-tree-nav)
- 新規追加予定: Tailwind CSS v4 + shadcn/ui, @atlaskit/pragmatic-drag-and-drop, lucide-react (005-page-tree-nav)
- SQLite (WAL mode), `{appDataDir}/rustydatabasenotes.db`, sqlx::migrate!() によるコンパイル時マイグレーション埋め込み。新規マイグレーション 0007 で `pages` テーブルに `parent_id TEXT` (自己参照 FK) と `sort_order INTEGER DEFAULT 0` を追加 (005-page-tree-nav)
