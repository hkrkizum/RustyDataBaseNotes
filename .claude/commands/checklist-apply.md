---
name: checklist-apply
description: >
  /checklist-review で生成されたレビューレポート（review-report.md）と更新済みチェックリストを
  入力として、spec.md・plan.md・その他付属ドキュメントを差分更新するスキル。
  既存ドキュメントを再作成せず、ギャップ項目に対応する最小限の追記・修正・移動を行う。
  「ギャップ修正」「レポート反映」「checklist apply」「仕様更新」「spec update」
  「plan update」「ドキュメント更新」と言われたときに使用する。
  /checklist-review の後に使うことを想定している。
---

# /checklist-apply — レビューレポートに基づく成果物の差分更新

## 概要

`/checklist-review` が出力した `review-report.md` と更新済みチェックリストを元に、
spec.md・plan.md・その他の付属ドキュメントに対して**最小限の差分更新**を行う。

**鉄則: 既存ドキュメントをゼロから再作成しない。**

このスキルは Spec Kit コミュニティで広く報告されている
「plan を再生成すると人間のレビュー済み内容が消える」問題
（github/spec-kit #1059, #1191, Discussion #839, #775）を回避するために設計されている。

---

## 前提条件

このスキルを実行する前に、以下が揃っていること：

1. `/checklist-review` が実行済みで `review-report.md` が存在する
2. チェックリストファイルに判定タグ（✅ / ⚠️ / ❌ / 🔀）が付与済み
3. 対象のフィーチャーブランチ上で作業している
4. **推奨**: 更新前に `git add -A && git commit -m "pre-checklist-apply snapshot"` を実行済み

---

## 実行フロー

```
┌─────────────────────────────────────────────────────┐
│ Phase 0: 事前スナップショット                          │
│   既存の全成果物の現在の状態を把握する                    │
├─────────────────────────────────────────────────────┤
│ Phase 1: review-report.md の解析                      │
│   Gap / Partial / Misplaced 項目を抽出・分類する        │
├─────────────────────────────────────────────────────┤
│ Phase 2: spec.md の差分更新                           │
│   仕様側の問題（機能要件・UX・ビジネスロジック）を修正      │
├─────────────────────────────────────────────────────┤
│ Phase 3: plan.md + 付属ドキュメントの差分更新            │
│   計画側の問題（技術設計・非機能要件）を修正               │
├─────────────────────────────────────────────────────┤
│ Phase 4: Misplaced 項目の移動                         │
│   配置ミスのある記述を正しいドキュメントに移す             │
├─────────────────────────────────────────────────────┤
│ Phase 5: 変更サマリーの生成                            │
│   何をどう変えたかの差分レポートを出力する                 │
└─────────────────────────────────────────────────────┘
```

---

## Phase 0: 事前スナップショット

更新対象の全ファイルを読み込み、現在の構造を把握する。

### 読み込むファイル

```
specs/<NNN>-<feature>/
├── spec.md                    ← 必須
├── plan.md                    ← 必須
├── data-model.md              ← 存在すれば
├── contracts/                 ← 存在すれば
├── research.md                ← 存在すれば
├── tasks.md                   ← 存在すれば
└── checklists/
    ├── review-report.md       ← 必須（入力）
    ├── <domain>.md            ← 更新済みチェックリスト
    └── ...
.specify/memory/
└── constitution.md            ← 参照用
```

### 構造マップの作成

各ドキュメントのセクション構造（見出しレベル・既存の要件ID）を把握し、
追記位置を正確に決定するための内部マップを作成する。

```
例:
spec.md:
  - ## Overview (L1-L15)
  - ## User Stories (L16-L45)
    - ### US-001: ユーザー登録 (L17-L25)
    - ### US-002: ログイン (L26-L45)
  - ## Functional Requirements (L46-L90)
    - FR-001 ~ FR-012
  - ## Success Criteria (L91-L105)
    - SC-001 ~ SC-008
  - ## Assumptions (L106-L115)

plan.md:
  - ## Architecture Overview (L1-L30)
  - ## Technology Stack (L31-L50)
  - ## Implementation Phases (L51-L100)
  - ## Error Handling Strategy (L101-L120)
  - ## Performance Requirements (L121-L140)
```

