# Feature Specification: IPC テストおよび E2E テストの追加

**Feature Branch**: `005-ipc-e2e-tests`
**Created**: 2026-03-22
**Status**: Draft
**Input**: User description: "このプロジェクトにはipcのテスト，およびE2Eテストが存在しないので追加したい"

## Clarifications

### Session 2026-03-22

- Q: E2E テストが検証する対象範囲は？ → A: Tauri デスクトップアプリ全体（WebView + IPC + バックエンド）を対象とする
- Q: ブロックエディタ操作を E2E テストの主要ワークフローに含めるか？ → A: 含める — ブロックエディタ操作を 4 つ目の主要ワークフローとして追加する
- Q: E2E テストの QA パイプライン統合方式は？ → A: 独立タスク（`cargo make e2e`）として分離し，`qa` には含めず手動またはマージ前に実行する
- Q: E2E テストで Tauri アプリを駆動するフレームワークは？ → A: `tauri-driver` + WebDriverIO（Tauri 公式 WebDriver ベース）を採用する
- Q: IPC テストでコマンドハンドラを呼び出す方式は？ → A: ハンドラ関数を直接呼び出し（`SqlitePool` を渡す統合テスト）。Tauri ランタイム不要
- Q: IPC テストのテスト間 DB 分離粒度は？ → A: テストごとに一時 SQLite ファイルを作成・マイグレーション適用・テスト後に削除する
- Q: スコープ外として明示すべき項目は？ → A: 視覚回帰テスト，パフォーマンスベンチマーク，クロスプラットフォーム検証はすべてスコープ外
- Q: IPC コマンド数は？ → A: コードベース実数 38（database:5, page:5, editor:8, property:9, table:5, view:6）に修正
- Q: E2E テストのアプリビルドモードは？ → A: デバッグビルド（`cargo build`）で実行する
- Q: E2E テストのデータベース分離方式は？ → A: スイート開始時に一時 DB を作成し，各シナリオ前にデータリセット（マイグレーション再適用またはテーブルクリア）する

## Out of Scope

- 視覚回帰テスト（スクリーンショット比較等）
- パフォーマンスベンチマーク（レイテンシ・スループット計測）
- クロスプラットフォーム検証（Windows / macOS / Linux 間の差異テスト）
- IPC コマンドの並行呼び出しテスト（テスト間 DB 分離で間接的に安全性を確保する。Constitution V: YAGNI） <!-- added by checklist-apply: G-15 -->
- 大量レコード（数百件）に対する IPC パフォーマンステスト（パフォーマンスベンチマークと同様に初期スコープ外） <!-- added by checklist-apply: G-16 -->
- E2E レベルの異常系シナリオ（ネットワーク断，DB ロック，WebView クラッシュ等）。正常系ワークフローの検証を優先する <!-- added by checklist-apply: G-08 -->
- ワークフロー横断 E2E シナリオ（DB 作成→ページ追加→ブロック編集→ビュー確認の一連操作）。各ワークフロー独立のシナリオで基本動作を検証する <!-- added by checklist-apply: G-09 -->

## User Scenarios & Testing *(mandatory)*

### User Story 1 - IPC コマンドハンドラの正常系テスト (Priority: P1)

開発者として，すべての IPC コマンドハンドラが正しい入力に対して期待どおりの結果を返すことを
自動テストで検証したい。これにより，ドメイン層やリポジトリ層の変更が IPC 境界の動作を
壊していないことを迅速に確認できる。

**Why this priority**: IPC 層はフロントエンドとバックエンドの唯一の契約境界であり，
ここが壊れるとアプリケーション全体が動作しなくなる。現在 38 のコマンドが存在するが，
いずれも自動テストが存在しない。最も影響範囲が広く，リスクが高い。

**Independent Test**: IPC テストを実行し，全コマンドの正常系がパスすることを確認する。
テストはデータベースを使用する統合テストとして実行され，実際の永続化層を通じて動作を検証する。

**Acceptance Scenarios**:

