# Data Model: ブロックエディタ

**Feature Branch**: `002-block-editor`
**Date**: 2026-03-21

## Entities

### Page（集約ルート） — 既存，変更なし

001-page-persistence で定義済み。Block の親エンティティとして機能する。

| Field | Type (Rust) | Type (SQLite) | Type (TypeScript) | Constraints |
|---|---|---|---|---|
| `id` | `PageId(Uuid)` | `TEXT PRIMARY KEY` | `string` | UUIDv7，不変，自動生成 |
| `title` | `PageTitle(String)` | `TEXT NOT NULL` | `string` | 1〜255 文字 |
| `created_at` | `chrono::DateTime<Utc>` | `TEXT NOT NULL` | `string` (ISO 8601) | 自動付与，不変 |
| `updated_at` | `chrono::DateTime<Utc>` | `TEXT NOT NULL` | `string` (ISO 8601) | 作成時 = created_at，更新時に自動更新 |

### Block（エンティティ） — 新規

ページに属するテキストコンテンツの最小単位。Page の子要素として position 順に表示される。

| Field | Type (Rust) | Type (SQLite) | Type (TypeScript) | Constraints |
|---|---|---|---|---|
| `id` | `BlockId(Uuid)` | `TEXT PRIMARY KEY` | `string` | UUIDv7，`add_block()` 時にバックエンドで即座に生成 |
| `page_id` | `PageId(Uuid)` | `TEXT NOT NULL` | `string` | 親ページの ID，外部キー（CASCADE） |
| `block_type` | `String` | `TEXT NOT NULL DEFAULT 'text'` | `string` | ブロック種別，初期は 'text' のみ |
| `content` | `BlockContent(String)` | `TEXT NOT NULL DEFAULT ''` | `string` | 0〜10,000 文字 |
| `position` | `BlockPosition(i64)` | `INTEGER NOT NULL` | `number` | 表示順序，0 始まり連番，EditorSession が自動管理 |
| `created_at` | `chrono::DateTime<Utc>` | `TEXT NOT NULL` | `string` (ISO 8601) | `add_block()` 時に自動付与，不変 |
| `updated_at` | `chrono::DateTime<Utc>` | `TEXT NOT NULL` | `string` (ISO 8601) | 保存時に自動更新 |

### Value Objects

#### BlockId

- UUIDv7 のニュータイプラッパー（PageId と同パターン）
- `add_block()` 呼び出し時にバックエンドで `Uuid::now_v7()` で即座に生成
- 仮 ID は不要 — フロントエンドには常に正式 ID が返される
- SQLite には TEXT（36 文字ハイフン付き）で保存
- `Display`, `FromStr`, `Serialize`, `Deserialize`, `Hash`, `Eq`, `PartialEq` を実装
- `serde(transparent)` でシリアライズを内部 `Uuid` に委譲

#### BlockContent

- `String` のニュータイプラッパー
- バリデーションルール:
  - 空文字列を許容（FR-011: ユーザーが意図的に空行を作ることを許容）
  - 最大 10,000 文字（Unicode 文字数 = `chars().count()`）
- `TryFrom<String>` で生成 — バリデーション失敗時は `BlockError::ContentTooLong` を返す
- `Display`, `Serialize`, `Deserialize` を実装

#### BlockPosition

- `i64` のニュータイプラッパー
- 0 以上の整数値
- `TryFrom<i64>` で生成 — 負の値は `BlockError::InvalidPosition` を返す
- EditorSession が内部的に管理 — 外部から直接操作する場面は少ない
- `Display`, `Serialize`, `Deserialize`, `Ord`, `PartialOrd` を実装

## Domain Service

### EditorSession

ページ単位のブロック編集状態を管理するドメインサービス。DB に一切依存しない純粋なドメインロジック。

| Field | Type | Description |
|---|---|---|
| `page_id` | `PageId` | 編集中のページ ID |
| `blocks` | `Vec<Block>` | 現在のブロック配列（position 順） |
| `is_dirty` | `bool` | 未保存の変更があるか |

**コンストラクタ**:

```rust
/// Creates a new editor session with blocks loaded from persistence.
pub fn new(page_id: PageId, blocks: Vec<Block>) -> Self
```

**操作メソッド**:

| Method | Signature | Description |
|---|---|---|
| `add_block` | `(&mut self) -> &Block` | 末尾に空テキストブロックを追加。UUIDv7 を即座に生成。`is_dirty = true` |
| `edit_block_content` | `(&mut self, id: &BlockId, content: String) -> Result<(), BlockError>` | 指定ブロックの内容を更新。BlockContent バリデーション適用。`is_dirty = true` |
| `move_block_up` | `(&mut self, id: &BlockId) -> Result<(), BlockError>` | 指定ブロックを 1 つ上に移動。先頭の場合はエラー。`is_dirty = true` |
| `move_block_down` | `(&mut self, id: &BlockId) -> Result<(), BlockError>` | 指定ブロックを 1 つ下に移動。末尾の場合はエラー。`is_dirty = true` |
| `remove_block` | `(&mut self, id: &BlockId) -> Result<(), BlockError>` | 指定ブロックを削除し，position を振り直す。`is_dirty = true` |
| `blocks` | `(&self) -> &[Block]` | 現在のブロック一覧を position 順で返す |
| `is_dirty` | `(&self) -> bool` | 未保存の変更があるか |
| `mark_saved` | `(&mut self)` | 保存完了後に `is_dirty = false` に設定 |
| `page_id` | `(&self) -> &PageId` | 編集中のページ ID |

## Relationships

```
Page (aggregate root)
├── PageId, PageTitle (value objects)
└── Block* (child entities, 0..N)
    ├── BlockId, BlockContent, BlockPosition (value objects)
    └── FK: page_id → pages.id (ON DELETE CASCADE)

EditorSession (domain service, in-memory)
├── page_id: PageId
├── blocks: Vec<Block>
└── is_dirty: bool
```

- Page : Block = 1 : N（ページは 0 個以上のブロックを持つ）
- Block は必ず 1 つの Page に属する（page_id は NOT NULL）
- ページが削除されると，そのページに属する全ブロックが CASCADE で自動削除
- EditorSession はページごとに 1 つ存在し，`AppState` で管理される

## State Transitions

### EditorSession ライフサイクル

```
    open_editor(page_id)
    ──────────────────────►  EditorSession::new(page_id, blocks_from_db)
                             │
                             ▼
                      ┌─────────────┐
                      │    Clean    │  (is_dirty = false)
                      │   (loaded)  │
                      └──────┬──────┘
                             │
              ┌──────────────┼──────────────┐──────────────┐
              │              │              │              │
         add_block    edit_content     move_up/down   remove_block
              │              │              │              │
              ▼              ▼              ▼              ▼
                      ┌─────────────┐
                      │    Dirty    │  (is_dirty = true)
                      │  (modified) │ ◄─── さらなる操作
                      └──────┬──────┘
                             │
                  ┌──────────┴──────────┐
                  │                     │
            save_editor           close_editor
                  │                     │
                  ▼                     ▼
           ┌─────────────┐      ┌─────────────┐
           │    Clean    │      │   Dropped   │
           │   (saved)   │      │  (discarded)│
           └─────────────┘      └─────────────┘
```

### Block ライフサイクル（EditorSession 内）

```
    add_block()
    ──────────────────────►  Block::new(page_id)  [UUIDv7 即座に生成]
                             │
                             ▼
                      ┌─────────────┐
                      │    New      │  (メモリ内のみ)
                      └──────┬──────┘
                             │
              ┌──────────────┼──────────────┐
              │              │              │
         edit_content   move_up/down   remove_block
              │              │              │
              ▼              ▼              ▼
        ┌─────────────┐           ┌─────────────┐
        │   Modified  │           │   Removed   │
        └──────┬──────┘           └─────────────┘
               │
               │ save_editor()
               ▼
        ┌─────────────┐
        │  Persisted  │  (DB に保存)
        └─────────────┘
```

### save_editor バックエンド処理フロー

```
1. EditorSession から blocks() で全ブロックを取得
2. トランザクション開始
3. DELETE FROM blocks WHERE page_id = ?
4. 各ブロックを INSERT:
   - id: EditorSession が保持する正式 ID（UUIDv7）
   - page_id: セッションの page_id
   - block_type: 'text'
   - content: EditorSession が保持する内容
   - position: EditorSession が保持する position（0 始まり連番）
   - created_at: EditorSession が保持する created_at（不変）
   - updated_at: 現在時刻
5. トランザクション COMMIT
6. EditorSession::mark_saved()
7. EditorState を返却
```

