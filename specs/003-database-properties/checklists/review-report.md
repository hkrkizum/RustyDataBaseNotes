# Checklist Review Report: プロパティシステムとデータベース概念の導入

**レビュー日時**: 2026-03-21（再レビュー — checklist-apply 適用後）
**対象チェックリスト**: api.md
**前回レビュー結果**: ✅ 12 / ⚠️ 12 / ❌ 11 / 🔀 0 — カバレッジ率 34%
**今回レビュー結果サマリー**:
- ✅ Covered: 35 項目
- ⚠️ Partial: 0 項目
- ❌ Gap: 0 項目
- 🔀 Misplaced: 0 項目
- **カバレッジ率**: 100%（35/35）

---

## レビュー結果

### 前回 Gap → 今回 Covered（11項目）

| ID | チェック項目 | 解決方法 |
|----|------------|---------|
| G-01 | CHK001 — US8 完全削除の IPC コマンド | spec.md FR-006 に「既存の `delete_page` コマンドを再利用し，新規 IPC コマンドは追加しない」を追記 |
| G-02 | CHK005 — バッチ操作のスコープ除外 | spec.md §Assumptions に「バッチ操作は本スコープ対象外」を追記 |
| G-03 | CHK024 — テキスト型空文字列の意味 | spec.md §Assumptions + data-model.md §Text に「空文字列は有効な値，クリアには clear_property_value」を追記 |
| P-01 | CHK004 — CommandError の TS interface | contracts §DTO Definitions に `interface CommandError { kind, message }` を追加 |
| P-05 | CHK014 — config 省略時のレスポンス | contracts §add_property に「config 省略時は null を返す」を追記 |
| P-06 | CHK016 — 未設定値の DTO 表現 | contracts §TableRowDto に「未入力はキー欠落」を追記 |
| P-07 | CHK019 — クロスデータベース値設定 | contracts §set_property_value に `pageNotInDatabase` エラーを追加 |
| P-08 | CHK021 — standalone ページの除外操作 | contracts §remove_page_from_database に「no-op（エラーなし）」を追記 |
| P-09 | CHK023 — list_standalone_pages ソート順 | contracts §list_standalone_pages に「作成日時降順」を追記 |
| P-10 | CHK026 — 不正日付エラー kind | contracts §set_property_value + Error Kind 表に `invalidDate` を追加 |
| P-11 | CHK033 — serde ↔ TS マッピング前提 | contracts プリアンブルに serde シリアライズ規則を追記 |

### 前回 Partial → 今回 Covered（12項目）

| ID | チェック項目 | 解決方法 |
|----|------------|---------|
| G-04 | CHK029 — フロントエンド事前バリデーション | spec.md CC-004 に「楽観的事前バリデーション SHOULD，バックエンドが最終権威」を追記 |
| P-02 | CHK008 — PropertyConfigDto 型ごと制約 | contracts に serde internally tagged 対応説明 + JSON 例 + 型ごとの必須注記を追記 |
| P-03 | CHK010 — TS DTO と Rust serde 形式 | contracts プリアンブルに serde 規則 + PropertyConfigDto に JSON ワイヤーフォーマット例を追記 |
| P-04 | CHK012 — ISO 8601 / RFC 3339 用語統一 | contracts の全タイムスタンプを "RFC 3339 / UTC" に統一 |
| P-12 | CHK035 — 同期的 IPC 前提 | contracts プリアンブルに「同期的 Request→Response パターン」を追記 |
| P-13 | CHK003 — エラーメッセージ形式 | CommandError interface + Error Kind Extensions に message 形式方針を追記 |
| P-14 | CHK006 — 型と config の不整合 | contracts §update_property_config に「invalidConfig エラー」を追記 |
| P-15 | CHK013 — snake_case → camelCase 規則 | contracts プリアンブルに変換ルールを追記 |
| P-16 | CHK017 — エラー kind 命名規則 | Error Kind Extensions に「camelCase で命名」を追記 |
| P-17 | CHK022 — CASCADE 件数非通知 | contracts §delete_property に「void，件数非通知」を追記 |
| P-18 | CHK031 — propertyType-config 不整合 | contracts §add_property に「invalidConfig エラー」を具体例付きで追記 |
| P-19 | CHK032 — PropertyName トリミング | data-model.md §Property に「トリム後」を追記 |

### 前回 Partial → 今回 Covered（追加）

| ID | チェック項目 | 解決方法 |
|----|------------|---------|
| P-20 | CHK034 — PageDto 差分明示 | contracts §PageDto に「既存/新規フィールド差分」をコメントで追記 |

### 前回から変更なし — Covered 維持（12項目）

CHK002, CHK007, CHK009, CHK011, CHK015, CHK018, CHK020, CHK025, CHK027, CHK028, CHK030

---

## 仕様側の問題（spec.md で対応すべき項目）

**なし。** 全項目が解決済み。

## 計画側の問題（plan.md / contracts / data-model.md で対応すべき項目）

**なし。** 全項目が解決済み。

## 配置ミス（Misplaced 項目）

該当なし。

## 意図的な除外の確認

全 Gap/Partial 項目が解決されたため，除外確認は不要。

---

## Constitution 準拠チェック

### カバーされている原則

| Constitution 原則 | 対応するチェック項目 |
|------------------|-------------------|
| Article I: Local-First Product Integrity | CHK005, CHK009, CHK019, CHK021 |
| Article II: Domain-Faithful Information Model | CHK011, CHK016, CHK024 |
| Article III: Typed Boundaries and DDD | CHK004, CHK007, CHK008, CHK010, CHK013, CHK030, CHK033, CHK035 |
| Article V: Safe Rust, SOLID, Maintainability | CHK005, CHK014, CHK022 |
| Article VII: Defensive Error Handling | CHK003, CHK006, CHK015, CHK019, CHK021, CHK026, CHK028, CHK031 |

### カバーされていない原則

以下の constitution 原則に対応するチェック項目が不足しています:

- **Article IV（Test-First Delivery）**: API チェックリストにテスト戦略に関する検証項目がない。IPC コマンドの統合テスト網羅性，エラーパスのテストカバレッジ等の項目を検討すべき。ただし，テスト専用チェックリストの管轄であり API チェックリストのスコープ外。**対応不要。**
- **Article VI（Rust Documentation Standards）**: API 契約のドキュメント品質に関する検証項目がない。スコープ外。**対応不要。**

### 矛盾・過剰設計の指摘

| チェック項目 | 関連する原則 | 指摘内容 |
|------------|------------|---------|
| （該当なし） | Article V（Simplicity / YAGNI） | 過剰設計・矛盾は検出されなかった |

---

## 改善サマリー

| 指標 | 前回（初回レビュー） | 今回（再レビュー） | 改善 |
|------|-------------------|-------------------|------|
| ✅ Covered | 12 | 35 | +23 |
| ⚠️ Partial | 12 | 0 | -12 |
| ❌ Gap | 11 | 0 | -11 |
| カバレッジ率 | 34% | **100%** | +66pt |

API チェックリストの全項目がカバーされた。
次のステップとして `/speckit.tasks` → `/speckit.analyze` → `/speckit.implement` へ進むことを推奨する。
