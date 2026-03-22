# Implementation Plan: Page Tree Navigation

**Branch**: `005-page-tree-nav` | **Date**: 2026-03-22 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/005-page-tree-nav/spec.md`

## Summary

ページ階層（親子関係）をドメインモデルに導入し，サイドバーによるツリーナビゲーションを実装する。
同時に Tailwind CSS + shadcn/ui へのデザインシステム全面移行を行い，既存の CSS Modules を廃止する。
エディタの保存方式を手動保存から debounce 付き自動保存に統一する。

技術的には，SQLite マイグレーションで `parent_id` / `sort_order` カラムを追加し，
ドメイン層で循環参照検出・深度制限・データベースページの階層不参加を保証する。
フロントエンドは shadcn/ui コンポーネント + @atlaskit/pragmatic-drag-and-drop によるツリー D&D を構築する。

## Technical Context

**Language/Version**: Rust 2024 (edition = "2024", toolchain 1.94.0), TypeScript ~5.8.3
**Primary Dependencies**:
- Backend: Tauri 2, sqlx 0.8 (SQLite), uuid 1 (v7), chrono 0.4, thiserror 2, serde 1, serde_json 1, tokio 1
- Frontend: React 19, Sonner (toast), Biome 2.4.8
- 新規追加予定: Tailwind CSS v4, @tailwindcss/vite, shadcn/ui, @atlaskit/pragmatic-drag-and-drop + hitbox, lucide-react
**Storage**: SQLite (WAL mode), `{appDataDir}/rustydatabasenotes.db`, sqlx::migrate!() によるコンパイル時マイグレーション埋め込み。新規マイグレーション 0007 で `pages` テーブルに `parent_id TEXT` (自己参照 FK) と `sort_order INTEGER DEFAULT 0` を追加
**Testing**: `cargo make qa`（fmt → clippy → nextest → doc-test → doc-check → lint-ts → ts-check → vitest）
**Target Platform**: Desktop (Linux/WSL2 primary, Windows/macOS secondary)
**Project Type**: desktop-app (Tauri 2)
**Performance Goals**: サイドバー初回レンダリング ≤200ms @500ページ，ツリー展開/折りたたみ ≤50ms，サイドバークリック→画面遷移 ≤100ms
**Constraints**: 完全オフライン動作，外部ネットワーク通信禁止，アプリケーションコードでのパニック禁止
**Scale/Scope**: 500+ ページ，最大5階層ネスト，数十データベース

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*
*Post-design re-check (2026-03-22): All 7 principles pass. No new violations.*

- **I. Local-First Product Integrity**: ページ階層は SQLite の `parent_id` 外部キー制約で保証される。
  階層変更（移動・親削除時の子昇格）はトランザクション内でアトミックに実行し，クラッシュ時は
  WAL モードにより変更前の状態が保持される。サイドバーの UI 状態（展開/折りたたみ，表示/非表示，
  最後に開いたアイテム）は localStorage に保持し，DB スキーマへの影響はない。
  外部サービスは一切導入しない（FR-012）。

- **II. Domain-Faithful Information Model**: `Page` エンティティに `parent_id` を追加し，
  既存のページ・ブロック・データベース・ビュー・プロパティの語彙を一貫して使用する。
  「スタンドアロンページ」（database_id なし）と「データベース所属ページ」（database_id あり）の
  区分を維持し，後者はページ階層に参加しない（FR-009）。ドメイン層で強制する。

- **III. Typed Boundaries and Domain-Driven Design**:
  - **Rust 境界型**: `PageId`（既存），`ParentId`（`Option<PageId>` で表現），新規エラーバリアント
    `PageError::CircularReference`, `PageError::MaxDepthExceeded`, `PageError::DatabasePageCannotNest`
  - **IPC コントラクト**: 新規コマンド `list_sidebar_items`, `create_child_page`, `move_page`,
    既存コマンド拡張 `PageDto` に `parentId: string | null` 追加
  - **ストレージスキーマ**: マイグレーション 0007 で `parent_id`, `sort_order` カラム追加
  - **影響する境界コンテキスト**: Page（主要），Editor（自動保存移行），Database（サイドバー表示）

- **IV. Test-First Delivery and Quality Gates**:
  - ドメイン層テスト（Red-Green-Refactor）: 循環参照検出，深度制限（6階層目の拒否），
    親削除時の子昇格，データベースページの階層不参加，ページ移動の正当性検証
  - リポジトリ統合テスト: `parent_id` を含む CRUD 操作
  - フロントエンド Vitest: サイドバー描画，ツリー展開/折りたたみ状態管理
  - 品質ゲート: `cargo make qa`（全チェック通過必須）

- **V. Safe Rust, SOLID Principles, and Maintainability First**: `unsafe` 使用なし。
  - SRP: ページ階層ロジックはドメインサービスとして分離（`PageHierarchyService` 等）
  - OCP: `PageRepository` トレイトに新メソッド追加（既存実装の修正は最小限）
  - DIP: ドメイン層は `PageRepository` トレイトに依存し，具象 `SqlxPageRepository` を知らない
  - 投機的最適化なし。500ページ規模では仮想化は不要と判断し，必要性が計測されてから導入する

- **VI. Rust Documentation Standards**: 新規公開アイテム（`create_child_page`, `move_page`,
  `list_sidebar_items` 等の IPC コマンド，ドメインサービスメソッド，新規エラーバリアント）に
  `///` ドキュメントコメントを付与。`# Examples`（自明なアクセサ免除），`# Errors` セクションを
  含む。`cargo doc --no-deps` がクリーンに通過することを確認。

