# Quickstart: Page Tree Navigation

**Feature**: 005-page-tree-nav | **Date**: 2026-03-22

## Prerequisites

```bash
# devshell が有効であることを確認（direnv + Nix）
cd /home/hikaru/Develop/RustyDataBaseNotes

# 依存関係の確認
rustup show          # 1.94.0 (Rust 2024)
pnpm --version       # pnpm installed
cargo make --version  # cargo-make installed
```

## Getting Started

### 1. ブランチ切り替え

```bash
git checkout 005-page-tree-nav
```

### 2. フロントエンド依存追加

```bash
# Tailwind CSS v4 + Vite プラグイン（postcss.config 不要）
pnpm add tailwindcss @tailwindcss/vite

# shadcn/ui セットアップ（パスエイリアス @/* → ./src/* を先に tsconfig.json と vite.config.ts に設定）
pnpm dlx shadcn@latest init

# shadcn/ui コンポーネント追加
pnpm dlx shadcn@latest add sidebar collapsible button input dropdown-menu \
  context-menu tooltip scroll-area

# D&D ライブラリ（Atlassian pragmatic-drag-and-drop）
pnpm add @atlaskit/pragmatic-drag-and-drop @atlaskit/pragmatic-drag-and-drop-hitbox @atlaskit/pragmatic-drag-and-drop-react-drop-indicator

# アイコン（shadcn/ui 標準）
pnpm add lucide-react
```

### 3. 開発サーバー起動

```bash
cargo make serve
```

### 4. DB マイグレーション

マイグレーションは自動実行（sqlx::migrate!() でコンパイル時埋め込み）。
開発用 DB をリセットしたい場合:

```bash
cargo make dev-db-reset
cargo make serve
```

### 5. sqlx キャッシュ再生成

マイグレーション追加後，CI 用の `.sqlx/` キャッシュを更新:

```bash
cargo make sqlx-prepare
```

## Quality Gates

### コミット前（自動: pre-commit hook）

```bash
cargo make check-all
# fmt-check → clippy → lint-ts → ts-check
```

### フル QA（main マージ前: pre-merge-commit hook）

```bash
cargo make qa
# sqlx-prepare → fmt → clippy → test → doc-test → doc-check
# → fmt-ts → lint-ts → ts-check → test-ts
```

### 個別実行

```bash
# Rust テスト（nextest）
cargo make test

# 特定テストのみ
TEST_FILTER=hierarchy cargo make test-filter

# TypeScript テスト（Vitest）
cargo make test-ts

# フォーマット
cargo make fmt

# Clippy
cargo make clippy
```

## Key Files to Modify

### Backend (src-tauri/)

| File | Action | Purpose |
|------|--------|---------|
| `migrations/0007_add_page_hierarchy.sql` | **Create** | parent_id, sort_order 追加 |
| `src/domain/page/entity.rs` | Modify | Page に parent_id, sort_order 追加 |
| `src/domain/page/hierarchy.rs` | **Create** | 階層ドメインサービス |
| `src/domain/page/error.rs` | Modify | 新エラーバリアント追加 |
| `src/domain/page/repository.rs` | Modify | 階層クエリメソッド追加 |
| `src/domain/page/mod.rs` | Modify | hierarchy モジュール公開 |
| `src/infrastructure/persistence/page_repository.rs` | Modify | 新メソッド実装 |
| `src/ipc/dto.rs` | Modify | PageDto 拡張, SidebarItemDto 新規 |
| `src/ipc/page_commands.rs` | Modify | 新コマンド追加，delete 修正 |
| `src/ipc/editor_commands.rs` | Modify | isDirty 削除 |
| `src/ipc/error.rs` | Modify | 新エラーマッピング |
| `src/lib.rs` | Modify | 新コマンド登録 |

### Frontend (src/)

| File | Action | Purpose |
|------|--------|---------|
| `components/ui/*.tsx` | **Create** | shadcn/ui コンポーネント群（CLI 生成） |
| `lib/utils.ts` | **Create** | cn() ヘルパー（CLI 生成） |
| `features/sidebar/Sidebar.tsx` | **Create** | サイドバーコンテナ |
| `features/sidebar/SidebarTree.tsx` | **Create** | ツリーレンダリング（再帰） |
| `features/sidebar/SidebarItem.tsx` | **Create** | 個別アイテム（D&D対応） |
| `features/sidebar/SidebarContextMenu.tsx` | **Create** | コンテキストメニュー |
| `features/sidebar/SidebarCreateButton.tsx` | **Create** | 新規作成ボタン |
| `features/sidebar/useSidebar.ts` | **Create** | サイドバーデータ管理フック |
| `hooks/useLocalStorage.ts` | **Create** | localStorage 汎用フック |
| `hooks/useAutoSave.ts` | **Create** | 自動保存フック |
| `App.tsx` | Modify | レイアウト変更（サイドバー + メインコンテンツ） |
| `App.css` | **Delete** | Tailwind に完全移行 |
| `features/editor/BlockEditor.tsx` | Modify | 自動保存移行，保存ボタン削除 |
| `features/editor/EditorToolbar.tsx` | Modify | 保存 UI 削除 |
| `features/editor/UnsavedConfirmModal.tsx` | **Delete** | 不要 |
| `features/pages/PageListView.tsx` | Modify | CSS Modules → Tailwind |
| `features/database/DatabaseListView.tsx` | Modify | CSS Modules → Tailwind |
| `features/database/TableView.tsx` | Modify | CSS Modules → Tailwind |
| `**/*.module.css` (18 files) | **Delete** | Tailwind に完全移行 |

## Architecture Notes

### User Story 優先順位と依存関係

```
US1 (Unified Visual Design)
 └─→ US2 (Sidebar Navigation)
      └─→ US3 (Page Hierarchy)
```

- US1 はデザイン基盤であり，US2/US3 の前提条件
- US2 はフラットなリスト表示（階層なし）で先行実装可能
- US3 は US2 のサイドバー上にツリー機能を追加

### 自動保存移行のタイミング

エディタの自動保存移行は US2（サイドバーナビゲーション）の実装時に行う。
理由: サイドバーからのページ切り替え時に未保存確認ダイアログが不要になることが
自動保存の主要な動機であるため。
