# IPC Commands: Page Tree Navigation

**Feature**: 005-page-tree-nav | **Date**: 2026-03-22

## New Commands

### `list_sidebar_items`

サイドバー表示に必要なすべてのアイテム（スタンドアロンページ + データベース + DB 所属ページ）を
一括取得する。

```
Input:  (none)
Output: SidebarItemDto[]

SidebarItemDto {
  id: string
  title: string
  itemType: "page" | "database"
  parentId: string | null      // ページの親ページ ID（ルート or DB所属 = null）
  databaseId: string | null    // DB所属ページの場合のみ
  createdAt: string            // RFC 3339
}
```

**動作**:
1. 全スタンドアロンページ（`database_id IS NULL`）を取得し `itemType: "page"` で返す
2. 全データベースを取得し `itemType: "database"` で返す
3. 全DB所属ページ（`database_id IS NOT NULL`）を取得し `itemType: "page"`, `databaseId` 付きで返す
4. ソート順はフロントエンドに委任（`createdAt` を返すのみ）

**エラー**: `CommandError`（DB接続失敗等）

---

### `create_child_page`

既存ページの子としてページを作成する。

```
Input: {
  parentId: string    // 親ページの ID
  title: string       // ページタイトル
}
Output: PageDto
```

**動作**:
1. `parentId` のページを取得（`NotFound` チェック）
2. 親が DB 所属ページでないことを確認（`DatabasePageCannotNest`）
3. `PageHierarchyService::depth()` で親の深度を計算し，`validate_create_child()` で深度上限を検証（`MaxDepthExceeded` if depth >= MAX_DEPTH） <!-- refined by checklist-apply: P-03 -->
4. 新規ページを `parent_id = parentId` で作成
5. `PageDto` を返す

**エラー**:
- `PageError::NotFound` — 親ページが存在しない（処理中に並行して削除された場合，FK 制約違反を `NotFound` に変換） <!-- refined by checklist-apply: P-10 -->
- `PageError::DatabasePageCannotNest` — 親が DB 所属ページ
- `PageError::MaxDepthExceeded` — 深度上限超過
- `PageError::TitleEmpty` / `PageError::TitleTooLong` — タイトルバリデーション

---

### `move_page`

ページの親を変更する（別の親の下に移動，またはルートに昇格）。

```
Input: {
  pageId: string              // 移動対象ページの ID
  newParentId: string | null  // 新しい親の ID（null = ルートに昇格）
}
Output: PageDto
```

**動作**:
1. `pageId` のページを取得（`NotFound` チェック）
2. ページが DB 所属でないことを確認（`DatabasePageCannotNest`）
3. `newParentId` が指定された場合:
   a. 新しい親ページを取得（`NotFound` チェック）
   b. 新しい親が DB 所属でないことを確認
   c. 循環参照チェック: `newParentId` の祖先に `pageId` が含まれないことを確認
   d. 深度チェック: 移動後の最大深度が MAX_DEPTH 以内
   `newParentId` が `null` の場合: 手順2（DB ページチェック）のみ適用し，`parent_id` を `NULL` に更新（ルートに昇格）。循環参照チェック・深度チェックはスキップする <!-- added by checklist-apply: P-07 -->
4. `parent_id` を更新
5. 更新後の `PageDto` を返す

**トランザクション**: 手順1-4は単一トランザクション内で実行する（validate → update のアトミック性保証）。 <!-- added by checklist-apply: P-01 -->

**エラー**:
- `PageError::NotFound` — ページ/親ページが存在しない
- `PageError::CircularReference` — 循環参照
- `PageError::MaxDepthExceeded` — 深度上限超過
- `PageError::DatabasePageCannotNest` — DB 所属ページの移動

---

## Modified Commands

### `create_page`（変更）

ルートレベルのスタンドアロンページを作成する（既存動作を維持）。
`parent_id = NULL`, `sort_order = 0` で作成される。

```
Input: { title: string }
Output: PageDto    // parentId: null, sortOrder: 0 が追加
```

**変更点**: レスポンスの `PageDto` に `parentId`, `sortOrder` フィールドが追加される。

---

### `get_page`（変更）

レスポンスの `PageDto` に `parentId`, `sortOrder` が追加される。

---

### `list_pages`（変更）

レスポンスの `PageDto[]` の各要素に `parentId`, `sortOrder` が追加される。

---

### `update_page_title`（変更）

レスポンスの `PageDto` に `parentId`, `sortOrder` が追加される。

---

### `save_editor`（変更）

レスポンスの `EditorStateDto` から `isDirty` フィールドを削除。

```
Output: EditorStateDto {
  pageId: string
  blocks: BlockDto[]
  // isDirty は削除
}
```

---

### `open_editor`（変更）

レスポンスの `EditorStateDto` から `isDirty` フィールドを削除。

---

### `delete_page`（変更: 子ページ昇格ロジック追加）

ページを削除し，子ページを削除された親の親（またはルート）に昇格させる。
コマンド名は `delete_page` のまま維持する。

```
Input: {
  pageId: string    // 削除対象ページの ID
}
Output: ()
```

**動作**:
1. `pageId` のページを取得（`NotFound` チェック）
2. 削除対象の `parent_id` を取得（昇格先）
3. トランザクション内で:
   a. 子ページの `parent_id` を削除対象の `parent_id` に一括更新
   b. 対象ページを削除（blocks は CASCADE で削除）
4. void 返却

**エラー**:
- `PageError::NotFound` — ページが存在しない

**注**: 子ページが存在しない場合は既存の動作と同一。

---

## Error Mapping

新規エラーバリアントの IPC シリアライゼーション:

| Domain Error | CommandError kind | message |
|-------------|-------------------|---------|
| `CircularReference` | `"circularReference"` | `"Circular reference detected: page {page_id} cannot be moved under {target_parent_id}"` |
| `MaxDepthExceeded` | `"maxDepthExceeded"` | `"Maximum nesting depth ({max_depth}) exceeded for page {page_id}"` |
| `DatabasePageCannotNest` | `"databasePageCannotNest"` | `"Database page {page_id} cannot participate in page hierarchy"` |

---

## Command Summary

| Command | Type | Description |
|---------|------|-------------|
| `list_sidebar_items` | **New** | サイドバー全アイテム一括取得 |
| `create_child_page` | **New** | 子ページ作成 |
| `move_page` | **New** | ページ移動（親変更/ルート昇格） |
| `create_page` | Modified | PageDto に parentId, sortOrder 追加 |
| `get_page` | Modified | PageDto に parentId, sortOrder 追加 |
| `list_pages` | Modified | PageDto に parentId, sortOrder 追加 |
| `update_page_title` | Modified | PageDto に parentId, sortOrder 追加 |
| `delete_page` | Modified | 子ページ昇格ロジック追加 |
| `save_editor` | Modified | EditorStateDto から isDirty 削除 |
| `open_editor` | Modified | EditorStateDto から isDirty 削除 |
