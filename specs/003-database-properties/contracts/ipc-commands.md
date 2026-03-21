# IPC Commands Contract: 003-database-properties

**Date**: 2026-03-21

すべてのコマンドは Tauri IPC（`@tauri-apps/api/core#invoke`）経由で呼び出される。
レスポンスは `Result<T, CommandError>` 形式。エラーは `{kind, message}` オブジェクト
として Frontend に伝達される。

## Database Commands

### `create_database`

新しいデータベースを作成する。

| 方向 | 型 |
|------|-----|
| **Args** | `{ title: string }` |
| **Response** | `DatabaseDto` |
| **Errors** | `titleEmpty`, `titleTooLong`, `storage` |

### `list_databases`

すべてのデータベースを作成日時降順で取得する。

| 方向 | 型 |
|------|-----|
| **Args** | なし |
| **Response** | `DatabaseDto[]` |
| **Errors** | `storage` |

### `get_database`

指定 ID のデータベースを取得する。

| 方向 | 型 |
|------|-----|
| **Args** | `{ id: string }` |
| **Response** | `DatabaseDto` |
| **Errors** | `databaseNotFound`, `storage` |

### `update_database_title`

データベースのタイトルを変更する。

| 方向 | 型 |
|------|-----|
| **Args** | `{ id: string, title: string }` |
| **Response** | `DatabaseDto` |
| **Errors** | `titleEmpty`, `titleTooLong`, `databaseNotFound`, `storage` |

### `delete_database`

データベースを削除する。プロパティ・プロパティ値を削除し，ページの database_id を NULL 化。

| 方向 | 型 |
|------|-----|
| **Args** | `{ id: string }` |
| **Response** | `void` |
| **Errors** | `databaseNotFound`, `storage` |

---

## Property Commands

### `add_property`

データベースにプロパティを追加する。

| 方向 | 型 |
|------|-----|
| **Args** | `{ databaseId: string, name: string, propertyType: PropertyTypeDto, config?: PropertyConfigDto }` |
| **Response** | `PropertyDto` |
| **Errors** | `propertyNameEmpty`, `propertyNameTooLong`, `duplicatePropertyName`, `tooManyProperties`, `invalidConfig`, `databaseNotFound`, `storage` |

### `update_property_name`

プロパティ名を変更する。

| 方向 | 型 |
|------|-----|
| **Args** | `{ id: string, name: string }` |
| **Response** | `PropertyDto` |
| **Errors** | `propertyNameEmpty`, `propertyNameTooLong`, `duplicatePropertyName`, `propertyNotFound`, `storage` |

### `update_property_config`

プロパティの型固有設定を更新する（セレクト選択肢の追加・削除，日付モード変更）。

| 方向 | 型 |
|------|-----|
| **Args** | `{ id: string, config: PropertyConfigDto }` |
| **Response** | `PropertyDto` |
| **Errors** | `invalidConfig`, `tooManyOptions`, `optionValueEmpty`, `duplicateOptionValue`, `propertyNotFound`, `storage` |

### `reorder_properties`

プロパティの表示順を変更する。

| 方向 | 型 |
|------|-----|
| **Args** | `{ databaseId: string, propertyIds: string[] }` |
| **Response** | `PropertyDto[]` |
| **Errors** | `databaseNotFound`, `propertyNotFound`, `storage` |

### `delete_property`

プロパティを削除する。関連するすべてのプロパティ値も削除される。

| 方向 | 型 |
|------|-----|
| **Args** | `{ id: string }` |
| **Response** | `void` |
| **Errors** | `propertyNotFound`, `storage` |

---

## Property Value Commands

### `set_property_value`

ページのプロパティ値を設定する（upsert）。

| 方向 | 型 |
|------|-----|
| **Args** | `{ pageId: string, propertyId: string, value: PropertyValueInputDto }` |
| **Response** | `PropertyValueDto` |
| **Errors** | `invalidNumber`, `invalidSelectOption`, `typeMismatch`, `pageNotFound`, `propertyNotFound`, `storage` |

### `clear_property_value`

ページのプロパティ値をクリア（NULL に戻す）。

| 方向 | 型 |
|------|-----|
| **Args** | `{ pageId: string, propertyId: string }` |
| **Response** | `void` |
| **Errors** | `storage` |

---

## Table View Commands

### `get_table_data`

テーブルビュー表示に必要なデータを一括取得する。

| 方向 | 型 |
|------|-----|
| **Args** | `{ databaseId: string }` |
| **Response** | `TableDataDto` |
| **Errors** | `databaseNotFound`, `storage` |

