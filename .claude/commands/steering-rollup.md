---
name: steering-rollup
description: >
  フィーチャー実装完了後に steering/ ドキュメント群を更新し、
  specs/ のライフサイクル管理（Active → Merged → Archived）を行うスキル。
  featureブランチのマージ前後に実行し、個別specの what/why を全体像に吸い上げ、
  作業用の成果物（plan/tasks/checklists）を整理する。
  「ロールアップ」「steering 更新」「spec アーカイブ」「全体像 更新」
  「rollup」「feature完了」と言われたときに使用する。
---

# /steering-rollup — steering 更新 & spec ライフサイクル管理

## 概要

フィーチャー実装の完了を契機に、2つのことを行うスキル：

1. **steering/ の差分更新** — 完了したfeatureの成果を全体像ドキュメントに反映する
2. **specs/ のライフサイクル遷移** — Active → Merged → Archived の状態管理を行う

```
実行前:
  steering/product.md        ← featureの成果が未反映
  steering/architecture.md   ← 同上
  steering/tech.md           ← 同上
  steering/current-state.md  ← 同上
  specs/005-new-feature/     ← Active（全ファイル存在）

実行後:
  steering/product.md        ← featureの what/why が反映済み
  steering/architecture.md   ← 構成変更が反映済み
  steering/tech.md           ← 技術選定変更が反映済み
  steering/current-state.md  ← 現在の姿が更新済み
  specs/005-new-feature/
    └── ARCHIVED.md          ← ポインタファイル（archive/ への参照）
  archive/005-new-feature/   ← 全成果物の物理保管
```

---

## 前提条件

- 対象featureの `/speckit.implement` が完了している
- steering/ ディレクトリと4つのファイルが存在する
  （初回は `/steering-rollup init` で雛形を生成する）
- constitution.md が存在する
- **推奨**: 実行前に `git commit` でクリーンな状態にしておく

---

## サブコマンド

### `/steering-rollup init` — steering/ の初期構築

プロジェクト開始時、または既存プロジェクトへの導入時に一度だけ実行する。
既存のコードベース・constitution・既存specを読み取り、
steering/ の4ファイルを**新規生成**する。

### `/steering-rollup` — 通常のロールアップ（メイン機能）

featureの完了後に実行する。steering/ を差分更新し、specs/ を整理する。

### `/steering-rollup status` — 現在の状態確認

specs/ 内の各featureのライフサイクル状態と、
steering/ の最終更新日時を一覧表示する。

---

## Phase 1: 情報収集

### 読み込むファイル

```
読み込み順序と目的:

1. steering/ の現在のドキュメント（更新のベースライン）
   - steering/product.md
   - steering/architecture.md
   - steering/tech.md
   - steering/current-state.md

2. 対象featureの成果物
   - specs/<NNN>-<feature>/spec.md        ← what/why の抽出元
   - specs/<NNN>-<feature>/plan.md        ← 技術判断の抽出元
   - specs/<NNN>-<feature>/data-model.md  ← 構造変更の抽出元（あれば）
   - specs/<NNN>-<feature>/contracts/     ← API変更の抽出元（あれば）
   - specs/<NNN>-<feature>/research.md    ← 技術調査結果の抽出元（あれば）

3. 参照用
   - .specify/memory/constitution.md
   - 実装済みのコードベースのディレクトリ構造（概要把握のみ）
```

### 対象featureの特定

引数でfeature番号を指定するか、省略時は現在のブランチ名から推定する。

```
/steering-rollup                  ← 現在のブランチのfeatureを対象
/steering-rollup 005              ← specs/005-* を対象
/steering-rollup 003,004,005      ← 複数featureを一括ロールアップ
```

---

## Phase 2: steering/ の差分更新

### 基本原則

```
絶対に守ること:
- steering/ の各ファイルをゼロから再作成しない
- 既存の記述を保持し、差分のみ追記・更新する
- 各ファイルの総量を制御する（後述のサイズ上限を守る）
- featureのspec.mdから what/why のエッセンスだけを抽出する
  plan/tasks の how の詳細は steering/ に持ち込まない
- 変更箇所に <!-- rollup: NNN-feature-name, YYYY-MM-DD --> コメントを付与する
```

