# Checklist Review Report: IPC テストおよび E2E テストの追加

**レビュー日時**: 2026-03-22
**対象チェックリスト**: requirements.md, ipc-tests.md
**レビュー結果サマリー**:

### requirements.md（仕様品質チェックリスト — 16 項目）
- ✅ Covered: 6 項目
- ⚠️ Partial: 10 項目
- ❌ Gap: 0 項目
- 🔀 Misplaced: 0 項目
- **カバレッジ率**: 37.5%（6/16）

### ipc-tests.md（IPC テスト要件品質チェックリスト — 32 項目）
- ✅ Covered: 3 項目（CHK014, CHK015, CHK017）
- ⚠️ Partial: 16 項目
- ❌ Gap: 13 項目
- 🔀 Misplaced: 0 項目
- **カバレッジ率**: 9.4%（3/32）

### 合計（48 項目）
- ✅ Covered: 9 項目
- ⚠️ Partial: 26 項目
- ❌ Gap: 13 項目
- 🔀 Misplaced: 0 項目
- **カバレッジ率**: 18.8%（9/48）

> **前回レポートとの差分**: 前回は requirements.md を「全項目 Covered」として対象外としていたが，
> 今回 spec.md と再突合した結果 10 項目が Partial に降格。合計カバレッジ率は 9% → 18.8% に変動
> （分母が 32 → 48 に拡大したため率は上昇したが，Partial 項目が 16 → 26 に増加）。

---

## 仕様側の問題（spec.md で対応すべき項目）

機能要件・ユーザー体験・ビジネスロジックに関するギャップ。
spec.md の修正で解決すべき項目を列挙する。

| ID | チェック項目 | 判定 | 推奨アクション |
|----|------------|------|--------------|
| G-01 | CHK001: CRUD 種別ごとの正常系の定義 | Partial | FR-001 の補足として「作成→ID 返却，一覧→全件返却，取得→一致確認，更新→反映確認，削除→不在確認」の基本パターンを定義。または data-model.md への相互参照を追加 |
| G-02 | CHK004: US-2 の異常系シナリオが 3 ドメインのみ | Gap | US-2 に Page（存在しないページの更新），Property（重複名作成・不正な型設定），Table（無効な DB/Page ID での追加）の受入シナリオを追加 |
| G-03 | CHK005: DTO フィールド変換テストの FR が不在 | Gap | FR-004 を拡張するか新規 FR を追加し「DTO の各フィールドが data-model.md の変換仕様に従って正確に変換されることを検証する（MUST）」を定義 |
| G-04 | CHK006: EditorSession ステートフルフロー要件の欠落 | Gap | US-2 に「open→操作→save→close の一連フロー」の IPC テスト要件を追加。または Edge Cases に Editor フロー検証を追加 |
| G-05 | CHK008: CC-003 の時間目標が定量化されていない | Gap | CC-003 に具体的な SLA（例:「IPC テストスイート全体 60 秒以内」）を追加するか，意図的に定量化しない理由を記載 |
| G-06 | CHK009: SC-002 が定性的で測定不能 | Gap | SC-002 を「全 38 コマンドに正常系テスト＋各ドメインの主要エラーパスに異常系テストが存在する」等の客観指標に改定。または SC-001 と統合 |
| G-07 | CHK010: 「モック」の定義 | Partial | FR-002 に「本仕様における『モック』とは実データベースを使用しないスタブ・フェイク実装を指す。テスト用一時 SQLite ファイルはモックに該当しない」と明記 |
| G-08 | CHK011: ドメイン別バリデーション条件 | Partial | US-1 シナリオ 3 に「各ドメインのバリデーション条件は data-model.md エラー種別マッピングに従う」の相互参照を追加 |
| G-09 | CHK012: 「適切なエラー種別」の期待値定義 | Partial | US-1 シナリオ 2 に「期待されるエラー種別は data-model.md のエラー種別マッピング表に準拠する」を追記 |
| G-10 | CHK018: 「正しい結果」の検証基準 | Gap | US-1 シナリオ 1 に「正しい結果とは返却 DTO の各フィールドが入力値および data-model.md の変換仕様と一致すること」を定義 |
| G-11 | CHK019: 「適切に処理される」の具体化 | Gap | US-2 シナリオ 3 を「参照先プロパティ削除時はビュー条件から自動除外され，エラーを返さない」等の具体的動作に置換 |
| G-12 | CHK021: Editor セッション異常系の網羅 | Partial | US-2 に「二重 open（既に開かれているページを再度開く）」「close 後のブロック操作」の異常系シナリオを追加 |
| G-13 | CHK022: カスケード削除のドメイン帰属 | Partial | US-2 シナリオ 1 に「カスケード削除テストは Database ドメインに含め，databases→pages→blocks，databases→properties→property_values，databases→views の 3 系統を検証する」と明記 |
| G-14 | CHK023: table_commands の追加操作異常系 | Partial | US-1 シナリオ 2 の対象を「取得・更新・削除コマンド」から「取得・更新・削除・追加コマンド」に拡大 |
| G-15 | CHK024: 並行呼び出しテスト要件の不在 | Gap | Edge Case を FR に昇格するか，Out of Scope に移動し理由を記載 |
| G-16 | CHK025: 大量レコードテスト要件の不在 | Gap | Edge Case を FR に昇格するか，Out of Scope に移動し理由を記載 |
| G-17 | CHK027: コマンド固有の境界値要件の不在 | Gap | US-2 に境界値テスト要件を追加（reorder_properties に空配列，toggle_group_collapsed に不在グループ名等）。または P2 スコープとして段階的追加方針を明記 |
| G-18 | CHK030: DTO 型安全性検証の具体化 | Gap | CC-004 に「シリアライズ/デシリアライズ往復テスト」「フィールド網羅検証」等の具体的検証方法を定義 |
| G-19 | CHK032: AppState pub フィールド依存リスク | Gap | Dependencies/Assumptions セクションを spec.md に追加し，AppState の pub フィールドへの直接依存と構造変更リスクを記載 |
| G-20 | requirements.md: 実装詳細の混入 | Partial | FR-002 の「SqlitePool」→「テスト用データベース接続プール」，FR-006 の具体フレームワーク名を Clarifications への参照に置換。または意図的決定として注記 |
| G-21 | requirements.md: 前提条件の未文書化 | Partial | spec.md に「前提条件（Assumptions）」セクションを追加し，init_pool() 依存，マイグレーション適用，FK 有効化を明記 |
| G-22 | requirements.md: SC-002 の測定不能性 | Partial | SC-002 を客観指標に改定（G-06 と同一アクション） |
| G-23 | requirements.md: SC-004 の技術依存 | Partial | SC-004 のツール名（cargo make qa/e2e）を技術非依存の表現に置換するか，意図的な制約として注記 |

