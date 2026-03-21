# Quickstart: 003-database-properties

**Date**: 2026-03-21

## 前提条件

- Nix devshell がアクティブ（`direnv allow` 済み）
- `cargo make check` がパス
- ブランチ `003-database-properties` にいること

## 開発サーバー起動

```bash
cargo make serve
```

## 品質ゲート

```bash
# フル QA（Rust + TypeScript）
cargo make qa

# Rust のみ
cargo make qa-rs

# TypeScript のみ
cargo make qa-ts

# 静的チェックのみ（テスト除外）
cargo make check-all
```

## テスト実行

```bash
# Rust テスト（cargo-nextest）
cargo make test

# 特定テストの絞り込み
TEST_FILTER=database cargo make test-filter

# TypeScript テスト
cargo make test-ts

# ドキュメントテスト
cargo make doc-test
```

## マイグレーション

sqlx の compile-time マイグレーションを使用。マイグレーションファイルは
`src-tauri/migrations/` に配置し，`sqlx::migrate!()` マクロで自動実行される。

### 新規マイグレーションの作成

```bash
# sqlx-cli でマイグレーション作成（タイムスタンプ付き）
cd src-tauri && sqlx migrate add <name>
```

### 開発 DB のリセット

```bash
cargo make dev-db-reset
```

## この機能で追加するファイル

### Rust (src-tauri/src/)

```
domain/
├── database/
│   ├── mod.rs
│   ├── entity.rs        # Database, DatabaseId, DatabaseTitle
│   ├── error.rs         # DatabaseError
│   └── repository.rs    # DatabaseRepository trait
└── property/
    ├── mod.rs
    ├── entity.rs         # Property, PropertyId, PropertyName, PropertyType,
    │                     # PropertyConfig, SelectOption, PropertyValue, PropertyValueId
    ├── error.rs          # PropertyError, PropertyValueError
    └── repository.rs     # PropertyRepository, PropertyValueRepository traits

infrastructure/persistence/
├── database_repository.rs    # SqlxDatabaseRepository
├── property_repository.rs    # SqlxPropertyRepository
└── property_value_repository.rs  # SqlxPropertyValueRepository

ipc/
├── database_commands.rs      # create_database, list_databases, ...
├── property_commands.rs      # add_property, set_property_value, ...
└── table_commands.rs         # get_table_data, add_page_to_database, ...
```

### TypeScript (src/)

```
features/database/
├── types.ts                 # Database, Property, PropertyValue, TableData 型
├── useDatabase.ts           # データベース CRUD フック
├── useTableData.ts          # テーブルデータ取得・操作フック
├── DatabaseListView.tsx     # 統合リスト（ページ + データベース混在表示）
├── TableView.tsx            # テーブルビュー（メインコンポーネント）
├── TableHeader.tsx          # 列ヘッダー（プロパティ名表示・管理）
├── TableRow.tsx             # 行コンポーネント
├── PropertyCell.tsx         # プロパティ型に応じたセルエディタ
├── AddPropertyModal.tsx     # プロパティ追加ダイアログ
├── AddPageModal.tsx         # 既存ページ追加ダイアログ
└── PropertyConfigPanel.tsx  # プロパティ設定パネル
```

### マイグレーション (src-tauri/migrations/)

```
0003_create_databases.sql
0004_create_properties.sql
0005_add_page_database_id_and_property_values.sql
```

## 実装順序の概要

1. **ドメイン層**: エンティティ + バリデーション + エラー型 + リポジトリトレイト
2. **マイグレーション**: SQLite スキーマ
3. **インフラ層**: リポジトリ実装（SqlxXxxRepository）
4. **IPC 層**: コマンド + DTO + エラーマッピング
5. **フロントエンド**: 型定義 → フック → コンポーネント

各ステップで TDD（Red → Green → Refactor）を適用。
