# CLAUDE.md

プロジェクトの原則，技術標準，ワークフロー，ガバナンスは
[`.specify/memory/constitution.md`](.specify/memory/constitution.md) に定義されている。
作業開始前に必ず参照すること。

## Recent Changes
- 001-page-persistence: Added Rust 2024 edition (toolchain 1.94.0), TypeScript ~5.8.3
- 001-page-persistence: Added Rust 2024 edition (toolchain 1.94.0), TypeScript ~5.8.3
- 001-page-persistence: Added [SQLite or equivalent local store, backup path, migration plan]

## Active Technologies
- SQLite (WAL mode), `app_data_dir()/rustydatabasenotes.db`, sqlx::migrate!() による組み込みマイグレーション（_sqlx_migrations テーブルで追跡） (001-page-persistence)
