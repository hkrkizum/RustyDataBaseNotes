# Implementation Plan: プロパティシステムとデータベース概念の導入

**Branch**: `003-database-properties` | **Date**: 2026-03-21 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/003-database-properties/spec.md`

## Summary

ページにプロパティ（構造化データ）を付与し，ページ集合を「データベース」として管理する
機能を導入する。データベースは共通のプロパティスキーマを持ち，テーブルビュー（表形式）で
一覧表示・インライン編集を可能にする。プロパティ型は テキスト・数値・日付・セレクト・
チェックボックスの5種類。既存のページ・ブロック基盤を拡張し，新たに `database`，
`property`，`property_value` の3ドメインエンティティを追加する。

## Technical Context

**Language/Version**: Rust 2024 (edition = "2024")，TypeScript ~5.8.3
**Primary Dependencies**: Tauri 2，React 19，sqlx 0.8 (SQLite)，uuid 1 (v7)，chrono 0.4，thiserror 2，serde 1，Sonner (toast)，Biome (lint/format)
**Storage**: SQLite (WAL mode)，`{appDataDir}/rustydatabasenotes.db`，sqlx::migrate!() によるコンパイル時マイグレーション埋め込み。新規マイグレーションで `databases`，`properties`，`property_values` テーブルを追加。既存テーブル (`pages`，`blocks`) は変更なし（`pages` に `database_id` 外部キーを追加）
**Testing**: `cargo make qa`（`cargo nextest run`，`cargo clippy --deny warnings`，`cargo doc --no-deps`，`pnpm vitest`，`pnpm biome lint`，`tsc --noEmit`）
**Target Platform**: Desktop（Linux WSL2 で開発，Windows/macOS 対応）
**Project Type**: desktop-app（Tauri ローカルファーストアプリ）
**Performance Goals**: テーブルビュー表示 ≤1s（100ページ×10プロパティ），インライン編集保存 ≤500ms，1,000ページでスムーズスクロール
**Constraints**: オフライン完結（外部通信禁止），パニック禁止（`unwrap`/`expect`/`panic!` deny），Clippy workspace lints 設定済み
**Scale/Scope**: データベースあたりプロパティ上限50，セレクト選択肢上限100，1,000ページ規模での動作を保証

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **I. Local-First Product Integrity**: すべてのデータ（データベース定義，プロパティスキーマ，プロパティ値）はローカル SQLite に保存。外部通信なし（FR-009）。プロパティ値の保存・プロパティ削除・データベース削除はトランザクション内で原子的に実行し，クラッシュ時のデータ破損を防止（CC-001）。既存のバックアップ・復旧手段（SQLite WAL + DB ファイルコピー）がそのまま適用される。

- **II. Domain-Faithful Information Model**: 中核語彙を一貫使用 — `Database`（ページ集合の上位概念），`Property`（スキーマ定義），`PropertyValue`（ページ×プロパティの値），`View`（テーブルビュー）。既存の `Page`，`Block` には影響を与えず拡張。将来のボードビュー・ガントチャートに対応可能なモデル設計。

- **III. Typed Boundaries and Domain-Driven Design**:
  - **新規 Rust 型**: `DatabaseId`，`DatabaseTitle`，`Database`（集約ルート），`PropertyId`，`PropertyName`，`PropertyType` (enum)，`PropertyConfig`，`Property`，`PropertyValueId`，`PropertyValue`，`SelectOption`
  - **新規 TypeScript 型**: `Database`，`Property`，`PropertyValue`，`DatabaseDto`，`PropertyDto`，`PropertyValueDto`
  - **IPC コマンド**: データベース CRUD，プロパティ CRUD，プロパティ値 CRUD，テーブルデータ取得
  - **ストレージスキーマ**: 3 マイグレーション（`databases`，`properties`，`property_values` テーブル + `pages.database_id` カラム追加）
  - **境界コンテキスト**: 既存の `page`，`block`，`editor` に加え，新規 `database`，`property` コンテキストを追加。ドメイン層は外部技術に依存せず，リポジトリトレイトで抽象化

- **IV. Test-First Delivery and Quality Gates**: 各ユーザーストーリーに対して先に失敗するテストを作成。ドメイン層テスト（バリデーション，ビジネスルール），リポジトリテスト（CRUD，トランザクション，カスケード削除），IPC コマンドテスト，フロントエンドコンポーネントテスト。品質ゲート: `cargo make qa`

- **V. Safe Rust, SOLID Principles, and Maintainability First**: `unsafe` 不使用。投機的最適化なし（仮想化は1,000ページ目標に対して必要時のみ検討）。SOLID 準拠:
  - SRP: `database`，`property` を独立モジュールに分離
  - OCP: `PropertyType` enum + バリデーショントレイトで型追加に対応
  - LSP: リポジトリトレイト実装は契約遵守
  - ISP: リポジトリトレイトは `DatabaseRepository`，`PropertyRepository`，`PropertyValueRepository` に分離
  - DIP: IPC → ドメイン ← インフラの依存方向

- **VI. Rust Documentation Standards**: 新規公開アイテムすべてに `///` ドキュメントコメント（要約行 + Examples + Errors）。`#![warn(missing_docs)]` 既設。`cargo doc --no-deps` クリーン維持。

