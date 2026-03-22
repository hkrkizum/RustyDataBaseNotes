# Checklist Review Report: IPC テストおよび E2E テストの追加

**レビュー日時**: 2026-03-22
**対象チェックリスト**: ipc-tests.md, requirements.md
**レビュー結果サマリー**:

- ✅ Covered: 3 項目 (CHK014, CHK015, CHK017)
- ⚠️ Partial: 16 項目 (CHK001, CHK002, CHK003, CHK007, CHK010, CHK011, CHK012, CHK013, CHK016, CHK020, CHK021, CHK022, CHK023, CHK026, CHK029, CHK031)
- ❌ Gap: 13 項目 (CHK004, CHK005, CHK006, CHK008, CHK009, CHK018, CHK019, CHK024, CHK025, CHK027, CHK028, CHK030, CHK032)
- 🔀 Misplaced: 0 項目
- **カバレッジ率**: 9% (3/32)

> **注**: requirements.md は全項目が既に ✅ でありレビュー対象外。本レポートは ipc-tests.md（32 項目）を対象とする。

---

## 仕様側の問題（spec.md で対応すべき項目）

機能要件・ユーザー体験・ビジネスロジックに関するギャップ。

| ID | チェック項目 | 判定 | 推奨アクション |
|----|------------|------|--------------|
| G-01 | CHK001: CRUD 種別ごとの正常系の定義 | Partial | FR-001 の補足として、CRUD 操作種別ごとの期待動作（作成→ID 返却、一覧→全件返却、取得→一致確認、更新→反映確認、削除→不在確認）を定義する |
| G-02 | CHK002: ドメイン横断操作の所属ドメイン | Partial | FR-001 に「table_commands に含まれる page 操作（add_page_to_database 等）は Table ドメインのテストに含める」旨を明記する |
| G-03 | CHK003: ドメイン別エラー種別の仕様組込 | Partial | FR-004 に data-model.md のエラー種別マッピングへの参照を追加し、各ドメインの検証対象 variant を明示する |
| G-04 | CHK004: 全 6 ドメインの異常系シナリオ | Gap | US-2 に Page（存在しないページの更新）、Property（重複名作成、不正な型設定）、Table（無効な DB/Page ID での追加）の受入シナリオを追加する |
| G-05 | CHK005: DTO フィールド変換の FR 定義 | Gap | FR を新設し「IPC テストは DTO の各フィールドが data-model.md の変換仕様に従って正確に変換されることを検証する（MUST）」を追加する |
| G-06 | CHK006: Editor ステートフルフロー要件 | Gap | FR を新設し「Editor ドメインの IPC テストは個別コマンドテストに加え、open→操作→save→close の一連フローテストを含む（SHOULD）」を追加する |
| G-07 | CHK009: SC-002 の測定可能性 | Gap | SC-002 を「全 38 コマンドに対して正常系テストが存在し、かつ各ドメインの主要エラーパスに対して異常系テストが存在する」等の客観的基準に改定する |
| G-08 | CHK010: 「モック」の定義 | Partial | FR-002 に「本仕様における『モック』とは、実際のデータベースを使用しないスタブ・フェイク実装を指す。テスト用一時 SQLite ファイルはモックに該当しない」と明記する |
| G-09 | CHK011: ドメイン別バリデーション条件 | Partial | US-1 シナリオ3 を拡充し、主要ドメインのバリデーション条件例（Database: 空タイトル、Page: 空タイトル、Property: 空名称・重複名・不正型等）を列挙する。または data-model.md への参照を追加する |
| G-10 | CHK012: エラー期待値の定義 | Partial | US-1 シナリオ2 に「期待されるエラー種別は data-model.md のエラー種別マッピングに従う」と参照を追加する |
| G-11 | CHK018: 「正しい結果」の検証基準 | Gap | US-1 シナリオ1 に「正しい結果とは、返却 DTO の各フィールドが入力値および data-model.md の変換仕様と一致すること」を定義する |
| G-12 | CHK019: ビューのプロパティ削除時の動作 | Gap | US-2 シナリオ3 の「適切に処理される」を「参照先プロパティが削除された条件は無視される（ビュー操作はエラーにならない）」等の具体的期待動作に置換する |
| G-13 | CHK021: Editor セッション異常系の網羅 | Partial | US-2 に「二重 open（既に開かれているページを再度開く）」「close 後のブロック操作」の異常系シナリオを追加する |
| G-14 | CHK022: カスケード削除のドメイン帰属 | Partial | US-2 シナリオ1 に「カスケード削除テストは Database ドメインテストに含め、databases→pages→blocks、databases→properties→property_values、databases→views の 3 系統を検証する」と明記する |
| G-15 | CHK023: table_commands の追加操作異常系 | Partial | US-1 シナリオ2 の対象を「取得・更新・削除コマンド」から「取得・更新・削除・追加コマンド」に拡大する |

