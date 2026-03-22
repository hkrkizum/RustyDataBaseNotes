# サイドバー sort_order 並び替え実装計画

## Context

現在のサイドバーは D&D による親子階層変更（reparent）のみ対応し、同一親内の並び替えは明示的にスキップされている（`SidebarTree.tsx:175`）。DB スキーマには `sort_order` カラムが存在するが、全ページ DEFAULT 0 で未使用。この計画では、同一親内のドラッグ＆ドロップ並び替えを有効化する。

## 設計方針

### ソート戦略

- **SQL**: `ORDER BY sort_order ASC, created_at DESC`（複合ソート）
- 全既存ページは sort_order=0 → `created_at DESC` フォールバックで現行動作を維持
- 手動並び替え時：兄弟全員に sort_order 1, 2, 3... を連番付与
- 新規ページは sort_order=0（デフォルト）→ 並び替え済みグループでは先頭に表示

### IPC 設計: 独立した `reorder_siblings` コマンド

既存の `reorder_properties` パターン（`property_commands.rs:257`）に倣い、並び替え後の兄弟 ID リストを受け取る専用コマンドを追加。`move_page` は変更しない（関心の分離）。

- **同一親並び替え**: `reorder_siblings(ordered_page_ids)` のみ
- **クロス親移動 + 位置指定**: `move_page` → `reorder_siblings` の2段階
- **make-child**: `move_page` のみ（既存通り、sort_order=0 で先頭に配置）

---

## 実装ステップ

### Step 1: Repository — `bulk_update_sort_order` メソッド追加

**`src-tauri/src/domain/page/repository.rs`**
- `bulk_update_sort_order(&self, ordered_page_ids: &[PageId]) -> Result<(), Self::Error>` を trait に追加

**`src-tauri/src/infrastructure/persistence/page_repository.rs`**
- 実装: IDs をイテレートし `UPDATE pages SET sort_order = ?, updated_at = ? WHERE id = ?` を index+1 で実行
- 既存の `bulk_update_parent_id`（同ファイル）と同パターン

### Step 2: Repository — ORDER BY 変更

**`src-tauri/src/infrastructure/persistence/page_repository.rs`**
- 以下5箇所の `ORDER BY created_at DESC` → `ORDER BY sort_order ASC, created_at DESC`:
  - `find_all`
  - `find_standalone_pages`
  - `find_by_database_id`
  - `find_children`
  - `find_root_pages`
- 変更後 `cargo make sqlx-prepare` で `.sqlx/` キャッシュ再生成

### Step 3: DTO — `SidebarItemDto` に `sort_order` 追加

**`src-tauri/src/ipc/dto.rs`** (`SidebarItemDto`)
- `pub sort_order: i64` フィールド追加

**`src-tauri/src/ipc/page_commands.rs`** (`build_sidebar_items`)
- Page 用: `sort_order: page.sort_order()`
- Database 用: `sort_order: 0`

### Step 4: IPC — `reorder_siblings` コマンド追加

**`src-tauri/src/ipc/page_commands.rs`**
```rust
#[tauri::command]
pub async fn reorder_siblings(
    state: State<'_, AppState>,
    ordered_page_ids: Vec<String>,
) -> Result<(), CommandError>
```
- ID を `PageId` にパース → `repo.bulk_update_sort_order()` を呼ぶ

**`src-tauri/src/lib.rs`**
- `generate_handler!` に `reorder_siblings` を追加（`move_page` の隣）

### Step 5: Rust テスト

**`page_repository.rs` テスト**
- `bulk_update_sort_order` が連番を正しく付与するか
- `find_children` が `sort_order ASC, created_at DESC` で返すか
- sort_order=0 の複数ページが `created_at DESC` で返るか（後方互換性）

**`page_commands.rs` テスト**
- `reorder_siblings` IPC の動作確認
- `list_sidebar_items` が `sort_order` を含むか

### Step 6: フロントエンド型 — `sortOrder` 追加

**`src/features/sidebar/types.ts`**
- `SidebarItem` に `sortOrder: number` 追加
- `SidebarTreeNode` に `sortOrder: number` 追加

