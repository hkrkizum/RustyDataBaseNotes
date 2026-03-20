# Research: ブロックエディタ

**Feature Branch**: `002-block-editor`
**Date**: 2026-03-21

## Decision 1: アーキテクチャ — Rust 中心の EditorSession

**Decision**: バックエンドに EditorSession（インメモリ状態）を導入し，全ブロック操作を Rust のドメインロジックで処理する

**Rationale**:
- **型安全性**: ブロックの追加・編集・並び替え・削除の全操作が Rust の型システムで保護される。`BlockContent` の文字数制限，`BlockPosition` の非負制約など，値オブジェクトのバリデーションがコンパイル時に保証される
- **テスト容易性**: EditorSession は DB に依存しない純粋なドメインロジック — `#[cfg(test)]` で DB セットアップなしにユニットテストが完結する。フロントエンドのテストは表示確認のみでよい
- **ロジックの一元化**: ビジネスロジックが Rust 側に集約されるため，TypeScript との二重実装が不要。バリデーション漏れのリスクが減る
- **状態の一貫性**: EditorSession が唯一の信頼できる状態源（single source of truth）として機能し，フロントエンドは常にバックエンドから受け取った状態を表示するだけ
- **IPC オーバーヘッド**: 操作ごとに IPC 呼び出しが発生するが，Tauri のローカル IPC はプロセス内通信であり，遅延は無視できる（<1ms）

**Alternatives considered**:
- **フロントエンド中心（最初の案）**: `list_blocks` + `save_blocks` の 2 コマンドのみ。ブロック操作はすべて TypeScript の配列操作。シンプルだが，バリデーション・状態遷移が TypeScript に分散し，Rust の型安全性を活かせない
- **ハイブリッド（バリデーションのみ Rust）**: 個別のバリデーション IPC コマンド（`validate_content` 等）を提供する方式。呼び出しが煩雑で，状態管理はフロントエンドに残る

## Decision 2: EditorSession の設計 — 純粋ドメインサービス

**Decision**: `domain::editor::EditorSession` として実装。DB に一切依存しない純粋なドメインオブジェクト

**Rationale**:
- 憲章 Principle III（DDD）に準拠: ドメイン層は外部技術（DB，IPC）に依存してはならない
- `EditorSession::new(page_id, blocks)` でインスタンス化 — ブロックのロードは IPC 層が担当
- メソッド: `add_block()`, `edit_block_content()`, `move_block_up()`, `move_block_down()`, `remove_block()`, `blocks()`, `is_dirty()`, `mark_saved()`
- 各メソッドは `Result<(), BlockError>` を返し，不正な操作（存在しないブロック ID 等）はエラーで伝播

**セッションのライフサイクル**:
1. `open_editor`: IPC 層がリポジトリからブロックをロード → `EditorSession::new()` → `AppState.sessions` に格納
2. 操作コマンド: IPC 層が `sessions` から取得 → EditorSession のメソッド呼び出し → EditorState を返却
3. `save_editor`: IPC 層が EditorSession からブロックを取得 → リポジトリで永続化 → `mark_saved()`
4. `close_editor`: `sessions` から削除

**Alternatives considered**:
- **Application Service 層の導入**: EditorSession をアプリケーション層に配置し，リポジトリとの橋渡しをアプリケーションサービスが行う方式。本スライスではユースケースが単純なため，IPC 層が直接リポジトリと EditorSession を橋渡しすれば十分。複雑化した段階で導入を検討
- **EditorSession に Repository を注入**: EditorSession が自身で DB にアクセスする方式。ドメイン層の DB 依存を招き，Principle III に違反

## Decision 3: セッション状態管理 — Mutex + HashMap

**Decision**: `AppState` に `sessions: Mutex<HashMap<PageId, EditorSession>>` を追加

**Rationale**:
- Tauri の `State<'_, AppState>` で IPC コマンド間で共有可能
- `Mutex` は操作ごとにロック → 操作 → アンロック（ロック粒度はコマンド単位）
- シングルユーザーのデスクトップアプリのため，同時に開くエディタは基本 1 つ — 競合は事実上発生しない
- `HashMap` により複数ページの同時編集にも対応可能（将来のタブ UI 等）

