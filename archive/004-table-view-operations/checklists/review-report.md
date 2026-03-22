# Checklist Review Report: テーブルビュー操作拡張（第3回・最終）

**レビュー日時**: 2026-03-22（第2回 checklist-apply 後の最終レビュー）
**対象チェックリスト**: data-integrity.md, type-operator.md, ux.md
**レビュー結果サマリー**:
- ✅ Covered: 79 項目
- ⚠️ Partial: 9 項目
- ❌ Gap: 0 項目
- 🔀 Misplaced: 0 項目
- **カバレッジ率**: 89.8%（Covered のみ），100%（Covered + Partial）

| チェックリスト | Covered | Partial | Gap | カバレッジ率 | 推移 |
|---|---|---|---|---|---|
| data-integrity.md (28) | 24 | 4 | 0 | 85.7% | 39.3% → 85.7% → 85.7% |
| type-operator.md (28) | 27 | 1 | 0 | 96.4% | 17.9% → 78.6% → 96.4% |
| ux.md (32) | 28 | 4 | 0 | 87.5% | 3.1% → 62.5% → 87.5% |
| **合計 (88)** | **79** | **9** | **0** | **89.8%** | **19.3% → 75.0% → 89.8%** |

---

## 全体推移

| 指標 | 第1回 | 第2回 | 第3回（最終） |
|---|---|---|---|
| Covered | 17 (19.3%) | 66 (75.0%) | 79 (89.8%) |
| Partial | 25 (28.4%) | 15 (17.0%) | 9 (10.2%) |
| Gap | 44 (50.0%) | 7 (8.0%) | 0 (0.0%) |
| Misplaced | 2 (2.3%) | 0 (0.0%) | 0 (0.0%) |

**Gap: 44 → 7 → 0（全解消）**

---

## 残存 Partial 項目（9件）

すべて低リスク。実装に大きな影響を与える曖昧性はない。

### data-integrity.md（4件）

| ID | 内容 | リスク |
|----|------|-------|
| DI-CHK011 | マイグレーション失敗時のロールバック要件 | 低: sqlx::migrate!() の標準トランザクション動作で十分 |
| DI-CHK013 | 既存 DB 0 件時のマイグレーション | 低: INSERT...SELECT の SQL セマンティクスで自明 |
| DI-CHK017 | views 書き込み失敗時のユーザー通知方法 | 低: CC-004 のエラー伝達 + 既存 toast パターンで対応可能 |
| DI-CHK025 | collapsed_groups の旧値残存クリーンアップ | 低: 値ベース管理で残存は無害 |

### type-operator.md（1件）

| ID | 内容 | リスク |
|----|------|-------|
| TO-CHK018 | FilterValue の具体的な不正入力バリデーションルール一覧 | 低: Tagged Union + invalidFilterValue エラー型で構造的にカバー |

### ux.md（4件）

| ID | 内容 | リスク |
|----|------|-------|
| UX-CHK023 | カテゴリ単位の一括クリア（フィルタのみ等） | 低: IPC 契約で技術的に可能。UI 上は個別削除で代替可 |
| UX-CHK024 | ソート・グルーピングのアクティブ状態サマリ表示 | 低: フィルタのバッジ表示は定義済み。他は実装時に判断可 |
| UX-CHK025 | フィルタ非表示ページ追加時の個別通知 | 低: FR-011 の動作は明確。通知は実装裁量で対応可 |
| UX-CHK032 | 3 機能同時操作の複合受入シナリオ | 低: 2 機能組み合わせは複数定義済み。3 機能は統合テストで検証 |

---

## Constitution 準拠チェック

### カバーされている原則

| Constitution 原則 | 状況 |
|------------------|------|
| I. Local-First Product Integrity | ✅ 完全カバー |
| II. Domain-Faithful Information Model | ✅ 完全カバー |
| III. Typed Boundaries and DDD | ✅ 完全カバー |
| IV. Test-First Delivery | ✅ 受入シナリオ充実 |
| V. Safe Rust / SOLID / Maintainability | ✅ YAGNI 明示的除外あり |
| VII. Defensive Error Handling | ✅ FR-015 + ViewError + CC-004 |

### カバーされていない原則

- **VI. Rust ドキュメント標準**: チェックリストのスコープ外（別途対応）

### 矛盾・過剰設計の指摘

なし。

---

## 結論

**Gap 0 件を達成**。残存 Partial 9 件はすべて低リスクであり，実装フェーズへの移行に
支障はない。`/speckit.tasks` でのタスク分解に進むことを推奨する。