## 計画側の問題（plan.md で対応すべき項目）

技術的な設計・アーキテクチャ・非機能要件に関するギャップ。
plan.md の修正で解決すべき項目を列挙する。

| ID | チェック項目 | 判定 | 推奨アクション |
|----|------------|------|--------------|
| P-01 | CHK002: ドメイン横断操作のテスト所属先 | Partial | plan.md の Project Structure に「table_commands_test.rs には add_page_to_database 等のドメイン横断操作テストを含む」旨を明記 |
| P-02 | CHK003: エラー種別マッピングの相互参照 | Partial | plan.md のテスト設計方針に data-model.md エラー種別マッピング表への参照を追加し，各ドメインテストで検証すべき variant を列挙 |
| P-03 | CHK007: パニック時クリーンアップの明示 | Partial | data-model.md に TempDbGuard/Drop の記載済み。spec.md FR-003 に「パニック時を含む」と補足すれば十分 |
| P-04 | CHK013: ドメイン分類とテストファイルの対応表 | Partial | plan.md に spec.md の 6 ドメイン分類と tests/ ディレクトリのファイルマッピング表を追加 |
| P-05 | CHK016: FR-004 とエラー variant の対応 | Partial | plan.md のテスト設計に「各ドメインテストで検証するエラー variant 一覧」を追加 |
| P-06 | CHK026: 並列実行下の DB 分離保証 | Partial | plan.md に「cargo-nextest のデフォルト並列実行において uuid_v7 ベースの一時ディレクトリにより DB 分離が保証される」を明記 |
| P-07 | CHK028/CHK008: テスト実行時間の目安 | Gap | plan.md に想定実行時間（例:「38 テスト × DB 作成/破棄で約 30-60 秒を想定」）を参考値として追記 |
| P-08 | CHK029: テスト失敗時の診断情報 | Partial | plan.md のテスト設計方針に「assert マクロのカスタムメッセージで入力値・期待値・実際値を出力する」方針を追加 |
| P-09 | CHK031: init_pool() 依存の前提明示 | Partial | contracts/test-helpers.md に記載済み。spec.md の Dependencies & Assumptions にも追記（G-21 と連動） |
| P-10 | CHK032: AppState 構造変更の影響 | Gap | plan.md の Complexity Tracking に「テストは AppState の pub フィールドに直接依存し，構造変更時はテストヘルパーの修正が必要」とリスク認識を追加 |

## 配置ミス（Misplaced 項目）

該当なし。spec.md と plan.md の責任分界は概ね適切に維持されている。

## 意図的な除外の確認

以下の Gap 項目について、意図的に対象外としている場合は理由を記録してください。
（人間が判断して記入するセクション）

