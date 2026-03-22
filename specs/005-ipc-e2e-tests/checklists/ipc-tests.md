# IPC Test Requirements Quality Checklist: IPC テストおよび E2E テストの追加

**Purpose**: IPC テスト仕様（38 コマンド / 6 ドメイン）の要件品質を検証する
**Created**: 2026-03-22
**Feature**: [spec.md](../spec.md)
**Focus**: IPC テスト要件の網羅性・明確性・一貫性
**Depth**: Standard
**Audience**: Reviewer (PR)

## Requirement Completeness

- [ ] CHK001 - FR-001 は「正常系テスト」を要求しているが、コマンド種別（CRUD）ごとの正常系の定義（作成→ID 返却、一覧→全件返却、取得→一致確認、更新→反映確認、削除→不在確認）が明示されているか？ [Partial] [Completeness, Spec §FR-001] <!-- FR-001 は「全コマンドの正常系テスト」を要求し US-1 シナリオ1 で「CRUD 操作が正しい結果を返す」とあるが、CRUD 種別ごとの期待動作定義がない -->
- [ ] CHK002 - 38 コマンドのうち、ドメイン横断操作（`add_page_to_database`, `add_existing_page_to_database`, `remove_page_from_database`）のテスト要件は、どのドメインテストに所属するか明示されているか？ [Partial] [Completeness, Spec §FR-001] <!-- plan.md のディレクトリ構造で table_commands_test.rs に暗黙的に配置されているが、spec.md では明示されていない -->
- [ ] CHK003 - FR-004 は「エラーレスポンスの種別（kind）とメッセージの妥当性」を要求しているが、各ドメインで検証すべきエラー種別の一覧（data-model.md のエラー種別マッピングとの対応）が仕様に組み込まれているか？ [Partial] [Completeness, Spec §FR-004] <!-- data-model.md にエラー種別マッピング表あり（7 種）だが、spec.md FR-004 からの相互参照がなく、検証対象が曖昧 -->
- [ ] CHK004 - User Story 2 の受入シナリオは 3 件だが、6 ドメインすべての異常系パターンをカバーする要件が存在するか？ [Gap] [Completeness, Gap] <!-- US-2 シナリオは Database（カスケード）, Editor（セッション未開始）, View（プロパティ削除）の 3 ドメインのみ。Page, Property, Table の異常系シナリオが未定義 -->
- [ ] CHK005 - DTO フィールドの正確な変換（data-model.md「IPC 境界で検証する項目」列）をテストする要件が FR として存在するか？ [Gap] [Completeness, Gap] <!-- CC-004 が「DTO の型安全性をテストで担保」と記載するが FR レベルの要件なし。data-model.md に検証項目列はあるが spec.md から参照されていない -->
- [ ] CHK006 - `EditorSession` のステートフルな操作フロー（open → add/edit/move/remove → save → close）をテストする要件は、個別コマンドテストとは別に定義されているか？ [Gap] [Completeness, Gap] <!-- IPC テストレベルでのステートフルフロー要件なし。US-3 シナリオ4 は E2E のみ。Editor の 8 コマンドを連続実行するシナリオ要件が欠落 -->

## Requirement Clarity

- [ ] CHK007 - FR-003 の「テスト後に削除」は、テストがパニックした場合のクリーンアップ保証まで含むか明示されているか？（research.md の TempDbGuard/Drop 方式は plan に記載があるが、spec レベルの要件として明確か） [Partial] [Clarity, Spec §FR-003] <!-- plan.md Constitution Check V で TempDbGuard/Drop に言及、data-model.md で RAII ガードの Drop 実装を定義済み。ただし spec.md FR-003 は「テスト後に削除」のみでパニック時の保証に言及なし -->
- [ ] CHK008 - CC-003 の「妥当な時間内（目安: 数分以内）」は、38 コマンド × 正常系 + 異常系の規模に対して具体的な数値目標に定量化されているか？ [Gap] [Clarity, Spec §CC-003] <!-- CC-003 は「数分以内」のみ。具体的な秒数 SLA なし -->
- [ ] CHK009 - SC-002 の「コマンドハンドラ層の不具合が本番到達前にテストで検出される」は、客観的に測定可能な基準として定義されているか？ [Gap] [Clarity, Spec §SC-002] <!-- SC-002 は定性的な目標であり、カバレッジ率やテスト数等の客観的指標がない -->
- [ ] CHK010 - FR-002 の「モックによるテストとしてはならない」の「モック」の範囲は明確か？（例: テスト用 DB はモックに該当しないことが明示されているか） [Partial] [Clarity, Spec §FR-002] <!-- Clarifications で「ハンドラ関数を直接呼び出し（SqlitePool を渡す統合テスト）」と明記。一時 SQLite ファイルが実 DB であることは文脈から推定可能だが「モック」の定義・境界は明示されていない -->
- [ ] CHK011 - User Story 1 シナリオ 3 の「不正な入力（空文字列，不正な型）」は、各ドメインごとに具体的なバリデーション条件が定義されているか？ [Partial] [Clarity, Spec §US-1] <!-- US-1 シナリオ3 は「空文字列，不正な型」の例示のみ。data-model.md エラー種別（titleEmpty, titleTooLong, propertyNameEmpty 等）にドメイン別条件があるが spec.md からの参照なし -->
- [ ] CHK012 - 「適切なエラー種別とメッセージが返される」（US-1 シナリオ 2）の「適切」が各コマンドごとに期待値として定義されているか？ [Partial] [Clarity, Spec §US-1] <!-- data-model.md のエラー種別マッピング表（PageError→notFound, DatabaseError→databaseNotFound 等）が期待値に相当するが、spec.md 内では「適切」の定義なし -->