---

## Phase 1: review-report.md の解析

`review-report.md` から以下のデータを抽出する：

### 抽出対象

1. **仕様側の問題テーブル**（「spec.md で対応すべき項目」セクション）
   - 各行の ID、チェック項目、判定（Gap / Partial）、推奨アクション
2. **計画側の問題テーブル**（「plan.md で対応すべき項目」セクション）
   - 各行の ID、チェック項目、判定（Gap / Partial）、推奨アクション
3. **配置ミステーブル**（「Misplaced 項目」セクション）
   - 各行の ID、現在の場所、移動先、内容
4. **意図的な除外テーブル**
   - 人間が「除外理由」を記入済みの項目は**更新対象から除外する**
5. **Constitution 準拠チェック結果**
   - 「カバーされていない原則」「矛盾・過剰設計の指摘」

### 除外ルール

以下の項目はスキップする：
- 「意図的な除外の確認」に除外理由が記入されている項目
- ✅ Covered と判定された項目（対応不要）
- review-report.md に存在しない項目

---

## Phase 2: spec.md の差分更新

### 基本原則

```
絶対に守ること:
- spec.md を新規作成・全面書き換えしない
- 既存のセクション構造・見出し階層を維持する
- 既存の要件ID（FR-XXX, SC-XXX, US-XXX）の採番規則を継続する
- 人間が書いた文章のトーン・粒度に合わせる
- 技術的な実装詳細を spec.md に書かない（plan.md の責務）
```

### 更新パターン

#### パターン A: Gap（セクションまたは要件が完全に欠落）

対象のレポート項目の「推奨アクション」に従い、適切なセクションに新しい要件を追記する。

```
手順:
1. 推奨アクションから、追記すべきセクションを特定する
2. Phase 0 の構造マップを参照し、該当セクションの末尾を見つける
3. 既存の採番規則に従って新しい ID を付与する
   例: FR-012 の次なら FR-013
4. 既存の記述スタイル（箇条書き / テーブル / 散文）に合わせて追記する
5. 追記箇所に <!-- added by checklist-apply: G-XX --> コメントを付与する
```

#### パターン B: Partial（記述はあるが詳細不足）

既存の記述を保持しつつ、不足している詳細を補完する。

```
手順:
1. 該当する既存の要件（例: FR-005）を特定する
2. 既存の文章はそのまま残す
3. 不足している情報（具体的な数値、エッジケース、受入基準など）を
   既存の段落の直後に追記する
4. 追記箇所に <!-- refined by checklist-apply: G-XX --> コメントを付与する
```

#### パターン C: Constitution 準拠の補完

constitution で定義されているがチェックリストでカバーされていない原則について、
関連する要件を spec.md に追加する。

```
手順:
1. review-report.md の「カバーされていない原則」を参照する
2. constitution.md の該当原則を読む
3. その原則に対応する機能要件 or 成功基準を spec.md に追記する
   例: Accessibility 原則 → "SC-XXX: WCAG 2.1 AA 準拠" を Success Criteria に追加
4. 追記箇所に <!-- constitution-align by checklist-apply --> コメントを付与する
```

---

## Phase 3: plan.md + 付属ドキュメントの差分更新

### 基本原則

```
絶対に守ること:
- plan.md を新規作成・全面書き換えしない
- 既存のアーキテクチャ判断・技術選定はそのまま残す
- 人間がレビュー済みのセクションの構造を壊さない
- 新しいセクションの追加は、既存のセクション体系に合わせる
- 付属ドキュメント（data-model.md, contracts/ 等）も同じ差分更新ルールに従う
```

### 更新パターン

#### パターン D: 技術的な Gap の補完