1. **Given** テストが実行可能な環境がある, **When** IPC テストスイートを実行する, **Then** 全ドメイン（Database, Page, Property, Editor, Table, View）の CRUD 操作が正しい結果を返す。「正しい結果」とは返却 DTO の各フィールドが入力値および [data-model.md](./data-model.md) の「IPC 境界で検証する項目」と一致することを指す <!-- refined by checklist-apply: G-01, G-10 -->
2. **Given** エディタを open し，ブロックを追加・編集・移動・削除して save し close する, **When** 一連のフローを IPC テストとして実行する, **Then** 各操作が期待どおりの EditorStateDto を返し，save 後のデータが永続化される <!-- moved from US2-S9 by analyze: F2 -->

---

### User Story 2 - IPC コマンドハンドラの異常系・境界値テスト (Priority: P2)

開発者として，IPC コマンドハンドラがエラーケースや境界値を適切に処理することを
自動テストで検証したい。これにより，ユーザーが予期しない操作を行った場合でも
アプリケーションがクラッシュせず，回復可能なエラーを返すことを保証できる。

**Why this priority**: Constitution VII（防御的エラーハンドリング）の遵守を IPC 境界で
検証するために不可欠。正常系テストの次に取り組むことで，段階的にカバレッジを拡大できる。

**Independent Test**: 異常系テストを実行し，全コマンドがパニックせずにエラーを返すことを確認する。

**Acceptance Scenarios**:

1. **Given** データベースに関連ページが存在する, **When** データベースを削除する, **Then** カスケード削除により databases→pages→blocks，databases→properties→property_values，databases→views の 3 系統の関連データが整合的に削除される。カスケード削除テストは Database ドメインテストに含める <!-- refined by checklist-apply: G-13 -->
2. **Given** エディタセッションが開かれていない, **When** ブロック操作コマンドを呼び出す, **Then** セッション未開始エラーが返される
3. **Given** ビューにソート・フィルタ・グループ条件が設定されている, **When** 参照先のプロパティが削除される, **Then** 該当する条件はビューから自動除外され，ビュー操作はエラーを返さない <!-- refined by checklist-apply: G-11 -->
4. **Given** 存在しないページ ID を指定する, **When** ページの更新・削除コマンドを呼び出す, **Then** notFound エラーが返される <!-- added by checklist-apply: G-02 -->
5. **Given** データベースに同名のプロパティが存在する, **When** 同名でプロパティを追加する, **Then** duplicatePropertyName エラーが返される <!-- added by checklist-apply: G-02 -->
6. **Given** 存在しない database_id を指定する, **When** add_page_to_database を呼び出す, **Then** databaseNotFound エラーが返される <!-- added by checklist-apply: G-02 -->
7. **Given** エディタセッションが既に開かれている, **When** 同じページを再度 open する, **Then** 既存セッションが返されるか，適切なエラーが返される <!-- added by checklist-apply: G-12 -->
8. **Given** エディタセッションを close した後, **When** ブロック操作コマンドを呼び出す, **Then** セッション未開始エラーが返される <!-- added by checklist-apply: G-12 -->
9. **Given** 不正な入力（空文字列，不正な型）を渡す, **When** コマンドを呼び出す, **Then** [data-model.md](./data-model.md) のエラー種別マッピングに従ったバリデーションエラーが返され，データベースの状態は変化しない <!-- moved from US1-S3 by analyze: F1 -->

---

### User Story 3 - E2E テストによるユーザーワークフロー検証 (Priority: P3)

ユーザーとして，主要なワークフロー（ページ作成→編集→保存，ブロックエディタ操作，
データベース作成→レコード追加→ビュー操作）がアプリケーション全体を通じて正しく動作することを，
自動化された E2E テストで保証したい。

**Why this priority**: E2E テストは IPC テストの上位に位置し，フロントエンドからバックエンドまでの
統合動作を検証する。IPC テスト基盤が整った後に取り組むことで，効率的にテストピラミッドを構築できる。

**Independent Test**: E2E テストスイートを実行し，主要ワークフローがアプリケーション上で正しく
動作することを確認する。

**Acceptance Scenarios**:

1. **Given** アプリケーションが起動している, **When** ページ作成操作を行いタイトルを入力する, **Then** ページ一覧（サイドバー）に新しいページがタイトルテキストとして表示される <!-- refined by checklist-apply: G-01, G-07 -->
2. **Given** データベースが存在する, **When** レコードを追加しプロパティ値を設定する, **Then** テーブルビューに追加したレコードのプロパティ値が表示される（検証対象フィールドは [data-model.md](./data-model.md) の TableDataDto 構造を参照） <!-- refined by checklist-apply: G-13, G-07 -->
3. **Given** データベースにレコードが存在する, **When** テキストプロパティの等値フィルタ条件を設定する, **Then** 条件に一致するレコードのみが表示され，一致しないレコードは非表示となる <!-- refined by checklist-apply: G-04, G-07 -->
4. **Given** ページが開かれている, **When** テキストブロックを追加・編集・移動・削除して保存する, **Then** 変更が永続化され再表示時にも反映されている <!-- refined by checklist-apply: G-02, G-07 -->

### Edge Cases

- E2E テストがアプリケーションのクラッシュ後に適切にクリーンアップされること（クリーンアップ対象: tauri-driver プロセスの終了，一時 DB ファイルの削除） <!-- refined by checklist-apply: G-15 -->
- テスト間でデータベース状態が分離され，テスト順序に依存しないこと
- コマンド固有の境界値（`reorder_properties` に空配列，`toggle_group_collapsed` に存在しないグループ名等）は P2 スコープで段階的にテストを追加する <!-- refined by checklist-apply: G-17 -->
- 初期 E2E スコープはワークフローの主要フロー（作成→検証）を対象とする。ページ/データベースレベルの編集・削除フローは後続で追加可能 <!-- added by checklist-apply: G-16 -->

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: IPC テストスイートは，全 6 ドメイン（Database, Page, Property, Editor, Table, View）の全コマンドハンドラに対して正常系テストを提供しなければならない（MUST）。正常系テストは CRUD 操作種別ごとに以下の基本パターンを検証する: 作成→ID を含む DTO 返却，一覧→全件返却，取得→ID 一致確認，更新→変更フィールド反映確認，削除→対象リソースの不在確認 <!-- refined by checklist-apply: G-01 -->
- **FR-002**: IPC テストはハンドラ関数を直接呼び出し，テスト用 `SqlitePool` を渡す統合テストとして実装する（MUST）。Tauri ランタイムは不要とし，モックによるテストとしてはならない（MUST NOT）。本仕様における「モック」とは実データベースを使用しないスタブ・フェイク実装を指し，テスト用一時 SQLite ファイルはモックに該当しない <!-- refined by checklist-apply: G-07 -->
- **FR-003**: 各 IPC テストはテストごとに一時 SQLite ファイルを作成し，マイグレーションを適用した上でテスト後に削除（パニック時を含む）することで，テスト間のデータベース状態を完全に分離しなければならない（MUST）。テスト実行順序に依存してはならない（MUST NOT） <!-- refined by checklist-apply: P-03 -->
- **FR-004**: IPC テストはエラーレスポンスの種別（kind）とメッセージの妥当性を検証しなければならない（MUST）。検証対象のエラー種別は [data-model.md](./data-model.md) のエラー種別マッピング表に準拠する <!-- refined by checklist-apply: G-08, G-09 -->
- **FR-009**: IPC テストの正常系は，返却 DTO の各フィールドが [data-model.md](./data-model.md) の「IPC 境界で検証する項目」に定義された変換仕様と一致することを検証しなければならない（MUST） <!-- added by checklist-apply: G-03 -->
- **FR-005**: E2E テストは少なくとも主要ワークフロー（ページ操作，エディタ操作，データベース操作，ビュー操作）をカバーしなければならない（MUST）
- **FR-006**: E2E テストは `tauri-driver` + WebDriverIO を使用し，デバッグビルドの Tauri デスクトップアプリ全体（WebView + IPC + バックエンド）を対象として実際の UI を通じて操作を検証しなければならない（MUST）
- **FR-007**: IPC テストは既存の品質ゲート（`cargo make qa`）に統合されなければならない（MUST）。E2E テストは独立タスク（`cargo make e2e`）として提供し，マージ前または手動で実行する（MUST）。E2E テストは実行コストが高いため pre-merge-commit フック（`.githooks/`）には含めない
- **FR-008**: テスト実行結果は成功・失敗を明確に報告し，失敗時には原因特定に十分な情報を出力しなければならない（MUST）。E2E テスト失敗時は WebDriverIO の標準レポーター（spec reporter）出力を診断情報とする。スクリーンショット自動取得・DOM スナップショットは初期スコープ外とする <!-- refined by checklist-apply: G-03 -->
- **FR-010**: アプリケーションは環境変数 `RDBN_DB_PATH` が設定されている場合，そのパスを SQLite データベースファイルとして使用しなければならない（MUST）。E2E テストのデータベース分離に使用する <!-- added by checklist-apply: G-05 -->

