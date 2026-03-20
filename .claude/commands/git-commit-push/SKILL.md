---
name: git-commit-push
description: This skill should be used when the user wants to commit their work to git and push to GitHub. It guides through reviewing changes, crafting meaningful commit messages following project conventions (including Conventional Commits when detected), creating commits with security checks, and pushing to remote repositories.
---

**日本語が先頭に，そのあとに英語の説明があります**

# Git コミット＆プッシュ

## 概要

このスキルは、git への変更コミットと GitHub へのプッシュを構造化されたワークフローで提供する。変更のレビュー、意味のある規約に沿ったコミットメッセージの作成、シークレットのコミット防止、リモートリポジトリへの適切なプッシュを保証する。Conventional Commits とカスタムフォーマットの両方に対応し、プロジェクトの規約に自動適応する。

**実行哲学**: このスキルは**コンテキスト自律性**を採用する。安全で単純なコミットは即座に実行し、変更が大きい・複雑・リスクがある場合は確認を求める。速度と安全性のバランスを取る。

## このスキルを使うタイミング

以下のようなユーザーの要求があった場合に使用する：
- 「作業をコミットして」「GitHub にプッシュして」と明示的に要求された場合
- 変更を保存/コミット/プッシュしたいと言った場合
- コミットの作成やコードのプッシュを依頼された場合
- GitHub で作業を共有したい場合

## ワークフロー

### ステップ 1: 変更のレビュー

コミット前に、何が変更されたかをレビューする：

1. **git status を確認**して、変更・新規・削除されたファイルを把握する：
   ```bash
   git status
   ```

2. **差分を体系的に分析**する：
   ```bash
   git diff --stat  # 変更の概要
   git diff         # 行単位の詳細な変更
   ```

3. **変更をカテゴリ分類**してコミットメッセージに反映する：
   - **新機能**: 新しいファイル、関数、機能
   - **バグ修正**: ロジックの修正、エラーハンドリングの改善
   - **リファクタリング**: 動作変更なしの構造変更
   - **ドキュメント**: *.md ファイル、コードコメント、docstring
   - **テスト**: テストファイル、テストの追加・修正
   - **設定**: ビルドファイル、依存関係、設定
   - **スタイル**: フォーマット、空白、コードスタイルのみ

4. **直近のコミットを確認**してコミットメッセージのスタイルを把握する：
   ```bash
   git log --oneline -20
   ```

5. **サマリーをユーザーに提示**する：
   - 変更・新規・削除されたファイルの一覧
   - カテゴリ別の主要な変更点
   - 追加・削除された行数の合計
   - 異常なパターンの指摘（大きなファイル、大量の削除など）

### ステップ 2: プロジェクト規約の検出

プロジェクトが Conventional Commits を使用しているか、カスタムフォーマットかを判定する：

1. **Conventional Commits パターンを確認**する：
   ```bash
   # 直近のコミットで type(scope): フォーマットを検索
   git log --oneline -20 | grep -E "^[a-f0-9]+ (feat|fix|docs|style|refactor|test|chore|perf|ci|build|revert)(\(.+\))?:"
   ```

2. **結果を分析**する：
   - コミットの 50% 以上がパターンに一致 → Conventional Commits を使用
   - それ以外 → カスタムフォーマットを使用

3. **規約を記録**してコミットメッセージ作成に活用する

### ステップ 3: コミットメッセージの作成

検出された規約に従って意味のあるコミットメッセージを作成する：

#### Conventional Commits が検出された場合

フォーマット: `type(scope): description`

**主なタイプ**:
- `feat`: 新機能・新しい機能
- `fix`: バグ修正
- `docs`: ドキュメントのみの変更
- `style`: コードスタイル・フォーマット（ロジック変更なし）
- `refactor`: コード構造の変更（動作変更なし）
- `test`: テストの追加・修正
- `chore`: メンテナンス、依存関係、ビルド
- `perf`: パフォーマンス改善
- `ci`: CI/CD パイプラインの変更
- `build`: ビルドシステムまたは外部依存関係

**オプションのスコープ**: 影響を受けるコンポーネントやモジュール（例: `auth`, `api`, `ui`）

**フォーマット**:
```
type(scope): 命令形の要約（最大50文字）

変更の詳細を説明する任意の本文。
WHAT（何を）ではなく WHY（なぜ）を説明する（差分が何を示す）。

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

**例**:
```
feat(evaluation): add Inspect AI integration to experiment framework

