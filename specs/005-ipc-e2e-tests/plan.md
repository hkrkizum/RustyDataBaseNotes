# Implementation Plan: IPC テストおよび E2E テストの追加

**Branch**: `005-ipc-e2e-tests` | **Date**: 2026-03-22 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/005-ipc-e2e-tests/spec.md`

## Summary

全 38 の IPC コマンドハンドラに対する統合テストと，主要ワークフロー 4 つをカバーする
E2E テストを追加する。IPC テストはハンドラの内部ロジック関数を `AppState` を用いて
直接呼び出す形式（Tauri ランタイム不要）とし，E2E テストは `tauri-driver` + WebDriverIO
によるデスクトップアプリ全体の自動操作で検証する。

## Technical Context

**Language/Version**: Rust 2024 (edition = "2024", toolchain 1.94.0), TypeScript ~5.8.3
**Primary Dependencies**: Tauri 2, React 19, sqlx 0.8 (SQLite), uuid 1 (v7), chrono 0.4, thiserror 2, serde 1, serde_json 1, tokio 1 (sync)
**Storage**: SQLite (WAL mode), `sqlx::migrate!()` 埋め込みマイグレーション（6 ファイル），PRAGMA foreign_keys = ON
**Testing**: `cargo nextest run`（Rust），`pnpm vitest run`（TypeScript），`cargo make qa`（品質ゲート）。IPC テストは `cargo make test` に統合，E2E テストは `cargo make e2e` として独立
**Target Platform**: Desktop（WSL2 Linux + WSLg 開発環境）
**Project Type**: desktop-app
**Performance Goals**: IPC テストスイート全体 < 数分。E2E テストはシナリオ単位で独立実行可能
**Constraints**: オフライン完結，パニック禁止（テストコード除外），テスト間 DB 完全分離
**Scale/Scope**: 38 IPC コマンド × 正常系 + 異常系テスト，4 E2E ワークフロー
**E2E Environment**: WSLg 有効環境では直接実行。WSLg 無効時は `xvfb-run cargo make e2e` で代替（[quickstart.md](./quickstart.md) 参照）。E2E テスト環境は Nix devshell（flake.nix）により再現される。WebKitGTK は flake.nix devShell で管理（Constitution Technical Standards 参照）。tauri-driver は `cargo install tauri-driver` でインストールする（Nix devshell への統合は将来検討） <!-- added by checklist-apply: P-10, P-12, P-13 -->

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **I. Local-First Product Integrity**: テストインフラは本番データに一切触れない。IPC テストは
  テストごとに一時 SQLite ファイルを作成・削除し，E2E テストはスイート単位で一時 DB を使用する。
  外部通信なし。バックアップ・復旧動作への影響なし。

- **II. Domain-Faithful Information Model**: テストは既存の 6 ドメイン（Database, Page, Editor,
  Property, Table, View）の語彙を一貫して使用する。新しいドメインエンティティの導入はなく，
  既存のブロック，ページ，データベース，ビュー，プロパティの語彙をテスト内でも忠実に使用する。

- **III. Typed Boundaries and Domain-Driven Design**: IPC テストは既存の CommandError 型，
  DTO 型（PageDto, DatabaseDto, EditorStateDto 等），ドメインエラー型の境界を検証する。
  テスト用に新規の IPC 契約やストレージスキーマの変更はない。テスト可能化のために
  コマンドハンドラから内部ロジック関数を抽出するが，公開 API は変更しない。

- **IV. Test-First Delivery and Quality Gates**: 本フィーチャー自体がテスト追加であり，
  TDD サイクルの「Red」フェーズに相当する。IPC テストは `cargo make test`（既存 QA に統合），
  E2E テストは `cargo make e2e`（独立タスク）で実行する。品質ゲート: `cargo make qa` は
  IPC テストを含む。

- **V. Safe Rust, SOLID Principles, and Maintainability First**: `unsafe` の使用なし。
  テストコード内では `unwrap()`, `expect()`, `assert!()` を許可（Constitution VII 例外）。
  内部関数抽出は SRP（単一責任: Tauri State 抽出 vs ビジネスロジック）に従い，
  DIP（依存性逆転: テストが AppState 抽象に依存）を維持する。

- **VI. Rust ドキュメント標準**: 新規の `pub` テストヘルパー関数には `///` ドキュメントコメントを
  付与する。`pub(crate)` の内部ロジック関数にも要約行を付ける。`cargo doc --no-deps` の
  クリーンビルドを維持する。

