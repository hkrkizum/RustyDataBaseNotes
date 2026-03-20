# IPC Contract: Editor Commands

**Feature Branch**: `002-block-editor`
**Date**: 2026-03-21
**Boundary**: Rust backend (Tauri) ↔ TypeScript frontend (React)
**Protocol**: Tauri IPC (`invoke` / `#[tauri::command]`)

## Architecture

全ブロック操作はバックエンドの `EditorSession`（インメモリ状態）で処理される。
フロントエンドは IPC コマンドを呼び出し，返却された `EditorState` で画面を更新する。

```
Frontend (React)                    Backend (Rust)
                                    ┌─ AppState ──────────────────┐
  open_editor(pageId) ────────────► │ sessions: Mutex<HashMap<    │
  add_block(pageId) ──────────────► │   PageId, EditorSession     │
  edit_block_content(pageId, ...) ► │ >>                          │
  move_block_up(pageId, ...) ─────► │ EditorSession {             │
  move_block_down(pageId, ...) ───► │   blocks: Vec<Block>,       │
  remove_block(pageId, ...) ──────► │   is_dirty: bool,           │
  save_editor(pageId) ───────────► │ }                           │
  close_editor(pageId) ──────────► │                              │
                                    └─────────────────────────────┘
  ◄──── EditorState { blocks, isDirty }（各コマンドの返り値）
```

## Commands

### `open_editor`

指定ページのエディタセッションを開始する。DB からブロックをロードし，
EditorSession を作成して AppState に格納する。

| | Detail |
|---|---|
| **Direction** | Frontend → Backend |
| **Arguments** | `{ pageId: string }` |
| **Success** | `EditorState` — ロードされたブロック一覧と dirty 状態 |
| **Errors** | `storage` |

**Rust signature**:

```rust
#[tauri::command]
async fn open_editor(
    state: State<'_, AppState>,
    page_id: String,
) -> Result<EditorStateDto, CommandError>
```

**TypeScript usage**:

```typescript
const editorState = await invoke<EditorState>('open_editor', { pageId });
```

**備考**:
- ブロックが存在しないページでは `blocks: []`, `isDirty: false` を返す
- 既にセッションが開かれている場合は既存のセッションを返す（再ロードしない）
- ページの存在チェックは行わない（ページ一覧から遷移する設計のため）

---

### `add_block`

現在のセッションに空のテキストブロックを末尾に追加する。
UUIDv7 を即座に生成し，正式 ID を含む EditorState を返す。

| | Detail |
|---|---|
| **Direction** | Frontend → Backend |
| **Arguments** | `{ pageId: string }` |
| **Success** | `EditorState` — 追加後のブロック一覧 |
| **Errors** | `storage`（セッション未開始） |

**Rust signature**:

```rust
#[tauri::command]
async fn add_block(
    state: State<'_, AppState>,
    page_id: String,
) -> Result<EditorStateDto, CommandError>
```

**TypeScript usage**:

```typescript
const editorState = await invoke<EditorState>('add_block', { pageId });
```

**備考**:
- 追加されたブロック: `id` = UUIDv7, `content` = "", `block_type` = "text"
- `isDirty` は `true` になる
- フロントエンドは返却された `blocks` 配列の末尾要素にフォーカスを当てる

---

### `edit_block_content`

指定ブロックのテキスト内容を更新する。BlockContent のバリデーションを適用する。

| | Detail |
|---|---|
| **Direction** | Frontend → Backend |
| **Arguments** | `{ pageId: string, blockId: string, content: string }` |
| **Success** | `EditorState` — 更新後のブロック一覧 |
| **Errors** | `contentTooLong`, `blockNotFound`, `storage`（セッション未開始） |

**Rust signature**:

```rust
#[tauri::command]
async fn edit_block_content(
    state: State<'_, AppState>,
    page_id: String,
    block_id: String,
    content: String,
) -> Result<EditorStateDto, CommandError>
```

**TypeScript usage**:

```typescript
const editorState = await invoke<EditorState>('edit_block_content', {
  pageId,
  blockId,
  content: newContent,
});
```

**備考**:
- `content` が 10,000 文字超の場合は `contentTooLong` エラーを返す
- 空文字列は有効（FR-011）
- フロントエンドは `onBlur` 時にこのコマンドを呼び出す