Phase 4 の評価サポートを実装：
- ユーザー作成の評価タスクワークフロー
- アダプタのみのチェックポイント（200倍のストレージ削減）
- 完全な動作例とドキュメント

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

#### カスタムフォーマットが検出された場合

プロジェクトの既存スタイルに合わせる：

1. **変更を1〜2文で要約**する（命令形、現在時制）
2. **「何を」ではなく「なぜ」に焦点**を当てる（差分が変更内容を示す）
3. **観察された規約に従う**（大文字/小文字、句読点、長さ）
4. **簡潔だが情報量のある**メッセージにする

**フォーマット**:
```
要約行（命令形、現在時制）

変更の詳細な説明（任意）。
動機とコンテキストに焦点を当てる。

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

**例**:
```
Add evaluation configuration to experiment definition framework

Inspect AI の統合、アダプタのみのチェックポイントサポート、
Phase 4 実装のための完全な動作例を実装。

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

### ステップ 4: コミットリスクの評価

このコミットが「安全」（自律実行）か「リスクあり」（確認が必要）かを判定する：

**安全なコミット**（自律的に実行）：
- 変更ファイル数 ≤ 5
- 変更行数合計 ≤ 500
- main/master ブランチへのコミットではない
- すべての変更が関連している（ステップ1の同一カテゴリ）
- 異常なパターンがない（大量削除、バイナリファイルなど）
- ユーザーがコミット対象ファイルを指定している

**リスクのあるコミット**（確認が必要）：
- 変更ファイル数 > 5
- 変更行数合計 > 500
- main/master ブランチへのコミット
- 混在する変更タイプ（機能追加 + バグ修正 + リファクタリング）
- 多数の未追跡ファイル（何をコミットすべきか不明確）
- 大きなファイル（> 100KB）
- 異常なパターンが検出された

**リスクありの場合**、サマリーを表示してユーザーに確認する：
```
⚠️ 大規模/複雑なコミットを検出

ファイル: 15ファイル変更 (+2,500行, -300行)
タイプ: 機能追加 (8ファイル), ドキュメント (5ファイル), テスト (2ファイル)
ブランチ: main
メッセージ: "Add evaluation configuration and examples"

複数の論理的な変更が含まれているようです。
オプション:
  [Y] 単一コミットで続行
  [n] キャンセル（分割を手伝います）
  [s] 詳細なファイル一覧を表示
```

**安全な場合**、自律的にステップ5へ進む。

### ステップ 5: シークレットのチェック

ステージング前に、機密情報をスキャンする：

1. **未ステージの変更にシークレットパターンがないか確認**する：
   ```bash
   git diff | grep -E "(api[_-]?key|api[_-]?secret|password|token|secret[_-]?key|private[_-]?key|aws[_-]?access)" -i
   ```

2. **特定のパターンを確認**する：
   - AWS アクセスキー: `AKIA[0-9A-Z]{16}`
   - 汎用 API キー: `[a-zA-Z0-9_-]{32,}`
   - 秘密鍵: `-----BEGIN.*PRIVATE KEY-----`
   - GitHub トークン: `ghp_`, `gho_`, `ghs_`, `ghr_`
   - Bearer トークン: `Bearer [a-zA-Z0-9._-]+`
   - 設定ファイル内のパスワード: `password\s*[:=]\s*["']?[^"'\s]+`

3. **機密ファイルを確認**する：
   ```bash
   git status --short | grep -E "\.env|credentials|secrets|\.pem|\.key|\.p12"
   ```

4. **シークレットが検出された場合**：
   - ❌ **即座に停止**
   - 一致したパターンをユーザーに表示する（完全な値は公開しない）
   - リスクを説明する
   - ユーザーに以下を依頼する：
     - コードからシークレットを削除
     - 環境変数またはシークレット管理を使用
     - 必要に応じてファイルを `.gitignore` に追加
   - シークレットが削除されるまで**続行しない**

5. **シークレットが検出されなかった場合**：
   - ✅ ステージングに進む

### ステップ 6: ステージングとコミット