- **VII. 防御的エラーハンドリング**: テストコードは `#[cfg(test)]` 配下のため，
  `unwrap()`, `expect()`, `assert!()` の使用を許可。アプリケーションコード
  （内部ロジック関数）では禁止構文を使用しない。Clippy lint 設定（deny）は維持。

## Project Structure

### Documentation (this feature)

```text
specs/005-ipc-e2e-tests/
├── plan.md              # This file
├── research.md          # Phase 0: IPC テスト方式・E2E フレームワーク調査
├── data-model.md        # Phase 1: テスト対象エンティティマップ
├── quickstart.md        # Phase 1: テスト実行クイックスタート
├── contracts/           # Phase 1: テストヘルパー API 契約
│   └── test-helpers.md
└── tasks.md             # Phase 2: 実装タスク分解
```

### Source Code (repository root)

```text
src-tauri/src/ipc/
├── database_commands.rs     # 内部関数抽出（5 コマンド）
├── page_commands.rs         # 内部関数抽出（5 コマンド）
├── editor_commands.rs       # 内部関数抽出（8 コマンド）
├── property_commands.rs     # 内部関数抽出（9 コマンド）
├── table_commands.rs        # 内部関数抽出（5 コマンド）
├── view_commands.rs         # 内部関数抽出（6 コマンド）
└── tests/                   # IPC テスト（#[cfg(test)] モジュール）
    ├── mod.rs
    ├── helpers.rs           # テスト用 AppState 構築・DB セットアップ
    ├── database_commands_test.rs
    ├── page_commands_test.rs
    ├── editor_commands_test.rs
    ├── property_commands_test.rs
    ├── table_commands_test.rs
    └── view_commands_test.rs

e2e/                         # E2E テスト（WebDriverIO）
├── wdio.conf.ts             # 設定詳細は contracts/test-helpers.md を参照 <!-- refined by checklist-apply: P-14 -->
├── tsconfig.json
├── helpers/
│   └── app.ts               # アプリ起動・DB リセットヘルパー
└── specs/
    ├── page-workflow.spec.ts
    ├── editor-workflow.spec.ts
    ├── database-workflow.spec.ts
    └── view-workflow.spec.ts

Makefile.toml                # e2e タスク追加
```

**Structure Decision**: IPC テストは `src-tauri/src/ipc/tests/` に `#[cfg(test)]` モジュールとして
配置する。理由: 内部ロジック関数（`pub(crate)`）にアクセスするためクレート内テストが必要。
E2E テストはプロジェクトルートの `e2e/` に配置し，WebDriverIO（Node.js）で実行する。
各ドメインの境界コンテキスト（Database, Page, Editor, Property, Table, View）ごとに
テストファイルを分離し，テスト間の独立性を確保する。

### Domain-to-Test-File Mapping <!-- added by checklist-apply: P-01, P-04 -->

| spec.md ドメイン | テストファイル | 含まれるコマンド | 備考 |
|-----------------|-------------|----------------|------|
| Database | database_commands_test.rs | create/list/get/update/delete_database (5) | カスケード削除テストを含む |
| Page | page_commands_test.rs | create/list/get/update/delete_page (5) | |
| Editor | editor_commands_test.rs | open/close/add/edit/move_up/move_down/remove/save (8) | ステートフルフローテストを含む |
| Property | property_commands_test.rs | add/list/update_name/update_config/reorder/delete/reset_select_option/set_value/clear_value (9) | |
| Table | table_commands_test.rs | add_page_to_database/add_existing_page/list_standalone_pages/remove_page_from_database/get_table_data (5) | ドメイン横断操作（page↔database）を含む |
| View | view_commands_test.rs | get/reset/update_sort/update_filter/update_group/toggle_group_collapsed (6) | |

### E2E Workflow-to-Scenario Mapping <!-- added by checklist-apply: P-05 -->

| FR-005 ワークフロー | E2E テストファイル | US-3 シナリオ |
|-------------------|-----------------|-------------|
| ページ操作 | page-workflow.spec.ts | Scenario 1: ページ作成→タイトル入力→一覧表示確認 |
| データベース操作 | database-workflow.spec.ts | Scenario 2: レコード追加→プロパティ値設定→テーブルビュー表示確認 |
| ビュー操作 | view-workflow.spec.ts | Scenario 3: フィルタ条件設定→一致レコード表示確認 |
| エディタ操作 | editor-workflow.spec.ts | Scenario 4: ブロック追加・編集・移動・削除→保存→永続化確認 |

