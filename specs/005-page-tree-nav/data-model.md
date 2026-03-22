# Data Model: Page Tree Navigation

**Feature**: 005-page-tree-nav | **Date**: 2026-03-22

## Schema Changes

### Migration 0007: `pages` テーブル拡張

```sql
-- 0007_add_page_hierarchy.sql

-- ページ階層: 自己参照外部キー
ALTER TABLE pages ADD COLUMN parent_id TEXT REFERENCES pages(id) ON DELETE SET NULL;

-- 将来の手動並べ替え用（本スコープでは使用せず DEFAULT 0 のまま）
ALTER TABLE pages ADD COLUMN sort_order INTEGER NOT NULL DEFAULT 0;

-- 子ページ検索の高速化
CREATE INDEX idx_pages_parent_id ON pages(parent_id);
```

**ON DELETE SET NULL の理由**: 親ページ削除時に子の `parent_id` が NULL になる。
アプリケーション層のトランザクション内で，削除前に子ページの `parent_id` を
祖父母（削除される親の `parent_id`）に更新してから親を削除する。
DB レベルの SET NULL はフェイルセーフとして機能し，アプリクラッシュ時でも
孤児ページ（parent_id が存在しない ID を指す）を防ぐ。

### 結果スキーマ: `pages`

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | TEXT | PRIMARY KEY | UUIDv7 |
| title | TEXT | NOT NULL | ページタイトル (1-255文字) |
| database_id | TEXT | FK → databases(id) ON DELETE SET NULL | DB 所属ページの場合 |
| **parent_id** | TEXT | FK → pages(id) ON DELETE SET NULL | **新規**: 親ページ ID |
| **sort_order** | INTEGER | NOT NULL DEFAULT 0 | **新規**: 将来の並べ替え用 |
| created_at | TEXT | NOT NULL | RFC 3339 タイムスタンプ |
| updated_at | TEXT | NOT NULL | RFC 3339 タイムスタンプ |

**インデックス**:
- `idx_pages_created_at` (既存)
- `idx_pages_parent_id` (新規)

### 不変条件 (Invariants)

1. `database_id IS NOT NULL` → `parent_id IS NULL`（DB 所属ページは階層不参加）
2. `parent_id` が指すページは `database_id IS NULL`（DB 所属ページを親にできない）
3. `parent_id` の祖先チェーンに自身の `id` が含まれない（循環参照禁止）
4. 自身から根までの深度 ≤ 5（最大ネスト深度）
5. `parent_id = id` は禁止（自己参照）

---

## Domain Entities

### Page（拡張）

```rust
// src-tauri/src/domain/page/entity.rs

pub struct Page {
    id: PageId,
    title: PageTitle,
    database_id: Option<DatabaseId>,
    parent_id: Option<PageId>,        // 新規
    sort_order: i64,                  // 新規
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Page {
    /// Create a new standalone page at root level.
    pub fn new(title: PageTitle) -> Self { ... }

    /// Create a new standalone page as a child of the given parent.
    pub fn new_child(title: PageTitle, parent_id: PageId) -> Self { ... }

    /// Reconstruct from stored data.
    pub fn from_stored(
        id: PageId,
        title: PageTitle,
        database_id: Option<DatabaseId>,
        parent_id: Option<PageId>,
        sort_order: i64,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self { ... }

    // 既存のゲッターに加え:
    pub fn parent_id(&self) -> Option<&PageId> { ... }
    pub fn sort_order(&self) -> i64 { ... }
    pub fn is_standalone(&self) -> bool { self.database_id.is_none() }
    pub fn is_database_page(&self) -> bool { self.database_id.is_some() }
}
```

### PageHierarchyService（新規ドメインサービス）

