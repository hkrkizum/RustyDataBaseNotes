# E2E Test Requirements Quality Checklist: IPC テストおよび E2E テストの追加

**Purpose**: E2E テスト仕様（4 ワークフロー / tauri-driver + WebDriverIO）の要件品質を検証する
**Created**: 2026-03-22
**Feature**: [spec.md](../spec.md)
**Focus**: E2E テスト要件の網羅性・明確性・一貫性・シナリオカバレッジ
**Depth**: Standard
**Audience**: Reviewer (PR)

## Requirement Completeness

- [x] CHK001 - US-3 の 4 ワークフロー（ページ操作・エディタ操作・データベース操作・ビュー操作）は、各ワークフローで操作対象となる UI 要素（ボタン・入力フィールド・リスト等）のセレクタ戦略（data-testid / CSS / ARIA）が要件として定義されているか？ <!-- plan.md E2E テスト設計 > セレクタ戦略で data-testid 優先方針を定義済み。contracts/test-helpers.md の findByTestId() と連携 -->
- [x] CHK002 - FR-006 は「実際の UI を通じて操作を検証」と要求しているが、WebView 内の要素をどのようなセレクタ（data-testid 属性、CSS セレクタ、ARIA ロール等）で特定するかの方針が要件またはplan に定義されているか？ <!-- plan.md E2E テスト設計 > セレクタ戦略で定義済み -->
- [ ] CHK003 - US-3 シナリオ 1「ページを作成しタイトルを入力する」の操作手順（どの UI 要素をクリックし、どのフィールドに入力するか）が E2E テストの実装に十分な粒度で要件に含まれているか？ [Partial] <!-- spec.md を業務シナリオレベルに改善済み（サイドバーにタイトルテキスト表示）。UI 要素の具体的指定は Constitution V (YAGNI) に基づき実装フェーズで確定する方針 -->
- [x] CHK004 - US-3 シナリオ 4「ブロックを追加・編集・移動・削除して保存する」のブロック操作は、具体的にどのブロックタイプ（テキスト等）を対象とするか定義されているか？ <!-- spec.md US-3 Scenario 4 に「テキストブロック」を明記済み -->
- [x] CHK005 - E2E テストのアプリ起動・停止のライフサイクル管理要件（起動タイムアウト、準備完了の判定条件、停止時のクリーンアップ手順）が定義されているか？ <!-- plan.md E2E テスト設計 > ライフサイクル管理で起動→テスト→停止の手順とタイムアウト (30s) を定義済み -->
- [x] CHK006 - E2E テストで tauri-driver の起動失敗・接続エラー・WebView レンダリング遅延に対するリトライ・タイムアウト要件が定義されているか？ <!-- plan.md E2E テスト設計 > リトライ・待機戦略で WebDriverIO デフォルト機構と mocha timeout (30s) を定義済み -->
- [x] CHK007 - FR-008「失敗時には原因特定に十分な情報を出力」は IPC・E2E 両方に適用されるが、E2E テスト固有の診断情報（スクリーンショット取得、DOM スナップショット、コンソールログ収集等）の要件が定義されているか？ <!-- spec.md FR-008 に E2E 補足追記済み: WebDriverIO 標準レポーター出力。スクリーンショット・DOM スナップショットは初期スコープ外と明記 -->
- [x] CHK008 - E2E テストの前提条件として、tauri-driver のインストール・バージョン要件が仕様レベルで定義されているか？ <!-- quickstart.md に Tauri v2 互換バージョン要件と Nix devshell 推奨を追記済み。plan.md Technical Context にも記載 -->

## Requirement Clarity

