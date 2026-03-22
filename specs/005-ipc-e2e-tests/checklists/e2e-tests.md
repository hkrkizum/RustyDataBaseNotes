# E2E Test Requirements Quality Checklist: IPC テストおよび E2E テストの追加

**Purpose**: E2E テスト仕様（4 ワークフロー / tauri-driver + WebDriverIO）の要件品質を検証する
**Created**: 2026-03-22
**Feature**: [spec.md](../spec.md)
**Focus**: E2E テスト要件の網羅性・明確性・一貫性・シナリオカバレッジ
**Depth**: Standard
**Audience**: Reviewer (PR)

## Requirement Completeness

- [ ] CHK001 - US-3 の 4 ワークフロー（ページ操作・エディタ操作・データベース操作・ビュー操作）は、各ワークフローで操作対象となる UI 要素（ボタン・入力フィールド・リスト等）のセレクタ戦略（data-testid / CSS / ARIA）が要件として定義されているか？ [Partial] <!-- contracts/test-helpers.md の findByTestId() で data-testid 方式を暗示するが、spec/plan に正式な方針記載なし -->
- [ ] CHK002 - FR-006 は「実際の UI を通じて操作を検証」と要求しているが、WebView 内の要素をどのようなセレクタ（data-testid 属性、CSS セレクタ、ARIA ロール等）で特定するかの方針が要件またはplan に定義されているか？ [Partial] <!-- CHK001 と同様。test-helpers.md の findByTestId() が唯一の言及 -->
- [ ] CHK003 - US-3 シナリオ 1「ページを作成しタイトルを入力する」の操作手順（どの UI 要素をクリックし、どのフィールドに入力するか）が E2E テストの実装に十分な粒度で要件に含まれているか？ [Gap] <!-- spec.md は Gherkin レベルの記述のみ。UI 操作の具体的手順は未記載 -->
- [ ] CHK004 - US-3 シナリオ 4「ブロックを追加・編集・移動・削除して保存する」のブロック操作は、具体的にどのブロックタイプ（テキスト等）を対象とするか定義されているか？ [Gap] <!-- spec.md・plan.md ともにブロックタイプの指定なし -->
- [ ] CHK005 - E2E テストのアプリ起動・停止のライフサイクル管理要件（起動タイムアウト、準備完了の判定条件、停止時のクリーンアップ手順）が定義されているか？ [Partial] <!-- research.md R-006 にビルド→起動→テスト→停止の流れ、test-helpers.md に waitForApp()、wdio.conf.ts に timeout:30000 あり。ただし spec レベルの要件定義なし -->
- [ ] CHK006 - E2E テストで tauri-driver の起動失敗・接続エラー・WebView レンダリング遅延に対するリトライ・タイムアウト要件が定義されているか？ [Partial] <!-- wdio.conf.ts に timeout:30000 あり。リトライ戦略は未定義 -->
- [ ] CHK007 - FR-008「失敗時には原因特定に十分な情報を出力」は IPC・E2E 両方に適用されるが、E2E テスト固有の診断情報（スクリーンショット取得、DOM スナップショット、コンソールログ収集等）の要件が定義されているか？ [Gap] <!-- FR-008 は汎用的。E2E 固有の診断（スクリーンショット、DOM ダンプ等）は spec/plan 未記載 -->
- [ ] CHK008 - E2E テストの前提条件として、tauri-driver のインストール・バージョン要件が仕様レベルで定義されているか？ [Partial] <!-- quickstart.md に cargo install tauri-driver（バージョン指定なし）、research.md R-004 に「Tauri v2 対応」の記述あり -->

## Requirement Clarity