```rust
// src-tauri/src/domain/page/hierarchy.rs

/// Maximum nesting depth for page hierarchy.
pub const MAX_DEPTH: usize = 5;

/// Domain service for page hierarchy operations.
///
/// Validates hierarchy constraints (circular reference, depth limit,
/// database page restriction) without depending on infrastructure.
pub struct PageHierarchyService;

impl PageHierarchyService {
    /// Validate that moving `page_id` under `new_parent_id` is safe.
    ///
    /// # Errors
    ///
    /// - [`PageError::CircularReference`] if `new_parent_id` is a descendant of `page_id`
    /// - [`PageError::MaxDepthExceeded`] if the resulting depth exceeds [`MAX_DEPTH`]
    /// - [`PageError::DatabasePageCannotNest`] if either page is a database page
    pub fn validate_move(
        page: &Page,
        new_parent_id: Option<&PageId>,
        ancestors_of_target: &[PageId],  // ターゲット親からルートまでの祖先リスト
        max_descendant_depth: usize,      // 移動対象ページの最深子孫までの深度
    ) -> Result<(), PageError> { ... }

    /// Validate that creating a child page under `parent_id` is safe.
    ///
    /// # Errors
    ///
    /// - [`PageError::MaxDepthExceeded`] if parent is already at max depth
    /// - [`PageError::DatabasePageCannotNest`] if parent is a database page
    pub fn validate_create_child(
        parent: &Page,
        parent_depth: usize,
    ) -> Result<(), PageError> { ... }

    /// Build a flat list of ancestor page IDs from the given page to root.
    pub fn ancestor_chain(
        page_id: &PageId,
        pages: &[Page],
    ) -> Vec<PageId> { ... }

    /// Calculate the depth of a page (root = 1).
    pub fn depth(
        page_id: &PageId,
        pages: &[Page],
    ) -> usize { ... }

    /// Calculate the maximum depth of descendants from a given page.
    pub fn max_descendant_depth(
        page_id: &PageId,
        pages: &[Page],
    ) -> usize { ... }
}
```

### PageError（拡張）

```rust
// src-tauri/src/domain/page/error.rs

#[derive(Debug, thiserror::Error)]
pub enum PageError {
    // 既存バリアント
    #[error("Page title cannot be empty")]
    TitleEmpty,
    #[error("Page title exceeds maximum length of 255 characters")]
    TitleTooLong,
    #[error("Page not found: {0}")]
    NotFound(String),
    #[error("Page is already in a database")]
    AlreadyInDatabase,

    // 新規バリアント
    #[error("Circular reference detected: page {page_id} cannot be moved under {target_parent_id}")]
    CircularReference {
        page_id: String,
        target_parent_id: String,
    },
    #[error("Maximum nesting depth ({max_depth}) exceeded for page {page_id}")]
    MaxDepthExceeded {
        page_id: String,
        current_depth: usize,
        max_depth: usize,
    },
    #[error("Database page {page_id} cannot participate in page hierarchy")]
    DatabasePageCannotNest {
        page_id: String,
    },
}
```

---

## Repository Changes

### PageRepository トレイト（拡張）

```rust
// src-tauri/src/domain/page/repository.rs — 追加メソッド

pub trait PageRepository {
    // 既存メソッド（変更なし）
    async fn create(&self, page: &Page) -> Result<(), Self::Error>;
    async fn find_by_id(&self, id: &PageId) -> Result<Page, Self::Error>;
    async fn find_all(&self) -> Result<Vec<Page>, Self::Error>;
    async fn update_title(&self, id: &PageId, title: &PageTitle) -> Result<Page, Self::Error>;
    async fn delete(&self, id: &PageId) -> Result<(), Self::Error>;
    async fn set_database_id(&self, page_id: &PageId, database_id: Option<&DatabaseId>) -> Result<(), Self::Error>;
    async fn find_standalone_pages(&self) -> Result<Vec<Page>, Self::Error>;
    async fn find_by_database_id(&self, database_id: &DatabaseId) -> Result<Vec<Page>, Self::Error>;

    // 新規メソッド
    /// Update the parent_id of a page (move operation).
    async fn update_parent_id(
        &self,
        page_id: &PageId,
        parent_id: Option<&PageId>,
    ) -> Result<Page, Self::Error>;

    /// Find all children of a given parent page.
    async fn find_children(&self, parent_id: &PageId) -> Result<Vec<Page>, Self::Error>;

    /// Find root-level standalone pages (parent_id IS NULL, database_id IS NULL).
    async fn find_root_pages(&self) -> Result<Vec<Page>, Self::Error>;

    /// Get ancestor chain from page to root using recursive CTE.
    async fn find_ancestors(&self, page_id: &PageId) -> Result<Vec<Page>, Self::Error>;

    /// Bulk update parent_id for multiple pages (used in parent deletion).
    async fn bulk_update_parent_id(
        &self,
        page_ids: &[PageId],
        new_parent_id: Option<&PageId>,
    ) -> Result<(), Self::Error>;
}
```