## Requirement Consistency

- [ ] CHK013 - FR-001 のドメイン列挙「Database, Page, Property, Editor, Table, View」と contracts/test-helpers.md のコマンド分類（table_commands に page 操作が含まれる等）の間に不整合はないか？ [Partial] [Consistency, Spec §FR-001] <!-- table_commands に add_page_to_database 等の page 操作が含まれる。コード構造上は妥当だが、spec.md のドメイン分類との対応関係が明示されていない -->
- [x] CHK014 - spec.md のコマンド数「38」と contracts/test-helpers.md の内部関数一覧（38 関数）は一致しているか？今後コマンドが増減した場合の更新方針は定義されているか？ [Consistency, Spec §FR-001] <!-- コマンド数 38 は一致（database:5+page:5+editor:8+property:9+table:5+view:6=38）。ただし更新方針は未定義 -->
- [x] CHK015 - Edge Cases の「テスト間でデータベース状態が分離」と FR-003 の要件は重複しているが、Edge Case 側に FR-003 を超える追加要件（テスト順序非依存の検証方法等）が含まれるか明確か？ [Consistency, Spec §Edge Cases] <!-- FR-003 が「テスト間の DB 状態を完全に分離（MUST）。テスト実行順序に依存してはならない（MUST NOT）」と明記しており、Edge Case の記述と完全に一致。追加要件なし -->
- [ ] CHK016 - data-model.md のエラー種別マッピングと FR-004 の「エラーレスポンスの種別」要件の間で、検証対象のエラー variant が一致しているか？ [Partial] [Consistency, Spec §FR-004] <!-- data-model.md に 7 種のエラー型（PageError〜StorageError）と具体的な kind 値を定義済み。FR-004 は「種別（kind）とメッセージの妥当性」とのみ記載し、具体的な variant への相互参照がない -->

## Acceptance Criteria Quality

- [x] CHK017 - SC-001 の「少なくとも正常系テスト」は、1 コマンドあたりのテスト数の最低基準として十分に具体的か？（例: CRUD 操作ごとに 1 テスト、または入力パターンごとに 1 テスト） [Measurability, Spec §SC-001] <!-- SC-001「全 38 の IPC コマンドに対して少なくとも正常系テスト」は明確な最低基準（1 コマンド = 1 正常系テスト）。P2（US-2）で異常系を段階的に追加する方針も明示 -->
- [ ] CHK018 - User Story 1 の受入シナリオ 1 は「全ドメインの CRUD 操作が正しい結果を返す」とあるが、「正しい結果」の検証基準（フィールド完全一致、部分一致、型のみ等）は定義されているか？ [Gap] [Measurability, Spec §US-1] <!-- 「正しい結果」の検証基準が spec.md に未定義。data-model.md「IPC 境界で検証する項目」列にフィールド一覧はあるが、一致基準（完全一致 / 部分一致）の指定なし -->
- [ ] CHK019 - User Story 2 の受入シナリオ 3（ビューのプロパティ削除時の動作）で「適切に処理される」の具体的な期待動作（条件の自動削除、エラー返却、無視等）は定義されているか？ [Gap] [Measurability, Spec §US-2] <!-- 「適切に処理される」は曖昧。条件の自動削除、エラー返却、無視のいずれかが明示されていない -->

## Scenario Coverage