### 2-A: product.md の更新

**目的**: このプロダクトが「いま何を提供しているか」の全体像。

**サイズ上限**: 200行以内

**更新ルール**:

```
spec.md から抽出する情報:
  - 新しいユースケース / ユーザーストーリーの要約（1-2行/件）
  - ドメイン境界の変更（新しいドメイン概念が追加された場合）
  - 対象ユーザーの拡張（新しいペルソナが追加された場合）

更新パターン:
  A. 新しいユースケース → 「主要ユースケース」セクションに1-2行で追記
  B. 既存ユースケースの変更 → 該当行を更新（旧記述は削除してよい）
  C. ドメイン境界の変更 → 「ドメイン境界」セクションを更新
  D. 上限超過時 → 古い詳細を集約して行数を削減してから追記する

やらないこと:
  - 技術的な実装詳細の記載
  - spec.md の全文コピー
  - plan.md / tasks.md からの情報引用
```

### 2-B: architecture.md の更新

**目的**: システムの現在の構成を把握できる技術ドキュメント。

**サイズ上限**: 300行以内

**更新ルール**:

```
plan.md + data-model.md + contracts/ から抽出する情報:
  - 新しいモジュール / コンポーネントの追加
  - 既存モジュール間の新しい依存関係
  - データモデルの重要な変更（新エンティティ、関係の変更）
  - API / インターフェースの追加・変更
  - データフローの変更

更新パターン:
  A. 新モジュール追加 → 「主要モジュール」セクションに追記（名前、責務、1-2行）
  B. 依存関係の変更 → 「依存関係」セクションを更新
  C. データモデル変更 → 主要エンティティの一覧を更新
  D. 変更しにくい境界の追加 → 「変更しにくい境界」セクションに理由付きで追記

やらないこと:
  - コードレベルのクラス設計やファイル構成の詳細
  - plan.md の全セクションの転記
```

### 2-C: tech.md の更新

**目的**: 技術選定と実装方針の現在地。

**サイズ上限**: 200行以内

**更新ルール**:

```
plan.md + research.md から抽出する情報:
  - 新しいライブラリ / フレームワークの採用
  - バージョンの重要な変更
  - 実装パターンの追加（新しく確立されたパターン）
  - テスト方針の変更

更新パターン:
  A. 新技術の採用 → 「技術スタック」セクションに追記（名前、バージョン、採用理由1行）
  B. バージョン変更 → 該当行を更新
  C. 新パターン確立 → 「実装パターン」セクションに追記
  D. constitution の技術原則に影響する変更
     → tech.md に反映し、constitution との乖離があればレポートで指摘

やらないこと:
  - research.md の調査結果の全文転記
  - ライブラリの使い方ガイド（コードが語る）
```

### 2-D: current-state.md の更新

**目的**: 「今このシステムはどういう状態か」のスナップショット。

**サイズ上限**: 150行以内。このファイルは毎回大部分が書き換わってよい。

**更新ルール**:

```
更新パターン:
  A.「主要機能一覧」を更新（featureの成果を反映）
  B.「直近で変わった重要点」を更新
     - 前回のロールアップ時の「直近」は「過去の変更」に格下げ or 削除
     - 今回のfeatureの変更を「直近」として記載
  C.「既知制約」を更新
     - featureで解決された制約は削除
     - featureで新たに判明した制約は追加
  D.「未解決課題」を更新
     - spec.md の「Deferred Decisions」や「Out of Scope」から引き継ぐ

特殊ルール:
  - このファイルだけは「上書き的更新」を許容する
  - 過去の状態を保持する必要はない（git 履歴がある）
  - 常に「今」を反映することが最優先
```

---

## Phase 3: specs/ のライフサイクル遷移

### ディレクトリ構成

