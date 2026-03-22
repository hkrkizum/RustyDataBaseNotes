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
| `check` | `cargo check --workspace`（高速コンパイルチェック） |
| `fmt` | Rust + TypeScript を一括フォーマット |
| `check-all` | テスト抜き静的チェック（fmt-check → clippy → lint-ts → ts-check） |
| `qa` | フル QA（`sqlx-prepare` → Rust QA → TypeScript QA を順次実行） |
| `test` | Rust テスト（cargo-nextest） |
| `test-filter` | `TEST_FILTER=<pattern> cargo make test-filter` で絞り込み |
| `test-ts` | TypeScript テスト（Vitest） |
| `sqlx-prepare` | dev.db リセット＋ `.sqlx/` キャッシュ再生成 |
| `dev-db-reset` | 開発用 SQLite DB を削除（再起動で再作成） |

サブタスク・カバレッジ等の全タスクは `Makefile.toml` を参照。

### Git Hooks

`.githooks/` にフックを管理。`pre-commit` で `check-all`，`pre-merge-commit`（main マージ時）で `qa` を実行。

## Recent Changes
- 005-ipc-e2e-tests: Added Rust 2024 (edition = "2024", toolchain 1.94.0), TypeScript ~5.8.3 + Tauri 2, React 19, sqlx 0.8 (SQLite), uuid 1 (v7), chrono 0.4, thiserror 2, serde 1, serde_json 1, tokio 1 (sync)
- 004-table-view-operations: Added Rust 2024 (edition = "2024", toolchain 1.94.0), TypeScript ~5.8.3 + Tauri 2, React 19, sqlx 0.8 (SQLite), uuid 1 (v7), chrono 0.4, thiserror 2, serde 1, serde_json 1, Sonner (toast), Biome (lint/format)
- 003-database-properties: Added Rust 2024 (edition = "2024")，TypeScript ~5.8.3 + Tauri 2，React 19，sqlx 0.8 (SQLite)，uuid 1 (v7)，chrono 0.4，thiserror 2，serde 1，Sonner (toast)，Biome (lint/format)

## Active Technologies
- Rust 2024 (edition = "2024", toolchain 1.94.0), TypeScript ~5.8.3 + Tauri 2, React 19, sqlx 0.8 (SQLite), uuid 1 (v7), chrono 0.4, thiserror 2, serde 1, serde_json 1, tokio 1 (sync) (005-ipc-e2e-tests)
- SQLite (WAL mode), `sqlx::migrate!()` 埋め込みマイグレーション（6 ファイル），PRAGMA foreign_keys = ON (005-ipc-e2e-tests)