---

### `move_block_up`

指定ブロックを 1 つ上に移動する（position を前のブロックと交換）。

| | Detail |
|---|---|
| **Direction** | Frontend → Backend |
| **Arguments** | `{ pageId: string, blockId: string }` |
| **Success** | `EditorState` — 移動後のブロック一覧 |
| **Errors** | `cannotMoveUp`, `blockNotFound`, `storage`（セッション未開始） |

**Rust signature**:

```rust
#[tauri::command]
async fn move_block_up(
    state: State<'_, AppState>,
    page_id: String,
    block_id: String,
) -> Result<EditorStateDto, CommandError>
```

**TypeScript usage**:

```typescript
const editorState = await invoke<EditorState>('move_block_up', { pageId, blockId });
```

**備考**:
- 先頭のブロック（position = 0）に対して呼び出すと `cannotMoveUp` エラー
- フロントエンドはエラーを受け取り，上移動ボタンを無効化する（US4-3）

---

### `move_block_down`

指定ブロックを 1 つ下に移動する（position を次のブロックと交換）。

| | Detail |
|---|---|
| **Direction** | Frontend → Backend |
| **Arguments** | `{ pageId: string, blockId: string }` |
| **Success** | `EditorState` — 移動後のブロック一覧 |
| **Errors** | `cannotMoveDown`, `blockNotFound`, `storage`（セッション未開始） |

**Rust signature**:

```rust
#[tauri::command]
async fn move_block_down(
    state: State<'_, AppState>,
    page_id: String,
    block_id: String,
) -> Result<EditorStateDto, CommandError>
```

**TypeScript usage**:

```typescript
const editorState = await invoke<EditorState>('move_block_down', { pageId, blockId });
```

**備考**:
- 末尾のブロックに対して呼び出すと `cannotMoveDown` エラー
- フロントエンドはエラーを受け取り，下移動ボタンを無効化する（US4-4）

---

### `remove_block`

指定ブロックをセッションから削除する。残りのブロックの position を振り直す。

| | Detail |
|---|---|
| **Direction** | Frontend → Backend |
| **Arguments** | `{ pageId: string, blockId: string }` |
| **Success** | `EditorState` — 削除後のブロック一覧 |
| **Errors** | `blockNotFound`, `storage`（セッション未開始） |

**Rust signature**:

```rust
#[tauri::command]
async fn remove_block(
    state: State<'_, AppState>,
    page_id: String,
    block_id: String,
) -> Result<EditorStateDto, CommandError>
```

**TypeScript usage**:

```typescript
const editorState = await invoke<EditorState>('remove_block', { pageId, blockId });
```

**備考**:
- 削除後，残りのブロックの position が 0 始まりで振り直される
- 最後のブロックを削除した場合，`blocks` は空配列になる

---

### `save_editor`

セッション内の全ブロックをトランザクションで一括永続化する。

| | Detail |
|---|---|
| **Direction** | Frontend → Backend |
| **Arguments** | `{ pageId: string }` |
| **Success** | `EditorState` — 保存後のブロック一覧（`isDirty: false`） |
| **Errors** | `storage` |

**Rust signature**:

```rust
#[tauri::command]
async fn save_editor(
    state: State<'_, AppState>,
    page_id: String,
) -> Result<EditorStateDto, CommandError>
```

**TypeScript usage**:

```typescript
const editorState = await invoke<EditorState>('save_editor', { pageId });
```

**バックエンド処理フロー**:
1. セッションから全ブロックを取得
2. `isDirty` が false の場合，そのまま現在の状態を返す（不要な書き込みを避ける，US6-5）
3. トランザクション開始
4. `DELETE FROM blocks WHERE page_id = ?`
5. 各ブロックを INSERT（EditorSession の状態をそのまま永続化）
6. `updated_at` は現在時刻に更新
7. トランザクション COMMIT
8. `EditorSession::mark_saved()` で `isDirty = false`
9. EditorState を返却

**エラー時の動作**:
- トランザクション失敗時はロールバック，EditorSession のインメモリ状態は保持される
- フロントエンドはエラー通知を表示し，再試行が可能（FR-014）

---

### `close_editor`

セッションをメモリから解放する。未保存の変更は破棄される。

