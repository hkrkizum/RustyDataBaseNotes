# Data Model: テーブルビュー操作拡張

**Feature**: 004-table-view-operations
**Date**: 2026-03-22

## Entity: View（ビュー）

View はデータベースの表示設定を保持する集約ルート。1 データベースにつき 1 ビュー（本スコープ）。
将来は 1:N に拡張し，ボード・ガントチャート等の複数ビュータイプを持つ想定。

### Fields

| Field | Type | Constraints | Description |
|---|---|---|---|
| id | ViewId (UUIDv7) | PK, NOT NULL | ビューの一意識別子 |
| database_id | DatabaseId (UUIDv7) | FK → databases(id), NOT NULL | 所属データベース |
| name | ViewName (String) | 1–100 文字, NOT NULL | ビュー名（デフォルト: "Table"） |
| view_type | ViewType (Enum) | NOT NULL | ビュータイプ（現在は Table のみ） |
| sort_conditions | Vec\<SortCondition\> | 最大 5 件 | ソート条件（優先順位順） |
| filter_conditions | Vec\<FilterCondition\> | 最大 20 件 | フィルタ条件（AND 結合） |
| group_condition | Option\<GroupCondition\> | 最大 1 件 | グルーピング条件 |
| collapsed_groups | HashSet\<String\> | — | 折りたたまれたグループ値の集合 |
| created_at | DateTime\<Utc\> | NOT NULL | 作成日時 |
| updated_at | DateTime\<Utc\> | NOT NULL | 更新日時 |

### Relationships

```
Database 1 ──── N View (本スコープでは 1:1)
    │
    └── N Property ←── View の conditions が property_id で参照
```

- View は Database に属する（`database_id` FK）
- Database 削除時に View も CASCADE 削除
- SortCondition / FilterCondition は Property を `property_id` で参照するが，
  FK 制約ではなくアプリケーション層で整合性を管理する（JSON 格納のため）

### Validation Rules

- `name`: 1〜100 文字，前後空白トリミング，空文字不可
- `sort_conditions`: 最大 5 件，同一 property_id の重複不可
- `filter_conditions`: 最大 20 件，同一 property_id の複数条件は許可
- `group_condition`: property_id が存在するプロパティを指す必要がある
- フィルタ演算子はプロパティ型と整合する必要がある（RQ-7 参照）

### State Transitions

```
Created (empty config) → Configured (sort/filter/group set) → Reset (empty config)
                              ↑                                      |
                              └──────────────────────────────────────┘
```

---

## Value Object: SortCondition（ソート条件）

| Field | Type | Description |
|---|---|---|
| property_id | PropertyId (UUIDv7) | ソート対象プロパティ |
| direction | SortDirection (Enum) | Ascending / Descending |

- Vec 内の位置が優先順位を表す（index 0 が最優先）
- 不変オブジェクト（変更は View の sort_conditions 全体を置き換え）

---

## Value Object: FilterCondition（フィルタ条件）

| Field | Type | Description |
|---|---|---|
| property_id | PropertyId (UUIDv7) | フィルタ対象プロパティ |
| operator | FilterOperator (Enum) | 演算子 |
| value | Option\<FilterValue\> | 比較値（IsEmpty/IsNotEmpty/IsChecked/IsUnchecked では None） |

### FilterOperator Enum

```rust
enum FilterOperator {
    // テキスト型
    Equals,
    NotEquals,
    Contains,
    NotContains,
    // 数値型
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    // 日付型
    Before,
    After,
    // セレクト型
    Is,
    IsNot,
    // チェックボックス型
    IsChecked,
    IsUnchecked,
    // 共通
    IsEmpty,
    IsNotEmpty,
}
```

### FilterValue Enum（型安全な比較値）

```rust
enum FilterValue {
    Text(String),
    Number(f64),
    Date(String),       // ISO 8601 形式
    SelectOption(String), // option value
}
```

### 演算子とプロパティ型の対応表