## 計画側の問題（plan.md で対応すべき項目）

技術的な設計・アーキテクチャ・非機能要件に関するギャップ。

| ID | チェック項目 | 判定 | 推奨アクション |
|----|------------|------|--------------|
| P-01 | CHK007: パニック時のクリーンアップ保証 | Partial | plan.md に TempDbGuard の Drop 実装がパニック時にもクリーンアップを保証する旨が記載済み（data-model.md にも記述あり）。spec.md FR-003 に「パニック時を含む」と補足すれば十分 |
| P-02 | CHK008/CHK028: テスト実行時間 SLA | Gap | plan.md の Performance Goals に具体的な SLA（例: 「IPC テストスイート全体 < 60 秒」）を追加する。または「初回計測後に SLA を設定」と明記する |
| P-03 | CHK024: 並行呼び出しテスト戦略 | Gap | Edge Case として記載済みだが、テスト戦略が未定義。plan.md に「並行テストは P3 以降のスコープとし、初期実装では各テストの DB 分離で間接的に安全性を確保する」と明記するか、tokio::spawn を用いた並行テスト設計を追加する |
| P-04 | CHK025: 大量レコードテスト戦略 | Gap | 同上。plan.md に「大量レコードテストは P3 以降のスコープとする」と明記するか、テストデータ生成ヘルパーの設計を追加する |
| P-05 | CHK026: 並列テスト実行下の DB 分離 | Partial | plan.md の TempDbGuard 設計（uuid_v7 ディレクトリ）で暗黙的に対応済み。明示的に「cargo-nextest の並列実行に対応」と記載すれば十分 |
| P-06 | CHK027: コマンド固有の境界値テスト | Gap | plan.md に各ドメインの主要境界値テストケース例（reorder_properties に空配列、toggle_group_collapsed に不在グループ等）を追加する |
| P-07 | CHK029: 診断情報の出力仕様 | Partial | plan.md のテストヘルパー設計に「テスト失敗時にカスタムメッセージ（入力値、期待値、実際値）を assert マクロで出力する」方針を追加する |
| P-08 | CHK030: DTO 型安全性の検証方法 | Gap | plan.md に CC-004 の具体的な検証手法を追加する（例: 「正常系テストで返却 DTO の全フィールドを assert で検証することで型安全性を担保する」） |
| P-09 | CHK031: init_pool() 依存の前提明示 | Partial | contracts/test-helpers.md に記載済み。spec.md の Dependencies & Assumptions にも「IPC テストは database::init_pool() によるマイグレーション適用と FK 有効化を前提とする」と追記する |
| P-10 | CHK032: AppState 構造変更の影響 | Gap | plan.md の Complexity Tracking または spec.md の Dependencies に「テストは AppState の pub フィールドに直接依存するため、構造変更時はテストヘルパーの修正が必要」とリスク認識を追加する |

## 配置ミス（Misplaced 項目）

該当なし。