1. **関連ファイルをステージング**する：
   ```bash
   git add <file1> <file2> ...
   ```

   **ステージングのガイドライン**：
   - この論理的変更に関連するファイルをステージングする
   - 無関係な変更はステージングしない（別のコミット用に残す）
   - ステップ1の変更分析に基づいてステージング対象を決定する
   - **自律実行の場合**: ステージングするファイルを通知する
   - **確認済みの場合**: ユーザーは確認時にファイル一覧を既に確認済み

2. **ステージングされた変更を検証**する：
   ```bash
   git diff --staged --stat
   ```

3. **コミットメッセージを通知または表示**する：
   - **自律実行の場合**: メッセージを簡潔に通知する（"コミット: 'Add feature X'"）
   - **確認済みの場合**: ユーザーは確認時にメッセージを既に確認済み

4. **フォーマットされたメッセージでコミットを作成**する：
   ```bash
   git commit -m "$(cat <<'EOF'
   <コミットメッセージ>

   🤖 Generated with [Claude Code](https://claude.com/claude-code)

   Co-Authored-By: Claude <noreply@anthropic.com>
   EOF
   )"
   ```

5. **コミットが作成されたことを検証**する：
   ```bash
   git log -1 --stat
   ```

### ステップ 7: リモートへのプッシュ

1. **リモートの状態を確認**する：
   ```bash
   git status  # ブランチがリモートを追跡しているか確認
   ```

2. **GitHub にプッシュ**する：
   ```bash
   # ブランチが既にリモートを追跡している場合
   git push

   # 新しいブランチの場合（初回プッシュ）
   git push -u origin <branch-name>
   ```

3. **成功を確認**する：
   - プッシュ結果をユーザーに報告
   - プッシュされたコミット数を表示
   - 利用可能な場合はリモート URL を提供
   - 警告や問題があれば通知

## 重要なガイドライン

### 安全ルール

- ユーザーの明示的な確認なしに main/master への**強制プッシュは絶対にしない**
- ユーザーが明示的に要求しない限り**フックをスキップしない**（--no-verify）
- **シークレットは絶対にコミットしない** - 検出したら停止してユーザーに警告
- コミットを修正する前に**作成者を確認**する（他人の作業を修正しない）
- **大きなファイルを確認**する - 100KB を超えるファイルは警告（Git LFS が必要な場合がある）
- 実行前に**コミット対象を必ず検証**する

### ベストプラクティス

1. **コミット前にレビュー** - コミット対象をユーザーに表示する
2. **意味のあるメッセージ** - 何を変更したかではなく、なぜ変更したかを説明する
3. **アトミックなコミット** - 各コミットは単一の論理的変更であるべき
4. **規約に従う** - プロジェクトのコミットスタイルを検出して合わせる
5. **共同作成者の帰属を追加** - Claude Code のフッターを含める
6. **コミットを焦点化する** - リファクタリングと機能追加を混ぜない

### コミットスコープのガイドライン

**良いコミットスコープ**：
- ✅ 1コミットにつき1つの論理的変更
- ✅ 関連ファイルをまとめる（例: コード + 対応するテスト）
- ✅ 1文で説明できる
- ✅ 通常 500行未満（リネーム/リファクタリングを除く）
- ✅ すべての変更が同じ目的を果たす

**コミットを分割すべきサイン**：
- ❌ コミットメッセージで「and」を2回以上使用
- ❌ 変更が複数の無関係な機能にまたがる
- ❌ バグ修正と新機能の混在
- ❌ 実験的な変更と本番コードの混在
- ❌ 「WIP」「misc fixes」「various updates」というメッセージ

**分割する場合**：
```bash
# 無関係な変更がある場合、選択的にステージングする：
git add -p  # インタラクティブにステージングするハンクを選択

# または特定のファイルをステージング：
git add file1.py file1_test.py  # 最初のコミット: 機能A
git commit -m "feat: add feature A"

git add file2.py file2_test.py  # 2番目のコミット: 機能B
git commit -m "feat: add feature B"
```

### エラーハンドリング

**コミットが失敗した場合**：
- pre-commit フックがファイルを変更していないか確認
- エラーメッセージを注意深く確認
- フックで変更されたファイルをステージングして修正が必要な場合がある
- 修正するのは以下の場合のみ: (1) ユーザーが要求した場合 または (2) pre-commit フックの変更を修正する場合で安全な場合（作成者を確認、未プッシュ）

**シークレットが検出された場合**：
- 即座に停止し、コミットしない
- 一致したパターンを説明
- シークレットの削除をユーザーにガイド
- 環境変数またはシークレット管理の使用を提案
- `.gitignore` エントリの追加を検討

