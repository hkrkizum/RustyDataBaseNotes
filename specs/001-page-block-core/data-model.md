# Data Model: Page Block Core

## 1. 集約境界

### PageAggregate

- 役割: 単一ページと，そのページに属する順序付き block 集合をひとまとまりで整合管理する，
- 識別子: `page_id: Uuid`
- 不変条件:
  - managed page は常に 1 件だけ存在する，
  - block の `page_id` は必ず親 page の `page_id` と一致する，
  - block の順序は重複無しの連続した整数で表現する，
  - title は空文字を許可するが，表示名は空時に必ず `無題` へ解決される，
  - block 本文は空文字を許可するが，plain text のみを扱う，
  - nested block，複数 page，database，view，property は本 increment の永続モデルに入れない，

## 2. 永続エンティティ

### Page

| Field | Type | Required | Rules |
|------|------|----------|-------|
| `id` | UUID | yes | 起動をまたいで不変 |
| `title` | TEXT | yes | 空文字許可 |
| `created_at` | RFC3339 UTC timestamp | yes | 初回生成時に設定 |
| `updated_at` | RFC3339 UTC timestamp | yes | 永続化成功時に更新 |
| `last_persisted_revision` | INTEGER | yes | 保存済みスナップショットの版管理 |

### Block

| Field | Type | Required | Rules |
|------|------|----------|-------|
| `id` | UUID | yes | 起動をまたいで不変 |
| `page_id` | UUID | yes | `pages.id` への参照 |
| `body` | TEXT | yes | 空文字許可。plain text のみ |
| `position` | INTEGER | yes | 0 始まりの連続値 |
| `created_at` | RFC3339 UTC timestamp | yes | block 作成時に設定 |
| `updated_at` | RFC3339 UTC timestamp | yes | 内容変更または reorder 後の保存成功時に更新 |

### SaveMetadata

| Field | Type | Required | Rules |
|------|------|----------|-------|
| `singleton_key` | TEXT | yes | 常に `page_block_core` |
| `last_persisted_revision` | INTEGER | yes | 成功した最新 revision |
| `last_persisted_at` | RFC3339 UTC timestamp | yes | 成功した保存時刻 |
| `last_backup_at` | RFC3339 UTC timestamp \| NULL | no | `.bak` 更新時刻 |
| `schema_version` | INTEGER | yes | マイグレーション追跡用 |

## 3. アプリケーション境界 DTO

### `BlockDto`

- `id: string`
- `pageId: string`
- `body: string`
- `position: number`
- `createdAt: string`
- `updatedAt: string`

### `PageSnapshotDto`

- `id: string`
- `title: string`
- `displayTitle: string`
- `createdAt: string`
- `updatedAt: string`
- `lastPersistedRevision: number`
- `blocks: BlockDto[]`

### `EditorSessionState`

- `page: PageSnapshotDto`
- `draftRevision: number`
- `persistedRevision: number`
- `saveStatus: "clean" | "dirty" | "saving" | "save_failed"`
- `lastErrorMessage?: string`
- `recoveryNotice?: string`

## 4. リレーション

- Page 1 : N Block，
- SaveMetadata 1 : 1 PageAggregate，
- EditorSessionState 1 : 1 現在編集中の PageAggregate，

## 5. バリデーション規則

- 起動時に保存データが無い場合は，block 0 件の空 page を自動生成する，
- 起動時に保存データが読めないか構造不正なら，通知用 `recoveryNotice` を付けた新規 page を返す，
- `blocks` 配列は `position` 昇順でシリアライズする，
- reorder 後の `position` は 0 から `n - 1` まで詰め直す，
- `lastPersistedRevision` は保存成功ごとに単調増加する，
- 保存失敗時は `persistedRevision` を進めず，UI 上の `draftRevision` のみ進める，

## 6. 状態遷移

### Editor Session

1. `Bootstrapping`
   - 起動時に永続化済み page を読込中，
2. `Clean`
   - `draftRevision == persistedRevision`，
3. `Dirty`
   - UI 編集によりドラフトが永続状態より新しい，
4. `Saving`
   - `persist_page_snapshot` 実行中，
5. `SaveFailed`
   - 永続化失敗。画面上のドラフトは保持される，
6. `RecoveredWithNotice`
   - 起動時回復で新規空 page を生成し，通知を伴って開始する，

### 遷移規則

- `Bootstrapping -> Clean`: 既存 page を正常復元した場合，
- `Bootstrapping -> RecoveredWithNotice`: 保存データが無いか，または読取不能時に新規 page を生成した場合，
- `Clean -> Dirty`: タイトル編集，block 本文編集，block 追加，reorder 完了，
- `Dirty -> Saving`: 500ms デバウンス満了，または即時保存トリガー発火，
- `Saving -> Clean`: 保存成功し，`persistedRevision` が更新された場合，
- `Saving -> SaveFailed`: 保存失敗し，最後の整合済み状態を保持した場合，
- `SaveFailed -> Saving`: 次の編集停止または reorder 完了で再試行した場合，

## 7. 初回マイグレーション案

```sql
CREATE TABLE pages (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  last_persisted_revision INTEGER NOT NULL
);

CREATE TABLE blocks (
  id TEXT PRIMARY KEY,
  page_id TEXT NOT NULL REFERENCES pages(id) ON DELETE CASCADE,
  body TEXT NOT NULL,
  position INTEGER NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  UNIQUE(page_id, position)
);

CREATE TABLE save_metadata (
  singleton_key TEXT PRIMARY KEY,
  last_persisted_revision INTEGER NOT NULL,
  last_persisted_at TEXT NOT NULL,
  last_backup_at TEXT,
  schema_version INTEGER NOT NULL
);
```
