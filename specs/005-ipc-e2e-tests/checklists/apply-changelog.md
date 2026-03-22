# Checklist Apply Changelog: E2E テスト要件品質（e2e-tests.md）

**実行日時**: 2026-03-22
**入力**: review-report.md（e2e-tests.md 由来の項目）
**モード**: 差分更新（非破壊）

---

## 変更統計

- spec.md: 6 箇所 補完 / 4 箇所 追記 / 0 箇所 移動受入
- plan.md: 1 箇所 補完 / 3 箇所 追記（E2E Test Design, E2E Workflow Mapping, Known Risks, Technical Context）
- quickstart.md: 1 箇所 補完
- 新規作成ファイル: なし
- 簡素化提案（要判断）: 0 箇所

---

## 変更詳細

### spec.md の変更

| レポート ID | 変更種別 | 対象箇所 | 変更内容の要約 |
|-----------|---------|---------|--------------|
| G-01, G-07 | 補完 | US-3 Scenario 1 | 「ページを作成し」→「ページ作成操作を行い」、Then 句に「サイドバーにタイトルテキストとして表示」を追記 |
| G-13, G-07 | 補完 | US-3 Scenario 2 | 「正しいデータが表示される」→「追加したレコードのプロパティ値が表示される」に具体化。data-model.md TableDataDto 参照追記 |
| G-04, G-07 | 補完 | US-3 Scenario 3 | フィルタ条件を「テキストプロパティの等値フィルタ」に具体化。Then 句に「一致しないレコードは非表示」を追記 |
| G-02, G-07 | 補完 | US-3 Scenario 4 | 「ブロック」→「テキストブロック」に対象タイプを明記 |
| G-15 | 補完 | Edge Cases | クリーンアップ対象を具体化:「tauri-driver プロセスの終了，一時 DB ファイルの削除」 |
| G-16 | 追記 | Edge Cases | 初期 E2E スコープはワークフローの主要フロー（作成→検証）。編集・削除は後続追加 |
| G-03 | 補完 | FR-008 | E2E 失敗時の診断は WebDriverIO 標準レポーター。スクリーンショット・DOM スナップショットは初期スコープ外 |
| G-05 | 追記 | FR-010（新規） | `RDBN_DB_PATH` 環境変数によるアプリ DB パス切り替え要件を新設 |
| G-14 | 補完 | CC-001 | E2E データリセット方式を「全テーブルの行を DELETE」に確定（research.md R-005 参照） |
| G-06 | 補完 | CC-005 | E2E テストを pre-merge-commit フックに含めない判断を明記 |
| G-10 | 補完 | CC-003 | E2E テスト実行時間目標は初回実装後に実測値で設定する方針を追記 |
| G-08 | 追記 | Out of Scope | E2E 異常系シナリオ（ネットワーク断，DB ロック，WebView クラッシュ）をスコープ外に明記 |
| G-09 | 追記 | Out of Scope | ワークフロー横断 E2E シナリオをスコープ外に明記 |
| G-11 | 追記 | Dependencies & Assumptions | デバッグビルド前提と動作差異の許容を明記 |
| G-12 | 追記 | Dependencies & Assumptions | `RDBN_DB_PATH` 環境変数依存と FR-010 への参照を追記 |

### plan.md の変更

| レポート ID | 変更種別 | 対象箇所 | 変更内容の要約 |
|-----------|---------|---------|--------------|
| P-01〜P-03, P-06〜P-08, P-11 | 追記 | Test Design > E2E テスト設計（新規サブセクション） | セレクタ戦略（data-testid），ライフサイクル管理，リトライ・待機戦略，シナリオ数，前提条件参照，インメモリ状態分離 |
| P-05 | 追記 | E2E Workflow-to-Scenario Mapping（新規テーブル） | FR-005 ワークフロー ↔ US-3 シナリオ ↔ E2E テストファイルの 3 方向マッピング |
| P-09 | 追記 | Known Risks | tauri-driver 異常終了リスクと exit code 検出による緩和策 |
| P-10, P-12, P-13 | 追記 | Technical Context > E2E Environment | WSLg/headless 代替，Nix devshell による環境再現，WebKitGTK 管理，tauri-driver バージョン方針 |
| P-14 | 補完 | Project Structure | wdio.conf.ts に contracts/test-helpers.md への参照コメントを追加 |

### quickstart.md の変更

| レポート ID | 変更種別 | 対象箇所 | 変更内容の要約 |
|-----------|---------|---------|--------------|
| P-04 | 補完 | 前提条件 | tauri-driver インストールに Tauri v2 互換バージョン要件と Nix devshell 推奨の注記を追加 |

### 要判断項目（人間のレビューが必要）

なし。すべての変更は自動適用済み。

---

## 次のステップ

1. `git diff` で変更内容を確認する
2. 満足したら `git commit` する
3. `/checklist-review e2e-test` を再実行してカバレッジ率の改善を確認する
