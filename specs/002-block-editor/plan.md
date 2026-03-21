# Implementation Plan: ブロックエディタ

**Branch**: `002-block-editor` | **Date**: 2026-03-21 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/002-block-editor/spec.md`

## Summary

Page エンティティの子要素として Block エンティティを導入し，バックエンド（Rust）に
EditorSession（インメモリ状態）を持たせることで，ブロックの追加・編集・並び替え・削除の
全操作を Rust のドメインロジックで処理する。フロントエンド（React/TypeScript）は薄い
UI 層として IPC コマンドを呼び出し，返却された EditorState で画面を更新する。
明示的な保存操作により EditorSession の状態をトランザクション内で一括永続化する。

**アーキテクチャ判断**: Rust の型安全性・テスト容易性を最大活用するため，ビジネスロジックを
バックエンドに集約する。フロントエンドはイベント発火と表示のみを担当し，状態管理・
バリデーション・状態遷移はすべて Rust 側で行う。

## Technical Context

**Language/Version**: Rust 2024 edition (toolchain 1.94.0), TypeScript ~5.8.3
**Primary Dependencies**:
- Backend: Tauri 2, sqlx 0.8 (runtime-tokio, sqlite, macros, migrate, chrono, uuid), uuid 1 (v7+serde), thiserror 2, chrono 0.4 (serde), serde 1 (derive), serde_json 1（すべて 001-page-persistence で導入済み，新規依存なし）
- Frontend: React 19.1, @tauri-apps/api 2, sonner（すべて導入済み，新規依存なし）
**Storage**: SQLite (WAL mode), `app_data_dir()/rustydatabasenotes.db`, sqlx::migrate!() による組み込みマイグレーション。新規マイグレーション `0002_create_blocks.sql` で blocks テーブルを追加
**Testing**: `cargo nextest run`, `cargo clippy --workspace`, `cargo doc --no-deps`, `pnpm test`, `cargo make qa`
**Target Platform**: Desktop (Linux WSL2 で開発，クロスプラットフォーム対応)
**Project Type**: desktop-app (Tauri 2)
**Performance Goals**: 1,000 ブロックの一覧表示 <1s，1,000 ブロックの一括保存 <1s，個別操作は遅延なし
**Constraints**: ローカル完結（外部通信禁止），`unwrap()`/`expect()`/`panic!()`/`unsafe` 禁止，トランザクション保護，1 ブロックあたり最大 10,000 文字
**Scale/Scope**: 1 ページあたり最大 ~1,000 ブロック，ブロック種別は初期スコープでは 'text' のみ
**Session State**: `AppState` に `Mutex<HashMap<PageId, EditorSession>>` を保持。EditorSession はページごとのインメモリ編集状態

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Local-First Product Integrity**: ✅ PASS
  - ブロック操作はすべてローカル SQLite + インメモリで完結，外部通信は一切なし
  - 一括保存はトランザクション内で実行 — 途中失敗時は全変更がロールバック
  - 保存失敗時はエラー通知を表示し，EditorSession のインメモリ状態は保持される（データロスなし）
  - WAL モードにより読み取り中の書き込みによる破損を防止（001 で設定済み）
  - CASCADE 削除によりページ削除時のブロック孤児を防止

- **Domain-Faithful Information Model**: ✅ PASS
  - 正規語彙「ブロック（block）」「ページ（page）」を一貫して使用
  - Block はエンティティとして定義，Page の子要素として設計
  - `block_type` カラムを初期スキーマに含め，将来のブロック種別追加に対応
  - EditorSession はページ単位の編集コンテキストとして機能

- **Typed Boundaries and Bounded Contexts**: ✅ PASS
  - **Rust 境界型**: `BlockId`, `BlockContent`, `BlockPosition`, `Block`（ドメイン），`EditorSession`（ドメイン），`EditorStateDto`, `BlockDto`（IPC），`CommandError`（IPC，既存を拡張）
  - **TypeScript 境界型**: `EditorState`, `Block`, `CommandError`（既存を拡張）
  - **IPC 契約**: 8 コマンド（`open_editor`, `add_block`, `edit_block_content`, `move_block_up`, `move_block_down`, `remove_block`, `save_editor`, `close_editor`）
  - **ストレージスキーマ**: Migration 0002_create_blocks.sql で `blocks` テーブル + 外部キー（CASCADE）+ 複合インデックス
  - **境界コンテキスト**: `domain::block`（新規），`domain::editor`（新規），`infrastructure::persistence`（既存に追加），`ipc`（既存に追加）

- **Test-First Delivery and Quality Gates**: ✅ PASS
  - EditorSession のユニットテスト: 全操作（追加・編集・並び替え・削除）とエッジケース（上限，境界，空）を Rust で完結してテスト可能
  - ドメイン層: BlockContent バリデーション（空許容，10,000 文字上限），Block 生成のユニットテスト
  - インフラ層: in-memory SQLite を用いた CRUD + 一括保存の統合テスト，CASCADE 削除の検証
  - IPC 層: コマンドのエラーハンドリング，セッション管理テスト
  - フロントエンド: コンポーネント表示テスト（ロジックは Rust 側のため薄い）
  - 品質ゲート: `cargo make qa`

- **Safe Rust and Maintainability First**: ✅ PASS
  - `unsafe`, `unwrap()`, `expect()`, `panic!()`, `unreachable!()` は使用しない
  - Clippy の deny 設定は 001 で既に適用済み
  - `Mutex` はセッション管理に使用するが，ロック粒度はコマンド単位で小さく保つ
  - SOLID 原則: `BlockRepository` トレイトで DIP，`EditorSession` は純粋なドメインロジック（DB 非依存）で SRP，既存の `PageRepository` とは独立（ISP）
  - YAGNI: application 層は不要（EditorSession はドメインサービスとして配置），自動保存・D&D・Undo/Redo は対象外

- **Rust ドキュメント標準**: ✅ PASS
  - すべての公開アイテム（`pub fn`, `pub struct`, `pub enum`, `pub trait`）に `///` ドキュメントコメントを付与する
  - 公開関数・メソッドに `# Examples` セクション（自明なアクセサは免除），`Result` を返す関数に `# Errors` セクションを付与する
  - `#![warn(missing_docs)]` を `lib.rs` で有効化し，未ドキュメントの公開アイテムをコンパイル時に検出する
  - 品質ゲートとして `cargo doc --no-deps` を実行し，警告なしにビルドされることを検証する（`cargo make doc-check`）

