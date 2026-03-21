# IPC Commands Contract: 003-database-properties

**Date**: 2026-03-21

すべてのコマンドは Tauri IPC（`@tauri-apps/api/core#invoke`）経由で呼び出される。
すべての IPC コマンドは同期的な Request→Response パターンで動作する
（ストリーミングやイベント通知は使用しない）。
<!-- added by checklist-apply: P-12 -->

レスポンスは `Result<T, CommandError>` 形式。エラーは `{kind, message}` オブジェクト
として Frontend に伝達される。

**シリアライズ規則**: Tauri IPC は Rust の serde シリアライズ結果をそのまま JSON
として送受信する。Rust の snake_case フィールド名は TS DTO では camelCase に変換する
（`#[serde(rename_all = "camelCase")]`）。TS 型はこの JSON 構造に対応する型として
定義する。
<!-- added by checklist-apply: P-11, P-15 -->

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

データベースにプロパティを追加する。`config` を省略した場合，`PropertyDto.config` は
`null` を返す（型固有のデフォルト config は自動生成しない）。`propertyType` と
`config` の型が不整合の場合（例: `propertyType: "text"` に `options` を含む config）は
`invalidConfig` エラーを返す。
<!-- refined by checklist-apply: P-05, P-18 -->

| 方向 | 型 |
|------|-----|
| **Args** | `{ databaseId: string, name: string, propertyType: PropertyTypeDto, config?: PropertyConfigDto }` |
| **Response** | `PropertyDto` |
| **Errors** | `propertyNameEmpty`, `propertyNameTooLong`, `duplicatePropertyName`, `tooManyProperties`, `invalidConfig`, `databaseNotFound`, `storage` |

### `list_properties`
<!-- added by speckit.analyze: C2 -->

指定データベースのプロパティ一覧を表示順（position ASC）で取得する。
Phase 4 以降でフロントエンドがプロパティを単独取得する際に使用する。

| 方向 | 型 |
|------|-----|
| **Args** | `{ databaseId: string }` |
| **Response** | `PropertyDto[]` |
| **Errors** | `databaseNotFound`, `storage` |

### `update_property_name`

プロパティ名を変更する。

| 方向 | 型 |
|------|-----|
| **Args** | `{ id: string, name: string }` |
| **Response** | `PropertyDto` |
| **Errors** | `propertyNameEmpty`, `propertyNameTooLong`, `duplicatePropertyName`, `propertyNotFound`, `storage` |

### `update_property_config`

プロパティの型固有設定を更新する（セレクト選択肢の追加・削除，日付モード変更）。
`propertyType` と異なる型の config を送信した場合は `invalidConfig` エラーを返す。
<!-- refined by checklist-apply: P-14 -->

| 方向 | 型 |
|------|-----|
| **Args** | `{ id: string, config: PropertyConfigDto }` |
| **Response** | `PropertyDto` |
| **Errors** | `invalidConfig`, `tooManyOptions`, `optionValueEmpty`, `duplicateOptionValue`, `propertyNotFound`, `storage` |

### `reorder_properties`

プロパティの表示順を変更する。`propertyIds` にはデータベース内の全プロパティ ID を
含む完全なリストを渡す必要がある。サブセット（一部のみ）の場合はエラーとする。
<!-- refined by checklist-apply: P-05 -->

| 方向 | 型 |
|------|-----|
| **Args** | `{ databaseId: string, propertyIds: string[] }` |
| **Response** | `PropertyDto[]` |
| **Errors** | `databaseNotFound`, `propertyNotFound`, `storage` |

### `delete_property`

プロパティを削除する。関連するすべてのプロパティ値も CASCADE で自動削除される。
削除件数はレスポンスに含まない（`void`）。
<!-- refined by checklist-apply: P-17 -->

| 方向 | 型 |
|------|-----|
| **Args** | `{ id: string }` |
| **Response** | `void` |
| **Errors** | `propertyNotFound`, `storage` |

