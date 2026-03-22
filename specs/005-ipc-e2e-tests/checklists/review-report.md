# Checklist Review Report: E2E テスト要件品質

**レビュー日時**: 2026-03-22
**対象チェックリスト**: e2e-tests.md
**レビュー結果サマリー**:
- ✅ Covered: 2 項目
- ⚠️ Partial: 16 項目
- ❌ Gap: 15 項目
- 🔀 Misplaced: 0 項目
- **カバレッジ率**: 6.1% (2/33)

**背景**: E2E テストは US-3 (P3) として IPC テスト (P1/P2) の後に位置づけられている。
spec.md・plan.md は IPC テスト要件を重点的に詳細化しており、E2E 固有の要件は
研究段階の設計判断（research.md, contracts/test-helpers.md）に留まっている項目が多い。

---

## 仕様側の問題（spec.md で対応すべき項目）

機能要件・ユーザー体験・ビジネスロジックに関するギャップ。

| ID | チェック項目 | 判定 | 推奨アクション |
|----|------------|------|--------------|
| G-01 | CHK003 — US-3 Scenario 1 の UI 操作手順が実装粒度で不足 | Gap | US-3 の各シナリオに「操作手順の概要」（例: サイドバーの「+」ボタン→タイトル入力→一覧に表示確認）を追記。ただし UI 詳細は実装時に確定するため、主要操作フローの概要レベルで十分 |
| G-02 | CHK004 — US-3 Scenario 4 の対象ブロックタイプが未定義 | Gap | US-3 Scenario 4 に「テキストブロックを対象とする」等の対象タイプを明記。初期スコープでは 1 タイプで十分（Constitution V: YAGNI） |
| G-03 | CHK007 — E2E 固有の診断情報要件が未定義 | Gap | FR-008 に E2E 補足を追記:「E2E テスト失敗時は WebDriverIO の標準レポーター出力を診断情報とする。スクリーンショット・DOM スナップショットは初期スコープ外」 |
| G-04 | CHK010 — フィルタ条件の具体的プロパティタイプ・演算子が未定義 | Gap | US-3 Scenario 3 に「テキストプロパティの等値フィルタを検証する」等の具体条件を追記 |
| G-05 | CHK012 — `RDBN_DB_PATH` 環境変数のアプリ側サポートが FR に未反映 | Gap | FR を追加:「FR-010: アプリケーションは環境変数 `RDBN_DB_PATH` が設定されている場合、そのパスを SQLite データベースファイルとして使用しなければならない（MUST）。E2E テストのデータベース分離に使用する」 |
| G-06 | CHK017 — pre-merge-commit フックに E2E を含めない判断が未記載 | Gap | CC-005 または FR-007 に補足:「E2E テストは実行コストが高いため pre-merge-commit フック（.githooks/）には含めない。マージ前に手動で `cargo make e2e` を実行する」 |
| G-07 | CHK019 — US-3 の検証方法が E2E 実装粒度で未定義 | Gap | US-3 の各 Then 句に検証ヒントを追記。例: Scenario 1「ページ一覧に新しいページが**タイトルテキストとして**表示される」。詳細な DOM 検証手法は plan/実装に委ねる |
| G-08 | CHK021 — E2E 異常系シナリオのスコープが未定義 | Gap | Out of Scope に追記:「E2E レベルの異常系シナリオ（ネットワーク断、DB ロック、WebView クラッシュ等）は初期スコープ外」 |
| G-09 | CHK022 — ワークフロー横断シナリオのスコープが未定義 | Gap | Out of Scope に追記:「ワークフロー横断 E2E シナリオ（DB 作成→ページ追加→ブロック編集→ビュー確認の一連操作）は初期スコープ外。各ワークフロー独立のシナリオで基本動作を検証する」 |
| G-10 | CHK028 — E2E テスト実行時間の目標値が未定義 | Gap | CC-003 に追記:「E2E テストスイート全体の実行時間目標は初回実装後に実測値を取得して設定する（IPC テストと同方針）」 |
| G-11 | CHK030 — デバッグビルド/リリースビルドの動作差異前提が未記述 | Gap | Dependencies & Assumptions に追記:「E2E テストはデバッグビルドで実行する。デバッグビルド固有の動作（アサーション有効、最適化なし）がテスト結果に影響する可能性があるが、機能の正確性検証には影響しないことを前提とする」 |
| G-12 | CHK031 — `RDBN_DB_PATH` がアプリ側依存として Dependencies に未反映 | Gap | G-05 の FR-010 追加に合わせ、Dependencies & Assumptions に追記:「E2E テストは `RDBN_DB_PATH` 環境変数によるアプリの DB パス切り替えに依存する。この環境変数サポートはアプリケーション側の実装変更（FR-010）を必要とする」 |
| G-13 | CHK009 — US-3 Scenario 2「正しいデータ」の検証フィールドが曖昧 | Partial | US-3 Scenario 2 の Then 句を具体化:「テーブルビューに追加したレコードのプロパティ値が表示される」。data-model.md TableDataDto の構造を参照先として明記 |
| G-14 | CHK011 — CC-001 の E2E データリセット方式が 2 択のまま | Partial | CC-001 を更新: research.md R-005 の決定に基づき「各シナリオ前に全テーブルの行を DELETE する方式でデータリセットする」に確定 |
| G-15 | CHK013 — Edge Cases のクリーンアップ対象が曖昧 | Partial | Edge Cases のクリーンアップ項目を具体化:「tauri-driver プロセスの終了、一時 DB ファイルの削除」 |
| G-16 | CHK024 — 既存データの編集・削除フローのスコープが不明確 | Partial | US-3 に注記を追加:「初期 E2E スコープはワークフローの主要フロー（作成→検証）を対象とする。ページ/データベースレベルの編集・削除フローは後続で追加可能」 |