```
specs/                          ← AIエージェントが通常参照するスコープ
├── 007-current-feature/        ← Active: 全ファイル存在
│   ├── spec.md
│   ├── plan.md
│   ├── tasks.md
│   └── checklists/
├── 006-recent-feature/
│   └── ARCHIVED.md             ← Merged: ポインタファイルのみ
├── 005-auth-system/
│   └── ARCHIVED.md             ← Merged: ポインタファイルのみ
└── ...

archive/                        ← constitution で通常アクセス禁止
├── 006-recent-feature/         ← 全成果物を物理保管
│   ├── spec.md
│   ├── plan.md
│   ├── tasks.md
│   ├── data-model.md
│   ├── research.md
│   ├── contracts/
│   └── checklists/
├── 005-auth-system/
│   ├── spec.md
│   ├── plan.md
│   └── ...
└── ...
```

### Active → Merged 遷移

```
手順:
1. ユーザーに遷移対象のファイル一覧を提示し確認を求める

2. archive/ ディレクトリが存在しなければ作成する

3. specs/<NNN>-<feature>/ の全内容を archive/<NNN>-<feature>/ に移動する
   mv specs/<NNN>-<feature>/ archive/<NNN>-<feature>/

4. specs/<NNN>-<feature>/ ディレクトリを再作成し、
   ポインタファイル ARCHIVED.md のみを配置する

5. ポインタファイルの内容:
```

#### ARCHIVED.md のフォーマット

```markdown
# <NNN>-<feature-name>

**Status**: Merged
**Rolled up to steering/**: YYYY-MM-DD
**Archive location**: archive/<NNN>-<feature-name>/

## Summary
（spec.md の Overview セクションから 3-5行で要約）

## What was built
（spec.md の主要ユーザーストーリーを箇条書き 3-5件）

## Key decisions
（plan.md の重要な技術判断を箇条書き 2-3件）

## Files in archive
- spec.md — 機能仕様（what/why）
- plan.md — 技術計画
- tasks.md — 実行されたタスク一覧
- data-model.md — データモデル定義
- checklists/ — 品質検証チェックリスト
- research.md — 技術調査結果

---
*このfeatureの詳細を参照する必要がある場合は、
archive/<NNN>-<feature-name>/ を確認してください。*
```

### ポインタファイルの設計意図

- **存在証明**: specs/ に痕跡が残るため、`ls specs/` で過去のfeature番号と
  概要が一覧できる。AIエージェントも「かつてこのfeatureがあった」ことを認知できる。
- **軽量サマリー**: ARCHIVED.md 自体が 20-30行の要約なので、
  AIがディレクトリを走査した際に最小限の情報だけ取得できる。
- **参照パスの提供**: 詳細が必要な場合の archive/ パスが明記されている。
  ユーザーが「005の plan を見て」と指示すれば、AIは archive/ を参照できる。
- **コピーの排除**: spec.md の複製が2箇所に存在する不整合を完全に防ぐ。

### Merged → Archived 遷移（オプション）

Merged 状態のポインタファイルが一定数（デフォルト: 10件）を超えた場合、
古いものから Archived への遷移を提案する。

```
手順:
1. Merged 状態の ARCHIVED.md を古い順にリストアップする
2. Archived 候補を提示する（強制はしない）
3. ユーザーが承認した場合:
   a. specs/<NNN>-<feature>/ ディレクトリごと削除する
      （ARCHIVED.md も含めて削除。archive/ には全て保管済み）
   b. git commit で変更を記録する

注: archive/ 側のファイルは一切触らない。永続保管される。
```

### constitution への Archive Policy 追記

`/steering-rollup init` 実行時に、constitution.md に以下のポリシーを追記する。
既に類似の記述が存在する場合はスキップする。

```markdown
## Archive Policy

- `archive/` ディレクトリはマージ済みフィーチャーの成果物保管場所である。
- 通常の開発作業で archive/ 配下のファイルを読み込んではならない。
- archive/ を参照してよいのは、以下の場合に限る:
  - ユーザーが明示的に過去のfeatureの参照を指示した場合
  - 過去のfeatureと同一ドメインの新featureで、判断経緯の確認が必要な場合
- archive/ を参照する場合は必要な1ファイルだけを対象とし、
  ディレクトリ全体を読み込まないこと。
- specs/ 配下の ARCHIVED.md ポインタファイルは参照してよい。
  軽量な要約とarchive/へのパスのみを含む。
```