- [x] CHK009 - US-3 シナリオ 2「テーブルビューに正しいデータが表示される」の「正しいデータ」は、検証すべきフィールド（カラム名・セル値・行数等）が具体的に定義されているか？ <!-- spec.md US-3 Scenario 2 を「追加したレコードのプロパティ値が表示される」に具体化。data-model.md TableDataDto 参照追記 -->
- [x] CHK010 - US-3 シナリオ 3「条件に一致するレコードのみが表示される」のフィルタ条件は、具体的にどのプロパティタイプ（テキスト・数値・日付等）のどの演算子（等値・含む・範囲等）を検証するか定義されているか？ <!-- spec.md US-3 Scenario 3 に「テキストプロパティの等値フィルタ」を明記済み -->
- [x] CHK011 - CC-001 の E2E データリセット方式「各シナリオ前にデータリセット（マイグレーション再適用またはテーブルクリア）」は、research.md では DELETE 方式を採用と記載されているが、spec レベルで方式が確定しているか？ <!-- spec.md CC-001 を「全テーブルの行を DELETE する方式」に確定済み (research.md R-005 参照) -->
- [x] CHK012 - R-005 の「環境変数 `RDBN_DB_PATH` で一時 DB パスを指定」は、アプリ側にこの環境変数をサポートする変更が必要だが、この変更要件が FR として明示されているか？ <!-- spec.md FR-010 として新設済み。Dependencies & Assumptions にも追記 -->
- [x] CHK013 - Edge Cases「E2E テストがアプリケーションのクラッシュ後に適切にクリーンアップされること」の「適切にクリーンアップ」は、具体的に何を回収するか（tauri-driver プロセス、一時 DB、ロックファイル等）が定義されているか？ <!-- spec.md Edge Cases にクリーンアップ対象を具体化済み: tauri-driver プロセス終了、一時 DB ファイル削除 -->

## Requirement Consistency

- [x] CHK014 - FR-005 は「少なくとも主要ワークフロー」を要求し、US-3 は 4 シナリオを定義しているが、FR-005 のワークフロー列挙（ページ操作・エディタ操作・データベース操作・ビュー操作）と US-3 シナリオ（1-4）の対応関係は明示されているか？ <!-- plan.md E2E Workflow-to-Scenario Mapping テーブルで 4 ワークフロー ↔ 4 シナリオ ↔ テストファイルの 3 方向マッピングを明示済み -->
- [x] CHK015 - plan.md の E2E テストファイル構成（page-workflow / editor-workflow / database-workflow / view-workflow）と US-3 の 4 シナリオの対応関係は一致しているか？ <!-- plan.md の Project Structure に 4 E2E ファイルが FR-005 の 4 ワークフローと命名規則で一致 -->
- [x] CHK016 - CC-005「E2E テストは独立タスク `cargo make e2e`」と FR-007「E2E テストは独立タスクとして提供し、マージ前または手動で実行する」は同一要件の重複記載か、それとも補完関係にあるか明確か？ <!-- 補完関係: FR-007 は IPC+E2E 両方の統合方針を定義、CC-005 は E2E の分離制約を定義。矛盾なし -->
- [x] CHK017 - quickstart.md の「マージ前の完全検証: `cargo make qa && cargo make e2e`」は spec の FR-007・CC-005 の記述と整合しているが、pre-merge-commit フック（.githooks/）に E2E を含めるかどうかの判断が仕様に反映されているか？ <!-- spec.md CC-005 に「E2E テストは実行コストが高いため pre-merge-commit フックには含めない」と明記済み -->

## Acceptance Criteria Quality

- [x] CHK018 - SC-003「主要ワークフローに対する E2E テストが存在し、全テストがパスする」は、各ワークフローの最低テストシナリオ数（例: ワークフローあたり 1 シナリオ or 複数バリエーション）が定義されているか？ <!-- plan.md E2E テスト設計 > シナリオ数で「各ワークフローにつき US-3 の 1 シナリオを最低限実装」と定義済み -->
- [x] CHK019 - US-3 の各シナリオで「表示される」「反映されている」の検証方法（DOM 要素の存在確認、テキスト一致、要素数カウント等）は、E2E テスト実装に十分な精度で定義されているか？ <!-- spec.md US-3 各シナリオの Then 句を具体化済み: Scenario 1「タイトルテキストとして表示」(テキスト一致)、Scenario 2「プロパティ値が表示」(要素存在)、Scenario 3「一致しないレコードは非表示」(要素不在) -->
- [x] CHK020 - SC-004「E2E テストが `cargo make e2e` で独立実行可能」の「独立実行可能」は、前提条件（tauri-driver インストール済み、デバッグビルド済み等）を含む完全な実行条件が定義されているか？ <!-- plan.md E2E テスト設計 > 前提条件で quickstart.md への参照を明記済み -->

## Scenario Coverage