### Recursive CTE クエリ例

```sql
-- 祖先チェーン取得（循環参照検出用）
WITH RECURSIVE ancestors AS (
    SELECT id, parent_id, 1 AS depth
    FROM pages
    WHERE id = ?1
    UNION ALL
    SELECT p.id, p.parent_id, a.depth + 1
    FROM pages p
    INNER JOIN ancestors a ON p.id = a.parent_id
    WHERE a.depth < 10  -- 安全上限
)
SELECT id, parent_id, depth FROM ancestors;
```

---

## IPC DTOs

### PageDto（拡張）

```rust
// src-tauri/src/ipc/dto.rs

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageDto {
    pub id: String,
    pub title: String,
    pub database_id: Option<String>,
    pub parent_id: Option<String>,     // 新規
    pub sort_order: i64,               // 新規
    pub created_at: String,
    pub updated_at: String,
}
```

### SidebarItemDto（新規）

```rust
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SidebarItemDto {
    pub id: String,
    pub title: String,
    pub item_type: SidebarItemType,    // "page" | "database"
    pub parent_id: Option<String>,     // ページの親（DB所属ページは None）
    pub database_id: Option<String>,   // DB所属ページの場合のみ
    pub created_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SidebarItemType {
    Page,
    Database,
}
```

### EditorStateDto（変更）

```rust
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorStateDto {
    pub page_id: String,
    pub blocks: Vec<BlockDto>,
    // is_dirty フィールドを削除（フロントエンドで不使用）
}
```

---

## Frontend Types

### Sidebar State (localStorage)

```typescript
// localStorage keys
const SIDEBAR_VISIBLE_KEY = "sidebar-visible";       // boolean
const SIDEBAR_EXPANDED_KEY = "sidebar-expanded";     // Record<string, boolean>
const LAST_OPENED_ITEM_KEY = "last-opened-item";     // { id: string, type: "page" | "database" }

// ツリー展開/折りたたみ状態
// key: ページ/データベースの ID
// value: true = 展開, false / 未設定 = 折りたたみ
type ExpandedState = Record<string, boolean>;
```

### SidebarTreeNode (フロントエンド内部)

```typescript
// フラットな SidebarItemDto[] からツリー構造を構築
type SidebarTreeNode = {
  id: string;
  title: string;
  itemType: "page" | "database";
  parentId: string | null;
  databaseId: string | null;
  createdAt: string;
  children: SidebarTreeNode[];  // ビルド時にフロントエンドで付与
};
```

---

## Entity Relationship Summary

```
Database (1) ──── (0..1) View
    │
    │ database_id (FK)
    ▼
Page (0..*) ──── database pages (cannot have parent_id or children)
    │
    │ parent_id (FK, self-referencing)
    ▼
Page (0..*) ──── standalone child pages (max depth 5)
    │
    │ page_id (FK)
    ▼
Block (0..*)
```

**Key Constraint**: `database_id IS NOT NULL` と `parent_id IS NOT NULL` は
同時に存在しない（DB 所属ページはページ階層に参加しない）。