---

## Phase 4: 変更サマリーの生成

### 出力先

ターミナル出力 + `steering/rollup-log.md` に追記

### フォーマット

```markdown
## Rollup: <NNN>-<feature-name> (YYYY-MM-DD)

### steering/ の更新内容
| ファイル | 変更種別 | 変更内容の要約 |
|---------|---------|--------------|
| product.md | 追記 | ユースケース「○○」を追加 |
| architecture.md | 更新 | 新モジュール「△△」を追加、依存関係を更新 |
| tech.md | 変更なし | — |
| current-state.md | 更新 | 主要機能一覧と直近変更を更新 |

### specs/ のライフサイクル遷移
| spec | 遷移 | specs/ | archive/ |
|------|------|--------|----------|
| 005-new-feature | Active → Merged | ARCHIVED.md（ポインタ） | 全成果物を保管 |

### steering/ のサイズ状況
| ファイル | 行数 | 上限 | 状態 |
|---------|------|------|------|
| product.md | 142 | 200 | ✅ OK |
| architecture.md | 287 | 300 | ⚠️ 近い |
| tech.md | 98 | 200 | ✅ OK |
| current-state.md | 120 | 150 | ✅ OK |

### 注意事項
- architecture.md が上限に近づいています。
  次回ロールアップ時に古い詳細の集約を検討してください。
- constitution.md との乖離: なし
```

---

## `/steering-rollup init` — 初期構築

steering/ がまだ存在しない場合に使用する。

### 情報源

```
1. .specify/memory/constitution.md     ← 原則・技術方針
2. specs/ 配下の既存spec（あれば）       ← 既存featureの what/why
3. コードベースのディレクトリ構造         ← 現在のシステム構成
4. package.json / Cargo.toml 等         ← 技術スタック
5. README.md（あれば）                  ← プロダクト概要
```

### 生成するファイル

```
steering/
├── product.md          ← プロダクトの what/why
├── architecture.md     ← システム構成
├── tech.md             ← 技術スタック・方針
├── current-state.md    ← 現在のスナップショット
└── rollup-log.md       ← ロールアップ履歴（空で初期化）

archive/                ← 空ディレクトリとして作成
└── .gitkeep
```

### constitution.md への追記

init 実行時に、constitution.md の末尾に Archive Policy セクションを追記する。
詳細は Phase 3 の「constitution への Archive Policy 追記」を参照。

各ファイルのテンプレート構造：

#### product.md
```markdown
# Product Overview
<!-- Last rollup: YYYY-MM-DD -->

## プロダクトの目的
（constitution と README から抽出）

## 対象ユーザー
（spec.md 群から抽出、なければ constitution から推定）

## ドメイン境界
（コードベースとspecから推定）

## 主要ユースケース
（既存specのユーザーストーリーを1-2行に集約して列挙）

## 提供している価値
（現在の機能群が提供している価値の概観）
```

#### architecture.md
```markdown
# Architecture Overview
<!-- Last rollup: YYYY-MM-DD -->

## システム構成
（コードベースのディレクトリ構造とplanから推定）

## 主要モジュール
（ディレクトリ構造とplanから推定。名前・責務・1-2行）

## 依存関係
（モジュール間の主要な依存。矢印記法でコンパクトに）

## データフロー
（主要なデータの流れ。ユーザー入力→処理→出力の概要）

## 変更しにくい境界
（外部API、DB スキーマ、公開インターフェース等）
```

#### tech.md
```markdown
# Technology Stack & Practices
<!-- Last rollup: YYYY-MM-DD -->

## 技術スタック
（package.json / Cargo.toml 等から抽出。名前・バージョン・用途）

## バージョン戦略
（constitution の方針を反映）

## 採用ライブラリの原則
（constitution から抽出）

## 実装パターン
（コードベースから検出された主要パターン）

## テスト方針
（constitution + 実際のテスト構成から）
```

