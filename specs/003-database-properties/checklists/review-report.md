# Checklist Review Report: プロパティシステムとデータベース概念の導入

**レビュー日時**: 2026-03-21
**対象チェックリスト**: data-integrity.md
**レビュー結果サマリー**:
- ✅ Covered: 13 項目
- ⚠️ Partial: 12 項目
- ❌ Gap: 8 項目
- 🔀 Misplaced: 0 項目
- **カバレッジ率**: 39.4%（13/33）

---

## 仕様側の問題（spec.md で対応すべき項目）

機能要件・ユーザー体験・ビジネスロジックに関するギャップ。
spec.md の修正で解決すべき項目を列挙する。

| ID | チェック項目 | 判定 | 推奨アクション |
|----|------------|------|--------------|
| G-01 | CHK004 — `updated_at` タイムスタンプの更新規則 | Gap | spec.md §Requirements または data-model.md に「updated_at 更新トリガー一覧」セクションを追加。各エンティティ（Database, Property, PropertyValue）について，どの操作で updated_at が更新されるかを明記する |
| G-02 | CHK009 — 数値型の有限値の境界定義 | Partial | spec.md §Assumptions に「任意の IEEE 754 finite f64 を受け入れる。min/max 制約は設けない」を明記するか，制約がある場合はその値を記載する |
| G-03 | CHK011 — 文字数制限の単位（バイト/文字/書記素クラスタ） | Gap | spec.md §Key Entities または data-model.md §バリデーション規則に，「文字」の定義を明記する。既存の PageTitle 実装と整合させること |
| G-04 | CHK017 — SC-001 の前提条件 | Partial | spec.md §SC-001 に前提条件を追記する（例: 初回ユーザー，空のデータベース状態から開始，開発用ハードウェア） |
| G-05 | CHK018 — パフォーマンス目標の測定条件 | Partial | spec.md §CC-003 に測定条件を追記する（コールドスタート/ウォームキャッシュ，データ形状，ハードウェアベースライン）。ローカルデスクトップアプリのため「開発マシン相当」で十分な可能性あり |
| G-06 | CHK022 — 同一データへの並行操作 | Gap | spec.md §Assumptions に「ローカルデスクトップアプリのため並行書き込みは single-writer を前提とする。SQLite WAL モードにより last-write-wins で動作する」等の方針を追記する |
| G-07 | CHK027 — 最後のプロパティ削除時の表示 | Partial | spec.md §FR-005 または User Story 5 に「プロパティが 0 個の場合はタイトル列のみ表示」を明記する |
| G-08 | CHK032 — 型変換のマイグレーションパス | Partial | spec.md §Assumptions の型変換除外に「将来の型変換対応時は別途マイグレーション設計が必要」の一文を追記する。現時点で詳細設計は不要 |

## 設計側の問題（data-model.md / contracts で対応すべき項目）

技術的な設計・アーキテクチャ・非機能要件に関するギャップ。
data-model.md または contracts/ipc-commands.md の修正で解決すべき項目を列挙する。

