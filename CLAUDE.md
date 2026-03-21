# CLAUDE.md

プロジェクトの原則，技術標準，ワークフロー，ガバナンスは
[`.specify/memory/constitution.md`](.specify/memory/constitution.md) に定義されている。
作業開始前に必ず参照すること。

## Task Runner (cargo-make)

タスク定義は `Makefile.toml`。すべて `cargo make <task>` で実行する。

### Build & Check

| タスク | 内容 |
|--------|------|
| `build` | Tauri 本番ビルド |
| `check` | `cargo check --workspace`（高速コンパイルチェック） |
| `clean` | ビルド成果物の削除（`target/` + `dist/`） |

### Quality Gate

| タスク | 内容 |
|--------|------|
| `qa` | フル QA（`sqlx-prepare` → `qa-rs` → `qa-ts` を順次実行） |
| `sqlx-prepare` | dev.db をマイグレーションからリセットし `.sqlx/` キャッシュを再生成 |
| `check-all` | テスト抜き静的チェック（fmt-check → clippy → lint-ts → ts-check） |

#### Formatting

| タスク | 内容 |
|--------|------|
| `fmt` | Rust + TypeScript を一括フォーマット |

#### Rust (`qa-rs`): fmt-rs → clippy → test → doc-test → doc-check

| タスク | 内容 |
|--------|------|
| `fmt-rs` | `cargo fmt --all`（自動修正） |
| `fmt-rs-check` | フォーマットチェックのみ（CI 向け） |
| `clippy` | `cargo clippy`（warnings = errors） |
| `test` | cargo-nextest でワークスペース全テスト |
| `test-filter` | `TEST_FILTER=<pattern> cargo make test-filter` で絞り込み |
| `test-std` | 標準 cargo test（nextest 不使用） |
| `doc-test` | ドキュメンテーションテスト |
| `doc-check` | `cargo doc --no-deps`（警告 = エラー，Principle VI 準拠） |

#### TypeScript (`qa-ts`): fmt-ts → lint-ts → ts-check → test-ts

| タスク | 内容 |
|--------|------|
| `fmt-ts` | Biome format（自動修正） |
| `fmt-ts-check` | フォーマットチェックのみ（CI 向け） |
| `lint-ts` | Biome lint |
| `ts-check` | `tsc --noEmit`（型チェック） |
| `test-ts` | Vitest（`--passWithNoTests`） |

### Coverage

| タスク | 内容 |
|--------|------|
| `coverage` | Rust + TypeScript のカバレッジを一括算出 |
| `coverage-rs` | Rust カバレッジ（cargo-llvm-cov，テキスト表示） |
| `coverage-rs-html` | Rust カバレッジ HTML レポート（ブラウザで開く） |
| `coverage-rs-lcov` | Rust カバレッジ LCOV 出力（CI 連携用） |
| `coverage-ts` | TypeScript カバレッジ（Vitest + v8） |

### その他

| タスク | 内容 |
|--------|------|
| `serve` | Tauri 開発サーバー（HMR） |
| `doc` | rustdoc をビルド＆ブラウザで開く |
| `dev-db-reset` | 開発用 SQLite DB を削除（再起動で再作成） |

## Recent Changes
- 003-database-properties: Added Rust 2024 (edition = "2024")，TypeScript ~5.8.3 + Tauri 2，React 19，sqlx 0.8 (SQLite)，uuid 1 (v7)，chrono 0.4，thiserror 2，serde 1，Sonner (toast)，Biome (lint/format)
- 002-block-editor: Added Rust 2024 edition (toolchain 1.94.0), TypeScript ~5.8.3
- 001-page-persistence: Added Rust 2024 edition (toolchain 1.94.0), TypeScript ~5.8.3

## Active Technologies
- Rust 2024 (edition = "2024")，TypeScript ~5.8.3 + Tauri 2，React 19，sqlx 0.8 (SQLite)，uuid 1 (v7)，chrono 0.4，thiserror 2，serde 1，Sonner (toast)，Biome (lint/format) (003-database-properties)
- SQLite (WAL mode)，`{appDataDir}/rustydatabasenotes.db`，sqlx::migrate!() によるコンパイル時マイグレーション埋め込み。新規マイグレーションで `databases`，`properties`，`property_values` テーブルを追加。既存テーブル (`pages`，`blocks`) は変更なし（`pages` に `database_id` 外部キーを追加） (003-database-properties)