plan.md に欠落している技術要件を、既存の構造に沿って追記する。

```
手順:
1. 推奨アクションから追記すべき内容を把握する
2. plan.md 内で最も適切なセクションを探す
   - エラーハンドリング → "Error Handling Strategy" セクション
   - パフォーマンス → "Performance Requirements" セクション
   - セキュリティ → "Security Considerations" セクション
   - 該当セクションが存在しない場合のみ、新しいセクションを追加する
3. 既存の記述スタイルに合わせて追記する
4. 追記箇所に <!-- added by checklist-apply: P-XX --> コメントを付与する
```

#### パターン E: 付属ドキュメントの更新

data-model.md や contracts/ 配下のファイルに対するギャップ。

```
手順:
1. 対象ファイルが存在するか確認する
2. 存在する場合 → 該当セクションに追記する
3. 存在しない場合 → 新規作成する
   ただし、plan.md から参照されるようにリンクを追記する
4. 追記箇所に <!-- added by checklist-apply: P-XX --> コメントを付与する
```

#### パターン F: 過剰設計の簡素化

review-report.md の「矛盾・過剰設計の指摘」に基づく修正。

```
手順:
1. 該当するチェック項目と constitution の原則を照合する
2. 過剰設計と判定された箇所を特定する
3. 削除ではなく、簡素化した代替案をインラインで提示する
   例:
   <!-- checklist-apply: simplification suggested for P-XX
        現在: 3層キャッシュ（L1/L2/L3）
        提案: 単一キャッシュレイヤーで十分（Article VII: Simplicity）
        要判断: 人間がどちらを採用するか決定してください
   -->
4. 自動で削除・書き換えはしない。コメントで提案のみ行う。
```

---

## Phase 4: Misplaced 項目の移動

配置ミスのある記述を正しいドキュメントに移す。

```
手順:
1. review-report.md の「配置ミス」テーブルから対象を取得する
2. 移動元のドキュメントで該当箇所を特定する
3. 移動先のドキュメントで適切な挿入位置を決定する
4. 移動先に記述をコピーする（既存スタイルに合わせて調整）
5. 移動元の該当箇所を以下のコメントに置き換える:
   <!-- moved to plan.md by checklist-apply: M-XX -->
6. 移動元の記述は削除せず、コメントアウトとして残す:
   <!-- [moved] 元の記述をここに保持 -->
   これにより人間が移動を確認・取り消しできる
```

---

## Phase 5: 変更サマリーの生成

すべての更新完了後、変更内容をまとめたサマリーを出力する。

### 出力先

`specs/<NNN>-<feature>/checklists/apply-changelog.md`

### フォーマット

```markdown
# Checklist Apply Changelog: <feature-name>
**実行日時**: YYYY-MM-DD
**入力**: review-report.md
**モード**: 差分更新（非破壊）

---

## 変更統計
- spec.md: N箇所 追記 / N箇所 補完 / N箇所 移動受入
- plan.md: N箇所 追記 / N箇所 補完 / N箇所 移動受入
- data-model.md: N箇所 追記
- 新規作成ファイル: なし / <ファイル名>
- 簡素化提案（要判断）: N箇所

---

## 変更詳細

### spec.md の変更

| レポートID | 変更種別 | 対象箇所 | 変更内容の要約 |
|-----------|---------|---------|--------------|
| G-01 | 追記 | FR-013 (L92) | エラーハンドリング要件を追加 |
| G-02 | 補完 | SC-003 (L98) | 「高速」を「200ms以内」に具体化 |
| M-01 | 移動受入 | (plan.md L42 から) | Redis キャッシュ戦略を技術要件として受入 |

### plan.md の変更

| レポートID | 変更種別 | 対象箇所 | 変更内容の要約 |
|-----------|---------|---------|--------------|
| P-01 | 追記 | Error Handling (L125) | API失敗時のフォールバック戦略を追加 |
| M-01 | 移動 | (spec.md L42 から) | Redis キャッシュ戦略を移動 |

### 要判断項目（人間のレビューが必要）

以下の項目は自動適用せず、コメントとして提案のみ行っています。
各ドキュメント内の `<!-- checklist-apply: simplification suggested` コメントを
確認し、採否を判断してください。

| レポートID | ファイル | 提案内容 |
|-----------|--------|---------|
| P-03 | plan.md L88 | 3層キャッシュ → 単一キャッシュレイヤーへの簡素化 |

---

## 次のステップ

1. `git diff` で変更内容を確認する
2. 「要判断項目」のコメントを確認し、採否を決定する
3. 採用する場合はコメントを削除して提案内容を本文に反映する
4. 却下する場合はコメントごと削除する
5. 移動元の `<!-- [moved] ... -->` コメントを確認し、問題なければ削除する
6. 満足したら `git commit` する
7. 必要に応じて `/checklist-review` を再実行し、カバレッジ率の改善を確認する
```

