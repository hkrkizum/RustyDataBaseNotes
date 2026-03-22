# 005-page-tree-nav

**Status**: Merged
**Rolled up to steering/**: 2026-03-23
**Archive location**: archive/005-page-tree-nav/

## Summary

ページ階層（親子関係）をドメインモデルに導入し，サイドバーによるツリーナビゲーションを実装。
同時に Tailwind CSS + shadcn/ui へのデザインシステム全面移行を行い，既存の CSS Modules を廃止。
エディタの保存方式を手動保存から debounce 付き自動保存に統一。

## What was built

- サイドバーナビゲーション（常時表示，1クリック遷移，表示/非表示トグル）
- ページ階層（親子関係，最大5階層，循環参照防止，D&D 移動，親削除時の子昇格）
- ツリー UI（展開/折りたたみ，コンテキストメニュー，インライン名前編集，起動時復元）
- エディタ自動保存（debounce 500ms，指数バックオフリトライ，手動保存 UI 廃止）
- 統一デザインシステム（Tailwind CSS v4 + shadcn/ui，ライト/ダークモード対応）

## Key decisions

- PageHierarchyService をリポジトリ非依存の純粋ドメインサービスとして設計し，IPC 層がデータロードを担当
- parent_id の ON DELETE SET NULL をフェイルセーフとし，アプリ層トランザクションで子の祖父母への昇格を実行
- sort_order カラムを先行追加（将来の手動並べ替え用，本スコープでは created_at DESC で表示）

## Files in archive

- spec.md — 機能仕様（what/why）
- plan.md — 技術計画
- tasks.md — 実行されたタスク一覧
- data-model.md — データモデル定義
- research.md — 技術調査結果（Tailwind v4, shadcn/ui, pragmatic-drag-and-drop）
- quickstart.md — 開発ガイド
- contracts/ — IPC コントラクト
- checklists/ — 品質検証チェックリスト

---
*このfeatureの詳細を参照する必要がある場合は、
archive/005-page-tree-nav/ を確認してください。*