## 意図的な除外の確認

以下の Gap 項目について、意図的に対象外としている場合は理由を記録してください。

| ID | チェック項目 | 除外理由（人間が記入） |
|----|------------|---------------------|
| G-04 | 全 6 ドメインの異常系シナリオ（US-2 に Page/Property/Table が不足） | |
| G-05 | DTO フィールド変換の FR 定義 | |
| G-06 | Editor ステートフルフロー要件 | |
| G-07 | SC-002 の測定可能性 | |
| G-11 | 「正しい結果」の検証基準 | |
| G-12 | ビューのプロパティ削除時の具体的期待動作 | |
| P-02 | テスト実行時間 SLA | |
| P-03 | 並行呼び出しテスト戦略 | |
| P-04 | 大量レコードテスト戦略 | |
| P-06 | コマンド固有の境界値テスト | |
| P-08 | DTO 型安全性の検証方法 | |
| P-10 | AppState 構造変更の影響 | |

---

## Constitution 準拠チェック

### カバーされている原則

| Constitution 原則 | 対応するチェック項目 |
|------------------|-------------------|
| I. Local-First Product Integrity | CHK007 (DB 分離/クリーンアップ), CHK015 (テスト間分離), CHK026 (並列実行分離) — FR-003, CC-001, CC-002 で対応 |
| II. Domain-Faithful Information Model | CHK002 (ドメイン横断操作), CHK013 (ドメイン分類一貫性), CHK020 (ドメイン別 CRUD) — FR-001 の 6 ドメイン分類で対応 |
| III. Typed Boundaries and DDD | CHK003 (エラー種別), CHK005 (DTO 変換), CHK016 (エラー variant), CHK030 (型安全性) — FR-004, CC-004 で部分対応 |
| IV. Test-First Delivery and Quality Gates | CHK014 (38 コマンド網羅), CHK017 (正常系テスト基準) — FR-007, SC-001, SC-004 で対応 |
| V. Safe Rust, SOLID, Maintainability | CHK032 (AppState 構造依存) — plan.md Constitution Check で言及 |
| VII. 防御的エラーハンドリング | CHK004 (異常系カバレッジ), CHK012 (エラー期待値), CHK019 (曖昧な期待動作) — FR-004, US-2 で部分対応 |

### カバーされていない原則

以下の constitution 原則に対応するチェック項目が不足しています:

- **VI. Rust ドキュメント標準**: チェックリストに「テストヘルパーの pub(crate) 関数にドキュメントコメントが必要か」を検証する項目がない。plan.md Constitution Check VI で「新規の pub テストヘルパー関数には /// ドキュメントコメントを付与する」と記載済みだが、チェックリスト側に反映されていない。
- **IV. Test-First Delivery（品質ゲート統合の検証）**: `cargo make qa` への IPC テスト統合（FR-007）と `cargo make e2e` の独立タスク化（CC-005）に対する具体的な Makefile.toml 変更の検証項目がない。

### 矛盾・過剰設計の指摘

| チェック項目 | 関連する原則 | 指摘内容 |
|------------|------------|---------|
| CHK024 | V (YAGNI) | 「並行呼び出しテスト」は Edge Case に記載されるが、FR 化は初期スコープとして過剰な可能性。Constitution V の YAGNI 原則に基づき、初期実装では DB 分離で間接的に安全性を確保し、並行テストは後続フェーズに延期するのが適切 |
| CHK025 | V (YAGNI) | 「大量レコードテスト」も同様。spec.md の Out of Scope に「パフォーマンスベンチマーク」が含まれており、大量レコードテストはこの延長線上と解釈可能。初期スコープでの FR 化は過剰 |
| CHK008/CHK028 | V (YAGNI) | テスト実行時間の定量的 SLA は、テスト数が確定し初回計測を行うまでは設定困難。初期は「数分以内」の定性基準で十分であり、計測後に具体化するアプローチが YAGNI に適合 |
