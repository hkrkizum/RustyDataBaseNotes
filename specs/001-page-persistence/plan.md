# Implementation Plan: ページの永続化（最小縦断スライス）

**Branch**: `001-page-persistence` | **Date**: 2026-03-21 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-page-persistence/spec.md`

## Summary

Page エンティティの基本 CRUD をバックエンド（Rust/Tauri）からフロントエンド（React/TypeScript）
まで縦断的に実装する最小スライス。SQLite + sqlx による永続化（コンパイル時クエリ検証付き），sqlx 組み込みの
マイグレーション基盤，Page ドメインモデル（UUIDv7 識別子，タイトル値オブジェクト），
リポジトリパターンによる CRUD，型付き IPC コマンド，React による一覧・作成・更新・削除の
UI を構築する。

## Technical Context

**Language/Version**: Rust 2024 edition (toolchain 1.94.0), TypeScript ~5.8.3
**Primary Dependencies**:
- Backend: Tauri 2, sqlx 0.8 (runtime-tokio, sqlite, macros, migrate, chrono, uuid), uuid 1 (v7+serde), thiserror 2, chrono 0.4 (serde), serde 1 (derive), serde_json 1
- Frontend: React 19.1, @tauri-apps/api 2, sonner (toast notifications)
**Storage**: SQLite (WAL mode), `app_data_dir()/rustydatabasenotes.db`, sqlx::migrate!() による組み込みマイグレーション（_sqlx_migrations テーブルで追跡）
**Testing**: `cargo nextest run`, `cargo clippy --workspace`, `cargo doc --no-deps`, `cargo make qa`
**Target Platform**: Desktop (Linux WSL2 で開発，クロスプラットフォーム対応)
**Project Type**: desktop-app (Tauri 2)
**Performance Goals**: CRUD 操作 <1s 体感，1,000 件一覧表示 <1s，起動時マイグレーション <1s 体感
**Constraints**: ローカル完結（外部通信禁止），`unwrap()`/`expect()`/`panic!()`/`unsafe` 禁止，トランザクション保護
**Scale/Scope**: ページ数 ~1,000 件（本スライス），ブロックは後続機能

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Local-First Product Integrity**: ✅ PASS
  - SQLite ファイルは `app_data_dir()` に保存，外部通信は一切なし
  - すべての書き込み（作成・更新・削除）はトランザクション内で完結
  - マイグレーションもトランザクション保護下で実行（sqlx::migrate!() はアトミック）
  - WAL モードにより読み取り中の書き込みによる破損を防止
  - `bundled` feature で SQLite をバイナリに静的リンク — 外部ライブラリ依存なし

- **Domain-Faithful Information Model**: ✅ PASS
  - 正規語彙「ページ（page）」を一貫して使用
  - Page はエンティティとして定義，PageId・PageTitle は値オブジェクト
  - 将来のブロック（block）追加を見据え，Page は集約ルートとして設計
  - データベース（database），ビュー（view），プロパティ（property）は本スライスでは登場しないが，
    モデル設計はこれらの将来的な追加を阻害しない

- **Typed Boundaries and Bounded Contexts**: ✅ PASS
  - **Rust 境界型**: `PageId`, `PageTitle`, `Page`（ドメイン），`PageDto`（IPC），`CommandError`（IPC）
  - **TypeScript 境界型**: `Page`, `CommandError`, `CreatePageArgs`, `UpdatePageTitleArgs`, `DeletePageArgs`
  - **IPC 契約**: 5 コマンド（create_page, list_pages, get_page, update_page_title, delete_page）
  - **ストレージスキーマ**: Migration 0001_create_pages.sql で `pages` テーブル + `idx_pages_created_at` インデックス
  - **境界コンテキスト**: `domain::page`（ドメイン），`infrastructure::persistence`（インフラ），`ipc`（コマンド境界）

- **Test-First Delivery and Quality Gates**: ✅ PASS
  - 各ユーザーストーリーに対応するテストを実装前に作成（Red-Green-Refactor）
  - ドメイン層: PageTitle バリデーション，Page 生成のユニットテスト
  - インフラ層: sqlx のインメモリ SQLite プールを用いた CRUD 統合テスト
  - IPC 層: コマンドのエラーハンドリングテスト
  - 品質ゲート: `cargo make qa`（fmt + clippy + nextest + doc）

- **Safe Rust and Maintainability First**: ✅ PASS
  - `unsafe`, `unwrap()`, `expect()`, `panic!()`, `unreachable!()` は使用しない
  - Clippy の `unwrap_used = "deny"`, `panic = "deny"`, `unreachable = "deny"` が既に設定済み
  - すべての失敗可能操作は `Result` で伝播
  - `thiserror` でエラー型を宣言的に定義
  - 公開 API には `///` ドキュメントコメント（英語）を付与
  - 投機的最適化なし — 1,000 件で性能問題が計測された場合のみ最適化を検討
  - 既存の `lib.rs` の `.expect()` を `Result` ベースに修正する