**Mutex の安全性**:
- `tokio::sync::Mutex` を使用（async 対応）
- ロック保持中に `await` する操作はない（EditorSession のメソッドはすべて同期）ため，`std::sync::Mutex` でも可。ただし Tauri の async コマンドとの統合を考慮し `tokio::sync::Mutex` を採用
- デッドロック: 単一の Mutex のみ使用するため，デッドロックの可能性はない

**Alternatives considered**:
- **RwLock**: 読み取りと書き込みを区別できるが，ほぼすべての操作が書き込みであるため `Mutex` で十分
- **Channel ベース（Actor パターン）**: mpsc チャンネルでメッセージパッシング。シングルユーザーアプリには過剰
- **Mutex なし（コマンドごとに DB 読み書き）**: 操作ごとに DB に書き込む方式。「明示的保存」の要件に反し，パフォーマンスも低下

## Decision 4: ブロック ID 管理 — 追加時に即座に UUIDv7 生成

**Decision**: `add_block()` 呼び出し時点でバックエンドが UUIDv7 を生成し，フロントエンドには常に正式 ID を返す

**Rationale**:
- EditorSession がバックエンドにあるため，ブロック追加時に即座に `Uuid::now_v7()` で ID を生成できる
- 仮 ID（temp-xxx）の概念が不要になり，設計が大幅に単純化される
- フロントエンドは常に安定した ID を持つ — React の `key` prop に直接使用可能
- 保存時の ID 差し替えロジックが不要

**Alternatives considered**:
- **仮 ID 方式（最初の案）**: フロントエンドが `temp-{n}` を割り当て，保存時にバックエンドが UUIDv7 に置換。EditorSession アプローチにより不要になった

## Decision 5: 一括保存のバックエンド戦略 — Delete-and-Reinsert

**Decision**: トランザクション内で該当ページの全ブロックを DELETE し，EditorSession の全ブロックを INSERT する

**Rationale**:
- 実装が最もシンプル — diff 計算，既存レコードとの照合，UPDATE/INSERT/DELETE の振り分けが不要
- トランザクション内で実行するため，途中失敗時は全変更がロールバックされる（CC-001 準拠）
- ブロック数が 1,000 件の場合でも，DELETE 1 回 + INSERT 1,000 回は SQLite の WAL モードで十分高速
- EditorSession が `created_at` を保持しているため，再挿入しても作成日時は保存される

**保存フロー**:
1. EditorSession から `blocks()` で全ブロックを取得
2. トランザクション開始
3. `DELETE FROM blocks WHERE page_id = ?`
4. 各ブロックを INSERT（EditorSession が保持する全フィールドをそのまま使用）
5. `updated_at` は保存時点の現在時刻に更新
6. トランザクション COMMIT
7. `EditorSession::mark_saved()` で dirty フラグをリセット

**Alternatives considered**:
- **Upsert（INSERT OR REPLACE）**: 削除されたブロックの検出に別途ロジックが必要
- **差分更新**: diff 計算の複雑性が大幅に増加

## Decision 6: 画面切り替え — React 状態管理

**Decision**: React の useState による画面切り替え（ルーティングライブラリ不使用）

**Rationale**:
- 仕様で明示的に「フロントエンドのルーティングライブラリは導入しない」と定義
- 画面は 2 つのみ（ページ一覧 / エディタ）— 条件分岐で十分
- `App.tsx` で `currentView: { type: 'list' } | { type: 'editor', pageId: string }` を管理
- ページクリックで `editor` に遷移，戻るボタンで `list` に遷移

**Alternatives considered**:
- **React Router / TanStack Router**: 2 画面に対して過剰。YAGNI 原則に反する

## Decision 7: フロントエンドの状態管理 — バックエンド駆動