## 計画側の問題（plan.md で対応すべき項目）

技術的な設計・アーキテクチャ・非機能要件に関するギャップ。

| ID | チェック項目 | 判定 | 推奨アクション |
|----|------------|------|--------------|
| P-01 | CHK001/002 — セレクタ戦略が plan に未記載 | Partial | plan.md の Test Design セクションに追記:「E2E テストのセレクタ戦略: `data-testid` 属性を優先し、`findByTestId()` ヘルパーで統一する（contracts/test-helpers.md 参照）」 |
| P-02 | CHK005 — E2E ライフサイクル管理の設計詳細が散在 | Partial | plan.md の Test Design セクションに E2E ライフサイクルのサブセクションを追加し、research.md R-006 と test-helpers.md の内容を集約:「起動: cargo build → tauri-driver バックグラウンド起動 → waitForApp() で準備完了待機（30s タイムアウト）、停止: kill tauri-driver → 一時 DB 削除」 |
| P-03 | CHK006 — リトライ・タイムアウト戦略が未設計 | Partial | plan.md に追記:「WebDriverIO のデフォルトリトライ機構（waitForExist, waitForDisplayed）を使用する。カスタムリトライ戦略は初期実装後に必要性を評価する」 |
| P-04 | CHK008 — tauri-driver バージョン要件が未記載 | Partial | quickstart.md に tauri-driver のバージョン情報を追記（`cargo install tauri-driver` の出力バージョンを記載）。Nix devshell での管理を推奨 |
| P-05 | CHK014 — FR-005 ワークフローと US-3 シナリオの対応表が plan に未記載 | Partial | plan.md の Domain-to-Test-File Mapping に E2E テスト用のマッピング表を追加: page-workflow→US-3 Scenario 1、editor-workflow→Scenario 4、database-workflow→Scenario 2、view-workflow→Scenario 3 |
| P-06 | CHK018 — ワークフローあたりの最低シナリオ数が不明確 | Partial | plan.md に追記:「各 E2E ワークフローにつき US-3 の 1 シナリオを最低限実装する。バリエーション追加は初期実装後に判断する」 |
| P-07 | CHK020 — SC-004 の前提条件が quickstart のみに記載 | Partial | plan.md から quickstart.md を前提条件の参照先として明記 |
| P-08 | CHK023 — E2E テストのインメモリ状態リセット戦略が未設計 | Gap | plan.md に追記:「E2E テストではアプリを再起動せず DB のみリセットする（R-005）。EditorSession 等のインメモリ状態は各シナリオが独立した UI 操作フロー（open→操作→close）で完結するため、シナリオ間の状態漏洩リスクは低い。問題が発生した場合はシナリオ間でのページリロードを検討する」 |
| P-09 | CHK025 — tauri-driver 異常終了時のリカバリが未設計 | Gap | plan.md の Known Risks に追加:「tauri-driver 異常終了時は Makefile.toml のスクリプトが exit code で検出し、残存プロセスを kill する。テストスイート全体が失敗として報告される」 |
| P-10 | CHK026 — WSLg/headless 要件が plan レベルで未整理 | Partial | plan.md の Technical Context に追記:「WSLg 有効環境では直接実行。WSLg 無効時は `xvfb-run cargo make e2e` で代替（quickstart.md 参照）」 |
| P-11 | CHK027 — WebView 待機戦略が未設計 | Partial | plan.md に追記:「WebView レンダリング完了は WebDriverIO の `waitForExist` / `waitForDisplayed` を使用し、wdio.conf.ts の mocha timeout（30s）を上限とする」 |
| P-12 | CHK029 — E2E 環境の再現性要件が未定義 | Gap | plan.md の Technical Context に追記:「E2E テスト環境は Nix devshell（flake.nix）により再現される。tauri-driver は cargo install で最新互換バージョンを使用する」 |
| P-13 | CHK032 — WebKitGTK 前提が E2E 文脈で未明記 | Partial | plan.md の Technical Context に「WebKitGTK は flake.nix devShell で管理（Constitution Technical Standards 参照）」を追記 |
| P-14 | CHK033 — wdio.conf.ts 設定が plan 本体から参照されていない | Partial | plan.md の Project Structure に「`e2e/wdio.conf.ts` — 設定詳細は [contracts/test-helpers.md](./contracts/test-helpers.md) を参照」のリンクを追記 |