| ID | チェック項目 | 除外理由（人間が記入） |
|----|------------|---------------------|
| G-02 | 全 6 ドメインの異常系シナリオ（US-2 に Page/Property/Table が不足） | |
| G-03 | DTO フィールド変換の FR 定義 | |
| G-04 | Editor ステートフルフロー要件 | |
| G-05 | CC-003 の定量的 SLA | |
| G-06 | SC-002 の客観指標 | |
| G-10 | 「正しい結果」の検証基準 | |
| G-11 | ビューのプロパティ削除時の具体的期待動作 | |
| G-15 | 並行呼び出しテスト（CHK024） | |
| G-16 | 大量レコードテスト（CHK025） | |
| G-17 | コマンド固有の境界値テスト（CHK027） | |
| G-18 | DTO 型安全性の検証方法（CHK030） | |
| G-19 | AppState 構造変更リスク（CHK032） | |
| P-07 | テスト実行時間 SLA | |
| P-10 | AppState 構造変更の影響分析 | |

---

## Constitution 準拠チェック

### カバーされている原則

| Constitution 原則 | 対応するチェック項目 |
|------------------|-------------------|
| I. Local-First Product Integrity | CHK007（DB 分離/クリーンアップ），CHK015（テスト間分離），CHK026（並列実行分離）— FR-003, CC-001, CC-002 で対応 |
| II. Domain-Faithful Information Model | CHK002（ドメイン横断操作），CHK013（ドメイン分類一貫性），CHK020（ドメイン別 CRUD）— FR-001 の 6 ドメイン分類で対応 |
| III. Typed Boundaries and DDD | CHK003（エラー種別），CHK005（DTO 変換），CHK016（エラー variant），CHK030（型安全性）— FR-004, CC-004 で部分対応 |
| IV. Test-First Delivery and Quality Gates | CHK014（38 コマンド網羅），CHK017（正常系テスト基準）— FR-007, SC-001, SC-004 で対応 |
| V. Safe Rust, SOLID, Maintainability | CHK032（AppState 構造依存）— plan.md Constitution Check V で言及 |
| VII. 防御的エラーハンドリング | CHK004（異常系カバレッジ），CHK012（エラー期待値），CHK019（曖昧な期待動作）— FR-004, US-2 で部分対応 |

### カバーされていない原則

以下の Constitution 原則に対応するチェック項目が不足しています:

- **VI. Rust ドキュメント標準**: チェックリストに「テストヘルパーの pub(crate) 関数（`setup_test_state`, `TempDbGuard`）に `///` ドキュメントコメントが付与されているか」「`cargo doc --no-deps` がクリーンビルドされるか」の検証項目がない。plan.md Constitution Check VI で「新規の pub テストヘルパー関数には /// ドキュメントコメントを付与する」と記載済みだが，チェックリスト側に反映されていない。
- **IV. Test-First Delivery（TDD サイクル順序）**: 内部関数抽出時の Red-Green-Refactor 順序遵守を検証するチェック項目がない。本フィーチャー自体がテスト追加であり TDD の「Red」フェーズに相当するが，実装順序の品質ゲートは未定義。
- **IV. Test-First Delivery（品質ゲート統合の具体化）**: `cargo make qa` への IPC テスト統合（FR-007）と `cargo make e2e` タスク追加（CC-005）に対する Makefile.toml 変更の検証項目がない。

### 矛盾・過剰設計の指摘

| チェック項目 | 関連する原則 | 指摘内容 |
|------------|------------|---------|
| CHK024 | V（YAGNI） | 「並行呼び出しテスト」は Edge Case に記載されるが FR 化は初期スコープとして過剰。Constitution V の YAGNI 原則に基づき，初期実装では DB 分離で間接的に安全性を確保し並行テストは後続フェーズに延期するのが適切 |
| CHK025 | V（YAGNI） | 「大量レコードテスト」も同様。spec.md の Out of Scope に「パフォーマンスベンチマーク」が含まれており，大量レコードテストはこの延長線上と解釈可能。初期スコープでの FR 化は過剰 |
| CHK008/CHK028 | V（YAGNI） | テスト実行時間の定量的 SLA はテスト数確定・初回計測前は設定困難。初期は「数分以内」の定性基準で十分であり計測後に具体化するアプローチが YAGNI に適合 |

### 総合評価

Constitution との整合性は概ね良好。主な改善ポイントは以下の 3 点:

1. **Principle III（型付き境界）との連携強化**: data-model.md にエラー種別マッピングや DTO 検証項目が詳細に定義されているが，spec.md の FR からの相互参照がない。spec.md → data-model.md の明示的な参照を追加することで CHK003, CHK005, CHK011, CHK012, CHK016 の 5 項目を Partial → Covered に昇格できる。**これが最も費用対効果の高い改善アクション**。

2. **Principle VI（ドキュメント標準）のチェック項目追加**: テストヘルパーの公開 API ドキュメントに関するチェック項目を ipc-tests.md に追加すべき。

3. **YAGNI 判断の明文化**: CHK024, CHK025, CHK027 の Gap 項目は YAGNI の観点から意図的に除外した可能性があるが判断が文書化されていない。「意図的な除外の確認」セクションに理由を記入することで解消できる。
