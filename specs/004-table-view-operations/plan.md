# Implementation Plan: テーブルビュー操作拡張（ソート・フィルタ・グルーピング）

**Branch**: `004-table-view-operations` | **Date**: 2026-03-22 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/004-table-view-operations/spec.md`

## Summary

テーブルビューにソート・フィルタ・グルーピング機能を追加し，View エンティティで設定を永続化する。
バックエンドに新しい `view` ドメインモジュールを追加し，View（ビュー設定）を独立した集約として管理する。
ソート・フィルタ・グルーピングのロジックは Rust バックエンドで実行し，処理済みデータを
フロントエンドに返却する。フィルタ条件・ソート条件は JSON カラムで `views` テーブルに格納し，
プロパティ削除時にはアプリケーション層で孤立条件を自動除去する。

## Technical Context

**Language/Version**: Rust 2024 (edition = "2024", toolchain 1.94.0), TypeScript ~5.8.3
**Primary Dependencies**: Tauri 2, React 19, sqlx 0.8 (SQLite), uuid 1 (v7), chrono 0.4, thiserror 2, serde 1, serde_json 1, Sonner (toast), Biome (lint/format)
**Storage**: SQLite (WAL mode), `{appDataDir}/rustydatabasenotes.db`, sqlx::migrate!() によるコンパイル時マイグレーション埋め込み。新規マイグレーションで `views` テーブルを追加。既存テーブル（pages, blocks, databases, properties, property_values）は変更なし
**Testing**: `cargo nextest run` (Rust), `cargo clippy` (lint), `cargo doc --no-deps` (docs), `pnpm vitest` (TypeScript), `cargo make qa` (フル QA)
**Target Platform**: Desktop (Linux/WSL2, Windows)
**Project Type**: desktop-app (Tauri)
**Performance Goals**: 100 ページ × 10 プロパティで 500ms 以内，1,000 ページで 2 秒以内（ソート・フィルタ・グルーピング各操作）
**Constraints**: ローカル完結（外部通信なし），パニック禁止，型安全な IPC 境界
**Scale/Scope**: データベースあたり最大 1,000 ページ，10 プロパティ，ソート条件 5 件，フィルタ条件 20 件

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **I. Local-First Product Integrity**: ビュー設定はローカル SQLite にのみ保存され，外部通信は一切発生しない。ビュー設定の保存はトランザクション内で原子的に実行し，部分書き込みを防止する。プロパティ削除時の設定自動修復により，クラッシュ後も整合性を維持する。既存データ（ページ，ブロック，プロパティ，プロパティ値）は一切変更しない。
- **II. Domain-Faithful Information Model**: 仕様書のドメイン語彙（view, database, page, property, sort condition, filter condition, group condition）を一貫して使用する。View は Database に属する独立エンティティとして設計し，将来の複数ビュー（ボード，ガントチャート）拡張を想定した構造とする。
- **III. Typed Boundaries and Domain-Driven Design**: 新規ドメインモジュール `domain::view` を追加（View エンティティ，ViewRepository トレイト，ViewError）。IPC 層に `ViewDto`, `SortConditionDto`, `FilterConditionDto`, `GroupConditionDto` 等の型付き DTO を定義。フロントエンドに対応する TypeScript 型を追加。マイグレーション `0006_create_views.sql` で views テーブルを追加。
- **IV. Test-First Delivery and Quality Gates**: 各ユーザーストーリーに対して先に失敗するテストを記述する。Rust 側: ソートロジック（プロパティ型別），フィルタロジック（演算子別），グルーピングロジック，ビュー永続化・復元，プロパティ削除時の自動修復。TypeScript 側: ビュー操作 UI のユニットテスト。品質ゲート: `cargo make qa`（fmt-check → clippy → lint-ts → ts-check → test）。
- **V. Safe Rust, SOLID Principles, and Maintainability First**: `unsafe` / 投機的最適化は使用しない。SOLID 準拠: View ドメインは独立モジュール（SRP），ViewRepository トレイトで永続化を抽象化（DIP），ソート・フィルタ・グルーピングの各ロジックは独立した関数として実装（ISP/SRP），PropertyType ごとの比較はトレイト実装で拡張可能（OCP）。
- **VI. Rust Documentation Standards**: すべての新規公開アイテムに `///` ドキュメントコメント（要約行 + Examples + Errors）を付与。`#![warn(missing_docs)]` が既存で有効。`cargo doc --no-deps` を品質ゲートで検証。
- **VII. Defensive Error Handling**: 非テストコードで `unwrap()` / `expect()` / `panic!()` / `todo!()` / `assert!()` を使用しない。新規 `ViewError` 列挙型を `thiserror` で定義し，十分なコンテキスト（view_id, property_id, operator 等）を保持する。Clippy ワークスペース lint（`unwrap_used = "deny"` 等）が既存で設定済み。