| ID | チェック項目 | 判定 | 推奨アクション |
|----|------------|------|--------------|
| P-01 | CHK005 — プロパティ削除時の position 再配置戦略 | Gap | data-model.md §Property に「削除時は既存 position のギャップを許容する。必要に応じて `reorder_properties` で再配置」の方針を追記する |
| P-02 | CHK008 — PropertyConfig の JSON ワイヤーフォーマット | Partial | data-model.md §PropertyConfig enum に serde tag 方式（例: `#[serde(tag = "type")]` internally tagged）と具体的な JSON 出力例を追記する |
| P-03 | CHK010 — SelectOption ID 格納の設計根拠 | Partial | data-model.md §PropertyValue バリデーション規則の Select 項に「選択肢名変更時の参照整合性を保持するため，表示値ではなく ID を格納する」の根拠を追記する |
| P-04 | CHK014 — PageDto の拡張定義 | Gap | contracts/ipc-commands.md §DTO Definitions に `PageDto`（`databaseId: string \| null` フィールドを含む）の定義を追加する。既存 feature の PageDto との差分を明示する |
| P-05 | CHK021 — `reorder_properties` のサブセット入力動作 | Gap | contracts/ipc-commands.md §reorder_properties に「全プロパティ ID を含む完全なリストを要求する。サブセットの場合はエラー」等の動作を明記する |
| P-06 | CHK023 — `clear_property_value` の不在値クリア動作 | Partial | contracts/ipc-commands.md §clear_property_value に「値が存在しない場合は no-op（エラーなし）」を明記する |
| P-07 | CHK024 — 数値型エッジ値（-0.0, subnormal, MAX/MIN f64） | Partial | data-model.md §PropertyValue バリデーション規則に「-0.0 は 0.0 として正規化する。subnormal/MAX/MIN f64 は有限値として受け入れる」等の方針を追記する |
| P-08 | CHK025 — 日付型の境界値 | Partial | data-model.md §PropertyValue バリデーション規則に「`DateTime<Utc>` が表現可能な範囲を受け入れる。UTC を強制する」を明記する |
| P-09 | CHK026 — SelectOption 値の JSON 特殊文字 | Partial | data-model.md §PropertyConfig に「serde の JSON シリアライズが特殊文字を自動エスケープするため，追加のサニタイズは不要」を注記する |
| P-10 | CHK028 — マイグレーションのロールバック要件 | Gap | data-model.md §SQLite Schema に「sqlx マイグレーションは forward-only。部分失敗時は手動でのデータベースファイル復旧（バックアップからのリストア）を前提とする」を追記する |
| P-11 | CHK030 — `PRAGMA foreign_keys = ON` の前提条件 | Gap | data-model.md §SQLite Schema に「アプリケーション起動時に `PRAGMA foreign_keys = ON` を実行済み（既存基盤で対応済み）」を明記する。未対応の場合は実装タスクとして追加する |
| P-12 | CHK033 — クロスリポジトリのトランザクション境界 | Partial | data-model.md §Repository Traits に「クロスリポジトリ操作のトランザクション要件」セクションを追加。ページ除外（database_id NULL 化 + property_values 削除），データベース削除等のトランザクション境界を網羅する |

## 配置ミス（Misplaced 項目）

該当なし。

## 意図的な除外の確認

以下の Gap 項目について，意図的に対象外としている場合は理由を記録してください。
（人間が判断して記入するセクション）

| ID | チェック項目 | 除外理由（人間が記入） |
|----|------------|---------------------|
| G-01 | CHK004 — `updated_at` 更新規則 | |
| G-03 | CHK011 — 文字数制限の単位 | |
| G-06 | CHK022 — 並行操作の動作 | |
| P-01 | CHK005 — position 再配置戦略 | |
| P-04 | CHK014 — PageDto 拡張定義 | |
| P-05 | CHK021 — reorder_properties サブセット動作 | |
| P-10 | CHK028 — マイグレーションロールバック | |
| P-11 | CHK030 — PRAGMA foreign_keys 前提条件 | |

---

## Constitution 準拠チェック

### カバーされている原則

| Constitution 原則 | 対応するチェック項目 |
|------------------|-------------------|
| I. Local-First Product Integrity | CHK001, CHK007, CHK012, CHK015, CHK028, CHK029, CHK030 |
| II. Domain-Faithful Information Model | CHK002, CHK010, CHK014 |
| III. Typed Boundaries and DDD | CHK008, CHK013, CHK033 |
| IV. Test-First Delivery and Quality Gates | CHK016, CHK017, CHK018 |
| V. Safe Rust, SOLID, Maintainability First | CHK027（簡素性の観点） |
| VII. Defensive Error Handling | CHK023, CHK024, CHK025 |

### カバーされていない原則

以下の constitution 原則に対応するチェック項目が不足しています:

- **VI（Rust ドキュメント標準）**: データモデルのドキュメント品質に関するチェック項目がない。ただし，data-integrity チェックリストのスコープ外であり，別ドメインのチェックリスト（例: code-quality.md）で対応すべき事項。**対応不要。**
- **IV（Test-First）の具体性**: テストの実行可能性に関する項目は CHK016〜018 で存在するが，「先に失敗するテストを作成する」というプロセス面のチェックがない。これもデータモデルの整合性チェックのスコープ外であり，タスク定義（tasks.md）で担保すべき。**対応不要。**

### 矛盾・過剰設計の指摘

| チェック項目 | 関連する原則 | 指摘内容 |
|------------|------------|---------|
| （該当なし） | — | チェックリストの要求水準は constitution の原則と整合しており，過剰設計・矛盾は検出されなかった |

**補足**: CHK022（並行操作）と CHK024（f64 エッジ値）は，ローカルデスクトップアプリという文脈では低リスクであり，Principle V（YAGNI）の観点から「意図的な除外」として記録することも合理的。ただし，仕様上の決定として明文化すべきである。
