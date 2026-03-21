# Data Model: 003-database-properties

**Date**: 2026-03-21

## Entity Overview

```
┌─────────────┐       ┌──────────────┐       ┌────────────────┐
│  Database   │1────*│   Property    │       │                │
│  (集約ルート) │       │  (スキーマ定義)│       │                │
└──────┬──────┘       └──────┬───────┘       │                │
       │ 1                    │ 1              │                │
       │                     │               │                │
       │ *                    │ *              │                │
┌──────┴──────┐       ┌──────┴───────┐       │                │
│    Page     │*────1│ PropertyValue │1────*│     Page        │
│  (既存拡張)  │       │   (交差値)    │       │   (既存)       │
└─────────────┘       └──────────────┘       └────────────────┘
```

## Entities

### Database（データベース）— 新規，集約ルート

ページ集合に共通のプロパティスキーマを付与する上位概念。

| フィールド | Rust 型 | SQLite 型 | 制約 |
|-----------|---------|-----------|------|
| id | `DatabaseId(Uuid)` | TEXT PK | UUIDv7 |
| title | `DatabaseTitle(String)` | TEXT NOT NULL | 1–255文字（トリム後） |
| created_at | `DateTime<Utc>` | TEXT NOT NULL | RFC 3339 |
| updated_at | `DateTime<Utc>` | TEXT NOT NULL | RFC 3339 |

**バリデーション規則**:
- `DatabaseTitle`: 空不可，255文字上限（`PageTitle` と同パターン）
- `DatabaseId`: UUIDv7（時刻ベース，ソート可能）

**ライフサイクル**:
- 作成: `Database::new(title)` — ID と timestamp を自動生成
- 復元: `Database::from_stored(id, title, created_at, updated_at)`
- 削除: カスケードで `Property`，`PropertyValue` を削除，`Page.database_id` を NULL 化

---

### Property（プロパティ）— 新規

データベースに属するスキーマ定義。

| フィールド | Rust 型 | SQLite 型 | 制約 |
|-----------|---------|-----------|------|
| id | `PropertyId(Uuid)` | TEXT PK | UUIDv7 |
| database_id | `DatabaseId` | TEXT NOT NULL FK | → databases.id |
| name | `PropertyName(String)` | TEXT NOT NULL | 1–100文字，DB 内一意 |
| property_type | `PropertyType` | TEXT NOT NULL | "text"\|"number"\|"date"\|"select"\|"checkbox" |
| config | `PropertyConfig` | TEXT | JSON，型固有設定。NULL 可 |
| position | `i64` | INTEGER NOT NULL | 0始まり表示順 |
| created_at | `DateTime<Utc>` | TEXT NOT NULL | RFC 3339 |
| updated_at | `DateTime<Utc>` | TEXT NOT NULL | RFC 3339 |

**バリデーション規則**:
- `PropertyName`: 空不可，100文字上限，同一データベース内で一意
- `PropertyType`: 5種類の enum（`Text`, `Number`, `Date`, `Select`, `Checkbox`）
- `position`: 非負整数

**PropertyType enum**:
```rust
pub enum PropertyType {
    Text,
    Number,
    Date,
    Select,
    Checkbox,
}
```

**PropertyConfig enum**（型固有設定，serde tagged）:
```rust
pub enum PropertyConfig {
    Text,
    Number,
    Date { mode: DateMode },
    Select { options: Vec<SelectOption> },
    Checkbox,
}

pub enum DateMode {
    Date,      // 日付のみ
    DateTime,  // 日時
}

pub struct SelectOption {
    pub id: SelectOptionId,  // UUIDv7
    pub value: String,       // 表示名（1–100文字，選択肢内一意）
}
```

**ライフサイクル**:
- Database に従属。Database 削除時にカスケード削除
- 削除時は関連する PropertyValue も原子的に削除

---

### PropertyValue（プロパティ値）— 新規

特定のページの特定のプロパティに対する値。Page と Property の交差エンティティ。