## Project Structure

### Documentation (this feature)

```text
specs/004-table-view-operations/
├── plan.md              # This file
├── research.md          # Phase 0: 設計判断の調査結果
├── data-model.md        # Phase 1: エンティティ定義
├── quickstart.md        # Phase 1: 開発者向けクイックスタート
├── contracts/           # Phase 1: IPC 契約定義
│   └── view-commands.md
└── tasks.md             # Phase 2: 実装タスク
```

### Source Code (repository root)

```text
src/
├── components/
│   └── toast/
├── features/
│   └── database/           # 既存: TableView, TableHeader, TableRow 等を拡張
│       ├── types.ts        # View 関連型を追加
│       ├── useTableData.ts # ビュー操作フックを追加
│       ├── TableView.tsx   # ソート/フィルタ/グルーピング UI を追加
│       ├── TableHeader.tsx # ソートインジケータを追加
│       ├── SortPanel.tsx          # 新規: 複数ソート設定パネル
│       ├── FilterPanel.tsx        # 新規: フィルタ条件設定パネル
│       ├── FilterConditionRow.tsx # 新規: 個別フィルタ条件行
│       ├── GroupPanel.tsx         # 新規: グルーピング設定パネル
│       └── GroupHeader.tsx        # 新規: グループヘッダー
└── main.tsx

src-tauri/
├── src/
│   ├── domain/
│   │   ├── view/              # 新規: ビュードメイン
│   │   │   ├── mod.rs
│   │   │   ├── entity.rs      # View, SortCondition, FilterCondition, GroupCondition
│   │   │   ├── repository.rs  # ViewRepository トレイト
│   │   │   ├── error.rs       # ViewError
│   │   │   ├── sort.rs        # ソートロジック
│   │   │   ├── filter.rs      # フィルタロジック
│   │   │   └── group.rs       # グルーピングロジック
│   │   ├── database/          # 既存（変更なし）
│   │   ├── page/              # 既存（変更なし）
│   │   └── property/          # 既存（変更なし）
│   ├── infrastructure/
│   │   └── persistence/
│   │       └── view_repository.rs  # 新規: SqlxViewRepository
│   └── ipc/
│       ├── view_commands.rs   # 新規: ビュー操作コマンド
│       └── dto.rs             # ViewDto 等を追加
└── migrations/
    └── 0006_create_views.sql  # 新規: views テーブル
```

**Structure Decision**: View は Database に属する独立した境界づけられたコンテキストとして `domain::view` モジュールに配置する。ソート・フィルタ・グルーピングのロジックは View ドメイン内の独立ファイル（sort.rs, filter.rs, group.rs）に分離し，プロパティ型への依存は `domain::property` の公開型を参照する形とする。これにより，将来のビュータイプ追加（ボード等）時にも View ドメインの拡張で対応できる。

## Complexity Tracking

> Constitution Check に違反なし。追加の複雑性正当化は不要。