## Test Design <!-- added by checklist-apply: P-02, P-05, P-06, P-07, P-08 -->

### エラー種別の検証方針

各ドメインテストで検証するエラー variant は [data-model.md](./data-model.md) のエラー種別マッピング表に準拠する:

| ドメイン | 検証対象の主要 variant |
|---------|---------------------|
| Database | `titleEmpty`, `databaseNotFound` |
| Page | `titleEmpty`, `titleTooLong`, `notFound` |
| Editor（Block） | `contentTooLong`, `blockNotFound`, `cannotMoveUp`（+ セッション未開始） |
| Property | `propertyNameEmpty`, `duplicatePropertyName` |
| PropertyValue | `invalidNumber`, `typeMismatch` |
| View | `viewNotFound`, `invalidSortCondition` |

### テスト実行時間の見積もり

IPC テスト 38 コマンド × DB 作成/破棄（マイグレーション適用含む）で **30-60 秒** を想定する。
初回実装後に実測値を取得し，CC-003 の SLA 具体化の判断材料とする。

### 並列実行における DB 分離

`cargo-nextest` はデフォルトでテストを並列実行する。`TempDbGuard` が uuid_v7 ベースの
一時ディレクトリを使用するため，並列実行下でも DB ファイルの衝突は発生しない。

### テスト失敗時の診断情報

`assert!` / `assert_eq!` マクロにはカスタムメッセージを付与し，以下の情報を出力する:
- テスト対象コマンド名
- 入力値（引数）
- 期待値と実際の値

### E2E テスト設計 <!-- added by checklist-apply: P-01, P-02, P-03, P-06, P-07, P-08, P-11 -->

#### セレクタ戦略

E2E テストでは `data-testid` 属性を優先セレクタとし，`findByTestId()` ヘルパーで統一する
（[contracts/test-helpers.md](./contracts/test-helpers.md) 参照）。CSS セレクタや ARIA ロールは
`data-testid` が適用困難な場合の代替手段とする。

#### ライフサイクル管理

E2E テストスイートのライフサイクルは以下の順序で実行される（research.md R-006 参照）:

1. **起動**: `cargo build` → `tauri-driver` バックグラウンド起動 → `waitForApp()` で準備完了待機
2. **テスト実行**: WebDriverIO がシナリオを順次実行。各シナリオの `beforeEach` で `clearDatabase()` を呼び出し DB リセット
3. **停止**: `kill tauri-driver` → 一時 DB ファイル削除

タイムアウト: wdio.conf.ts の mocha timeout（30s）を上限とする。

#### リトライ・待機戦略

WebDriverIO のデフォルトリトライ機構（`waitForExist`, `waitForDisplayed`）を使用する。
WebView レンダリング完了は `waitForExist` / `waitForDisplayed` で判定し，mocha timeout（30s）を上限とする。
カスタムリトライ戦略は初期実装後に必要性を評価する。

#### シナリオ数

各 E2E ワークフローにつき US-3 の 1 シナリオを最低限実装する。
バリエーション追加は初期実装後に判断する。

#### 前提条件

E2E テスト実行の前提条件は [quickstart.md](./quickstart.md) を参照。
SC-004「`cargo make e2e` で独立実行可能」は，quickstart.md に記載された前提条件（tauri-driver インストール，E2E 依存インストール）が満たされている環境を想定する。

#### インメモリ状態の分離

E2E テストではアプリを再起動せず DB のみリセットする（research.md R-005）。
EditorSession 等のインメモリ状態は，各シナリオが独立した UI 操作フロー（open→操作→close）で
完結するため，シナリオ間の状態漏洩リスクは低い。問題が発生した場合はシナリオ間でのページリロードを検討する。

## Complexity Tracking

> 該当する憲法違反なし。すべての設計判断は Constitution に適合している。

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| (なし) | — | — |

### Known Risks <!-- added by checklist-apply: P-10 -->

| リスク | 影響 | 緩和策 |
|--------|------|--------|
| `AppState` の pub フィールド（`db`, `sessions`）に直接依存 | 構造変更時にテストヘルパーの修正が必要 | `setup_test_state()` に構築ロジックを集約し，変更箇所を 1 箇所に限定する |
| tauri-driver プロセスが E2E テスト中に異常終了する | テストスイート全体が停止し，残存プロセスがリソースを占有する | Makefile.toml の `e2e` タスクで exit code を検出し，`kill` で残存プロセスを回収する。テストスイート全体を失敗として報告する <!-- added by checklist-apply: P-09 --> |