| フィールド | Rust 型 | SQLite 型 | 制約 |
|-----------|---------|-----------|------|
| id | `PropertyValueId(Uuid)` | TEXT PK | UUIDv7 |
| page_id | `PageId` | TEXT NOT NULL FK | → pages.id |
| property_id | `PropertyId` | TEXT NOT NULL FK | → properties.id |
| text_value | `Option<String>` | TEXT | テキスト型・セレクト型の値 |
| number_value | `Option<f64>` | REAL | 数値型の値 |
| date_value | `Option<DateTime<Utc>>` | TEXT | 日付型の値（RFC 3339） |
| boolean_value | `Option<bool>` | INTEGER | チェックボックス型の値 |
| created_at | `DateTime<Utc>` | TEXT NOT NULL | RFC 3339 |
| updated_at | `DateTime<Utc>` | TEXT NOT NULL | RFC 3339 |

**UNIQUE 制約**: `(page_id, property_id)` — 1ページ×1プロパティにつき最大1値

**バリデーション規則（PropertyType に応じて）**:
- `Text`: `text_value` に格納。文字数制限なし（将来検討）
- `Number`: `number_value` に格納。有限数値のみ（NaN, Infinity 拒否）
- `Date`: `date_value` に格納。mode に応じて date/datetime
- `Select`: `text_value` に選択肢 ID（UUID 文字列）を格納。存在する選択肢のみ許可
- `Checkbox`: `boolean_value` に格納（0 or 1）。新規作成時のデフォルト = false (0)

**ライフサイクル**:
- Page と Property の両方に従属（いずれかの削除で消滅）
- セレクト選択肢の削除時は，該当する値を NULL にリセット

---

### Page（ページ）— 既存拡張

| フィールド | 変更 | 内容 |
|-----------|------|------|
| database_id | **追加** | `Option<DatabaseId>` — NULL 可 FK → databases.id |

**制約**:
- `NULL` = スタンドアロンページ（従来通り）
- 非 NULL = 特定のデータベースに所属
- 1ページ = 最大1データベース（カラムの単一値性で保証）

---

## SQLite Schema（マイグレーション）

### 0003_create_databases.sql

```sql
CREATE TABLE databases (
    id         TEXT PRIMARY KEY NOT NULL,
    title      TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_databases_created_at ON databases (created_at DESC);
```

### 0004_create_properties.sql

```sql
CREATE TABLE properties (
    id            TEXT PRIMARY KEY NOT NULL,
    database_id   TEXT NOT NULL,
    name          TEXT NOT NULL,
    property_type TEXT NOT NULL,
    config        TEXT,
    position      INTEGER NOT NULL,
    created_at    TEXT NOT NULL,
    updated_at    TEXT NOT NULL,
    FOREIGN KEY (database_id) REFERENCES databases(id) ON DELETE CASCADE
);

CREATE INDEX idx_properties_database_id ON properties (database_id, position ASC);
CREATE UNIQUE INDEX idx_properties_name_unique ON properties (database_id, name);
```

### 0005_add_page_database_id_and_property_values.sql

```sql
ALTER TABLE pages ADD COLUMN database_id TEXT REFERENCES databases(id) ON DELETE SET NULL;

CREATE INDEX idx_pages_database_id ON pages (database_id);

CREATE TABLE property_values (
    id             TEXT PRIMARY KEY NOT NULL,
    page_id        TEXT NOT NULL,
    property_id    TEXT NOT NULL,
    text_value     TEXT,
    number_value   REAL,
    date_value     TEXT,
    boolean_value  INTEGER,
    created_at     TEXT NOT NULL,
    updated_at     TEXT NOT NULL,
    FOREIGN KEY (page_id) REFERENCES pages(id) ON DELETE CASCADE,
    FOREIGN KEY (property_id) REFERENCES properties(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX idx_property_values_page_property
    ON property_values (page_id, property_id);
CREATE INDEX idx_property_values_property_id
    ON property_values (property_id);
```

**マイグレーション設計ノート**:
- `pages.database_id` の `ON DELETE SET NULL`: データベース削除時にページを保持（FR-011）
- `property_values` の FK は両方 `ON DELETE CASCADE`:
  - ページ削除時 → property_values も削除
  - プロパティ削除時 → property_values も削除（FR-010）
- 既存データへの影響: `pages` への `database_id` カラム追加のみ（NULL デフォルト）

## Domain Error Types

### DatabaseError

```rust
pub enum DatabaseError {
    TitleEmpty,
    TitleTooLong { len: usize, max: usize },
    NotFound { id: DatabaseId },
}
```

### PropertyError