---

## 使い方

### 基本

```
/checklist-apply
```

`review-report.md` のすべての項目を処理する。

### 特定カテゴリのみ

```
/checklist-apply spec-only     # spec.md の更新のみ
/checklist-apply plan-only     # plan.md + 付属ドキュメントの更新のみ
/checklist-apply misplaced     # Misplaced 項目の移動のみ
```

### 推奨ワークフロー（/checklist-review との組み合わせ）

```
/speckit.checklist ux
/speckit.checklist security
  ↓  /clear
/checklist-review              ← レビューレポート生成
  ↓
（review-report.md を人間が確認。
  意図的な除外があれば「除外理由」を記入。）
  ↓  /clear
/checklist-apply               ← ★ このスキル。差分更新を実行
  ↓
（apply-changelog.md と git diff を確認。
  要判断項目の採否を決定。）
  ↓
git commit -m "apply checklist review findings"
  ↓  /clear
/checklist-review              ← 再実行してカバレッジ率の改善を確認
  ↓  /clear
/speckit.tasks → /speckit.analyze → /speckit.implement
```

---

## 安全設計

### やらないこと

- **ファイルの全面書き換え・再生成は絶対にしない**
- 既存の要件 ID の変更・振り直しはしない
- 既存のセクションの順序変更はしない
- 人間のレビューコメントや手動編集を上書きしない
- 「意図的な除外」に記入済みの項目は処理しない
- 過剰設計の指摘に対して自動で削除・書き換えしない（提案のみ）

### トレーサビリティ

すべての変更に HTML コメントでタグを付与する：
- `<!-- added by checklist-apply: G-XX -->` — 新規追記
- `<!-- refined by checklist-apply: G-XX -->` — 既存の補完
- `<!-- constitution-align by checklist-apply -->` — constitution 準拠の追加
- `<!-- moved to <file> by checklist-apply: M-XX -->` — 移動先
- `<!-- [moved] 元の記述 -->` — 移動元の保持
- `<!-- checklist-apply: simplification suggested for P-XX ... -->` — 簡素化提案

これらのコメントにより：
- `git diff` で変更箇所を一覧できる
- `grep -r "checklist-apply"` で全変更を追跡できる
- 人間が各変更を個別に承認・却下できる

### review-report.md が存在しない場合

エラーメッセージを表示し、先に `/checklist-review` の実行を促す。
決して推測で更新を行わない。

---

## 注意事項

- tasks.md は**このスキルでは更新しない**。
  tasks.md は spec.md / plan.md の変更が確定した後に
  `/speckit.tasks` で再生成するか、手動で調整する。
- 大量の Gap がある場合（10件以上）は、
  一度にすべて適用するのではなく、優先度の高い項目から段階的に適用することを推奨する。
  `/checklist-apply` 実行時に注意を表示する。
- 付属ドキュメントの新規作成が発生した場合、
  plan.md 内に該当ファイルへの参照リンクを自動追記する。