**プッシュが失敗した場合**：
- リモートブランチの状態を確認（先にプルが必要な場合がある）
- マージコンフリクトを確認
- リモート URL が正しいか確認
- 認証/権限を確認
- 修正案とともにエラーをユーザーに報告

**大きなファイルが検出された場合**：
- ファイルサイズについてユーザーに警告
- 100KB を超えるファイルには Git LFS を提案
- ファイルをコミットするか `.gitignore` に追加するか確認

## インタラクション例

### 例1: Conventional Commits プロジェクト

**ユーザー**: 「作業をコミットして GitHub にプッシュしたい」

**アシスタント**:
1. `git status` と `git diff --stat` を実行
2. 分析: 「2ファイル変更 (+1,269行)」
3. カテゴリ分類: 「新機能（eval config）、ドキュメント」
4. リスク評価: ✅ 安全（2ファイル、1,269行、フィーチャーブランチ）
5. 規約確認: Conventional Commits を検出（15/20が一致）
6. シークレットスキャン: ✅ 検出なし
7. 通知: 「コミット: `feat(evaluation): add Inspect AI integration`」
8. 実行: `git add`, `git commit`, `git push`
9. 報告: 「✓ 795efaf をコミットして GitHub にプッシュしました」

### 例2: 確認が必要な大規模コミット

**ユーザー**: 「変更をコミットして」

**アシスタント**:
1. 分析: 「15ファイル変更 (+2,847行, -156行)」
2. カテゴリ分類: 「機能追加 (8ファイル), ドキュメント (5ファイル), テスト (2ファイル)」
3. リスク評価: ⚠️ **リスクあり**（>5ファイル、>500行、混在タイプ）
4. シークレットスキャン: ✅ 検出なし
5. 確認を表示:
   ```
   ⚠️ 混在する変更を含む大規模コミットを検出

   ファイル: 15 (+2,847行, -156行)
   カテゴリ: 機能追加 (8), ドキュメント (5), テスト (2)
   メッセージ: "Add evaluation framework with examples and tests"

   分割を検討:
   - コミット1: コア評価フレームワーク (8ファイル)
   - コミット2: ドキュメント (5ファイル)
   - コミット3: テスト (2ファイル)

   単一コミットで続行しますか? [Y/n/split]
   ```
6. ユーザーの応答を**待つ**

### 例3: シークレット検出（常にブロック）

**ユーザー**: 「変更をコミットして」

**アシスタント**:
1. 変更をレビュー: config.py を変更 (+10行)
2. シークレットスキャン: ❌ API キーに一致するパターンを**検出**
3. **停止**: 「⚠️ セキュリティ警告: config.py で潜在的な API キーを検出しました」
4. 一致した行を表示: `api_key = "sk_live_..."`
5. アドバイス: 「代わりに環境変数を使用してください: `api_key = os.getenv('API_KEY')`」
6. シークレットが削除されたことをユーザーが確認するまで**続行しない**

## 注意事項

- このスキルは Bash ツールドキュメントの git 安全プロトコルに従う
- コミットメッセージのフォーマットにはプロジェクト規約に従った Claude Code の帰属表示を含む
- コマンドは常に順次実行する（ステージング、コミット、プッシュ）。並列実行はしない
- 単一の標準を強制するのではなく、プロジェクトの規約に適応する
- **コンテキスト自律性**を使用: 安全なコミットは即座に実行、リスクのあるコミットは確認する
- セキュリティチェックは交渉不可 - シークレットは常にブロックする
- 閾値: 安全 ≤ 5ファイル、≤ 500行、main/master 以外、単一の変更タイプ

## 参考資料

