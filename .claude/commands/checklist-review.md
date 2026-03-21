---
name: checklist-review
description: >
  Spec Kit チェックリストの自動レビュー・検証スキル。
  /speckit.checklist で生成されたドメイン別チェックリストを spec.md・plan.md と突合し、
  カバー済み項目のチェック、ギャップの分類、constitution 準拠の検証を一括で行う。
  「チェックリストレビュー」「checklist review」「チェックリスト検証」
  「ギャップ分析」「仕様レビュー」と言われたときに使用する。
---

# /checklist-review — チェックリスト自動レビュー＆ギャップ分析

## 概要

`/speckit.checklist` で生成されたドメイン別チェックリスト（ux.md, security.md, api.md 等）を
プロジェクトの成果物（spec.md, plan.md, constitution.md）と突合し、
品質ゲートとしてのレビューを自動化するスキル。

**このスキルは3フェーズで動作する：**

1. **自動クロスチェック** — チェックリスト項目を spec.md / plan.md と照合
2. **ギャップレポート** — 未カバー項目を分類し、アクションアイテムを提示
3. **Constitution 準拠チェック** — 憲法原則との整合性を検証

---

## Phase 1: 自動クロスチェック

### 入力ファイルの特定

以下の順序でファイルを探索・読み込む：

1. **チェックリストファイル**: `specs/<NNN>-<feature>/checklists/<domain>.md`
   - 引数でドメインが指定されていれば、そのファイルのみ対象にする
   - 指定がなければ `checklists/` 配下の全 `.md` ファイルを対象にする
2. **仕様書**: `specs/<NNN>-<feature>/spec.md`
3. **技術計画**: `specs/<NNN>-<feature>/plan.md`（存在する場合）
4. **実装詳細**: `specs/<NNN>-<feature>/` 配下の `data-model.md`, `contracts/` 等（存在する場合）
5. **憲法**: `.specify/memory/constitution.md`

### 判定ルール

チェックリストの各項目について、以下の基準で判定する：

| 判定 | 条件 | アクション |
|------|------|------------|
| ✅ Covered | spec.md または plan.md に該当する記述が**明確に**存在する | チェックボックスをオンにする |
| ⚠️ Partial | 関連する記述はあるが、チェック項目が求める詳細度・測定可能性を満たしていない | チェックボックスはオフのまま。`[Partial]` タグを付与 |
| ❌ Gap | 該当する記述がどの成果物にも見つからない | チェックボックスはオフのまま。`[Gap]` タグを付与 |
| 🔀 Misplaced | 技術的な詳細が spec.md に、機能要件が plan.md に混入している | `[Misplaced]` タグを付与し、正しい移動先を注記 |

### 実行手順

```
手順:
1. 対象チェックリストファイルを開く
2. 各チェック項目について:
   a. spec.md 内で対応する記述を検索する
   b. plan.md 内で対応する記述を検索する
   c. 該当するドキュメントがあれば、項目の詳細度・測定可能性を評価する
   d. 上記の判定ルールに基づいてタグを付与する
   e. Covered の場合はチェックボックスを `[x]` に更新する
3. 各項目の判定理由を簡潔にインラインコメントとして付記する
   例: `- [x] エラーハンドリングが定義されているか <!-- spec.md FR-005 で定義済み -->`
   例: `- [ ] レスポンスタイムの具体的な閾値 [Gap] <!-- spec/plan ともに未記載 -->`
```

---

## Phase 2: ギャップレポート

Phase 1 の結果から、Gap / Partial / Misplaced の項目を抽出し、
構造化されたレポートを生成する。

### レポートの出力先

`specs/<NNN>-<feature>/checklists/review-report.md` に出力する。

### レポート構成

以下のフォーマットで出力する：

