# Quickstart: ブロックエディタ

**Feature Branch**: `002-block-editor`
**Date**: 2026-03-21

## 前提条件

- 001-page-persistence が main にマージ済みであること
- Nix devshell が有効であること（`direnv allow` 済み）
- `rustup`, `cargo-make`, `cargo-nextest`, `pnpm` が利用可能であること

## 開発サーバーの起動

```bash
cargo make serve
```

Tauri 開発サーバーが起動し，フロントエンド（Vite HMR）とバックエンド（Rust）の
ホットリロードが有効になる。

## 品質ゲートの実行

```bash
cargo make qa
```

以下を順次実行する:

**Rust** (`qa-rs`):
1. `cargo fmt --all` — コードフォーマット
2. `cargo clippy --workspace` — Lint（`unwrap_used = deny` 等）
3. `cargo nextest run` — テスト
4. `cargo doc --no-deps` — ドキュメントビルド

**TypeScript** (`qa-ts`):
1. Biome format — フォーマット
2. Biome lint — Lint
3. `tsc --noEmit` — 型チェック
4. Vitest — テスト

## 個別コマンド

```bash
# Rust フォーマット
cargo make fmt-rs

# Rust Lint
cargo make clippy

# Rust テスト
cargo make test

# 特定テストの実行（ブロック関連）
TEST_FILTER="block" cargo make test-filter

# 特定テストの実行（エディタセッション関連）
TEST_FILTER="editor" cargo make test-filter

# TypeScript フォーマット
cargo make fmt-ts

# TypeScript Lint
cargo make lint-ts

# TypeScript 型チェック
cargo make ts-check

# TypeScript テスト
cargo make test-ts
```

## 本機能で追加するファイル

### Backend (Rust)

```
src-tauri/
├── src/
│   ├── lib.rs                         # editor_commands の登録 + AppState に sessions 追加
│   ├── domain/
│   │   ├── mod.rs                     # pub mod block; pub mod editor; を追加
│   │   ├── block/
│   │   │   ├── mod.rs                 # 新規
│   │   │   ├── entity.rs              # Block, BlockId, BlockContent, BlockPosition
│   │   │   └── error.rs              # BlockError
│   │   └── editor/
│   │       ├── mod.rs                 # 新規
│   │       └── session.rs            # EditorSession（純粋ドメインロジック）
│   ├── infrastructure/
│   │   └── persistence/
│   │       ├── mod.rs                 # pub mod block_repository; を追加
│   │       └── block_repository.rs    # BlockRepository トレイト + SqlxBlockRepository
│   └── ipc/
│       ├── mod.rs                     # pub mod editor_commands; を追加
│       ├── error.rs                   # CommandError に Block(BlockError) を追加
│       ├── dto.rs                     # EditorStateDto, BlockDto を追加
│       └── editor_commands.rs         # 8 IPC コマンド（新規）
└── migrations/
    └── 0002_create_blocks.sql         # 新規マイグレーション
```

### Frontend (TypeScript/React)

```
src/
├── App.tsx                            # 画面切り替えロジックを追加
├── features/
│   ├── pages/
│   │   ├── PageListView.tsx           # onPageClick コールバック追加
│   │   └── PageItem.tsx               # クリック時の遷移コールバック追加
│   └── editor/
│       ├── BlockEditor.tsx            # エディタ画面（新規）
│       ├── BlockEditor.module.css
│       ├── BlockItem.tsx              # 個別ブロック（新規）
│       ├── BlockItem.module.css
│       ├── EditorToolbar.tsx          # ツールバー（新規）
│       ├── EditorToolbar.module.css
│       ├── UnsavedConfirmModal.tsx    # 未保存確認ダイアログ（新規）
│       ├── UnsavedConfirmModal.module.css
│       ├── useEditor.ts              # IPC 呼び出しラッパー（新規）
│       └── types.ts                  # EditorState, Block 型（新規）
```

## 既存ファイルへの変更

| ファイル | 変更内容 |
|---|---|
| `src-tauri/src/lib.rs` | `invoke_handler` に 8 コマンドを登録，`AppState` に `sessions` フィールド追加 |
| `src-tauri/src/domain/mod.rs` | `pub mod block; pub mod editor;` を追加 |
| `src-tauri/src/infrastructure/persistence/mod.rs` | `pub mod block_repository;` を追加 |
| `src-tauri/src/infrastructure/persistence/database.rs` | `PRAGMA foreign_keys = ON` を追加 |
| `src-tauri/src/ipc/mod.rs` | `pub mod editor_commands;` を追加 |
| `src-tauri/src/ipc/error.rs` | `CommandError::Block(BlockError)` バリアント追加，Serialize impl 拡張 |
| `src-tauri/src/ipc/dto.rs` | `EditorStateDto`, `BlockDto` を追加 |
| `src/App.tsx` | 画面切り替えロジック（list / editor）を追加 |
| `src/features/pages/PageListView.tsx` | `onPageClick` コールバック prop を追加 |
| `src/features/pages/PageItem.tsx` | タイトルクリック時の遷移コールバック追加 |

## マイグレーション

新規マイグレーション `0002_create_blocks.sql` はアプリ起動時に `sqlx::migrate!()` で
自動適用される。手動操作は不要。

開発用 DB の `DATABASE_URL` が設定されている場合，コンパイル時クエリ検証のために
マイグレーションを手動で適用する必要がある:

```bash
cd src-tauri
sqlx migrate run
```

または，`cargo sqlx prepare` でオフラインモードを使用する:

```bash
cd src-tauri
cargo sqlx prepare
```

## PRAGMA foreign_keys

SQLite では外部キー制約がデフォルトで無効。`database.rs` の初期化処理で
`PRAGMA foreign_keys = ON` を設定する必要がある。

## データベースのリセット

開発中にデータベースをリセットするには，アプリケーションデータディレクトリの
DB ファイルを削除する:

- **Linux**: `~/.local/share/com.vscode.rustydatabasenotes/rustydatabasenotes.db`

アプリを再起動すると，マイグレーションが自動実行され，空の DB が作成される。