- [ ] CHK020 - 各ドメインの CRUD 操作（Create / Read / Update / Delete）すべてに対して、正常系テスト要件が個別に定義されているか？（例: Property は 9 コマンドだが add/list/update_name/update_config/reorder/delete/reset_select_option/set_value/clear_value の各操作に対する期待動作） [Partial] [Coverage, Spec §FR-001] <!-- FR-001「全コマンドハンドラに対して正常系テスト」+ contracts/test-helpers.md で全 38 関数のシグネチャを定義。ただし spec.md レベルでは個別コマンドの期待動作を列挙していない -->
- [ ] CHK021 - Editor ドメインのステートフルなセッション管理（open 前の操作、二重 open、close 後の操作）に対する異常系要件は定義されているか？ [Partial] [Coverage, Spec §US-2] <!-- US-2 シナリオ2「エディタセッションが開かれていない→セッション未開始エラー」で open 前のみカバー。二重 open、close 後の操作は未定義 -->
- [ ] CHK022 - カスケード削除（databases → pages → blocks, databases → properties → property_values, databases → views）の動作をテストする要件は、どのドメインのテスト要件として定義されているか？ [Partial] [Coverage, Spec §US-2] <!-- US-2 シナリオ1「データベースを削除→関連データの整合性が保たれる（カスケード削除またはエラー返却）」で言及あり。ただし所属ドメイン未指定、具体的なカスケードパス（3 系統）の個別テスト要件なし -->
- [ ] CHK023 - `table_commands` 内の操作（`add_page_to_database`, `add_existing_page_to_database` 等）で、存在しない database_id や page_id を指定した場合の異常系要件は定義されているか？ [Partial] [Coverage, Gap] <!-- US-1 シナリオ2「存在しないリソースの ID→取得・更新・削除コマンド」で汎用的にカバーされるが、「追加」操作（add_page_to_database 等）の invalid ID は明示的に含まれていない -->

## Edge Case Coverage

- [ ] CHK024 - Edge Cases に記載の「IPC コマンドの並行呼び出し時にデータ競合が発生しないこと」に対応する具体的なテスト要件（FR）が存在するか？ [Gap] [Edge Case, Gap] <!-- Edge Case として記載されるのみ。FR レベルの要件なし、テスト戦略（並行呼び出しの再現方法等）も未定義 -->
- [ ] CHK025 - Edge Cases に記載の「大量のレコード（数百件）」に対する IPC コマンドの動作要件が FR として定義されているか？ [Gap] [Edge Case, Gap] <!-- Edge Case として記載されるのみ。FR レベルの要件なし、「数百件」の具体的な件数やパフォーマンス基準も未定義 -->
- [ ] CHK026 - cargo-nextest はデフォルトでテストを並列実行するが、テスト間の DB 分離が並列実行下で保証されることに関する要件は明示されているか？ [Partial] [Edge Case, Spec §FR-003] <!-- FR-003 の DB 分離要件と plan.md の TempDbGuard（uuid_v7 ディレクトリ）で並列実行に暗黙的に対応。ただし spec.md で並列実行への明示的言及なし -->
- [ ] CHK027 - `reorder_properties` に空配列や重複 ID を渡した場合、`toggle_group_collapsed` に存在しないグループ名を渡した場合など、各コマンド固有の境界値要件は定義されているか？ [Gap] [Edge Case, Gap] <!-- コマンド固有の境界値テスト要件は spec.md / plan.md ともに未定義 -->

## Non-Functional Requirements

- [ ] CHK028 - IPC テストの実行時間に関する具体的な SLA（例: テストスイート全体で N 秒以内）は CC-003 の「数分以内」より具体的に定義されているか？ [Gap] [Non-Functional, Spec §CC-003] <!-- CC-003 は「数分以内」のみ。テスト規模（38+ テスト × DB 作成/破棄）に対する定量的 SLA なし -->
- [ ] CHK029 - テスト失敗時の診断情報要件（FR-008「原因特定に十分な情報」）は、具体的な出力内容（入力値、期待値、実際値、DB 状態等）として定義されているか？ [Partial] [Non-Functional, Spec §FR-008] <!-- FR-008「原因特定に十分な情報」は要件として存在するが、出力すべき具体的な情報項目（入力値、期待値、実際値等）が列挙されていない -->
- [ ] CHK030 - CC-004 の「DTO の型安全性をテストで担保する」は、具体的に何を検証するか（シリアライズ/デシリアライズの往復、フィールド網羅、型変換精度等）明確に定義されているか？ [Gap] [Non-Functional, Spec §CC-004] <!-- CC-004「DTO の型安全性をテストで担保」は抽象的。検証方法（往復テスト、フィールド網羅、型変換等）が未定義 -->

## Dependencies & Assumptions

- [ ] CHK031 - IPC テストが既存の `database::init_pool()` 関数に依存しているが、この関数がマイグレーション適用と FK 有効化を含むことは仕様内で前提条件として明示されているか？ [Partial] [Assumption, Gap] <!-- contracts/test-helpers.md の setup_test_state() に「database::init_pool() で SQLite プールを初期化（マイグレーション適用，FK 有効化）」と明記。ただし spec.md の前提条件セクションには記載なし -->
- [ ] CHK032 - `AppState` の公開フィールド（`pub db`, `pub sessions`）をテストで直接構築する前提は、将来の `AppState` 構造変更時の影響範囲として仕様で認識されているか？ [Gap] [Dependency, Gap] <!-- data-model.md に AppState 構造を記載するが、pub フィールドへの直接依存リスクや将来の構造変更への影響分析は spec/plan ともに未記載 -->

## Notes

- Check items off as completed: `[x]`
- Add comments or findings inline
- Link to relevant resources or documentation
- Items are numbered sequentially for easy reference