| PropertyType | 使用可能な演算子 |
|---|---|
| Text | Equals, NotEquals, Contains, NotContains, IsEmpty, IsNotEmpty |
| Number | Equals, NotEquals, GreaterThan, LessThan, GreaterOrEqual, LessOrEqual, IsEmpty, IsNotEmpty |
| Date | Equals, Before, After, IsEmpty, IsNotEmpty |
| Select | Is, IsNot, IsEmpty, IsNotEmpty |
| Checkbox | IsChecked, IsUnchecked |

---

## Value Object: GroupCondition（グルーピング条件）

| Field | Type | Description |
|---|---|---|
| property_id | PropertyId (UUIDv7) | グルーピング対象プロパティ |

- 同時に 1 つのプロパティのみグルーピング可能

### 型別グルーピングキーの粒度 <!-- added by checklist-apply: P-04 -->

| プロパティ型 | グループキー判定 | 備考 |
|---|---|---|
| テキスト | case-insensitive 完全一致 | フィルタの Equals と同じ扱い |
| 数値 | f64 の内部表現で同一判定 | 1.0 と 1.00 は同一グループ |
| 日付 (date) | 日単位で同一判定 | 年月日が一致すれば同一グループ |
| 日付 (datetime) | 分単位で同一判定 | 年月日時分が一致すれば同一グループ |
| セレクト | option value の完全一致 | 選択肢定義順でグループ表示 |
| チェックボックス | true / false の 2 グループ | null は存在しない |

---

## Enum: ViewType

```rust
enum ViewType {
    Table,
    // 将来: Board, GanttChart, Calendar, ...
}
```

---

## Enum: SortDirection

```rust
enum SortDirection {
    Ascending,
    Descending,
}
```

---

## SQLite Schema: views テーブル

```sql
CREATE TABLE IF NOT EXISTS views (
    id              TEXT    PRIMARY KEY NOT NULL,
    database_id     TEXT    NOT NULL REFERENCES databases(id) ON DELETE CASCADE,
    name            TEXT    NOT NULL DEFAULT 'Table',
    view_type       TEXT    NOT NULL DEFAULT 'table',
    sort_conditions TEXT    NOT NULL DEFAULT '[]',     -- JSON: Vec<SortCondition>
    filter_conditions TEXT  NOT NULL DEFAULT '[]',     -- JSON: Vec<FilterCondition>
    group_condition TEXT    DEFAULT NULL,               -- JSON: Option<GroupCondition>
    collapsed_groups TEXT   NOT NULL DEFAULT '[]',     -- JSON: Vec<String>
    created_at      TEXT    NOT NULL,
    updated_at      TEXT    NOT NULL
);

CREATE UNIQUE INDEX idx_views_database_id ON views(database_id); -- 1:1 不変条件を DB レベルで強制 <!-- refined by checklist-apply: P-02 -->
```

### Migration: 0006_create_views.sql

マイグレーションでは:
1. `views` テーブルを作成
2. 既存の全 `databases` に対してデフォルトビューを `INSERT ... SELECT` で一括生成

---

## 既存エンティティへの影響

### 変更なし
- **Database**: テーブル・エンティティともに変更なし
- **Page**: テーブル・エンティティともに変更なし
- **Property**: テーブル・エンティティともに変更なし
- **PropertyValue**: テーブル・エンティティともに変更なし
- **Block**: テーブル・エンティティともに変更なし

### 拡張
- **TableDataDto** (IPC): `view` フィールドを追加（現在のビュー設定）
- **TableDataDto** (IPC): `groups` フィールドを追加（グルーピング情報）

---

## プロパティ削除時の自動修復フロー

```
Property deleted
    ↓
ViewRepository::remove_property_references(property_id) 呼び出し
    ↓
全ビューの JSON を読み込み
    ↓
sort_conditions から property_id を参照する条件を除去
filter_conditions から property_id を参照する条件を除去
group_condition が property_id を参照する場合は None に設定
collapsed_groups をクリア（グルーピング解除の場合）
    ↓
更新した JSON を保存
```

この処理は Property 削除のトランザクション内で実行し，原子性を保証する。
