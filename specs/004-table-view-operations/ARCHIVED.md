# 004-table-view-operations

**Status**: Merged
**Rolled up to steering/**: 2026-03-22
**Archive location**: archive/004-table-view-operations/

## Summary

テーブルビューにソート・フィルタ・グルーピング機能を追加し，View エンティティで
設定を永続化する機能。バックエンドに `domain::view` モジュールを新設し，
ソート・フィルタ・グルーピングのロジックを Rust で実装。
条件は JSON カラムで `views` テーブルに格納。

## What was built

- カラムヘッダークリックによる単一ソート + パネルでの複数カラム優先順位付きソート（最大5件）
- プロパティ型別演算子によるフィルタリング（AND 結合，最大20件）
- 1プロパティによるグルーピング表示（折りたたみ・展開，件数表示）
- ビュー設定の自動保存・復元・一括リセット
- プロパティ削除時のビュー条件自動修復（孤立条件除去）

## Key decisions

- View を Database に属する独立した集約ルートとして設計（将来の複数ビュー拡張を想定）
- ソート・フィルタ・グルーピング条件を JSON カラムで永続化（FK 制約ではなくアプリケーション層で整合性管理）
- ソート・フィルタ・グルーピングのロジックを Rust バックエンドで実行し，処理済みデータをフロントに返却

## Files in archive

- spec.md — 機能仕様（what/why）
- plan.md — 技術計画
- tasks.md — 実行されたタスク一覧
- data-model.md — データモデル定義（View, SortCondition, FilterCondition, GroupCondition）
- research.md — 技術調査結果
- quickstart.md — 開発者向けクイックスタート
- contracts/ — IPC 契約定義（view-commands.md）
- checklists/ — 品質検証チェックリスト（6ファイル）

---
*このfeatureの詳細を参照する必要がある場合は、
archive/004-table-view-operations/ を確認してください。*
