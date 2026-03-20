# IPC Contract: Page Commands

**Feature Branch**: `001-page-persistence`
**Date**: 2026-03-21
**Boundary**: Rust backend (Tauri) ↔ TypeScript frontend (React)
**Protocol**: Tauri IPC (`invoke` / `#[tauri::command]`)

## Commands

### `create_page`

新しいページを作成し永続化する。

| | Detail |
|---|---|
| **Direction** | Frontend → Backend |
| **Arguments** | `{ title: string }` |
| **Success** | `Page` — 作成されたページ（id, title, createdAt, updatedAt） |
| **Errors** | `titleEmpty`, `titleTooLong`, `storage` |

**Rust signature**:
```rust
#[tauri::command]
async fn create_page(state: State<'_, AppState>, title: String) -> Result<PageDto, CommandError>
```

**TypeScript usage**:
```typescript
const page = await invoke<Page>('create_page', { title });
```

---

### `list_pages`

保存済みのすべてのページを作成日時の降順で取得する。

| | Detail |
|---|---|
| **Direction** | Frontend → Backend |
| **Arguments** | なし |
| **Success** | `Page[]` — ページ一覧（created_at DESC） |
| **Errors** | `storage` |

**Rust signature**:
```rust
#[tauri::command]
async fn list_pages(state: State<'_, AppState>) -> Result<Vec<PageDto>, CommandError>
```

**TypeScript usage**:
```typescript
const pages = await invoke<Page[]>('list_pages');
```

---

### `get_page`

指定 ID のページを取得する。

| | Detail |
|---|---|
| **Direction** | Frontend → Backend |
| **Arguments** | `{ id: string }` |
| **Success** | `Page` — 該当ページ |
| **Errors** | `notFound`, `storage` |

**Rust signature**:
```rust
#[tauri::command]
async fn get_page(state: State<'_, AppState>, id: String) -> Result<PageDto, CommandError>
```

**TypeScript usage**:
```typescript
const page = await invoke<Page>('get_page', { id });
```

---

### `update_page_title`

既存ページのタイトルを更新する。`updated_at` は自動更新される。

| | Detail |
|---|---|
| **Direction** | Frontend → Backend |
| **Arguments** | `{ id: string, title: string }` |
| **Success** | `Page` — 更新後のページ |
| **Errors** | `notFound`, `titleEmpty`, `titleTooLong`, `storage` |

**Rust signature**:
```rust
#[tauri::command]
async fn update_page_title(state: State<'_, AppState>, id: String, title: String) -> Result<PageDto, CommandError>
```

**TypeScript usage**:
```typescript
const page = await invoke<Page>('update_page_title', { id, title });
```

---

### `delete_page`

指定ページを永続化層から物理削除する。

| | Detail |
|---|---|
| **Direction** | Frontend → Backend |
| **Arguments** | `{ id: string }` |
| **Success** | `void` (Unit) |
| **Errors** | `notFound`, `storage` |

**Rust signature**:
```rust
#[tauri::command]
async fn delete_page(state: State<'_, AppState>, id: String) -> Result<(), CommandError>
```

**TypeScript usage**:
```typescript
await invoke('delete_page', { id });
```

## Shared Types

### Page (DTO)

Rust と TypeScript の間で共有されるページ表現。
フィールド名は Rust 側で `#[serde(rename_all = "camelCase")]` によりキャメルケースに変換。

**Rust**:
```rust
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageDto {
    pub id: String,
    pub title: String,
    pub created_at: String,  // ISO 8601
    pub updated_at: String,  // ISO 8601
}
```

**TypeScript**:
```typescript
interface Page {
  id: string;
  title: string;
  createdAt: string;
  updatedAt: string;
}
```

### CommandError

**Rust serialization**:
```rust
// Serialize impl maps to:
{ "kind": "<error_kind>", "message": "<user-facing message>" }
```

**TypeScript**:
```typescript
interface CommandError {
  kind: 'titleEmpty' | 'titleTooLong' | 'notFound' | 'storage';
  message: string;
}
```

## State Management

### AppState

Tauri の `manage()` で登録するアプリケーション状態。

```rust
pub struct AppState {
    pub db: sqlx::SqlitePool,
}
```

- `SqlitePool` は Send + Sync + Clone — Mutex 不要
- `app.setup()` フック内で `tauri::async_runtime::block_on` を用いて初期化
  （プール作成 + WAL モード設定 + マイグレーション実行）
- コマンドハンドラは `State<'_, AppState>` で受け取り
- コマンドは `async fn` として定義（sqlx の非同期 API に対応）

## Naming Convention

| Concept | Rust (snake_case) | TypeScript (camelCase) | IPC command name |
|---|---|---|---|
| ページ作成 | `create_page` | `createPage` | `create_page` |
| ページ一覧 | `list_pages` | `listPages` | `list_pages` |
| ページ取得 | `get_page` | `getPage` | `get_page` |
| タイトル更新 | `update_page_title` | `updatePageTitle` | `update_page_title` |
| ページ削除 | `delete_page` | `deletePage` | `delete_page` |

IPC コマンド名は Rust の関数名と一致する（Tauri のデフォルト動作）。
