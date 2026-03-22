# IPC Contract: View Commands

**Feature**: 004-table-view-operations
**Date**: 2026-03-22

## 概要

テーブルビューのソート・フィルタ・グルーピング操作に関する Tauri IPC コマンド契約。
すべてのコマンドはフロントエンドから `invoke()` で呼び出され，バックエンドで処理される。

---

## 既存コマンドの拡張

### get_table_data（拡張）

テーブルデータ取得。デフォルトビューの設定（ソート・フィルタ・グルーピング）を自動適用する。

**Command**: `get_table_data`
**Args**: `{ databaseId: string }`
**Returns**: `TableDataDto`（拡張版）

```typescript
interface TableDataDto {
  database: DatabaseDto;
  properties: PropertyDto[];
  rows: TableRowDto[];             // フィルタ適用 → ソート適用済み
  view: ViewDto;                   // 追加: 現在のビュー設定
  groups: GroupInfoDto[] | null;   // 追加: グルーピング情報（null = グルーピングなし）
}

interface GroupInfoDto {
  value: string | null;           // グループ値（null = 未設定グループ）
  displayValue: string;           // 表示用ラベル（"未設定" 等）
  count: number;                  // グループ内の行数
  isCollapsed: boolean;           // 折りたたみ状態
}
```

**動作変更**:
- ビューが存在しない場合はデフォルト設定で生成して返却
- rows はフィルタ条件で絞り込み → ソート条件で並び替え済み
- グルーピング有効時: rows はグループ順 → グループ内ソート順で返却
- 折りたたまれたグループの行は rows に含まれない（フロントエンド描画不要のため）

---

## 新規コマンド

### get_view

ビュー設定のみを取得する。

**Command**: `get_view`
**Args**: `{ databaseId: string }`
**Returns**: `ViewDto`

```typescript
interface ViewDto {
  id: string;
  databaseId: string;
  name: string;
  viewType: "table";
  sortConditions: SortConditionDto[];
  filterConditions: FilterConditionDto[];
  groupCondition: GroupConditionDto | null;
  collapsedGroups: string[];
  createdAt: string;
  updatedAt: string;
}
```

---

### update_sort_conditions

ソート条件を一括更新する。

**Command**: `update_sort_conditions`
**Args**: `{ databaseId: string, conditions: SortConditionDto[] }`
**Returns**: `ViewDto`
**Errors**: `invalidSortCondition`, `tooManySortConditions`, `duplicateSortProperty`, `propertyNotFound`, `viewNotFound`

```typescript
interface SortConditionDto {
  propertyId: string;
  direction: "ascending" | "descending";
}
```

**Validation**:
- `conditions.length <= 5`
- 各 `propertyId` が当該データベースに存在すること
- 同一 `propertyId` の重複不可

---

### update_filter_conditions

フィルタ条件を一括更新する。

**Command**: `update_filter_conditions`
**Args**: `{ databaseId: string, conditions: FilterConditionDto[] }`
**Returns**: `ViewDto`
**Errors**: `invalidFilterOperator`, `invalidFilterValue`, `tooManyFilterConditions`, `propertyNotFound`, `viewNotFound`

```typescript
interface FilterConditionDto {
  propertyId: string;
  operator: FilterOperatorDto;
  value: FilterValueDto | null;     // IsEmpty/IsNotEmpty/IsChecked/IsUnchecked では null
}

type FilterOperatorDto =
  // テキスト + 共通
  | "equals" | "notEquals" | "contains" | "notContains"
  // 数値
  | "greaterThan" | "lessThan" | "greaterOrEqual" | "lessOrEqual"
  // 日付
  | "before" | "after"
  // セレクト
  | "is" | "isNot"
  // チェックボックス
  | "isChecked" | "isUnchecked"
  // 共通
  | "isEmpty" | "isNotEmpty";

type FilterValueDto =
  | { type: "text"; value: string }
  | { type: "number"; value: number }
  | { type: "date"; value: string }           // ISO 8601
  | { type: "selectOption"; value: string };  // option value
```

**Validation**:
- `conditions.length <= 20`
- 各 `propertyId` が当該データベースに存在すること
- `operator` がプロパティ型と整合すること（data-model.md の対応表参照）
- `value` の型が `operator` と整合すること
- IsEmpty / IsNotEmpty / IsChecked / IsUnchecked は `value: null` 必須

---

### update_group_condition

グルーピング条件を設定または解除する。

**Command**: `update_group_condition`
**Args**: `{ databaseId: string, condition: GroupConditionDto | null }`
**Returns**: `ViewDto`
**Errors**: `propertyNotFound`, `viewNotFound`

```typescript
interface GroupConditionDto {
  propertyId: string;
}
```

**動作**:
- `condition: null` でグルーピング解除
- グルーピングプロパティ変更時に `collapsed_groups` をクリア

---

### toggle_group_collapsed

グループの折りたたみ状態を切り替える。

**Command**: `toggle_group_collapsed`
**Args**: `{ databaseId: string, groupValue: string | null }`
**Returns**: `ViewDto`
**Errors**: `viewNotFound`, `noGroupCondition`

**動作**:
- `groupValue` が `collapsed_groups` に含まれている場合は除去（展開）
- 含まれていない場合は追加（折りたたみ）
- `groupValue: null` は「未設定」グループに対応

---

### reset_view

ビュー設定をデフォルト状態にリセットする。

**Command**: `reset_view`
**Args**: `{ databaseId: string }`
**Returns**: `ViewDto`

**動作**:
- sort_conditions → 空配列
- filter_conditions → 空配列
- group_condition → null
- collapsed_groups → 空配列
- name, view_type は維持

---

## エラー型

```typescript
// CommandError の kind に追加される値
type ViewErrorKind =
  | "viewNotFound"              // 指定データベースのビューが存在しない
  | "invalidSortCondition"      // ソート条件のバリデーションエラー
  | "tooManySortConditions"     // ソート条件が 5 件を超過
  | "invalidFilterOperator"     // フィルタ演算子がプロパティ型と不整合
  | "invalidFilterValue"        // フィルタ比較値の型エラー
  | "tooManyFilterConditions"   // フィルタ条件が 20 件を超過
  | "propertyNotFound"          // 指定プロパティが存在しない
  | "noGroupCondition"          // グルーピング未設定状態で折りたたみ操作
  | "duplicateSortProperty";    // 同一プロパティのソート条件重複
```

---

## フロントエンド TypeScript 型まとめ

```typescript
// src/features/database/types.ts に追加

interface ViewDto {
  id: string;
  databaseId: string;
  name: string;
  viewType: "table";
  sortConditions: SortConditionDto[];
  filterConditions: FilterConditionDto[];
  groupCondition: GroupConditionDto | null;
  collapsedGroups: string[];
  createdAt: string;
  updatedAt: string;
}

interface SortConditionDto {
  propertyId: string;
  direction: "ascending" | "descending";
}

interface FilterConditionDto {
  propertyId: string;
  operator: FilterOperatorDto;
  value: FilterValueDto | null;
}

interface GroupConditionDto {
  propertyId: string;
}

interface GroupInfoDto {
  value: string | null;
  displayValue: string;
  count: number;
  isCollapsed: boolean;
}

// TableDataDto の拡張
interface TableDataDto {
  database: DatabaseDto;
  properties: PropertyDto[];
  rows: TableRowDto[];
  view: ViewDto;
  groups: GroupInfoDto[] | null;
}
```