## Project Structure

### Documentation (this feature)

```text
specs/001-page-persistence/
├── plan.md              # This file
├── research.md          # Phase 0: 技術選定調査
├── data-model.md        # Phase 1: データモデル定義
├── quickstart.md        # Phase 1: 開発手順ガイド
├── contracts/
│   └── ipc-commands.md  # Phase 1: IPC 契約定義
└── tasks.md             # Phase 2: タスク分解 (/speckit.tasks)
```

### Source Code (repository root)

```text
src/
├── main.tsx
├── App.tsx
├── App.css
├── features/
│   └── pages/
│       ├── PageListView.tsx         # 一覧画面
│       ├── PageListView.module.css
│       ├── PageItem.tsx             # 一覧行（インライン編集）
│       ├── PageItem.module.css
│       ├── CreatePageForm.tsx       # 新規作成フォーム
│       ├── CreatePageForm.module.css
│       ├── DeleteConfirmModal.tsx   # 削除確認ダイアログ
│       ├── DeleteConfirmModal.module.css
│       ├── usePages.ts             # ページ CRUD カスタムフック
│       └── types.ts                # Page, CommandError 型定義
└── components/
    └── toast/
        └── Toaster.tsx              # sonner Toaster 設定

src-tauri/
├── src/
│   ├── main.rs                      # エントリポイント
│   ├── lib.rs                       # Tauri Builder + コマンド登録
│   ├── domain/
│   │   ├── mod.rs
│   │   └── page/
│   │       ├── mod.rs
│   │       ├── entity.rs            # Page, PageId, PageTitle
│   │       ├── error.rs             # PageError
│   │       └── repository.rs        # PageRepository トレイト
│   ├── infrastructure/
│   │   ├── mod.rs
│   │   └── persistence/
│   │       ├── mod.rs
│   │       ├── database.rs          # SqlitePool 初期化，マイグレーション
│   │       ├── error.rs             # StorageError
│   │       └── page_repository.rs   # SqlxPageRepository
│   └── ipc/
│       ├── mod.rs
│       ├── error.rs                 # CommandError + Serialize
│       ├── dto.rs                   # PageDto（camelCase 変換）
│       └── page_commands.rs         # #[tauri::command] ハンドラ
├── migrations/
│   └── 0001_create_pages.sql        # pages テーブル + インデックス
├── .env                             # DATABASE_URL（コンパイル時クエリ検証用）
└── Cargo.toml
```

**Structure Decision**:
- `domain::page` — Page 集約の境界コンテキスト。エンティティ，値オブジェクト，ドメインエラー，
  リポジトリトレイトを含む。インフラ層に依存しない
- `infrastructure::persistence` — SQLite 実装の境界コンテキスト。sqlx への依存は
  ここに閉じ込める。ドメイン層のリポジトリトレイトを実装する。`SqlitePool` を受け取り
  非同期でクエリを実行する
- `ipc` — Tauri IPC コマンドの境界コンテキスト。ドメイン型を DTO に変換し，
  エラーをシリアライズ可能な形式に変換する。Tauri への依存はここに閉じ込める
- `src/features/pages` — フロントエンドのページ機能。IPC 呼び出し，状態管理，UI を含む
- `application/` 層は本スライスでは不要（ユースケースがシンプルなため IPC 層が直接リポジトリを呼ぶ）。
  複雑なビジネスロジックが必要になった段階で導入する
- `SqlitePool` は Send + Sync + Clone のため Mutex 不要。`AppState { db: SqlitePool }` として
  Tauri State に登録し，コマンドハンドラは `async fn` として定義する

## Complexity Tracking

> 憲章違反はなし。本スライスは最小構成であり，正当化が必要な複雑性は導入しない。

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| (なし) | — | — |