- [ ] CHK009 - US-3 シナリオ 2「テーブルビューに正しいデータが表示される」の「正しいデータ」は、検証すべきフィールド（カラム名・セル値・行数等）が具体的に定義されているか？ [Partial] <!-- data-model.md の TableDataDto 構造（database + properties + rows + view + groups）で暗黙的に定義。spec.md に明示的な検証フィールド指定なし -->
- [ ] CHK010 - US-3 シナリオ 3「条件に一致するレコードのみが表示される」のフィルタ条件は、具体的にどのプロパティタイプ（テキスト・数値・日付等）のどの演算子（等値・含む・範囲等）を検証するか定義されているか？ [Gap] <!-- spec.md にフィルタの具体的なプロパティタイプ・演算子の指定なし -->
- [ ] CHK011 - CC-001 の E2E データリセット方式「各シナリオ前にデータリセット（マイグレーション再適用またはテーブルクリア）」は、research.md では DELETE 方式を採用と記載されているが、spec レベルで方式が確定しているか？ [Partial] <!-- spec.md CC-001 は 2 択を列挙。research.md R-005 と test-helpers.md clearDatabase() は DELETE を採用。spec への反映が未完了 -->
- [ ] CHK012 - R-005 の「環境変数 `RDBN_DB_PATH` で一時 DB パスを指定」は、アプリ側にこの環境変数をサポートする変更が必要だが、この変更要件が FR として明示されているか？ [Gap] <!-- research.md R-005 に RDBN_DB_PATH のパターン記載あるが、spec.md の FR にも Dependencies にも未記載 -->
- [ ] CHK013 - Edge Cases「E2E テストがアプリケーションのクラッシュ後に適切にクリーンアップされること」の「適切にクリーンアップ」は、具体的に何を回収するか（tauri-driver プロセス、一時 DB、ロックファイル等）が定義されているか？ [Partial] <!-- research.md R-006 の Makefile パターンに kill $DRIVER_PID あり。spec.md ではクリーンアップ対象が列挙されていない -->

## Requirement Consistency

- [ ] CHK014 - FR-005 は「少なくとも主要ワークフロー」を要求し、US-3 は 4 シナリオを定義しているが、FR-005 のワークフロー列挙（ページ操作・エディタ操作・データベース操作・ビュー操作）と US-3 シナリオ（1-4）の対応関係は明示されているか？ [Partial] <!-- plan.md の E2E ファイル構成（page/editor/database/view-workflow）から暗黙的に推測可能だが、spec.md に明示的なマッピング表なし -->
- [x] CHK015 - plan.md の E2E テストファイル構成（page-workflow / editor-workflow / database-workflow / view-workflow）と US-3 の 4 シナリオの対応関係は一致しているか？ <!-- plan.md の Project Structure に 4 E2E ファイルが FR-005 の 4 ワークフローと命名規則で一致 -->
- [x] CHK016 - CC-005「E2E テストは独立タスク `cargo make e2e`」と FR-007「E2E テストは独立タスクとして提供し、マージ前または手動で実行する」は同一要件の重複記載か、それとも補完関係にあるか明確か？ <!-- 補完関係: FR-007 は IPC+E2E 両方の統合方針を定義、CC-005 は E2E の分離制約を定義。矛盾なし -->
- [ ] CHK017 - quickstart.md の「マージ前の完全検証: `cargo make qa && cargo make e2e`」は spec の FR-007・CC-005 の記述と整合しているが、pre-merge-commit フック（.githooks/）に E2E を含めるかどうかの判断が仕様に反映されているか？ [Gap] <!-- FR-007 は「マージ前または手動」と記載。CLAUDE.md の pre-merge-commit は qa のみ。E2E をフックに含めない判断が spec に明記されていない -->

## Acceptance Criteria Quality

- [ ] CHK018 - SC-003「主要ワークフローに対する E2E テストが存在し、全テストがパスする」は、各ワークフローの最低テストシナリオ数（例: ワークフローあたり 1 シナリオ or 複数バリエーション）が定義されているか？ [Partial] <!-- US-3 に 4 シナリオ（ワークフローあたり 1 つ）が定義されており暗黙的に最低 1。SC-003 に明示的な数値なし -->
- [ ] CHK019 - US-3 の各シナリオで「表示される」「反映されている」の検証方法（DOM 要素の存在確認、テキスト一致、要素数カウント等）は、E2E テスト実装に十分な精度で定義されているか？ [Gap] <!-- spec.md の受入シナリオは業務レベルの記述。DOM 検証手法は未定義 -->
- [ ] CHK020 - SC-004「E2E テストが `cargo make e2e` で独立実行可能」の「独立実行可能」は、前提条件（tauri-driver インストール済み、デバッグビルド済み等）を含む完全な実行条件が定義されているか？ [Partial] <!-- quickstart.md に前提条件（tauri-driver install, pnpm install）記載あり。SC-004 自体は前提条件を参照していない -->

## Scenario Coverage