- **VII. Defensive Error Handling**: `unwrap()`, `expect()`, `panic!()`, `todo!()`, `assert!()`
  はアプリケーションコードで使用しない。階層操作のエラー条件は `thiserror` 列挙型で定義:
  - `PageError::CircularReference { page_id, target_parent_id }` — 循環参照検出時
  - `PageError::MaxDepthExceeded { page_id, current_depth, max_depth }` — 深度超過時
  - `PageError::DatabasePageCannotNest { page_id }` — DB ページの階層操作時
  Clippy ワークスペース lint（`unwrap_used = "deny"` 等）は設定済み。

## Project Structure

### Documentation (this feature)

```text
specs/005-page-tree-nav/
├── plan.md              # This file
├── research.md          # Phase 0: 技術調査
├── data-model.md        # Phase 1: データモデル設計
├── quickstart.md        # Phase 1: 開発ガイド
├── contracts/           # Phase 1: IPC コントラクト
│   └── ipc-commands.md  # 新規・変更コマンド一覧
└── tasks.md             # Phase 2: タスク分解（/speckit.tasks）
```

### Source Code (repository root)

```text
src/
├── components/
│   ├── toast/               # Toaster（既存）
│   └── ui/                  # shadcn/ui コンポーネント（新規）
├── features/
│   ├── editor/              # ブロックエディタ（自動保存移行）
│   ├── database/            # テーブルビュー（デザイン移行）
│   ├── pages/               # ページ管理（デザイン移行）
│   └── sidebar/             # サイドバー（新規）
│       ├── Sidebar.tsx
│       ├── SidebarTree.tsx
│       ├── SidebarItem.tsx
│       ├── SidebarContextMenu.tsx
│       ├── SidebarCreateButton.tsx
│       └── useSidebar.ts
├── hooks/                   # 共有フック（新規）
│   ├── useLocalStorage.ts
│   └── useAutoSave.ts
└── lib/                     # ユーティリティ（新規）
    └── utils.ts             # cn() ヘルパー（shadcn/ui 標準）

src-tauri/
├── src/
│   ├── domain/
│   │   ├── page/
│   │   │   ├── entity.rs    # Page に parent_id, sort_order 追加
│   │   │   ├── hierarchy.rs # 新規: 階層ドメインサービス
│   │   │   ├── repository.rs # 新メソッド追加
│   │   │   └── error.rs     # 新エラーバリアント追加
│   │   ├── block/           # 変更なし
│   │   ├── database/        # 変更なし
│   │   ├── editor/
│   │   │   └── session.rs   # isDirty/mark_saved パターン廃止
│   │   ├── property/        # 変更なし
│   │   └── view/            # 変更なし
│   ├── infrastructure/
│   │   └── persistence/
│   │       └── page_repository.rs  # 階層クエリ追加
│   └── ipc/
│       ├── dto.rs           # PageDto に parentId 追加, SidebarItemDto 新規
│       ├── page_commands.rs # 階層操作コマンド追加
│       └── editor_commands.rs # save_editor 自動保存対応
└── migrations/
    └── 0007_add_page_hierarchy.sql  # parent_id, sort_order 追加
```

**Structure Decision**: ページ階層ロジックは `domain/page/hierarchy.rs` に集約し，
既存の `entity.rs` は Page 構造体の拡張に留める。サイドバーは `features/sidebar/` として
独立した機能モジュールにする。shadcn/ui コンポーネントは `components/ui/` に配置し，
shadcn/ui の標準的なプロジェクト構成に従う。

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| `sort_order` カラムの先行追加（YAGNI 近似） | 将来の手動並べ替え機能のスキーマ変更を回避するビジネス判断 | マイグレーション追加はコストが低く，後からのカラム追加は既存データの一括更新が必要になる |
| CSS Modules → Tailwind 全面移行（大規模変更） | デザインシステム統一（US1）の明示的要件 | 部分移行はスタイル二重管理を招き，保守コストが増大する |