## 配置ミス（Misplaced 項目）

該当なし。

## 意図的な除外の確認

以下の Gap 項目について、意図的に対象外としている場合は理由を記録してください。
（人間が判断して記入するセクション）

| ID | チェック項目 | 除外理由（人間が記入） |
|----|------------|---------------------|
| G-01 | CHK003 — US-3 の UI 操作手順詳細 | |
| G-02 | CHK004 — ブロックタイプの指定 | |
| G-03 | CHK007 — E2E 診断情報要件 | |
| G-04 | CHK010 — フィルタ条件の具体化 | |
| G-05 | CHK012 — RDBN_DB_PATH の FR 化 | |
| G-06 | CHK017 — pre-merge-commit フック判断 | |
| G-07 | CHK019 — 検証方法の具体化 | |
| G-08 | CHK021 — E2E 異常系スコープ | |
| G-09 | CHK022 — ワークフロー横断シナリオ | |
| G-10 | CHK028 — E2E 実行時間目標 | |
| G-11 | CHK030 — debug/release 差異の前提 | |
| G-12 | CHK031 — RDBN_DB_PATH 依存の明記 | |
| P-08 | CHK023 — インメモリ状態リセット | |
| P-09 | CHK025 — tauri-driver 異常終了リカバリ | |
| P-12 | CHK029 — E2E 環境再現性 | |

## Constitution 準拠チェック

### カバーされている原則