```rust
pub enum PropertyError {
    NameEmpty,
    NameTooLong { len: usize, max: usize },
    DuplicateName { name: String, database_id: DatabaseId },
    InvalidType { value: String },
    TooManyProperties { count: usize, max: usize },
    NotFound { id: PropertyId },
    InvalidConfig { reason: String },
    TooManyOptions { count: usize, max: usize },
    OptionValueEmpty,
    DuplicateOptionValue { value: String },
}
```

### PropertyValueError

```rust
pub enum PropertyValueError {
    InvalidNumber { reason: String },
    InvalidSelectOption { option_id: String, property_id: PropertyId },
    TypeMismatch { expected: PropertyType, property_id: PropertyId },
    NotFound { id: PropertyValueId },
}
```

## Repository Traits

### DatabaseRepository

```rust
pub trait DatabaseRepository {
    type Error: From<DatabaseError>;

    async fn create(&self, database: &Database) -> Result<(), Self::Error>;
    async fn find_by_id(&self, id: &DatabaseId) -> Result<Database, Self::Error>;
    async fn find_all(&self) -> Result<Vec<Database>, Self::Error>;
    async fn update_title(&self, id: &DatabaseId, title: &DatabaseTitle) -> Result<Database, Self::Error>;
    async fn delete(&self, id: &DatabaseId) -> Result<(), Self::Error>;
}
```

### PropertyRepository

```rust
pub trait PropertyRepository {
    type Error: From<PropertyError>;

    async fn create(&self, property: &Property) -> Result<(), Self::Error>;
    async fn find_by_database_id(&self, database_id: &DatabaseId) -> Result<Vec<Property>, Self::Error>;
    async fn find_by_id(&self, id: &PropertyId) -> Result<Property, Self::Error>;
    async fn update_name(&self, id: &PropertyId, name: &PropertyName) -> Result<Property, Self::Error>;
    async fn update_config(&self, id: &PropertyId, config: &PropertyConfig) -> Result<Property, Self::Error>;
    async fn update_positions(&self, updates: &[(PropertyId, i64)]) -> Result<(), Self::Error>;
    async fn delete(&self, id: &PropertyId) -> Result<(), Self::Error>;
    async fn count_by_database_id(&self, database_id: &DatabaseId) -> Result<usize, Self::Error>;
    async fn next_position(&self, database_id: &DatabaseId) -> Result<i64, Self::Error>;
}
```

### PropertyValueRepository

```rust
pub trait PropertyValueRepository {
    type Error: From<PropertyValueError>;

    async fn upsert(&self, value: &PropertyValue) -> Result<(), Self::Error>;
    async fn find_by_page_and_property(&self, page_id: &PageId, property_id: &PropertyId) -> Result<Option<PropertyValue>, Self::Error>;
    async fn find_by_page_id(&self, page_id: &PageId) -> Result<Vec<PropertyValue>, Self::Error>;
    async fn find_by_property_id(&self, property_id: &PropertyId) -> Result<Vec<PropertyValue>, Self::Error>;
    async fn delete_by_page_and_database(&self, page_id: &PageId, database_id: &DatabaseId) -> Result<(), Self::Error>;
    async fn reset_select_option(&self, property_id: &PropertyId, option_id: &str) -> Result<(), Self::Error>;
    async fn find_all_for_database(&self, database_id: &DatabaseId) -> Result<Vec<PropertyValue>, Self::Error>;
}
```

## State Transitions

### Page の所属状態

```
スタンドアロン (database_id = NULL)
    ↓ [データベースに追加]
所属中 (database_id = Some(id))
    ↓ [データベースから除外]
スタンドアロン (database_id = NULL) + PropertyValues 削除
    ↓ [完全削除]
削除済み (レコード消滅 + Blocks + PropertyValues も CASCADE 削除)
```

### PropertyValue のライフサイクル

```
未作成 (レコードなし — 未入力状態)
    ↓ [初回入力]
存在 (値あり)
    ↓ [値の更新]
存在 (新しい値)
    ↓ [プロパティ削除 or ページ除外 or ページ削除]
削除済み (CASCADE or アプリケーション削除)
```

### セレクト選択肢の削除フロー

```
選択肢削除要求
    ↓
1. property_values で該当 option_id を持つ行の text_value を NULL に更新
2. PropertyConfig から該当選択肢を除去
3. Property の config を更新
（すべてトランザクション内）
```
