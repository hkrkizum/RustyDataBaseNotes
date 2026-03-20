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
| `qa` | フル QA（`qa-rs` → `qa-ts` を順次実行） |
| `check-all` | テスト抜き静的チェック（fmt-check → clippy → lint-ts → ts-check） |

#### Formatting

| タスク | 内容 |
|--------|------|
| `fmt` | Rust + TypeScript を一括フォーマット |

#### Rust (`qa-rs`): fmt-rs → clippy → test → doc-test

| タスク | 内容 |
|--------|------|
| `fmt-rs` | `cargo fmt --all`（自動修正） |
| `fmt-rs-check` | フォーマットチェックのみ（CI 向け） |
| `clippy` | `cargo clippy`（warnings = errors） |
| `test` | cargo-nextest でワークスペース全テスト |
| `test-filter` | `TEST_FILTER=<pattern> cargo make test-filter` で絞り込み |
| `test-std` | 標準 cargo test（nextest 不使用） |
| `doc-test` | ドキュメンテーションテスト |

#### TypeScript (`qa-ts`): fmt-ts → lint-ts → ts-check → test-ts

| タスク | 内容 |
|--------|------|
| `fmt-ts` | Biome format（自動修正） |
| `fmt-ts-check` | フォーマットチェックのみ（CI 向け） |
| `lint-ts` | Biome lint |
| `ts-check` | `tsc --noEmit`（型チェック） |
| `test-ts` | Vitest（`--passWithNoTests`） |

### その他

| タスク | 内容 |
|--------|------|
| `serve` | Tauri 開発サーバー（HMR） |
| `doc` | rustdoc をビルド＆ブラウザで開く |
| `dev-db-reset` | 開発用 SQLite DB を削除（再起動で再作成） |

## Recent Changes
- 002-block-editor: Added Rust 2024 edition (toolchain 1.94.0), TypeScript ~5.8.3
- 001-page-persistence: Added Rust 2024 edition (toolchain 1.94.0), TypeScript ~5.8.3

## Active Technologies