- **防御的エラーハンドリング**: ✅ PASS
  - `unwrap()`, `expect()`, `panic!()`, `unreachable!()`, `todo!()`, 非テストコードの `assert!()` はアプリケーションコードで禁止
  - すべての失敗可能操作は `Result<T, E>` で伝播し，`?` 演算子を使用する
  - `BlockError` を `thiserror` 列挙型として `domain::block::error` に定義，各バリアントはコンテキスト（len, max, id, value）を保持
  - Clippy lint（`unwrap_used`, `expect_used`, `panic`, `unreachable`）を deny に設定（001 で適用済み，本スライスで継続）

## Project Structure

### Documentation (this feature)

```text
specs/002-block-editor/
├── plan.md              # This file
├── research.md          # Phase 0: 技術調査・設計判断
├── data-model.md        # Phase 1: データモデル定義
├── quickstart.md        # Phase 1: 開発手順ガイド
├── contracts/
│   └── ipc-commands.md  # Phase 1: IPC 契約定義
└── tasks.md             # Phase 2: タスク分解 (/speckit.tasks)
```

### Source Code (repository root)

```text
src/
├── main.tsx
├── App.tsx                            # 画面切り替えロジックを追加
├── App.css
├── features/
│   ├── pages/
│   │   ├── PageListView.tsx           # ページクリック時のコールバック追加
│   │   ├── PageItem.tsx               # クリックハンドラ追加
│   │   ├── CreatePageForm.tsx
│   │   ├── DeleteConfirmModal.tsx
│   │   ├── usePages.ts
│   │   └── types.ts
│   └── editor/
│       ├── BlockEditor.tsx            # エディタ画面（メインコンテナ）
│       ├── BlockEditor.module.css
│       ├── BlockItem.tsx              # 個別ブロック（テキスト入力 + 操作ボタン）
│       ├── BlockItem.module.css
│       ├── EditorToolbar.tsx          # ツールバー（戻る，保存ボタン，インジケータ）
│       ├── EditorToolbar.module.css
│       ├── UnsavedConfirmModal.tsx    # 未保存確認ダイアログ
│       ├── UnsavedConfirmModal.module.css
│       ├── useEditor.ts              # IPC 呼び出し + 画面更新フック（薄いレイヤー）
│       └── types.ts                  # EditorState, Block 型定義
└── components/
    └── toast/
        └── Toaster.tsx

src-tauri/
├── src/
│   ├── main.rs
│   ├── lib.rs                         # editor_commands を登録，AppState に sessions を追加
│   ├── domain/
│   │   ├── mod.rs                     # pub mod block, editor を追加
│   │   ├── page/                      # 変更なし
│   │   ├── block/
│   │   │   ├── mod.rs
│   │   │   ├── entity.rs              # Block, BlockId, BlockContent, BlockPosition
│   │   │   └── error.rs              # BlockError
│   │   └── editor/
│   │       ├── mod.rs
│   │       └── session.rs            # EditorSession（純粋ドメインロジック，DB 非依存）
│   ├── infrastructure/
│   │   ├── mod.rs
│   │   └── persistence/
│   │       ├── mod.rs                 # pub mod block_repository を追加
│   │       ├── database.rs            # PRAGMA foreign_keys = ON を追加
│   │       ├── error.rs               # StorageError（変更なし）
│   │       ├── page_repository.rs     # 変更なし
│   │       └── block_repository.rs    # SqlxBlockRepository（新規）
│   └── ipc/
│       ├── mod.rs                     # pub mod editor_commands を追加
│       ├── error.rs                   # CommandError に BlockError を追加
│       ├── dto.rs                     # EditorStateDto, BlockDto を追加
│       ├── page_commands.rs           # 変更なし
│       └── editor_commands.rs         # 8 コマンド（新規）
├── migrations/
│   ├── 0001_create_pages.sql          # 変更なし
│   └── 0002_create_blocks.sql         # 新規
└── Cargo.toml                         # 変更なし（新規依存なし）
```

