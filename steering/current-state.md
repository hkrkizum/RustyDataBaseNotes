# Current State
<!-- Last rollup: 2026-03-23, 005-page-tree-nav -->

## 主要機能一覧

<!-- rollup: init, 2026-03-22 -->

| 機能 | 由来 | 状態 |
|------|------|------|
| ページ CRUD（作成・一覧・更新・削除） | 001-page-persistence | 実装済み |
| SQLite 永続化 + マイグレーション基盤 | 001-page-persistence | 実装済み |
| ブロックエディタ（追加・編集・並び替え・削除・明示的保存） | 002-block-editor | 実装済み |
| EditorSession（インメモリ編集状態管理） | 002-block-editor | 実装済み |
| 未保存インジケータ + 画面離脱確認 | 002-block-editor | 実装済み |
| データベース CRUD | 003-database-properties | 実装済み |
| プロパティ定義（5型） | 003-database-properties | 実装済み |
| プロパティ値のインライン編集（即時保存） | 003-database-properties | 実装済み |
| テーブルビュー | 003-database-properties | 実装済み |
| 統合リスト（ページ + データベース混在表示） | 003-database-properties | 実装済み |
<!-- rollup: 004-table-view-operations, 2026-03-22 -->
| カラムソート（単一 + 複数条件，最大5件） | 004-table-view-operations | 実装済み |
| フィルタ（型別演算子，AND 結合，最大20件） | 004-table-view-operations | 実装済み |
| グルーピング（1プロパティ，折りたたみ・展開） | 004-table-view-operations | 実装済み |
| ビュー設定永続化（自動保存・復元・リセット） | 004-table-view-operations | 実装済み |
| プロパティ削除時のビュー条件自動修復 | 004-table-view-operations | 実装済み |
| View エンティティ + views テーブル | 004-table-view-operations | 実装済み |
<!-- rollup: 005-page-tree-nav, 2026-03-23 -->
| サイドバーナビゲーション（常時表示，1クリック遷移） | 005-page-tree-nav | 実装済み |
| ページ階層（親子関係，最大5階層，D&D 移動） | 005-page-tree-nav | 実装済み |
| ツリー表示（展開/折りたたみ，localStorage 永続化） | 005-page-tree-nav | 実装済み |
| コンテキストメニュー（子ページ作成・名前変更・削除） | 005-page-tree-nav | 実装済み |
| インライン名前編集（サイドバー内） | 005-page-tree-nav | 実装済み |
| エディタ自動保存（debounce 500ms + リトライ） | 005-page-tree-nav | 実装済み |
| 統一デザインシステム（Tailwind CSS + shadcn/ui） | 005-page-tree-nav | 実装済み |
| ライト/ダークモード（OS テーマ追従） | 005-page-tree-nav | 実装済み |
| 起動時復元（最後に開いたアイテム自動復元） | 005-page-tree-nav | 実装済み |
| サイドバー表示/非表示トグル（ボタン + Cmd/Ctrl+B） | 005-page-tree-nav | 実装済み |

## 既知制約

- 自動バックアップ未実装（WAL + トランザクション保護で書き込み時の破損は防止）
- ブロック種別は 'text' のみ（将来: 見出し・リスト・画像等）
- ビューはテーブルビューのみ（将来: ボード・ガントチャート等）
- 1DB=1ビュー（複数ビューの作成・切り替えは後続スコープ）
- プロパティの型変更は不可（削除→再作成で代替）
- フィルタ条件の結合は AND のみ（OR 結合・ネストは後続スコープ）
- グルーピングは同時に1プロパティのみ（多段グルーピングは後続スコープ）
- テキストソートは Unicode コードポイント順（ロケール依存照合は後続スコープ）
- Undo / Redo 未実装
- 仮想スクロール未実装
<!-- rollup: 005-page-tree-nav, 2026-03-23 — resolved: 自動保存実装済み, D&D実装済み（階層変更用） -->
- ページ階層の最大ネスト深度は5（個人利用には十分と判断）
- D&D は親変更のみ。同一親内の手動並べ替えは後続スコープ（sort_order カラムは追加済み）
- サイドバーツリーのキーボードナビゲーション（矢印キー，WAI-ARIA TreeView）は後続スコープ
- サイドバー幅は固定（リサイズ不可）
- ダークモード切り替えは OS 設定追従のみ（アプリ内手動切り替えは後続スコープ）
- データベース所属ページはページ階層に参加不可（設計上の制約）

## 未解決課題

- バックアップ・復旧機構の設計
- プロパティの型変換時の既存データマイグレーション設計
- 1,000ページ以上のテーブルビュースクロール性能の実測
<!-- rollup: 005-page-tree-nav, 2026-03-23 -->
- 500ページ超のサイドバーツリーでの仮想化必要性の実測
- 同一親内のページ手動並べ替え UI の設計（sort_order カラムは準備済み）

## 直近で変わった重要点

<!-- rollup: 005-page-tree-nav, 2026-03-23 -->
005-page-tree-nav（2026-03-23）: サイドバーナビゲーション + ページ階層 + デザインシステム全面移行。
`pages` テーブルに `parent_id`（自己参照 FK）と `sort_order` を追加（マイグレーション 0007）。
`domain::page::hierarchy` に PageHierarchyService を新設（循環参照・深度制限・DB ページ制約の検証）。
フロントエンドに `features/sidebar/` を新設し，ツリー表示・D&D・コンテキストメニュー・インライン編集を実装。
エディタを debounce 付き自動保存に移行し，手動保存 UI・UnsavedConfirmModal を廃止。
CSS Modules を全廃し Tailwind CSS v4 + shadcn/ui に完全移行。ライト/ダークモード対応。

## 過去の変更

<!-- rollup: 004-table-view-operations, 2026-03-22 -->
004-table-view-operations（2026-03-22）: テーブルビューにソート・フィルタ・グルーピング機能を追加。
View は Database に属する集約ルートとして設計，条件は JSON カラムで永続化。

初期構築（2026-03-22）: specs/ 001〜003 の成果を基に steering/ を新規作成。