#### current-state.md
```markdown
# Current State
<!-- Last rollup: YYYY-MM-DD -->

## 主要機能一覧
（既存specと実装から列挙）

## 既知制約
（specの Constraints / Limitations から集約）

## 未解決課題
（specの Deferred Decisions / Out of Scope から集約）

## 直近で変わった重要点
（init 時は「初期構築」と記載）
```

---

## `/steering-rollup status` — 状態確認

### 出力例

```
Steering Documents:
  product.md        142 lines (limit: 200)  Last rollup: 2026-03-15
  architecture.md   287 lines (limit: 300)  Last rollup: 2026-03-15
  tech.md            98 lines (limit: 200)  Last rollup: 2026-03-10
  current-state.md  120 lines (limit: 150)  Last rollup: 2026-03-15

Feature Specs:
  007-payment         Active    specs/007-payment/ (full)
  006-notification    Merged    specs/006-notification/ARCHIVED.md → archive/006-notification/
  005-auth-system     Merged    specs/005-auth-system/ARCHIVED.md → archive/005-auth-system/
  004-user-profile    Merged    specs/004-user-profile/ARCHIVED.md → archive/004-user-profile/
  003-dashboard       Merged    specs/003-dashboard/ARCHIVED.md → archive/003-dashboard/
  002-data-model      Merged    specs/002-data-model/ARCHIVED.md → archive/002-data-model/
  001-initial-setup   Merged    specs/001-initial-setup/ARCHIVED.md → archive/001-initial-setup/

Archive:
  archive/ contains 6 features, 42 files total

Suggestions:
  - Merged ポインタが 6件です（閾値: 10件）。まだ整理不要です。
```

---

## 推奨ワークフロー

```
/speckit.specify → /speckit.plan → /speckit.tasks → /speckit.implement
  ↓
（実装完了。テスト・レビュー済み）
  ↓
/clear
  ↓
/steering-rollup              ← ★ ここで実行
  ↓
（サマリーを確認。steering/ の変更を git diff で確認）
  ↓
git commit -m "rollup: 005-auth-system → steering/"
  ↓
git merge / PR マージ
  ↓
次のfeatureへ（steering/ が最新のコンテキストとして機能する）
```

### 複数featureの一括ロールアップ

しばらくロールアップを忘れていた場合：

```
/steering-rollup status       ← まず状態を確認
/steering-rollup 003,004,005  ← 古い順にまとめてロールアップ
```

---

## サイズ上限の管理

steering/ の各ファイルにサイズ上限を設けている理由は、
AIエージェントのコンテキストウィンドウを圧迫しないためである。

### 上限超過時の自動対応

```
上限の 80% に達した場合:
  → サマリーで警告を表示する

上限に達した場合:
  → 追記前に、既存の記述を集約する
  → 集約のルール:
     - 個別のユースケースを「○○関連のN件のユースケース」のように集約
     - 古い詳細を削って概要だけ残す
     - 最新の3-5件の変更は詳細を保持する
  → 集約後に追記する
  → 集約で失われた詳細は git 履歴から復元可能
```

---

## 安全設計

### やらないこと

- steering/ のファイルをゼロから再作成しない（init サブコマンド以外）
- constitution.md の Archive Policy 以外のセクションを変更しない
- Active 状態のspecに触らない
- archive/ 内のファイルを変更・削除しない（永続保管）
- コードベースを変更しない

### トレーサビリティ

- steering/ の全変更に `<!-- rollup: NNN-feature-name, YYYY-MM-DD -->` コメント
- specs/ の ARCHIVED.md ポインタファイルに遷移日時と archive パスを記載
- `steering/rollup-log.md` に全ロールアップの履歴を蓄積
- archive/ に全成果物を物理保管（git 履歴に依存しない）

### steering/ が存在しない場合

`/steering-rollup init` の実行を促すメッセージを表示し、
通常のロールアップは実行しない。

### archive/ が存在しない場合

`/steering-rollup init` の実行を促すか、
archive/ ディレクトリを自動作成して続行する。
