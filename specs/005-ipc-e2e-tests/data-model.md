# Data Model: IPC テストおよび E2E テスト

**Branch**: `005-ipc-e2e-tests` | **Date**: 2026-03-22

> 本フィーチャーではデータベーススキーマの変更はない。
> 以下はテスト対象となる既存エンティティのマップと，テスト固有のデータ構造を記述する。

## テスト対象エンティティマップ

### 既存テーブル（変更なし）

| テーブル | カラム | IPC コマンド数 | テストカバレッジ |
|----------|--------|---------------|-----------------|
| `pages` | id, title, database_id, created_at, updated_at | 5 (page) + 5 (table) | 正常系 + 異常系 |
| `blocks` | id, page_id, block_type, content, position, created_at, updated_at | 8 (editor) | 正常系 + 異常系 |
| `databases` | id, title, created_at, updated_at | 5 (database) | 正常系 + 異常系 |
| `properties` | id, database_id, name, property_type, config, position, created_at, updated_at | 9 (property) | 正常系 + 異常系 |
| `property_values` | id, page_id, property_id, text_value, number_value, date_value, boolean_value, created_at, updated_at | (property コマンド内) | 正常系 + 異常系 |
| `views` | id, database_id, name, view_type, sort_conditions, filter_conditions, group_condition, collapsed_groups, created_at, updated_at | 6 (view) | 正常系 + 異常系 |

### 外部キー関係

```text
pages.database_id → databases.id (nullable, CASCADE)
blocks.page_id → pages.id (CASCADE)
properties.database_id → databases.id (CASCADE)
property_values.page_id → pages.id (CASCADE)
property_values.property_id → properties.id (CASCADE)
views.database_id → databases.id (UNIQUE, CASCADE)
```

### ドメインエンティティ → DTO マッピング

| Domain Entity | DTO | IPC 境界で検証する項目 |
|---------------|-----|----------------------|
| `Database` | `DatabaseDto` | id, title, created_at, updated_at の正確な変換 |
| `Page` | `PageDto` | id, title, database_id, created_at, updated_at |
| `Block` | `BlockDto` | id, page_id, block_type, content, position, timestamps |
| `EditorSession` | `EditorStateDto` | page_id, blocks (Vec), is_dirty フラグ |
| `Property` | `PropertyDto` | id, database_id, name, property_type, config, position |
| `PropertyValue` | `PropertyValueDto` | id, page_id, property_id, typed value fields |
| `View` | `ViewDto` | id, database_id, sort/filter/group conditions |
| (複合) | `TableDataDto` | database + properties + rows + view + groups |

### エラー種別マッピング

| Domain Error | CommandError variant | JSON kind 例 |
|-------------|---------------------|--------------|
| `PageError` | `CommandError::Page` | `titleEmpty`, `titleTooLong`, `notFound` |
| `BlockError` | `CommandError::Block` | `contentTooLong`, `blockNotFound`, `cannotMoveUp` |
| `DatabaseError` | `CommandError::Database` | `titleEmpty`, `databaseNotFound` |
| `PropertyError` | `CommandError::Property` | `propertyNameEmpty`, `duplicatePropertyName` |
| `PropertyValueError` | `CommandError::PropertyValue` | `invalidNumber`, `typeMismatch` |
| `ViewError` | `CommandError::View` | `viewNotFound`, `invalidSortCondition` |
| `StorageError` | `CommandError::Storage` | `storage` |

## テスト固有のデータ構造

### AppState（テスト用構築）

```rust
pub struct AppState {
    pub db: SqlitePool,           // テスト用一時 DB の接続プール
    pub sessions: Mutex<HashMap<PageId, EditorSession>>,  // エディタセッション
}
```

テストではこの構造体を直接構築し，内部ロジック関数に渡す。

### TempDbGuard（テストヘルパー）

テスト終了時に一時 DB ファイルを自動削除する RAII ガード。
`Drop` トレイト実装により，テストのパニック時にもクリーンアップが保証される。

## 新規マイグレーション

なし。本フィーチャーでは既存スキーマの変更を行わない。