- [x] CHK021 - US-3 の 4 シナリオはすべて正常系だが、E2E レベルの異常系シナリオ（ネットワーク断、DB ロック、WebView クラッシュ等）の扱いが Out of Scope として明示されているか、あるいは要件として定義されているか？ <!-- spec.md Out of Scope に「E2E レベルの異常系シナリオは初期スコープ外」を追記済み -->
- [x] CHK022 - US-3 のワークフローは独立したシナリオとして定義されているが、ワークフロー横断シナリオ（例: データベース作成→ページ追加→ブロック編集→ビュー確認の一連操作）が E2E テストのスコープ内か外か定義されているか？ <!-- spec.md Out of Scope に「ワークフロー横断 E2E シナリオは初期スコープ外」を追記済み -->
- [x] CHK023 - E2E テストの「テスト間でデータベース状態が分離」要件は CC-001 と Edge Cases で言及されているが、E2E 特有のアプリケーション状態（メモリ内の EditorSession、React コンポーネント状態等）のリセット要件は定義されているか？ <!-- plan.md E2E テスト設計 > インメモリ状態の分離で DB のみリセットの方針と状態漏洩リスク評価を定義済み -->
- [x] CHK024 - US-3 シナリオ 1-4 では新規作成フローのみが記述されているが、既存データの編集・削除フローが E2E テストのスコープとして定義されているか？ <!-- spec.md Edge Cases に「初期 E2E スコープはワークフローの主要フロー（作成→検証）。編集・削除は後続追加」を明記済み -->

## Edge Case Coverage

- [x] CHK025 - E2E テスト実行中に tauri-driver プロセスが異常終了した場合のリカバリ要件（再起動、テストスキップ、エラー報告等）が定義されているか？ <!-- plan.md Known Risks に tauri-driver 異常終了リスクと exit code 検出による緩和策を追記済み -->
- [x] CHK026 - WSLg 無効環境での E2E テスト実行要件（xvfb-run による代替）は research.md に記載があるが、spec レベルの要件として定義されているか？ <!-- plan.md Technical Context > E2E Environment に WSLg 有効/無効時の実行方法を追記済み -->
- [x] CHK027 - E2E テストで WebView のレンダリング完了を待機する戦略（ポーリング間隔、最大待機時間、要素可視性判定）が要件として定義されているか？ <!-- plan.md E2E テスト設計 > リトライ・待機戦略で waitForExist/waitForDisplayed と mocha timeout (30s) を定義済み -->

## Non-Functional Requirements

- [x] CHK028 - E2E テストの実行時間に関する目標値（個別シナリオ・スイート全体）が定義されているか？CC-003 の「E2E テストは個別シナリオごとに独立して実行可能」は実行時間要件を含むか？ <!-- spec.md CC-003 に「E2E テスト実行時間目標は初回実装後に実測値で設定」方針を追記済み。YAGNI 整合 -->
- [x] CHK029 - E2E テスト環境の再現性要件（OS、WebKitGTK バージョン、tauri-driver バージョン、Node.js バージョン等）が定義されているか？ <!-- plan.md Technical Context > E2E Environment に Nix devshell による環境再現を定義済み -->
- [x] CHK030 - FR-006 の「デバッグビルドの Tauri デスクトップアプリ全体」は、デバッグビルドとリリースビルドの動作差異がテスト結果に影響しないことの前提・制約が仕様に記述されているか？ <!-- spec.md Dependencies & Assumptions にデバッグビルド前提と動作差異の許容を追記済み -->

## Dependencies & Assumptions

- [x] CHK031 - E2E テストは `RDBN_DB_PATH` 環境変数によるアプリの DB パス切り替えに依存するが、この環境変数のサポートがアプリケーション側の実装変更として tasks.md や FR に含まれているか？ <!-- spec.md FR-010 として新設 + Dependencies & Assumptions に依存関係を追記済み -->
- [x] CHK032 - E2E テストは tauri-driver が WebKitGTK の WebDriver 実装を利用する前提だが、開発環境（devcontainer / Nix）に WebKitGTK がインストール済みであることの前提は仕様に記述されているか？ <!-- plan.md Technical Context > E2E Environment に「WebKitGTK は flake.nix devShell で管理」を追記済み -->
- [x] CHK033 - E2E テストの wdio.conf.ts 設定ファイルの主要設定項目（baseUrl、capabilities、タイムアウト値等）の要件が plan レベルで定義されているか？ <!-- plan.md Project Structure に wdio.conf.ts → contracts/test-helpers.md 参照を追記。contracts/test-helpers.md に具体的設定あり -->

## Notes

- Check items off as completed: `[x]`
- Add comments or findings inline
- Link to relevant resources or documentation
- Items are numbered sequentially for easy reference