**Decision**: フロントエンドは `EditorState` を受け取って表示するだけ。ローカル状態は持たない

**Rationale**:
- EditorSession がバックエンドにあるため，フロントエンドが独自のブロック配列を管理する必要がない
- 各 IPC コマンドの返り値が `EditorState { blocks, isDirty }` — これをそのまま React の state にセット
- 状態の不一致が構造的に起きない（single source of truth = バックエンド）
- `useEditor` フックは IPC 呼び出しのラッパーのみ

**唯一のフロントエンドローカル状態**:
- `isLoading` / `isSaving`: 非同期操作の進行状態（UI フィードバック用）
- テキスト入力のリアルタイム反映: 入力値はローカルに保持し，`onBlur` または debounce で `edit_block_content` を呼び出す（入力ごとの IPC 呼び出しを避けるため）

## Decision 8: テキスト入力の同期戦略 — onBlur でバックエンドに送信

**Decision**: テキスト入力は `<textarea>` のローカル state で保持し，`onBlur`（フォーカス喪失）時に `edit_block_content` IPC を呼び出す

**Rationale**:
- キーストロークごとに IPC を呼び出すと，高速タイピング時に不要な呼び出しが大量発生する
- `onBlur` はユーザーが他のブロックや操作ボタンに移動したタイミングで発火 — 自然な同期ポイント
- 保存操作（Ctrl+S / ボタン）の前にも，フォーカス中のブロックの内容を送信する必要がある — 保存前に明示的に同期する
- textarea のローカル state はあくまでバッファであり，ビジネスロジックではない

**Alternatives considered**:
- **キーストロークごとの IPC**: 遅延は小さいが不要な呼び出しが多い。1 文字入力するたびにバックエンド往復は過剰
- **debounce（300ms）**: onBlur より細かい粒度で同期できるが，エディタの操作フローでは onBlur で十分

## Decision 9: Ctrl+S ショートカット

**Decision**: `useEffect` で `keydown` イベントをリッスンし，保存前にフォーカス中ブロックを同期

**Rationale**:
- エディタ画面がマウントされている間のみ有効
- `e.ctrlKey && e.key === 's'`（+ macOS の `e.metaKey`）を検出
- 保存前にフォーカス中のブロックの textarea 内容を `edit_block_content` で同期してから `save_editor` を呼び出す
- ボタンクリックと同じ保存パスを使用

## Decision 10: 未保存確認ダイアログ

**Decision**: カスタムモーダルコンポーネント（既存の DeleteConfirmModal と同パターン）

**Rationale**:
- `window.confirm()` はデスクトップアプリの UX に合わない
- 既存の `DeleteConfirmModal` と同じパターンで `UnsavedConfirmModal` を作成
- `isDirty` はバックエンドの EditorState から取得
- 「破棄」選択時は `close_editor` → ページ一覧に遷移

## Decision 11: position の管理

**Decision**: EditorSession が position を自動管理。フロントエンドは配列順序のみを意識する

**Rationale**:
- FR-012:「保存時にフロントエンドの配列順序を 0 始まりで振り直す」— EditorSession が内部で position を管理するため，フロントエンドは配列のインデックスとして暗黙的に position を持つ
- `add_block()`: 末尾に追加（position = 現在の最大 + 1）
- `move_block_up(id)` / `move_block_down(id)`: 隣接ブロックと position を交換
- `remove_block(id)`: 削除後に position を 0 始まりで振り直し
- `save_editor`: 保存前に position を 0 始まりの連番に正規化

## Decision 12: ブロック内容の文字数制限

**Decision**: バックエンド（BlockContent 値オブジェクト）で一元管理 + フロントエンドでの UX ヒント

**Rationale**:
- EditorSession アプローチにより，バリデーションはバックエンドの `BlockContent::try_from()` に一元化
- `edit_block_content` の返り値がエラーの場合，フロントエンドは toast で通知
- フロントエンドでは `maxLength` 属性で入力制限のヒントを表示（UX 向上のため）
- バックエンドが唯一のバリデーション権限を持つ