| | Detail |
|---|---|
| **Direction** | Frontend → Backend |
| **Arguments** | `{ pageId: string }` |
| **Success** | `void` (Unit) |
| **Errors** | なし |

**Rust signature**:

```rust
#[tauri::command]
async fn close_editor(
    state: State<'_, AppState>,
    page_id: String,
) -> Result<(), CommandError>
```

**TypeScript usage**:

```typescript
await invoke('close_editor', { pageId });
```

**備考**:
- セッションが存在しない場合でもエラーにしない（冪等）
- 未保存の確認はフロントエンドが `isDirty` を見て事前に行う
- フロントエンドは `close_editor` 後にページ一覧に遷移する

## Shared Types

### EditorState (DTO)

全コマンドの返り値として使用されるエディタ状態。
フロントエンドはこの型をそのまま React の state にセットする。

**Rust**:

```rust
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorStateDto {
    pub page_id: String,
    pub blocks: Vec<BlockDto>,
    pub is_dirty: bool,
}
```

**TypeScript**:

```typescript
interface EditorState {
  pageId: string;
  blocks: Block[];
  isDirty: boolean;
}
```

### Block (DTO)

**Rust**:

```rust
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockDto {
    pub id: String,
    pub page_id: String,
    pub block_type: String,
    pub content: String,
    pub position: i64,
    pub created_at: String,  // ISO 8601
    pub updated_at: String,  // ISO 8601
}
```

**TypeScript**:

```typescript
interface Block {
  id: string;
  pageId: string;
  blockType: string;
  content: string;
  position: number;
  createdAt: string;
  updatedAt: string;
}
```

### CommandError — 拡張

既存の `CommandError` に `BlockError` の変換を追加。

**Rust**:

```rust
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error(transparent)]
    Page(#[from] PageError),
    #[error(transparent)]
    Block(#[from] BlockError),
    #[error(transparent)]
    Storage(#[from] StorageError),
}
```

**Serialize impl に追加するマッピング**:

| BlockError variant | kind | message |
|---|---|---|
| `ContentTooLong { len, max }` | `"contentTooLong"` | `"block content too long: {len} characters (max: {max})"` |
| `InvalidPosition { value }` | `"invalidPosition"` | `"invalid block position: {value}"` |
| `NotFound { id }` | `"blockNotFound"` | `"block not found: {id}"` |
| `CannotMoveUp { id }` | `"cannotMoveUp"` | `"cannot move block up: {id} is at the top"` |
| `CannotMoveDown { id }` | `"cannotMoveDown"` | `"cannot move block down: {id} is at the bottom"` |

**TypeScript**:

```typescript
interface CommandError {
  kind:
    | 'titleEmpty' | 'titleTooLong' | 'notFound'
    | 'contentTooLong' | 'invalidPosition'
    | 'blockNotFound' | 'cannotMoveUp' | 'cannotMoveDown'
    | 'storage';
  message: string;
}
```

## Naming Convention

| Concept | Rust (snake_case) | TypeScript (camelCase) | IPC command name |
|---|---|---|---|
| エディタ開始 | `open_editor` | `openEditor` | `open_editor` |
| ブロック追加 | `add_block` | `addBlock` | `add_block` |
| 内容編集 | `edit_block_content` | `editBlockContent` | `edit_block_content` |
| 上移動 | `move_block_up` | `moveBlockUp` | `move_block_up` |
| 下移動 | `move_block_down` | `moveBlockDown` | `move_block_down` |
| ブロック削除 | `remove_block` | `removeBlock` | `remove_block` |
| 保存 | `save_editor` | `saveEditor` | `save_editor` |
| エディタ終了 | `close_editor` | `closeEditor` | `close_editor` |

IPC コマンド名は Rust の関数名と一致する（Tauri のデフォルト動作）。

## State Management

### AppState — 拡張

```rust
pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub sessions: tokio::sync::Mutex<HashMap<PageId, EditorSession>>,
}
```

- `sessions` は `tokio::sync::Mutex` でラップ — async コマンド内でロック可能
- `HashMap<PageId, EditorSession>` で複数ページの同時編集に対応（将来）
- IPC コマンドは `sessions.lock().await` でセッションにアクセスし，操作完了後に自動アンロック
- ロック保持中は同期操作のみ（EditorSession のメソッドはすべて同期）