### `add_page_to_database`

新規ページを作成してデータベースに追加する。

| 方向 | 型 |
|------|-----|
| **Args** | `{ databaseId: string, title: string }` |
| **Response** | `PageDto` |
| **Errors** | `titleEmpty`, `titleTooLong`, `databaseNotFound`, `storage` |

### `add_existing_page_to_database`

既存のスタンドアロンページをデータベースに追加する。

| 方向 | 型 |
|------|-----|
| **Args** | `{ databaseId: string, pageId: string }` |
| **Response** | `PageDto` |
| **Errors** | `pageNotFound`, `pageAlreadyInDatabase`, `databaseNotFound`, `storage` |

### `remove_page_from_database`

ページをデータベースから除外する（ページ自体は保持，プロパティ値を削除）。

| 方向 | 型 |
|------|-----|
| **Args** | `{ pageId: string }` |
| **Response** | `void` |
| **Errors** | `pageNotFound`, `storage` |

### `list_standalone_pages`

どのデータベースにも属していないページの一覧を取得する（「既存ページを追加」の候補用）。

| 方向 | 型 |
|------|-----|
| **Args** | なし |
| **Response** | `PageDto[]` |
| **Errors** | `storage` |

---

## DTO Definitions (TypeScript)

```typescript
// Database
interface DatabaseDto {
  id: string;
  title: string;
  createdAt: string;   // ISO 8601
  updatedAt: string;   // ISO 8601
}

// Property
type PropertyTypeDto = "text" | "number" | "date" | "select" | "checkbox";

interface SelectOptionDto {
  id: string;
  value: string;
}

interface PropertyConfigDto {
  mode?: "date" | "datetime";        // 日付型のみ
  options?: SelectOptionDto[];       // セレクト型のみ
}

interface PropertyDto {
  id: string;
  databaseId: string;
  name: string;
  propertyType: PropertyTypeDto;
  config: PropertyConfigDto | null;
  position: number;
  createdAt: string;
  updatedAt: string;
}

// Property Value
interface PropertyValueDto {
  id: string;
  pageId: string;
  propertyId: string;
  textValue: string | null;
  numberValue: number | null;
  dateValue: string | null;         // ISO 8601
  booleanValue: boolean | null;
  createdAt: string;
  updatedAt: string;
}

// Property Value Input (set_property_value 用)
type PropertyValueInputDto =
  | { type: "text"; value: string }
  | { type: "number"; value: number }
  | { type: "date"; value: string }         // ISO 8601
  | { type: "select"; optionId: string }
  | { type: "checkbox"; value: boolean };

// Table View
interface TableRowDto {
  page: PageDto;
  values: Record<string, PropertyValueDto>;  // key = propertyId
}

interface TableDataDto {
  database: DatabaseDto;
  properties: PropertyDto[];
  rows: TableRowDto[];
}
```

## Error Kind Extensions

既存の `CommandError` に以下の kind を追加:

| kind | 発生元 | 説明 |
|------|--------|------|
| `databaseNotFound` | DatabaseError::NotFound | 指定 ID のデータベースが存在しない |
| `propertyNameEmpty` | PropertyError::NameEmpty | プロパティ名が空 |
| `propertyNameTooLong` | PropertyError::NameTooLong | プロパティ名が上限超過 |
| `duplicatePropertyName` | PropertyError::DuplicateName | 同名プロパティが既に存在 |
| `tooManyProperties` | PropertyError::TooManyProperties | プロパティ数上限（50）超過 |
| `propertyNotFound` | PropertyError::NotFound | 指定 ID のプロパティが存在しない |
| `invalidConfig` | PropertyError::InvalidConfig | 型固有設定が不正 |
| `tooManyOptions` | PropertyError::TooManyOptions | セレクト選択肢上限（100）超過 |
| `optionValueEmpty` | PropertyError::OptionValueEmpty | 選択肢の値が空 |
| `duplicateOptionValue` | PropertyError::DuplicateOptionValue | 選択肢の値が重複 |
| `invalidNumber` | PropertyValueError::InvalidNumber | 数値が NaN/Infinity |
| `invalidSelectOption` | PropertyValueError::InvalidSelectOption | 存在しない選択肢 |
| `typeMismatch` | PropertyValueError::TypeMismatch | プロパティ型と値型の不一致 |
| `propertyValueNotFound` | PropertyValueError::NotFound | プロパティ値が存在しない |
| `pageAlreadyInDatabase` | PageError（拡張） | ページが既にデータベースに所属 |