- [ ] CHK021 - US-3 の 4 シナリオはすべて正常系だが、E2E レベルの異常系シナリオ（ネットワーク断、DB ロック、WebView クラッシュ等）の扱いが Out of Scope として明示されているか、あるいは要件として定義されているか？ [Gap] <!-- spec.md Out of Scope に E2E 異常系シナリオへの言及なし -->
- [ ] CHK022 - US-3 のワークフローは独立したシナリオとして定義されているが、ワークフロー横断シナリオ（例: データベース作成→ページ追加→ブロック編集→ビュー確認の一連操作）が E2E テストのスコープ内か外か定義されているか？ [Gap] <!-- ワークフロー横断シナリオのスコープ判断が spec.md に未記載 -->
- [ ] CHK023 - E2E テストの「テスト間でデータベース状態が分離」要件は CC-001 と Edge Cases で言及されているが、E2E 特有のアプリケーション状態（メモリ内の EditorSession、React コンポーネント状態等）のリセット要件は定義されているか？ [Gap] <!-- research.md R-005「テストごとのアプリ再起動は避ける」と明記。DB のみリセットし、インメモリ状態のリセット戦略は未定義 -->
- [ ] CHK024 - US-3 シナリオ 1-4 では新規作成フローのみが記述されているが、既存データの編集・削除フローが E2E テストのスコープとして定義されているか？ [Partial] <!-- US-3 Scenario 4 はブロックの編集・移動・削除を含む。ページ/データベースレベルの編集・削除フローは未定義 -->

## Edge Case Coverage

- [ ] CHK025 - E2E テスト実行中に tauri-driver プロセスが異常終了した場合のリカバリ要件（再起動、テストスキップ、エラー報告等）が定義されているか？ [Gap] <!-- research.md R-006 は正常終了時の kill のみ。異常終了時のリカバリ要件なし -->
- [ ] CHK026 - WSLg 無効環境での E2E テスト実行要件（xvfb-run による代替）は research.md に記載があるが、spec レベルの要件として定義されているか？ [Partial] <!-- research.md R-004 と quickstart.md に xvfb-run 記載あり。spec レベルでは未定義 -->
- [ ] CHK027 - E2E テストで WebView のレンダリング完了を待機する戦略（ポーリング間隔、最大待機時間、要素可視性判定）が要件として定義されているか？ [Partial] <!-- test-helpers.md に waitForApp() ヘルパー定義あり、wdio.conf.ts に timeout:30000。具体的な待機戦略は未定義 -->

## Non-Functional Requirements

- [ ] CHK028 - E2E テストの実行時間に関する目標値（個別シナリオ・スイート全体）が定義されているか？CC-003 の「E2E テストは個別シナリオごとに独立して実行可能」は実行時間要件を含むか？ [Gap] <!-- CC-003 は IPC テスト時間（数分以内）のみ。E2E テストの時間目標は未定義 -->
- [ ] CHK029 - E2E テスト環境の再現性要件（OS、WebKitGTK バージョン、tauri-driver バージョン、Node.js バージョン等）が定義されているか？ [Gap] <!-- spec/plan に E2E 環境の具体的バージョン要件なし -->
- [ ] CHK030 - FR-006 の「デバッグビルドの Tauri デスクトップアプリ全体」は、デバッグビルドとリリースビルドの動作差異がテスト結果に影響しないことの前提・制約が仕様に記述されているか？ [Gap] <!-- Clarifications で「デバッグビルドで実行」と決定済みだが、debug/release 差異の前提が未記述 -->

## Dependencies & Assumptions

- [ ] CHK031 - E2E テストは `RDBN_DB_PATH` 環境変数によるアプリの DB パス切り替えに依存するが、この環境変数のサポートがアプリケーション側の実装変更として tasks.md や FR に含まれているか？ [Gap] <!-- CHK012 と同根。research.md R-005 にパターンあるが FR/Dependencies セクションに未反映 -->
- [ ] CHK032 - E2E テストは tauri-driver が WebKitGTK の WebDriver 実装を利用する前提だが、開発環境（devcontainer / Nix）に WebKitGTK がインストール済みであることの前提は仕様に記述されているか？ [Partial] <!-- Constitution Technical Standards に flake.nix で WebKitGTK 管理と記載。本 spec の E2E 前提としては未明記 -->
- [ ] CHK033 - E2E テストの wdio.conf.ts 設定ファイルの主要設定項目（baseUrl、capabilities、タイムアウト値等）の要件が plan レベルで定義されているか？ [Partial] <!-- contracts/test-helpers.md に wdio.conf.ts の具体的な設定（hostname, port, capabilities, timeout）が記載されている。plan.md 本体からの参照はなし -->

## Notes

- Check items off as completed: `[x]`
- Add comments or findings inline
- Link to relevant resources or documentation
- Items are numbered sequentially for easy reference