- **VII. Defensive Error Handling**: `thiserror` で `DatabaseError`，`PropertyError`，`PropertyValueError` を定義。各バリアントに十分なコンテキスト（フィールド名，期待値・実際値等）。Clippy lints（`unwrap_used = "deny"` 等）既設。テストコードのみ `unwrap`/`assert` 許可。

**GATE 結果: PASS** — 違反なし。

**Post-Design Re-check (Phase 1 完了後)**:
- I. ローカル完結: マイグレーション3本は追加的変更のみ。`pages.database_id` の `ON DELETE SET NULL` でデータベース削除時もページ保護。トランザクション境界は data-model.md で明示 ✓
- II. ドメイン語彙: `Database`，`Property`，`PropertyValue`，`View` を一貫使用。既存の `Page`，`Block` と衝突なし ✓
- III. 型付き境界: IPC コマンド17本を contracts/ipc-commands.md で定義。DTO はすべて camelCase。エラー kind 17種を追加 ✓
- IV. テスト先行: quickstart.md に TDD フロー明記。ドメイン→インフラ→IPC→フロントエンドの順で各層にテスト ✓
- V. SOLID: リポジトリ3分割（ISP），ドメイン層の外部技術非依存（DIP），独立モジュール（SRP） ✓
- VI. ドキュメント: 新規公開アイテムに doc コメント必須。`#![warn(missing_docs)]` 既設 ✓
- VII. エラーハンドリング: 3つの thiserror enum（Database/Property/PropertyValue）。Clippy deny lints 既設 ✓
- **Re-check 結果: PASS**

## Project Structure

### Documentation (this feature)

```text
specs/003-database-properties/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (/speckit.tasks)
```

### Source Code (repository root)

```text
src/
├── components/
│   └── toast/
├── features/
│   ├── pages/           # 既存: ページ一覧・CRUD
│   ├── editor/          # 既存: ブロックエディタ
│   └── database/        # 新規: データベース管理・テーブルビュー
└── App.tsx

src-tauri/
├── src/
│   ├── domain/
│   │   ├── page/        # 既存: Page エンティティ（database_id 追加）
│   │   ├── block/       # 既存: Block エンティティ（変更なし）
│   │   ├── editor/      # 既存: EditorSession（変更なし）
│   │   ├── database/    # 新規: Database 集約ルート
│   │   └── property/    # 新規: Property + PropertyValue エンティティ
│   ├── infrastructure/
│   │   └── persistence/
│   │       ├── database_repository.rs   # 新規
│   │       ├── property_repository.rs   # 新規
│   │       └── ...                      # 既存ファイル
│   └── ipc/
│       ├── database_commands.rs         # 新規
│       ├── property_commands.rs         # 新規
│       ├── dto.rs                       # 拡張
│       └── ...                          # 既存ファイル
└── migrations/
    ├── 0001_create_pages.sql            # 既存
    ├── 0002_create_blocks.sql           # 既存
    ├── 0003_create_databases.sql        # 新規
    ├── 0004_create_properties.sql       # 新規
    └── 0005_add_page_database_id_and_property_values.sql  # 新規
```

**Structure Decision**: `database` と `property` を独立した境界コンテキストとしてモジュール分離。`database` は集約ルートとして `Property` のライフサイクルを管理。`PropertyValue` は `Page` と `Property` の両方に従属するクロスカッティングエンティティのため `property` コンテキスト内に配置。フロントエンドは `features/database/` に集約し，テーブルビュー・プロパティ編集・データベース管理を含む。

## Complexity Tracking

> 違反なし — 記載不要。