## SQLite Schema

### Migration 0002_create_blocks.sql

sqlx のマイグレーション規約に従い，`src-tauri/migrations/` 配下に配置する。

```sql
CREATE TABLE blocks (
    id         TEXT PRIMARY KEY NOT NULL,
    page_id    TEXT NOT NULL,
    block_type TEXT NOT NULL DEFAULT 'text',
    content    TEXT NOT NULL DEFAULT '',
    position   INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (page_id) REFERENCES pages(id) ON DELETE CASCADE
);

CREATE INDEX idx_blocks_page_position ON blocks (page_id, position ASC);
```

**設計判断**:
- `id` は TEXT（UUIDv7 ハイフン付き文字列） — PageId と同パターン
- `page_id` は外部キー — CASCADE 削除でページ削除時にブロックも自動削除
- `block_type` は初期値 'text' — 将来のブロック種別追加時にマイグレーション不要
- `content` は空文字を許容（DEFAULT ''） — FR-011 準拠
- `position` は INTEGER — 0 始まりの連番，EditorSession が管理
- 複合インデックス `(page_id, position ASC)` — ページごとのブロック取得を高速化
- 日時は ISO 8601 TEXT — Page と同パターン
- SQLite では外部キー制約のために `PRAGMA foreign_keys = ON` が必要 — database.rs の初期化で設定

## Validation Rules

| Rule | Layer | Error |
|---|---|---|
| ブロック内容が 10,000 文字超 | Domain (`BlockContent::try_from`) | `BlockError::ContentTooLong { len, max: 10_000 }` |
| position が負の値 | Domain (`BlockPosition::try_from`) | `BlockError::InvalidPosition { value }` |
| 指定 ID のブロックが存在しない（操作時） | Domain (`EditorSession`) | `BlockError::NotFound { id }` |
| 先頭ブロックの上移動 | Domain (`EditorSession`) | `BlockError::CannotMoveUp { id }` |
| 末尾ブロックの下移動 | Domain (`EditorSession`) | `BlockError::CannotMoveDown { id }` |
| セッションが存在しない | IPC layer | `CommandError::Storage`（セッション未開始） |

## Error Types

### BlockError（ドメイン層）

```
BlockError
├── ContentTooLong { len, max }    — 内容が最大文字数超過
├── InvalidPosition { value }      — position が不正（負の値）
├── NotFound { id: BlockId }       — 指定 ID のブロックが存在しない
├── CannotMoveUp { id: BlockId }   — 先頭ブロックは上に移動できない
└── CannotMoveDown { id: BlockId } — 末尾ブロックは下に移動できない
```

### CommandError（IPC 境界層） — 既存を拡張

```
CommandError
├── Page(PageError)              — ページのドメインエラー（既存）
├── Block(BlockError)            — ブロックのドメインエラー（新規追加）
└── Storage(StorageError)        — インフラエラー（既存）
```

シリアライズ形式（フロントエンド向け）:

```json
{
  "kind": "contentTooLong" | "invalidPosition" | "blockNotFound" | "cannotMoveUp" | "cannotMoveDown" | "storage",
  "message": "ユーザー向けエラーメッセージ"
}
```

## TypeScript 対応型

```typescript
// バックエンドから返却されるエディタ状態
interface EditorState {
  pageId: string;
  blocks: Block[];
  isDirty: boolean;
}

// ブロック
interface Block {
  id: string;          // UUIDv7（常に正式 ID）
  pageId: string;      // 親ページ ID
  blockType: string;   // 'text'（初期スコープ）
  content: string;     // 0-10,000 chars
  position: number;    // 表示順序（0 始まり）
  createdAt: string;   // ISO 8601
  updatedAt: string;   // ISO 8601
}

// CommandError の kind を拡張
interface CommandError {
  kind:
    | 'titleEmpty' | 'titleTooLong' | 'notFound'    // 既存（Page）
    | 'contentTooLong' | 'invalidPosition'           // Block バリデーション
    | 'blockNotFound' | 'cannotMoveUp'               // Block 操作
    | 'cannotMoveDown'                               // Block 操作
    | 'storage';                                      // インフラ
  message: string;
}
```