### Key Entities

- **IPC テストケース**: 特定のコマンドハンドラに対する入力・期待出力・前提条件の組み合わせ。ドメインごとにグループ化される
- **E2E テストシナリオ**: ユーザーの操作手順と期待される画面状態の組み合わせ。ワークフロー単位でグループ化される
- **テストデータベース**: テスト実行ごとに作成・破棄される一時的なデータベース。本番データベースとは完全に分離される

## Constraints & Compliance *(mandatory)*

- **CC-001 Data Integrity**: IPC テスト用データベースは各テストごとに一時 SQLite ファイルとして作成・マイグレーション適用され，テスト後に削除される。E2E テストではスイート開始時に一時 DB を作成し，各シナリオ前に全テーブルの行を DELETE する方式でデータリセットする（マイグレーション再適用より高速。research.md R-005 参照）。いずれも本番データに影響を与えてはならない <!-- refined by checklist-apply: G-14 -->
- **CC-002 Privacy**: テストはローカル環境内で完結し，外部サービスへの通信を行わない。テストデータに個人情報を含まない
- **CC-003 Performance**: IPC テストスイート全体は妥当な時間内（目安: 数分以内）に完了すること。初回実装後に実測値を取得し，必要に応じて具体的な SLA を設定する。E2E テストは個別シナリオごとに独立して実行可能であること。E2E テストスイート全体の実行時間目標は初回実装後に実測値を取得して設定する（IPC テストと同方針） <!-- refined by checklist-apply: G-05, G-10 -->
- **CC-004 Boundary Types**: IPC テストは CommandError 型とフロントエンド側のエラーハンドリングの整合性を検証する。DTO の型安全性は正常系テストで返却 DTO の全フィールドを検証することで担保する（FR-009 参照） <!-- refined by checklist-apply: G-18 -->
- **CC-005 Testability**: IPC テストは既存の `cargo make test` で実行可能であること（FR-007 参照）。E2E テストのパイプライン分離方針は FR-007 に定義する <!-- refined by checklist-apply: G-06 -->

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 全 38 の IPC コマンドに対して少なくとも正常系テストが存在し，全テストがパスする
- **SC-002**: 全 38 コマンドに正常系テストが存在し，各ドメインの主要エラーパス（存在しないリソース，バリデーション違反，セッション未開始等）に対して異常系テストが存在する <!-- refined by checklist-apply: G-06 -->
- **SC-003**: 主要ワークフロー（ページ操作，エディタ操作，データベース操作，ビュー操作）に対する E2E テストが存在し，全テストがパスする
- **SC-004**: IPC テストが `cargo make qa` の品質ゲートに組み込まれ，E2E テストが `cargo make e2e` で独立実行可能である

## Dependencies & Assumptions <!-- added by checklist-apply: G-19, G-21, P-09 -->

- IPC テストは `database::init_pool()` 関数に依存する。この関数はマイグレーション適用（`sqlx::migrate!()`）と外部キー有効化（`PRAGMA foreign_keys = ON`）を含む
- IPC テストは `AppState` の公開フィールド（`pub db`, `pub sessions`）を直接構築してテストに使用する。`AppState` の構造変更時にはテストヘルパーの修正が必要となる
- テスト対象のコマンドハンドラから内部ロジック関数を抽出する前提であるが，公開 API（IPC コマンドのシグネチャ）は変更しない
- E2E テストは環境変数 `RDBN_DB_PATH` によるアプリの DB パス切り替えに依存する。この環境変数サポートはアプリケーション側の実装変更（FR-010）を必要とする <!-- added by checklist-apply: G-12 -->
- E2E テストはデバッグビルドで実行する。デバッグビルド固有の動作（Clippy lint 有効，最適化なし）が存在するが，機能の正確性検証には影響しないことを前提とする <!-- added by checklist-apply: G-11 -->
