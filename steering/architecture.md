# Architecture Overview
<!-- Last rollup: 2026-03-22, 004-table-view-operations -->

## システム構成

Tauri 2 デスクトップアプリ。バックエンド（Rust）がドメインロジックと永続化を担当し，
フロントエンド（React/TypeScript）は薄い UI 層として型付き IPC コマンドでバックエンドと通信する。

```
┌──────────────────────────────────┐
│  Frontend (React 19 / TypeScript)│
│  features/pages, editor, database│
└─────────────┬────────────────────┘
              │ Tauri IPC (型付きコマンド)
┌─────────────┴────────────────────┐
│  IPC Layer (src-tauri/src/ipc/)  │
│  DTO 変換 + エラーシリアライズ     │
└─────────────┬────────────────────┘
              │
┌─────────────┴────────────────────┐
│  Domain Layer                    │
│  page, block, editor,            │
│  database, property, view        │
│  (外部技術に非依存)               │
└─────────────┬────────────────────┘
              │ Repository トレイト
┌─────────────┴────────────────────┐
│  Infrastructure Layer            │
│  persistence/ (sqlx + SQLite)    │
└──────────────────────────────────┘
```

## 主要モジュール

<!-- rollup: init, 2026-03-22 -->

### バックエンド（Rust）

| モジュール | 責務 |
|-----------|------|
| `domain::page` | Page エンティティ（集約ルート）。PageId（UUIDv7），PageTitle（1-255文字），PageError，PageRepository トレイト |
| `domain::block` | Block エンティティ。BlockId，BlockContent（0-10,000文字），BlockPosition，BlockError |
| `domain::editor` | EditorSession ドメインサービス。ブロック操作の全ロジックをインメモリで管理。DB 非依存 |
| `domain::database` | Database エンティティ（集約ルート）。DatabaseId，DatabaseTitle（1-255文字），DatabaseError，DatabaseRepository トレイト |
| `domain::property` | Property + PropertyValue エンティティ。PropertyType（5種），PropertyConfig，PropertyName（1-100文字），SelectOption。PropertyRepository，PropertyValueRepository トレイト |
| `domain::view` | View エンティティ（集約ルート）。ViewId，ViewName，ViewType。SortCondition，FilterCondition（16演算子），GroupCondition 値オブジェクト。ソート・フィルタ・グルーピングのロジック（sort.rs, filter.rs, group.rs）。ViewRepository トレイト，ViewError <!-- rollup: 004-table-view-operations, 2026-03-22 --> |
| `infrastructure::persistence` | sqlx による SQLite 実装。Page/Block/Database/Property/PropertyValue/View の各リポジトリ |
| `ipc` | Tauri IPC コマンド。page_commands，editor_commands，database_commands，property_commands，table_commands，view_commands。DTO（camelCase）変換，CommandError シリアライズ |

### フロントエンド（TypeScript/React）

| モジュール | 責務 |
|-----------|------|
| `features/pages` | ページ一覧・CRUD UI |
| `features/editor` | ブロックエディタ UI |
| `features/database` | データベース管理・テーブルビュー UI |
| `components/toast` | Sonner による Toast 通知 |

## 依存関係

```
Frontend (React) ──IPC──→ ipc/ ──→ domain/ ←── infrastructure/persistence/
                                       ↑
                              (依存の方向: 外→内)
```

- `domain/` は外部技術に依存しない（DDD の依存性逆転）
- `infrastructure/` は `domain/` のリポジトリトレイトを実装
- `ipc/` は `domain/` 型を DTO に変換し，`infrastructure/` を呼び出す

## データフロー

### ブロック編集フロー

```
ユーザー操作 → React UI → IPC → EditorSession（インメモリ）→ EditorStateDto 返却
    → [保存] → トランザクション内で SQLite に一括永続化
```

### プロパティ値編集フロー

```
セルクリック → React UI → IPC: set_property_value → ドメインバリデーション → SQLite 即時保存
```

<!-- rollup: 004-table-view-operations, 2026-03-22 -->
### テーブルデータ取得フロー（ソート・フィルタ・グルーピング適用）

```
テーブルビュー表示 → IPC: get_table_data → View 設定読込 → フィルタ適用 → ソート適用
    → グルーピング適用 → TableDataDto（ページ群 + ビュー設定 + グループ情報）返却
```

## データモデル（主要エンティティ関係）

```
Database 1──* Property
Database 1──* Page (via database_id, nullable)
Database 1──1 View              <!-- rollup: 004-table-view-operations, 2026-03-22 -->
Property 1──* PropertyValue
Page     1──* PropertyValue
Page     1──* Block
```

6テーブル: `pages`, `blocks`, `databases`, `properties`, `property_values`, `views`

## 変更しにくい境界

<!-- rollup: init, 2026-03-22 -->

| 境界 | 理由 |
|------|------|
| SQLite スキーマ（6テーブル） | sqlx の forward-only マイグレーション |
| IPC コマンド名とレスポンス型 | フロントエンドが直接参照 |
| UUIDv7 識別子 | 全エンティティの PK |
| PropertyType enum（5種） | SQLite に文字列で永続化済み |
| EditorSession のインメモリ管理 | `Mutex<HashMap<PageId, EditorSession>>` で AppState に保持 |