```markdown
# Checklist Review Report: <feature-name>
**レビュー日時**: YYYY-MM-DD
**対象チェックリスト**: <domain>.md [, <domain>.md, ...]
**レビュー結果サマリー**:
- ✅ Covered: N 項目
- ⚠️ Partial: N 項目
- ❌ Gap: N 項目
- 🔀 Misplaced: N 項目
- **カバレッジ率**: XX%

---

## 仕様側の問題（spec.md で対応すべき項目）

機能要件・ユーザー体験・ビジネスロジックに関するギャップ。
spec.md の修正で解決すべき項目を列挙する。

| ID | チェック項目 | 判定 | 推奨アクション |
|----|------------|------|--------------|
| G-01 | ... | Gap | spec.md に「〇〇」セクションを追加 |
| G-02 | ... | Partial | FR-003 の受入基準に具体的な数値を追記 |

## 計画側の問題（plan.md で対応すべき項目）

技術的な設計・アーキテクチャ・非機能要件に関するギャップ。
plan.md の修正で解決すべき項目を列挙する。

| ID | チェック項目 | 判定 | 推奨アクション |
|----|------------|------|--------------|
| P-01 | ... | Gap | plan.md にエラーハンドリング戦略を追加 |
| P-02 | ... | Misplaced | spec.md の「レスポンスタイム200ms以下」を plan.md に移動 |

## 配置ミス（Misplaced 項目）

技術詳細が spec.md に混入、または機能要件が plan.md に混入している項目。

| ID | 現在の場所 | 移動先 | 内容 |
|----|----------|--------|------|
| M-01 | spec.md L42 | plan.md | 「Redis キャッシュによる高速化」は技術詳細 |

## 意図的な除外の確認

以下の Gap 項目について、意図的に対象外としている場合は理由を記録してください。
（人間が判断して記入するセクション）

| ID | チェック項目 | 除外理由（人間が記入） |
|----|------------|---------------------|
| G-XX | ... | |
```

---

## Phase 3: Constitution 準拠チェック

チェックリストの各項目と `.specify/memory/constitution.md` の原則を照合し、
以下の観点で検証する。

### 検証観点

1. **原則カバレッジ**: constitution で定義された各原則（テスト方針・UX原則・パフォーマンス要件・
   セキュリティ基準など）に対応するチェック項目が存在するか
2. **原則違反の検出**: チェックリスト項目の内容が constitution の原則と矛盾していないか
3. **過剰設計の検出**: constitution の Simplicity / Anti-Abstraction 原則に照らして、
   チェックリストが不必要に高い基準を要求していないか

### 出力

review-report.md の末尾に以下のセクションを追記する：

```markdown
## Constitution 準拠チェック

### カバーされている原則
| Constitution 原則 | 対応するチェック項目 |
|------------------|-------------------|
| Article I: ... | CL-01, CL-05, CL-12 |

### カバーされていない原則
以下の constitution 原則に対応するチェック項目が不足しています:
- Article VII（Simplicity）: チェックリストに簡素性の検証項目がない
- ...

### 矛盾・過剰設計の指摘
| チェック項目 | 関連する原則 | 指摘内容 |
|------------|------------|---------|
| CL-08 | Article VII | 「3層キャッシュ戦略」は Simplicity 原則に反する可能性 |
```

---

## 使い方

### 基本

```
/checklist-review
```

現在のフィーチャーブランチの全チェックリストを対象にレビューを実行する。

### ドメイン指定

```
/checklist-review ux
/checklist-review security
/checklist-review api,testing
```

特定のドメインのチェックリストのみを対象にする。

### 推奨ワークフロー

```
/speckit.specify → /speckit.clarify → /speckit.plan
  ↓
/clear
  ↓
/speckit.checklist ux
/speckit.checklist security
  ↓
/clear
  ↓
/checklist-review          ← ★ ここでこのスキルを使う
  ↓
（レポートを確認し、Gap 項目を修正）
  ↓
/checklist-review          ← 修正後に再実行して改善を確認
  ↓
/clear
  ↓
/speckit.tasks → /speckit.analyze → /speckit.implement
```

---

## 注意事項

- このスキルはチェックリストファイルを**直接更新する**（チェックボックスとタグの付与）。
  必要に応じて事前に git commit しておくこと。
- review-report.md は毎回**上書き**される。前回のレポートを保持したい場合は
  リネームするか git commit すること。
- ギャップレポートの「意図的な除外の確認」セクションは人間が記入する。
  AI は除外理由を推測しない。
- spec.md / plan.md の**直接修正はこのスキルでは行わない**。
  レポートを元に人間が判断し、必要に応じて手動修正または `/speckit.clarify` で対応する。