---

## Property Value Commands

### `set_property_value`

ページのプロパティ値を設定する（upsert）。ページは対象プロパティのデータベースに
属している必要がある。属していない場合は `pageNotInDatabase` エラーを返す。
不正な日付文字列（RFC 3339 パース失敗）の場合は `invalidDate` エラーを返す。
<!-- refined by checklist-apply: P-07, P-10 -->

| 方向 | 型 |
|------|-----|
| **Args** | `{ pageId: string, propertyId: string, value: PropertyValueInputDto }` |
| **Response** | `PropertyValueDto` |
| **Errors** | `invalidNumber`, `invalidDate`, `invalidSelectOption`, `typeMismatch`, `pageNotInDatabase`, `pageNotFound`, `propertyNotFound`, `storage` |

### `clear_property_value`

ページのプロパティ値をクリア（NULL に戻す）。値が存在しない（未設定の）場合は
no-op（エラーなし）として正常終了する。
<!-- refined by checklist-apply: P-06 -->

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
スタンドアロンページ（`database_id` が NULL）に対して呼び出した場合は
冪等に成功する（no-op，エラーなし）。
<!-- refined by checklist-apply: P-08 -->

| 方向 | 型 |
|------|-----|
| **Args** | `{ pageId: string }` |
| **Response** | `void` |
| **Errors** | `pageNotFound`, `storage` |

### `list_standalone_pages`

どのデータベースにも属していないページの一覧を取得する（「既存ページを追加」の候補用）。
作成日時降順で返す。
<!-- refined by checklist-apply: P-09 -->

| 方向 | 型 |
|------|-----|
| **Args** | なし |
| **Response** | `PageDto[]` |
| **Errors** | `storage` |

---

## DTO Definitions (TypeScript)

