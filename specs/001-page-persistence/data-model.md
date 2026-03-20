# Data Model: ページの永続化（最小縦断スライス）

**Feature Branch**: `001-page-persistence`
**Date**: 2026-03-21

## Entities

### Page（集約ルート）

ノートブックの基本単位。本スライスではメタデータのみを扱い，ブロックは後続機能で追加する。

| Field | Type (Rust) | Type (SQLite) | Type (TypeScript) | Constraints |
|---|---|---|---|---|
| `id` | `PageId(Uuid)` | `TEXT PRIMARY KEY` | `string` | UUIDv7，不変，自動生成 |
| `title` | `PageTitle(String)` | `TEXT NOT NULL` | `string` | 1〜255 文字 |
| `created_at` | `chrono::DateTime<Utc>` | `TEXT NOT NULL` | `string` (ISO 8601) | 自動付与，不変 |
| `updated_at` | `chrono::DateTime<Utc>` | `TEXT NOT NULL` | `string` (ISO 8601) | 作成時 = created_at，更新時に自動更新 |

### Value Objects

#### PageId

- UUIDv7 のニュータイプラッパー
- `Uuid::now_v7()` で生成（不可謬）
- SQLite には TEXT（36 文字ハイフン付き）で保存
- `Display`, `FromStr`, `Serialize`, `Deserialize` を実装
- `serde(transparent)` でシリアライズを内部 `Uuid` に委譲

#### PageTitle

- `String` のニュータイプラッパー
- バリデーションルール:
  - 空文字列は不可（trim 後に判定）
  - 最大 255 文字（Unicode 文字数 = `chars().count()`）
- `TryFrom<String>` で生成 — バリデーション失敗時は `PageError` を返す
- `Display`, `Serialize`, `Deserialize` を実装

## Relationships

```
Page (aggregate root)
└── [future] Block* (child entities, not in this slice)
```

本スライスでは Page は独立した集約であり，他のエンティティへの参照を持たない。

## State Transitions

```
                    ┌─────────────┐
    create_page()   │             │   update_title()
    ──────────────► │   Active    │ ◄──────────────
                    │             │ ────────────────►
                    └──────┬──────┘
                           │
                           │ delete_page()
                           ▼
                    ┌─────────────┐
                    │  Deleted    │  (物理削除 — レコード消滅)
                    │  (no record)│
                    └─────────────┘
```

- 作成: `PageId` と `created_at` は不変
- 更新: `title` のみ変更可能，`updated_at` を自動更新
- 削除: 物理削除（論理削除・ゴミ箱は後続機能）

## SQLite Schema

### Migration 0001_create_pages.sql

sqlx のマイグレーション規約に従い，`src-tauri/migrations/` 配下に配置する。

```sql
CREATE TABLE pages (
    id         TEXT PRIMARY KEY NOT NULL,
    title      TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_pages_created_at ON pages (created_at DESC);
```

**設計判断**:
- `id` は TEXT（UUIDv7 ハイフン付き文字列） — 辞書順 = 時系列順
- 日時は ISO 8601 TEXT — SQLite に専用日時型がないため，文字列比較で正しくソート可能
- `created_at DESC` のインデックス — デフォルトのソート順（新しい順）を高速化
- CHECK 制約は Rust ドメイン層で実施（SQLite の CHECK よりもエラーメッセージが制御しやすい）
- sqlx は `_sqlx_migrations` テーブルを自動作成し，適用済みマイグレーションをチェックサム付きで追跡する

## Validation Rules

| Rule | Layer | Error |
|---|---|---|
| タイトルが空（trim 後） | Domain (`PageTitle::try_from`) | `PageError::TitleEmpty` |
| タイトルが 255 文字超 | Domain (`PageTitle::try_from`) | `PageError::TitleTooLong { len, max: 255 }` |
| 指定 ID のページが存在しない | Repository | `PageError::NotFound { id }` |

## Error Types

### PageError（ドメイン層）

```
PageError
├── TitleEmpty                    — タイトルが空
├── TitleTooLong { len, max }     — タイトルが最大文字数超過
└── NotFound { id: PageId }       — 指定 ID のページが存在しない
```

### StorageError（インフラ層）

```
StorageError
├── Sqlx(sqlx::Error)             — SQLite / sqlx 操作エラー
├── Migration(sqlx::migrate::MigrateError) — マイグレーションエラー
└── DatabasePath(std::io::Error)  — DB ファイルパス関連エラー
```

### CommandError（IPC 境界層）

```
CommandError
├── Page(PageError)               — ドメインエラー（#[from]）
└── Storage(StorageError)         — インフラエラー（#[from]）
```

シリアライズ形式（フロントエンド向け）:
```json
{
  "kind": "titleEmpty" | "titleTooLong" | "notFound" | "storage",
  "message": "ユーザー向けエラーメッセージ"
}
```

## TypeScript 対応型

```typescript
interface Page {
  id: string;         // UUIDv7
  title: string;      // 1-255 chars
  createdAt: string;  // ISO 8601
  updatedAt: string;  // ISO 8601
}

interface CommandError {
  kind: 'titleEmpty' | 'titleTooLong' | 'notFound' | 'storage';
  message: string;
}

// IPC コマンド引数
interface CreatePageArgs {
  title: string;
}

interface UpdatePageTitleArgs {
  id: string;
  title: string;
}

interface DeletePageArgs {
  id: string;
}

interface GetPageArgs {
  id: string;
}
```
