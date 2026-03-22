# Quickstart: テーブルビュー操作拡張

**Feature**: 004-table-view-operations
**Date**: 2026-03-22

## 前提条件

- Nix devshell が有効（`direnv allow` 済み）
- `cargo make sqlx-prepare` でデータベースキャッシュが最新

## 開発サーバー起動

```bash
cargo make serve
```

## テスト実行

```bash
# Rust テスト全体
cargo make test

# 特定パターンでフィルタ
TEST_FILTER=view cargo make test-filter

# TypeScript テスト
cargo make test-ts

# フル QA（コミット前に必須）
cargo make qa
```

## マイグレーション適用

```bash
# dev.db リセット + .sqlx/ キャッシュ再生成
cargo make sqlx-prepare
```

新しいマイグレーション `0006_create_views.sql` を追加した後は必ず実行すること。

## 主要ファイル

### バックエンド（Rust）

| ファイル | 役割 |
|---|---|
| `src-tauri/src/domain/view/entity.rs` | View エンティティ，SortCondition，FilterCondition，GroupCondition |
| `src-tauri/src/domain/view/repository.rs` | ViewRepository トレイト |
| `src-tauri/src/domain/view/error.rs` | ViewError 列挙型 |
| `src-tauri/src/domain/view/sort.rs` | プロパティ型別ソートロジック |
| `src-tauri/src/domain/view/filter.rs` | プロパティ型別フィルタロジック |
| `src-tauri/src/domain/view/group.rs` | グルーピングロジック |
| `src-tauri/src/infrastructure/persistence/view_repository.rs` | SqlxViewRepository |
| `src-tauri/src/ipc/view_commands.rs` | Tauri IPC コマンド |
| `src-tauri/migrations/0006_create_views.sql` | views テーブルマイグレーション |

### フロントエンド（TypeScript）

| ファイル | 役割 |
|---|---|
| `src/features/database/types.ts` | ViewDto 等の型定義 |
| `src/features/database/useTableData.ts` | ビュー操作フック（拡張） |
| `src/features/database/TableView.tsx` | テーブルビュー（拡張） |
| `src/features/database/TableHeader.tsx` | ソートインジケータ（拡張） |
| `src/features/database/SortPanel.tsx` | 複数ソート設定パネル |
| `src/features/database/FilterPanel.tsx` | フィルタ条件設定パネル |
| `src/features/database/FilterConditionRow.tsx` | 個別フィルタ条件行 |
| `src/features/database/GroupPanel.tsx` | グルーピング設定パネル |
| `src/features/database/GroupHeader.tsx` | グループヘッダー |

## 開発フロー

1. **マイグレーション追加** → `cargo make sqlx-prepare`
2. **ドメインエンティティ実装** → テスト先行（`#[cfg(test)]` モジュール）
3. **リポジトリ実装** → `cargo make test` で検証
4. **IPC コマンド追加** → `lib.rs` の `generate_handler!` にコマンドを登録
5. **フロントエンド型追加** → `types.ts` に DTO を追加
6. **UI コンポーネント実装** → `cargo make serve` で動作確認
7. **コミット前** → `cargo make qa` を実行

## IPC コマンド一覧

| コマンド | 説明 |
|---|---|
| `get_table_data` | テーブルデータ取得（ビュー設定自動適用） |
| `get_view` | ビュー設定取得 |
| `update_sort_conditions` | ソート条件一括更新 |
| `update_filter_conditions` | フィルタ条件一括更新 |
| `update_group_condition` | グルーピング条件設定/解除 |
| `toggle_group_collapsed` | グループ折りたたみ切替 |
| `reset_view` | ビュー設定リセット |

詳細は [contracts/view-commands.md](./contracts/view-commands.md) を参照。