```typescript
// Command Error（全コマンド共通のエラーレスポンス型）
// added by checklist-apply: P-01
interface CommandError {
  kind: string;       // エラー種別（camelCase，下記 Error Kind Extensions 参照）
  message: string;    // デバッグ用メッセージ（英語）。ユーザー向け表示文は Frontend が kind に基づいて生成する
}

// Database
interface DatabaseDto {
  id: string;
  title: string;
  createdAt: string;   // RFC 3339 / UTC
  updatedAt: string;   // RFC 3339 / UTC
}

// Property
type PropertyTypeDto = "text" | "number" | "date" | "select" | "checkbox";

interface SelectOptionDto {
  id: string;
  value: string;
}

// PropertyConfigDto — Rust 側の serde internally tagged enum（#[serde(tag = "type")]）と
// 同一のワイヤーフォーマットを使用する判別共用体（discriminated union）。
// IPC ワイヤーフォーマット例:
//   Text 型:       {"type": "Text"}
//   Number 型:     {"type": "Number"}
//   Date 型:       {"type": "Date", "mode": "Date"}
//   Select 型:     {"type": "Select", "options": [{"id": "...", "value": "..."}]}
//   Checkbox 型:   {"type": "Checkbox"}
//
// ⚠️ ケーシング注意: PropertyTypeDto は lowercase（"text", "number" 等 — serde
//   rename_all = "camelCase" による）だが，PropertyConfigDto の tag は PascalCase
//   （"Text", "Number" 等 — serde internally tagged enum のデフォルト）。
//   同一 API コール内（例: add_property）で両形式が共存するため，
//   フロントエンド実装時は注意すること。
//   例: { propertyType: "select", config: { type: "Select", options: [...] } }
// refined by checklist-apply: P-02, P-03; refined by speckit.analyze: F1
type PropertyConfigDto =
  | { type: "Text" }
  | { type: "Number" }
  | { type: "Date"; mode: "Date" | "DateTime" }
  | { type: "Select"; options: SelectOptionDto[] }
  | { type: "Checkbox" };

interface PropertyDto {
  id: string;
  databaseId: string;
  name: string;
  propertyType: PropertyTypeDto;
  config: PropertyConfigDto | null;
  position: number;
  createdAt: string;               // RFC 3339 / UTC
  updatedAt: string;               // RFC 3339 / UTC
}
// refined by checklist-apply: P-04

// Property Value
interface PropertyValueDto {
  id: string;
  pageId: string;
  propertyId: string;
  textValue: string | null;
  numberValue: number | null;
  dateValue: string | null;         // RFC 3339 / UTC
  booleanValue: boolean | null;
  createdAt: string;                // RFC 3339 / UTC
  updatedAt: string;                // RFC 3339 / UTC
}

// Page（既存 PageDto の拡張 — databaseId フィールドを追加）
// 既存フィールド: id, title, createdAt, updatedAt
// 新規追加: databaseId
// added by checklist-apply: P-04, refined by checklist-apply: P-20
interface PageDto {
  id: string;
  title: string;
  databaseId: string | null;  // null = スタンドアロンページ（新規追加）
  createdAt: string;           // RFC 3339 / UTC
  updatedAt: string;           // RFC 3339 / UTC
}

// Property Value Input (set_property_value 用)
type PropertyValueInputDto =
  | { type: "text"; value: string }
  | { type: "number"; value: number }
  | { type: "date"; value: string }         // RFC 3339 / UTC
  | { type: "select"; optionId: string }
  | { type: "checkbox"; value: boolean };

// Table View
// values: 未入力（値未設定）のプロパティはキーが欠落する（Record に含まれない）。
// Frontend は properties 配列のキーで存在チェックし，欠落時は未入力として表示する。
// added by checklist-apply: P-06
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

既存の `CommandError` に以下の kind を追加する。エラー kind は **camelCase** で命名する。
`message` フィールドはデバッグ用の英語文字列とし，ユーザー向け表示文は Frontend が
`kind` に基づいて生成する。
<!-- refined by checklist-apply: P-13, P-16 -->

| kind | 発生元 | 説明 |
|------|--------|------|
| `databaseNotFound` | DatabaseError::NotFound | 指定 ID のデータベースが存在しない |
| `propertyNameEmpty` | PropertyError::NameEmpty | プロパティ名が空 |
| `propertyNameTooLong` | PropertyError::NameTooLong | プロパティ名が上限超過 |
| `duplicatePropertyName` | PropertyError::DuplicateName | 同名プロパティが既に存在 |
| `tooManyProperties` | PropertyError::TooManyProperties | プロパティ数上限（50）超過 |
| `propertyNotFound` | PropertyError::NotFound | 指定 ID のプロパティが存在しない |
| `invalidConfig` | PropertyError::InvalidConfig | 型固有設定が不正（型と config の不整合を含む） |
| `tooManyOptions` | PropertyError::TooManyOptions | セレクト選択肢上限（100）超過 |
| `optionValueEmpty` | PropertyError::OptionValueEmpty | 選択肢の値が空 |
| `duplicateOptionValue` | PropertyError::DuplicateOptionValue | 選択肢の値が重複 |
| `invalidNumber` | PropertyValueError::InvalidNumber | 数値が NaN/Infinity |
| `invalidDate` | PropertyValueError::InvalidDate | 日付文字列が RFC 3339 パース不可 |
| `invalidSelectOption` | PropertyValueError::InvalidSelectOption | 存在しない選択肢 |
| `typeMismatch` | PropertyValueError::TypeMismatch | プロパティ型と値型の不一致 |
| `pageNotInDatabase` | PropertyValueError::PageNotInDatabase | ページが対象プロパティのデータベースに属していない |
| `propertyValueNotFound` | PropertyValueError::NotFound | プロパティ値が存在しない |
| `pageAlreadyInDatabase` | PageError（拡張） | ページが既にデータベースに所属 |
<!-- added by checklist-apply: P-07 (pageNotInDatabase), P-10 (invalidDate) -->