- [Conventional Commits 仕様](https://www.conventionalcommits.org/)
- [Git コミットのベストプラクティス](https://cbea.ms/git-commit/)
- [Pre-commit フレームワーク](https://pre-commit.com/)
- [Git フックドキュメント](https://git-scm.com/docs/githooks)

# Git Commit and Push

## Overview

This skill provides a structured workflow for committing changes to git and pushing to GitHub. It ensures changes are reviewed, commit messages are meaningful and follow conventions, secrets are not committed, and commits are properly pushed to remote repositories. The skill adapts to project conventions, supporting both Conventional Commits and custom formats.

**Execution Philosophy**: This skill uses **contextual autonomy** - it executes immediately for safe, straightforward commits, but asks for confirmation when changes are large, complex, or potentially risky. This balances speed with safety.

## When to Use This Skill

Use this skill when the user:
- Explicitly requests to "commit my work" or "push to GitHub"
- Says they want to save/commit/push their changes
- Asks to create a commit or push code
- Wants to share their work on GitHub

## Workflow

### Step 1: Review Changes

Before committing, review what has changed:

1. **Check git status** to see modified, new, and deleted files:
   ```bash
   git status
   ```

2. **Analyze diff systematically**:
   ```bash
   git diff --stat  # Overview of changes
   git diff         # Detailed line-by-line changes
   ```

3. **Categorize changes** to inform commit message:
   - **New features**: New files, new functions, new capabilities
   - **Bug fixes**: Modified logic, error handling improvements
   - **Refactoring**: Structure changes with no behavior change
   - **Documentation**: *.md files, code comments, docstrings
   - **Tests**: Test files, test additions/modifications
   - **Configuration**: Build files, dependencies, settings
   - **Styling**: Formatting, whitespace, code style only

4. **Check recent commits** to understand commit message style:
   ```bash
   git log --oneline -20
   ```

5. **Present summary** to the user:
   - List all modified, new, and deleted files
   - Highlight key changes by category
   - Note total lines added/removed
   - Flag any unusual patterns (large files, many deletions, etc.)

### Step 2: Detect Project Convention

Determine if project uses Conventional Commits or custom format:

1. **Check for Conventional Commits pattern**:
   ```bash
   # Look for type(scope): format in recent commits
   git log --oneline -20 | grep -E "^[a-f0-9]+ (feat|fix|docs|style|refactor|test|chore|perf|ci|build|revert)(\(.+\))?:"
   ```

2. **Analyze result**:
   - If ≥50% of commits match pattern → Project uses Conventional Commits
   - Otherwise → Project uses custom format

3. **Note the convention** for commit message crafting

### Step 3: Craft Commit Message

Create a meaningful commit message following detected convention:

#### If Conventional Commits Detected

Use format: `type(scope): description`

**Common types**:
- `feat`: New feature or capability
- `fix`: Bug fix
- `docs`: Documentation changes only
- `style`: Code style/formatting (no logic change)
- `refactor`: Code restructuring (no behavior change)
- `test`: Adding or modifying tests
- `chore`: Maintenance, dependencies, build
- `perf`: Performance improvements
- `ci`: CI/CD pipeline changes
- `build`: Build system or external dependencies

**Optional scope**: Component or module affected (e.g., `auth`, `api`, `ui`)

**Format**:
```
type(scope): imperative summary (max 50 chars)

Optional body explaining the change in detail.
Explain WHY, not WHAT (the diff shows what).

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

**Example**:
```
feat(evaluation): add Inspect AI integration to experiment framework

Implements Phase 4 evaluation support with:
- User-written evaluation tasks workflow
- Adapter-only checkpointing (200x storage reduction)
- Complete working examples and documentation

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

#### If Custom Format Detected

Match the project's existing style:

1. **Summarize changes** in 1-2 sentences (imperative, present tense)
2. **Focus on "why"** rather than "what" (diff shows what changed)
3. **Follow observed conventions** (capitalization, punctuation, length)
4. **Keep concise** but informative

**Format**:
```
Summary line (imperative mood, present tense)

Optional detailed explanation of the changes.
Focus on motivation and context.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

**Example**:
```
Add evaluation configuration to experiment definition framework

Implements Inspect AI integration, adapter-only checkpointing support,
and complete working examples for Phase 4 implementation.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

### Step 4: Assess Commit Risk

Determine whether this commit is "safe" (autonomous) or "risky" (requires confirmation):

**Safe commits** (execute autonomously):
- ≤ 5 files changed
- ≤ 500 total lines changed
- Not committing to main/master branch
- All changes are related (same category from Step 1)
- No unusual patterns (massive deletions, binary files, etc.)
- User specified which files to commit

**Risky commits** (require confirmation):
- > 5 files changed
- > 500 total lines changed
- Committing to main/master branch
- Mixed change types (features + fixes + refactoring)
- Many untracked files (unclear what to commit)
- Large files (> 100KB)
- Unusual patterns detected

**If risky**, show summary and ask user:
```
⚠️ Large/complex commit detected

Files: 15 files changed (+2,500 lines, -300 lines)
Types: Features (8 files), Documentation (5 files), Tests (2 files)
Branch: main
Message: "Add evaluation configuration and examples"

This appears to contain multiple logical changes.
Options:
  [Y] Proceed with single commit
  [n] Cancel (I'll help you split it)
  [s] Show detailed file list
```

**If safe**, proceed autonomously to Step 5.

### Step 5: Check for Secrets

Before staging, scan for sensitive information:

1. **Check unstaged changes for secret patterns**:
   ```bash
   git diff | grep -E "(api[_-]?key|api[_-]?secret|password|token|secret[_-]?key|private[_-]?key|aws[_-]?access)" -i
   ```

2. **Check for specific patterns**:
   - AWS Access Keys: `AKIA[0-9A-Z]{16}`
   - Generic API keys: `[a-zA-Z0-9_-]{32,}`
   - Private keys: `-----BEGIN.*PRIVATE KEY-----`
   - GitHub tokens: `ghp_`, `gho_`, `ghs_`, `ghr_`
   - Bearer tokens: `Bearer [a-zA-Z0-9._-]+`
   - Passwords in configs: `password\s*[:=]\s*["']?[^"'\s]+`

3. **Check for sensitive files**:
   ```bash
   git status --short | grep -E "\.env|credentials|secrets|\.pem|\.key|\.p12"
   ```

4. **If secrets detected**:
   - ❌ **STOP immediately**
   - Show user the matched patterns (without revealing full values)
   - Explain the risk
   - Ask user to:
     - Remove secrets from code
     - Use environment variables or secret management
     - Add file to `.gitignore` if appropriate
   - **Do not proceed** until secrets are removed

5. **If no secrets detected**:
   - ✅ Proceed to staging

### Step 6: Stage and Commit Changes

1. **Stage relevant files**:
   ```bash
   git add <file1> <file2> ...
   ```

   **Staging guidelines**:
   - Stage files related to this logical change
   - Do NOT stage unrelated changes (save for separate commit)
   - Determine files to stage based on change analysis from Step 1
   - **If autonomous**: Announce which files are being staged
   - **If confirmed**: User already saw file list in confirmation

2. **Verify staged changes**:
   ```bash
   git diff --staged --stat
   ```

3. **Announce or show commit message**:
   - **If autonomous**: Announce message briefly ("Committing: 'Add feature X'")
   - **If confirmed**: User already saw message in confirmation

4. **Create commit** with formatted message:
   ```bash
   git commit -m "$(cat <<'EOF'
   <commit message here>

   🤖 Generated with [Claude Code](https://claude.com/claude-code)

   Co-Authored-By: Claude <noreply@anthropic.com>
   EOF
   )"
   ```

5. **Verify commit** was created:
   ```bash
   git log -1 --stat
   ```

### Step 7: Push to Remote

1. **Check remote status**:
   ```bash
   git status  # Check if branch tracks remote
   ```

2. **Push to GitHub**:
   ```bash
   # If branch already tracks remote
   git push

   # If new branch (first push)
   git push -u origin <branch-name>
   ```

3. **Confirm success**:
   - Report push result to user
   - Show number of commits pushed
   - Provide remote URL if available
   - Note any warnings or issues

## Important Guidelines

### Safety Rules

- **Never push force** to main/master without explicit user confirmation
- **Never skip hooks** (--no-verify) unless explicitly requested by user
- **Never commit secrets** - stop and warn user if detected
- **Check authorship** before amending commits (don't amend others' work)
- **Check for large files** - warn if files > 100KB (may need Git LFS)
- **Always verify** what's being committed before executing

### Best Practices

1. **Review before committing** - show user what will be committed
2. **Meaningful messages** - explain why changes were made, not just what
3. **Atomic commits** - each commit should be a single logical change
4. **Follow conventions** - detect and match project's commit style
5. **Add co-author attribution** - include Claude Code footer
6. **Keep commits focused** - don't mix refactoring with features

### Commit Scope Guidelines

**Good commit scope**:
- ✅ One logical change per commit
- ✅ Related files together (e.g., code + corresponding test)
- ✅ Can be described in one sentence
- ✅ Typically < 500 lines (unless it's a rename/refactor)
- ✅ All changes serve the same purpose

**Signs to split commits**:
- ❌ Using "and" more than once in commit message
- ❌ Changes span multiple unrelated features
- ❌ Mix of bug fixes and new features
- ❌ Experimental changes alongside production code
- ❌ "WIP", "misc fixes", or "various updates" messages

**When to split**:
```bash
# If you have unrelated changes, stage selectively:
git add -p  # Interactively choose hunks to stage

# Or stage specific files:
git add file1.py file1_test.py  # First commit: feature A
git commit -m "feat: add feature A"

git add file2.py file2_test.py  # Second commit: feature B
git commit -m "feat: add feature B"
```

### Error Handling

**If commit fails**:
- Check for pre-commit hooks modifying files
- Review error message carefully
- May need to stage hook-modified files and amend
- Only amend if: (1) user requested it OR (2) fixing pre-commit hook changes AND it's safe (check authorship, not pushed)

**If secrets detected**:
- Stop immediately, do not commit
- Explain which patterns matched
- Guide user to remove secrets
- Suggest using environment variables or secret management
- Consider adding `.gitignore` entries

**If push fails**:
- Check remote branch status (might need pull first)
- Check for merge conflicts
- Verify remote URL is correct
- Check authentication/permissions
- Report error to user with suggested fixes

**If large files detected**:
- Warn user about file sizes
- Suggest Git LFS for files > 100KB
- Ask if files should be committed or added to `.gitignore`

## Example Interaction

### Example 1: Conventional Commits Project

**User**: "I'd like to commit my work and push to GitHub"

**Assistant**:
1. Runs `git status` and `git diff --stat`
2. Analyzes: "2 files changed (+1,269 lines)"
3. Categorizes: "New feature (eval config), documentation"
4. Risk assessment: ✅ Safe (2 files, 1,269 lines, feature branch)
5. Checks convention: Detects Conventional Commits (15/20 matches)
6. Scans for secrets: ✅ None detected
7. Announces: "Committing: `feat(evaluation): add Inspect AI integration`"
8. Executes: `git add`, `git commit`, `git push`
9. Reports: "✓ Committed 795efaf and pushed to GitHub"

### Example 2: Large Commit Requiring Confirmation

**User**: "commit my changes"

**Assistant**:
1. Analyzes: "15 files changed (+2,847 lines, -156 lines)"
2. Categorizes: "Features (8 files), docs (5 files), tests (2 files)"
3. Risk assessment: ⚠️ **Risky** (>5 files, >500 lines, mixed types)
4. Scans for secrets: ✅ None detected
5. Shows confirmation:
   ```
   ⚠️ Large commit with mixed changes detected

   Files: 15 (+2,847 lines, -156 lines)
   Categories: Features (8), Documentation (5), Tests (2)
   Message: "Add evaluation framework with examples and tests"

   Consider splitting into:
   - Commit 1: Core evaluation framework (8 files)
   - Commit 2: Documentation (5 files)
   - Commit 3: Tests (2 files)

   Proceed with single commit? [Y/n/split]
   ```
6. **Waits** for user response

### Example 3: Secrets Detected (Always Blocks)

**User**: "commit my changes"

**Assistant**:
1. Reviews changes: Modified config.py (+10 lines)
2. Scans for secrets: ❌ **FOUND** pattern matching API key
3. **STOPS**: "⚠️ Security Warning: Detected potential API key in config.py"
4. Shows matched line: `api_key = "sk_live_..."`
5. Advises: "Please use environment variables instead: `api_key = os.getenv('API_KEY')`"
6. **Does not proceed** until user confirms secrets are removed

## Notes

- This skill follows the git safety protocol from the Bash tool documentation
- Commit message format includes Claude Code attribution as per project conventions
- Always run commands sequentially (staging, committing, pushing) not in parallel
- Adapts to project conventions rather than enforcing a single standard
- Uses **contextual autonomy**: executes immediately for safe commits, confirms for risky ones
- Security checks are non-negotiable - always blocks on secrets
- Thresholds: Safe ≤ 5 files, ≤ 500 lines, not main/master, single change type

## References

- [Conventional Commits Specification](https://www.conventionalcommits.org/)
- [Git Commit Best Practices](https://cbea.ms/git-commit/)
- [Pre-commit Framework](https://pre-commit.com/)
- [Git Hooks Documentation](https://git-scm.com/docs/githooks)
