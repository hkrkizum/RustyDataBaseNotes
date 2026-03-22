---
name: implement-phase
description: >
  /speckit.implement を Phase 単位でゲートし、Phase 完了ごとに
  コミット・/clear を挟んでコンテキスト汚染を防ぐ。
  tasks.md の Phase 構造と依存関係を解析し、1 Phase ずつ実行する。
  サブエージェント不要。「implement」「実装」「phase」で起動。
---

# /implement-phase

## MUST（必ず守ること）

- tasks.md を解析し、**1 Phase だけ**実行して停止する
- Phase 完了時にコミットコマンドと `/clear` 指示を提示する
- tasks.md のチェックボックス `[x]` を完了タスクごとに更新する
- Phase 間の**依存関係**（`Depends on Phase:` / `Blocked by:` 等）を尊重し、
  依存先 Phase が未完了なら実行しない
- Phase 内の `Depends on:` タスク依存も尊重する
- テストが constitution で要求されているなら Phase 完了前に実行する
- Phase 内タスクが 10 超の場合、分割を提案する

## MUST NOT（絶対にしないこと）

- 2 Phase 以上を同一コンテキストで連続実行しない
- git commit / `/clear` を自動実行しない（提案のみ）
- tasks.md のチェックボックス以外の spec 成果物を変更しない
- Phase の順序を依存関係に反して入れ替えない
- 完了済み Phase のコード詳細をコンテキストに読み込まない

---

## サブコマンド

| コマンド | 動作 |
|---------|------|
| `/implement-phase` | 次の実行可能 Phase を1つ実行 |
| `/implement-phase plan` | 実行計画を表示（実行しない） |
| `/implement-phase status` | 進捗を表示 |
| `/implement-phase <N>` | Phase N を実行（依存関係チェック付き） |

---

## Phase 解析

### tasks.md の構造検出

```
Phase 境界: "## Phase N:" 見出し
フォールバック: "## " 見出し → [US1],[US2] ユーザーストーリー境界

タスク状態: [ ] = 未完了, [x] = 完了, [P] = 並列可
タスク依存: "Depends on: N.M" = タスク間依存
Phase 依存: "Depends on Phase: N" / "Blocked by: Phase N" = Phase 間依存

Phase 状態:
  Completed = 全タスク [x]
  In Progress = 一部 [x]
  Pending = 全タスク [ ]
  Blocked = 依存先 Phase が未完了
```

### Phase 依存関係の解決

tasks.md に Phase 間依存が記述されている場合、依存グラフに従って
実行可能な Phase を判定する。

```
例: tasks.md に以下の記述がある場合

## Phase 1: Foundation
...
## Phase 2: Backend API
Depends on Phase: 1
...
## Phase 3: Frontend Components
Depends on Phase: 1
...
## Phase 4: Integration
Depends on Phase: 2, 3
...
## Phase 5: Testing & Polish
Depends on Phase: 4

依存グラフ:
  1 → 2 → 4 → 5
  1 → 3 ↗

Phase 1 完了後:
  Phase 2, 3 が両方 実行可能（並列候補）
  → /implement-phase → Phase 2 を提案
  → /implement-phase 3 → Phase 3 も実行可能

Phase 2 完了、Phase 3 未完了:
  Phase 4 は Blocked（Phase 3 に依存）
  → /implement-phase → Phase 3 を提案

明示的な依存記述がない場合:
  Phase 番号の昇順で直線的に実行する
```

### 次の Phase の選択ロジック

```
1. tasks.md から全 Phase の状態と依存関係を取得する
2. Completed でも Blocked でもない Phase を列挙する
3. 複数候補がある場合、Phase 番号が最小のものを選択する
4. 複数候補を /implement-phase plan で表示し、
   ユーザーが /implement-phase <N> で選択できるようにする
```

---

## 実行サイクル

### 1. コンテキスト構築（最小限）

```
読み込む:
  constitution.md, spec.md, plan.md, tasks.md
  + 該当 Phase のタスクが参照する付属ドキュメントのみ
    （data-model.md, contracts/ 等）
  + steering/architecture.md, steering/tech.md（存在すれば）

読み込まない:
  他 Phase で生成されたコード詳細
  checklists/, research.md
  完了済みタスクの実装詳細
```

### 2. Phase 内タスク実行

- タスクを上から順に実行。[P] は依存なしならまとめて実行可
- `Depends on:` 未完了タスクはスキップし、依存解決後に戻る
- 各タスク完了時に tasks.md を `[x]` に更新
- 設計レベルの問題発生時は Phase を中断し、問題と選択肢を報告

### 3. Phase 完了レポート

Phase 内の全タスク完了後、以下を表示して**停止する**:

```
Phase N: <title> — Complete ✅
  Completed: N.1, N.2, N.3 ...
  Tests: <結果 or "手動テストを推奨">
  Files changed: <一覧>

Next steps:
  1. git diff --stat で確認
  2. git add -A && git commit -m "feat(NNN): phase N - <title>"
  3. /clear
  4. /implement-phase

Next phase: Phase M (<title>) — 実行可能
  or: Phase M (<title>) — Blocked by Phase K（別 Phase を先に）
```

### 4. `/clear` 後の再開

tasks.md の `[x]` / `[ ]` から進捗を復元する。追加の状態ファイル不要。
中断した Phase は「未完了タスクから再開するか」をユーザーに確認する。

---

## 大きい Phase の分割

Phase 内タスクが 10 超の場合:

```
提案を表示する（自動分割はしない）:
  "Phase N に <M> タスクがあります。分割を推奨します:
   a) タスク N.1-N.7 / N.8-N.M で分割実行
   b) そのまま実行
   c) plan を見直す"

分割実行する場合:
  tasks.md は編集しない。実行時に内部的に区切り、区切りごとにコミット + /clear
```

---

## 全 Phase 完了時

```
🎉 Implementation Complete — <NNN>-<feature>
  Phase 1: ✅  Phase 2: ✅  Phase 3: ✅  ...
  Total: N/N tasks, M commits

Next:
  1. 全体テスト実行
  2. /steering-rollup で全体像更新
  3. PR 作成
```

---

## コミットメッセージ規約

```
feat(<NNN>): phase <N> - <phase title>

- N.1 <task description>
- N.2 <task description>
...

Spec: specs/<NNN>-<feature>/spec.md
```
