# Checklist Apply Changelog: IPC テストおよび E2E テストの追加

**実行日時**: 2026-03-22
**入力**: review-report.md（ipc-tests.md 由来の項目）
**モード**: 差分更新（非破壊）

---

## 変更統計

- spec.md: 7 箇所 補完 / 5 箇所 追記 / 0 箇所 移動受入
- plan.md: 0 箇所 補完 / 3 箇所 追記（Test Design, Domain Mapping, Known Risks）
- 新規作成ファイル: なし
- 簡素化提案（要判断）: 0 箇所

---

## 変更詳細

### spec.md の変更

| レポート ID | 変更種別 | 対象箇所 | 変更内容の要約 |
|-----------|---------|---------|--------------|
| G-01 | 補完 | FR-001 | CRUD 操作種別ごとの基本検証パターン（作成→ID 返却，一覧→全件，取得→一致，更新→反映，削除→不在）を追記 |
| G-01, G-10 | 補完 | US-1 シナリオ 1 | 「正しい結果」を data-model.md の DTO 変換仕様との一致として定義 |
| G-02 | 追記 | US-2 シナリオ 4-6 | Page（notFound），Property（duplicatePropertyName），Table（databaseNotFound）の異常系シナリオを追加 |
| G-03 | 追記 | FR-009（新規） | DTO フィールド変換検証の FR を新設。data-model.md「IPC 境界で検証する項目」への参照 |
| G-04 | 追記 | US-2 シナリオ 9 | Editor ステートフルフロー（open→操作→save→close）のテスト要件を追加 |
| G-05 | 補完 | CC-003 | 「初回実装後に実測値を取得し SLA を設定」の YAGNI 整合的な注記を追加 |
| G-06 | 補完 | SC-002 | 定性的な記述を「全 38 コマンドに正常系＋主要エラーパスに異常系テストが存在する」に改定 |
| G-07 | 補完 | FR-002 | 「モック」の定義（スタブ・フェイク＝モック，一時 SQLite ファイル≠モック）を追記 |
| G-08 | 補完 | US-1 シナリオ 3 | data-model.md エラー種別マッピングへの相互参照を追加 |
| G-09, G-14 | 補完 | US-1 シナリオ 2 | data-model.md 参照追加＋対象を「取得・更新・削除」→「取得・更新・削除・追加」に拡大 |
| G-11 | 補完 | US-2 シナリオ 3 | 「適切に処理される」→「条件を自動除外しエラーを返さない」に具体化 |
| G-12 | 追記 | US-2 シナリオ 7-8 | 二重 open，close 後のブロック操作の異常系シナリオを追加 |
| G-13 | 補完 | US-2 シナリオ 1 | カスケード削除の 3 系統（pages→blocks, properties→property_values, views）と Database ドメイン帰属を明記 |
| G-15 | 追記 | Out of Scope | 並行呼び出しテストを YAGNI 理由付きで Out of Scope に移動 |
| G-16 | 追記 | Out of Scope | 大量レコードテストを Out of Scope に移動 |
| G-17 | 補完 | Edge Cases | コマンド固有の境界値テストを P2 スコープとして注記 |
| G-18 | 補完 | CC-004 | DTO 型安全性の具体的な検証方法（FR-009 参照）を追記 |
| G-19, G-21, P-09 | 追記 | Dependencies & Assumptions（新規セクション） | init_pool() 依存，AppState pub フィールド依存，内部関数抽出前提を明記 |
| P-03 | 補完 | FR-003 | 「テスト後に削除」→「テスト後に削除（パニック時を含む）」に補完 |

### plan.md の変更

| レポート ID | 変更種別 | 対象箇所 | 変更内容の要約 |
|-----------|---------|---------|--------------|
| P-01, P-04 | 追記 | Domain-to-Test-File Mapping（新規サブセクション） | 6 ドメインとテストファイルの対応表。ドメイン横断操作の帰属を明示 |
| P-02, P-05, P-06, P-07, P-08 | 追記 | Test Design（新規セクション） | エラー variant 検証方針，実行時間見積もり（30-60 秒），並列実行 DB 分離，診断情報方針 |
| P-10 | 追記 | Known Risks（Complexity Tracking 内） | AppState pub フィールド依存リスクと setup_test_state() への集約による緩和策 |

### 要判断項目（人間のレビューが必要）

なし。すべての変更は自動適用済み。

---

## 次のステップ

1. `git diff` で変更内容を確認する
2. 満足したら `git commit` する
3. `/checklist-review` を再実行してカバレッジ率の改善を確認する