### Step 7: ツリー構築 — ソートロジック変更

**`src/features/sidebar/useSidebar.ts`** (`buildTree` 関数)
- `sortDesc` コンパレータを複合ソートに変更:
  ```typescript
  const sortCompare = (a: SidebarTreeNode, b: SidebarTreeNode) => {
    if (a.sortOrder !== b.sortOrder) return a.sortOrder - b.sortOrder;
    return b.createdAt.localeCompare(a.createdAt);
  };
  ```

### Step 8: D&D ハンドラ — 同一親並び替え実装

**`src/features/sidebar/SidebarTree.tsx`**
- `SidebarTreeProps` に `onReorderSiblings` コールバック追加:
  ```typescript
  onReorderSiblings: (pageId: string, anchorId: string, position: "before" | "after", parentId: string | null) => void;
  ```
- `monitorForElements` の `reorder-above` / `reorder-below` ケース:
  - 同一親 → `onReorderSiblings(pageId, targetId, position, parentId)` を呼ぶ
  - クロス親 → 既存の `onMovePage` + 後続で `onReorderSiblings`

**`src/features/sidebar/Sidebar.tsx`**
- `handleReorderSiblings` コールバック追加:
  1. `items` から同一親の兄弟を抽出・ソート
  2. ドラッグ元を除去し、anchor の前/後に挿入
  3. 楽観更新（`setItems` で sortOrder を 1, 2, 3... に設定）
  4. `invoke("reorder_siblings", { orderedPageIds })` を呼ぶ
  5. エラー時はロールバック + `refreshItems`
- `handleMovePage` の クロス親+位置指定 バリアント追加:
  1. `move_page` IPC → 再フェッチ → `reorder_siblings` IPC

### Step 9: ドロップインジケーター

**`src/features/sidebar/SidebarItem.tsx`**
- `instruction?.type === "reorder-above"` → 上部に青ライン（`before:` pseudo-element）
- `instruction?.type === "reorder-below"` → 下部に青ライン（`after:` pseudo-element）

### Step 10: フロントエンドテスト

- `buildTree` が `sortOrder` でソートするか
- 同一親 D&D で `reorder_siblings` が呼ばれるか
- 楽観更新とロールバックのテスト

### Step 11: QA

```bash
cargo make sqlx-prepare
cargo make check-all
cargo make test-rs
cargo make test-ts
```

---

## 変更対象ファイル

| ファイル | 変更内容 |
|---------|---------|
| `src-tauri/src/domain/page/repository.rs` | `bulk_update_sort_order` メソッド追加 |
| `src-tauri/src/infrastructure/persistence/page_repository.rs` | 実装 + ORDER BY 変更 (5箇所) + テスト |
| `src-tauri/src/ipc/dto.rs` | `SidebarItemDto` に `sort_order` 追加 |
| `src-tauri/src/ipc/page_commands.rs` | `reorder_siblings` コマンド + `build_sidebar_items` 更新 + テスト |
| `src-tauri/src/lib.rs` | handler 登録 |
| `src/features/sidebar/types.ts` | `sortOrder` フィールド追加 |
| `src/features/sidebar/useSidebar.ts` | ソートコンパレータ変更 |
| `src/features/sidebar/SidebarTree.tsx` | 同一親並び替えハンドラ + props |
| `src/features/sidebar/Sidebar.tsx` | `handleReorderSiblings` + クロス親位置指定 |
| `src/features/sidebar/SidebarItem.tsx` | ドロップインジケーター（青ライン） |
| TS テストファイル | 並び替えテスト追加 |

## 検証方法

1. `cargo make test-rs` — Rust テスト通過
2. `cargo make test-ts` — TS テスト通過
3. `cargo make check-all` — 静的解析通過
4. 手動テスト: 同一親内でページを上下にドラッグし、順序が保存されること
5. 手動テスト: アプリ再起動後も順序が維持されること
6. 手動テスト: 新規ページ作成後、並び替え済みグループの先頭に表示されること
7. 手動テスト: クロス親ドラッグ時、ドロップ位置に正しく配置されること