| Constitution 原則 | 対応するチェック項目 |
|------------------|-------------------|
| I. Local-First Product Integrity | CHK012, CHK023, CHK031 — テスト用 DB 分離・一時ファイル管理で本番データ保護を検証 |
| II. Domain-Faithful Information Model | CHK014, CHK015 — US-3 シナリオとワークフローが 6 ドメイン語彙と一致 |
| IV. Test-First Delivery and Quality Gates | CHK016, CHK017, CHK020 — QA パイプライン統合・独立実行・品質ゲート構成 |
| V. Safe Rust, SOLID, YAGNI | CHK004, CHK021, CHK022, CHK028 — E2E スコープの適正化に YAGNI 判断が必要 |
| VII. 防御的エラーハンドリング | CHK007, CHK025 — テスト失敗時の診断・リカバリ要件 |

### カバーされていない原則

以下の Constitution 原則に対応するチェック項目が不足しています:

- **III. Typed Boundaries and DDD**: E2E テストにおける IPC 型境界（CommandError のフロントエンド側表示、DTO のレンダリング検証）に対応するチェック項目がない。ただし E2E テストは UI レベルの検証であり、型境界は IPC テスト（別チェックリスト: ipc-tests.md）でカバーされるため、本チェックリストでの不足は許容範囲
- **VI. Rust ドキュメント標準**: E2E テストは TypeScript で記述するため直接適用されない。E2E ヘルパー（TypeScript）のドキュメント基準はチェックリストに含まれていないが、Constitution VI は Rust 固有であり、E2E チェックリストの範囲外

### 矛盾・過剰設計の指摘

| チェック項目 | 関連する原則 | 指摘内容 |
|------------|------------|---------|
| CHK029 | V (YAGNI) | 「OS、WebKitGTK バージョン、tauri-driver バージョン、Node.js バージョン」の完全な再現性要件は Nix devshell で既に担保されており、spec レベルで個別バージョンを列挙する必要性は低い。flake.nix への参照で十分 |
| CHK001/002 | V (YAGNI) | セレクタ戦略を spec.md の要件として定義するのは過剰。plan レベルの設計判断（contracts/test-helpers.md）で十分であり、spec には含めない方が適切 |
| CHK028 | V (YAGNI) | E2E テスト実行時間の目標値を初期スコープで定めるのは時期尚早。CC-003 と同様「初回実装後に実測値を取得」方針で十分 |
| CHK003/019 | V (YAGNI) | UI 操作の具体的手順・DOM 検証手法を spec レベルで要求するのは過剰設計の可能性。E2E テストの spec は業務シナリオレベルが適切であり、UI 実装詳細は plan/実装フェーズで確定すべき |

### 総合評価

E2E テスト（P3）の要件は IPC テスト（P1/P2）に比べて詳細化が不十分だが、これは優先度に
応じた意図的な段階的詳細化と見なせる。

**重要な Gap（対応必須）**:
1. **G-05/G-12 (RDBN_DB_PATH)** — E2E テストの実行に不可欠なアプリ側変更が FR にも Dependencies にも未反映。実装フェーズで混乱を招く可能性が高い
2. **G-08/G-09 (スコープ明確化)** — E2E 異常系・横断シナリオのスコープが未定義のまま実装すると、スコープクリープのリスクがある

**YAGNI で軽減される Gap（対応任意）**:
- G-01, G-02, G-04, G-07 (UI 詳細) — 実装フェーズで自然に確定する
- G-10, G-11 (時間目標, debug/release) — 初回実装後に実測・判断で十分
- P-08, P-09, P-12 (plan 側 Gap) — plan レベルの追記で解消可能

**結論**: RDBN_DB_PATH の FR 化（G-05/G-12）とスコープ明確化（G-08/G-09）の 4 項目は
`/checklist-apply` 前に対応を推奨する。それ以外の Gap は YAGNI 原則に基づき実装フェーズ
での対応で問題ない。