**Structure Decision**:
- `domain::block` — Block エンティティと値オブジェクトの境界コンテキスト。`entity.rs` に Block, BlockId, BlockContent, BlockPosition を定義。`error.rs` に BlockError を定義。リポジトリトレイトは `infrastructure` に配置（BlockRepository は永続化の関心であり，EditorSession のドメインロジックには不要）
- `domain::editor` — EditorSession ドメインサービス。`session.rs` にブロック操作の全ロジックを実装。`Vec<Block>` を保持し，追加・編集・並び替え・削除のメソッドを提供。DB に一切依存しない純粋なドメインロジック。テストは DB 不要で完結する
- `infrastructure::persistence` — 既存の境界コンテキストに `block_repository.rs` を追加。`BlockRepository` トレイトと `SqlxBlockRepository` 実装を配置。一括保存のトランザクション管理もここで実装
- `ipc::editor_commands` — EditorSession のライフサイクル管理。`AppState` のセッション `HashMap` を操作し，リポジトリと EditorSession を橋渡しする。8 コマンドを定義
- `src/features/editor` — フロントエンドのエディタ UI。IPC 呼び出しと返却された `EditorState` の表示に特化。ビジネスロジックは含まない
- `application/` 層は本スライスでも不要 — EditorSession はドメインサービスとして `domain::editor` に配置し，IPC 層がリポジトリとの橋渡しを行う

## Complexity Tracking

> 憲章違反はなし。

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| `Mutex<HashMap<PageId, EditorSession>>` の導入 | ブロック操作を Rust の型安全性で保護するため，インメモリ状態管理が必要 | フロントエンドで状態管理する方式は TypeScript にビジネスロジックが分散し，型安全性とテスト容易性が低下する |
