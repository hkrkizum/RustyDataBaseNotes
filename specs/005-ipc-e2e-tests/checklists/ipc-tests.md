# IPC Test Requirements Quality Checklist: IPC テストおよび E2E テストの追加

**Purpose**: IPC テスト仕様（38 コマンド / 6 ドメイン）の要件品質を検証する
**Created**: 2026-03-22
**Feature**: [spec.md](../spec.md)
**Focus**: IPC テスト要件の網羅性・明確性・一貫性
**Depth**: Standard
**Audience**: Reviewer (PR)

## Requirement Completeness

- [x] CHK001 - FR-001 は「正常系テスト」を要求しているが、コマンド種別（CRUD）ごとの正常系の定義（作成→ID 返却、一覧→全件返却、取得→一致確認、更新→反映確認、削除→不在確認）が明示されているか？ <!-- FR-001 に CRUD 操作種別ごとの基本パターンが明示的に定義済み（checklist-apply G-01 で追記） -->
- [x] CHK002 - 38 コマンドのうち、ドメイン横断操作（`add_page_to_database`, `add_existing_page_to_database`, `remove_page_from_database`）のテスト要件は、どのドメインテストに所属するか明示されているか？ <!-- plan.md Domain-to-Test-File Mapping 表で Table ドメイン（table_commands_test.rs）に帰属を明記済み（checklist-apply P-01, P-04） -->
- [x] CHK003 - FR-004 は「エラーレスポンスの種別（kind）とメッセージの妥当性」を要求しているが、各ドメインで検証すべきエラー種別の一覧（data-model.md のエラー種別マッピングとの対応）が仕様に組み込まれているか？ <!-- FR-004 に「検証対象のエラー種別は data-model.md のエラー種別マッピング表に準拠する」と明記。plan.md Test Design にドメイン別 variant 一覧あり（checklist-apply G-08, G-09, P-02, P-05） -->
- [x] CHK004 - User Story 2 の受入シナリオは 3 件だが、6 ドメインすべての異常系パターンをカバーする要件が存在するか？ <!-- US-2 が 9 シナリオに拡充。Database（カスケード），Editor（セッション未開始・二重open・close後・フロー），Page（notFound），Property（duplicatePropertyName），Table（databaseNotFound），View（プロパティ削除）の全 6 ドメインをカバー（checklist-apply G-02, G-04, G-12） -->
- [x] CHK005 - DTO フィールドの正確な変換（data-model.md「IPC 境界で検証する項目」列）をテストする要件が FR として存在するか？ <!-- FR-009（新規追加）で「返却 DTO の各フィールドが data-model.md の変換仕様と一致することを検証する（MUST）」を定義済み（checklist-apply G-03） -->
- [x] CHK006 - `EditorSession` のステートフルな操作フロー（open → add/edit/move/remove → save → close）をテストする要件は、個別コマンドテストとは別に定義されているか？ <!-- US-2 シナリオ 9 で「open→操作→save→close の一連フロー」を IPC テスト要件として追加済み（checklist-apply G-04） -->

## Requirement Clarity

- [x] CHK007 - FR-003 の「テスト後に削除」は、テストがパニックした場合のクリーンアップ保証まで含むか明示されているか？（research.md の TempDbGuard/Drop 方式は plan に記載があるが、spec レベルの要件として明確か） <!-- FR-003 に「テスト後に削除（パニック時を含む）」と明記済み（checklist-apply P-03） -->
- [ ] CHK008 - CC-003 の「妥当な時間内（目安: 数分以内）」は、38 コマンド × 正常系 + 異常系の規模に対して具体的な数値目標に定量化されているか？ [Partial] <!-- CC-003 は「数分以内」＋「初回実装後に実測値を取得し SLA 設定」の YAGNI 整合的アプローチ。plan.md Test Design に 30-60 秒の見積もりあり。定量的 SLA は初回計測後に設定予定 -->
- [x] CHK009 - SC-002 の「コマンドハンドラ層の不具合が本番到達前にテストで検出される」は、客観的に測定可能な基準として定義されているか？ <!-- SC-002 を「全 38 コマンドに正常系テスト＋各ドメインの主要エラーパスに異常系テストが存在する」に改定済み（checklist-apply G-06） -->
- [x] CHK010 - FR-002 の「モックによるテストとしてはならない」の「モック」の範囲は明確か？（例: テスト用 DB はモックに該当しないことが明示されているか） <!-- FR-002 に「モック」の定義を明記:「スタブ・フェイク実装＝モック，テスト用一時 SQLite ファイル≠モック」（checklist-apply G-07） -->
- [x] CHK011 - User Story 1 シナリオ 3 の「不正な入力（空文字列，不正な型）」は、各ドメインごとに具体的なバリデーション条件が定義されているか？ <!-- US-1 シナリオ 3 に「data-model.md のエラー種別マッピングに従ったバリデーションエラー」への相互参照を追加済み（checklist-apply G-08） -->
- [x] CHK012 - 「適切なエラー種別とメッセージが返される」（US-1 シナリオ 2）の「適切」が各コマンドごとに期待値として定義されているか？ <!-- US-1 シナリオ 2 に「data-model.md のエラー種別マッピングに準拠したエラー種別（kind）とメッセージ」と明記済み（checklist-apply G-09） -->

## Requirement Consistency

- [x] CHK013 - FR-001 のドメイン列挙「Database, Page, Property, Editor, Table, View」と contracts/test-helpers.md のコマンド分類（table_commands に page 操作が含まれる等）の間に不整合はないか？ <!-- plan.md Domain-to-Test-File Mapping 表で 6 ドメインとテストファイルの対応関係を明示。Table ドメインにドメイン横断操作（page↔database）が含まれることを注記済み（checklist-apply P-01, P-04） -->
- [x] CHK014 - spec.md のコマンド数「38」と contracts/test-helpers.md の内部関数一覧（38 関数）は一致しているか？今後コマンドが増減した場合の更新方針は定義されているか？ <!-- コマンド数 38 は一致（database:5+page:5+editor:8+property:9+table:5+view:6=38）。ただし更新方針は未定義 -->
- [x] CHK015 - Edge Cases の「テスト間でデータベース状態が分離」と FR-003 の要件は重複しているが、Edge Case 側に FR-003 を超える追加要件（テスト順序非依存の検証方法等）が含まれるか明確か？ <!-- FR-003 が「テスト間の DB 状態を完全に分離（MUST）。テスト実行順序に依存してはならない（MUST NOT）」と明記しており、Edge Case の記述と完全に一致。追加要件なし -->
- [x] CHK016 - data-model.md のエラー種別マッピングと FR-004 の「エラーレスポンスの種別」要件の間で、検証対象のエラー variant が一致しているか？ <!-- FR-004 に「data-model.md のエラー種別マッピング表に準拠する」と明記。plan.md Test Design にドメイン別検証対象 variant 一覧を定義済み（checklist-apply P-02, P-05） -->

## Acceptance Criteria Quality

- [x] CHK017 - SC-001 の「少なくとも正常系テスト」は、1 コマンドあたりのテスト数の最低基準として十分に具体的か？（例: CRUD 操作ごとに 1 テスト、または入力パターンごとに 1 テスト） <!-- SC-001「全 38 の IPC コマンドに対して少なくとも正常系テスト」は明確な最低基準（1 コマンド = 1 正常系テスト）。P2（US-2）で異常系を段階的に追加する方針も明示 -->
- [x] CHK018 - User Story 1 の受入シナリオ 1 は「全ドメインの CRUD 操作が正しい結果を返す」とあるが、「正しい結果」の検証基準（フィールド完全一致、部分一致、型のみ等）は定義されているか？ <!-- US-1 シナリオ 1 に「正しい結果」を「返却 DTO の各フィールドが入力値および data-model.md の『IPC 境界で検証する項目』と一致すること」と定義済み（checklist-apply G-01, G-10） -->
- [x] CHK019 - User Story 2 の受入シナリオ 3（ビューのプロパティ削除時の動作）で「適切に処理される」の具体的な期待動作（条件の自動削除、エラー返却、無視等）は定義されているか？ <!-- US-2 シナリオ 3 を「該当する条件はビューから自動除外され，ビュー操作はエラーを返さない」に具体化済み（checklist-apply G-11） -->

## Scenario Coverage

- [x] CHK020 - 各ドメインの CRUD 操作（Create / Read / Update / Delete）すべてに対して、正常系テスト要件が個別に定義されているか？（例: Property は 9 コマンドだが add/list/update_name/update_config/reorder/delete/reset_select_option/set_value/clear_value の各操作に対する期待動作） <!-- FR-001 に CRUD 操作種別ごとの基本検証パターンを定義。contracts/test-helpers.md で全 38 関数のシグネチャを定義。FR-009 で DTO フィールド検証を要求（checklist-apply G-01, G-03） -->
- [x] CHK021 - Editor ドメインのステートフルなセッション管理（open 前の操作、二重 open、close 後の操作）に対する異常系要件は定義されているか？ <!-- US-2 シナリオ 2（open 前），シナリオ 7（二重 open），シナリオ 8（close 後）で 3 パターンすべてカバー済み（checklist-apply G-12） -->
- [x] CHK022 - カスケード削除（databases → pages → blocks, databases → properties → property_values, databases → views）の動作をテストする要件は、どのドメインのテスト要件として定義されているか？ <!-- US-2 シナリオ 1 に 3 系統のカスケードパスを明記し「Database ドメインテストに含める」と帰属を定義済み（checklist-apply G-13）。plan.md Domain-to-Test-File Mapping にも反映 -->
- [x] CHK023 - `table_commands` 内の操作（`add_page_to_database`, `add_existing_page_to_database` 等）で、存在しない database_id や page_id を指定した場合の異常系要件は定義されているか？ <!-- US-1 シナリオ 2 の対象を「追加コマンド」に拡大。US-2 シナリオ 6 で add_page_to_database の databaseNotFound エラーを明示（checklist-apply G-14, G-02） -->

## Edge Case Coverage

- [x] CHK024 - Edge Cases に記載の「IPC コマンドの並行呼び出し時にデータ競合が発生しないこと」に対応する具体的なテスト要件（FR）が存在するか？ <!-- Out of Scope に移動済み。Constitution V（YAGNI）に基づき，DB 分離で間接的に安全性を確保し，並行テストは初期スコープ外とした（checklist-apply G-15） -->
- [x] CHK025 - Edge Cases に記載の「大量のレコード（数百件）」に対する IPC コマンドの動作要件が FR として定義されているか？ <!-- Out of Scope に移動済み。パフォーマンスベンチマークと同様に初期スコープ外（checklist-apply G-16） -->
- [x] CHK026 - cargo-nextest はデフォルトでテストを並列実行するが、テスト間の DB 分離が並列実行下で保証されることに関する要件は明示されているか？ <!-- FR-003 の「テスト実行順序に依存してはならない（MUST NOT）」が並列実行を暗黙的にカバー。plan.md Test Design に「uuid_v7 ベースの一時ディレクトリにより並列実行下でも DB ファイルの衝突は発生しない」と明記（checklist-apply P-06） -->
- [x] CHK027 - `reorder_properties` に空配列や重複 ID を渡した場合、`toggle_group_collapsed` に存在しないグループ名を渡した場合など、各コマンド固有の境界値要件は定義されているか？ <!-- Edge Cases に「P2 スコープで段階的にテストを追加する」と方針を明記済み（checklist-apply G-17） -->

## Non-Functional Requirements

- [ ] CHK028 - IPC テストの実行時間に関する具体的な SLA（例: テストスイート全体で N 秒以内）は CC-003 の「数分以内」より具体的に定義されているか？ [Partial] <!-- CC-003 は「数分以内」＋「初回実装後に実測値を取得し SLA 設定」。plan.md に 30-60 秒の見積もりあり。YAGNI 原則に基づき計測後に具体化するアプローチ。CHK008 と同一 -->
- [ ] CHK029 - テスト失敗時の診断情報要件（FR-008「原因特定に十分な情報」）は、具体的な出力内容（入力値、期待値、実際値、DB 状態等）として定義されているか？ [Partial] <!-- FR-008 は「原因特定に十分な情報」のままだが、plan.md Test Design に「テスト対象コマンド名，入力値（引数），期待値と実際の値」を出力する方針を定義済み（checklist-apply P-08）。spec.md レベルでは抽象的 -->
- [x] CHK030 - CC-004 の「DTO の型安全性をテストで担保する」は、具体的に何を検証するか（シリアライズ/デシリアライズの往復、フィールド網羅、型変換精度等）明確に定義されているか？ <!-- CC-004 に「正常系テストで返却 DTO の全フィールドを検証することで担保する（FR-009 参照）」と具体的な検証方法を明記済み（checklist-apply G-18） -->

## Dependencies & Assumptions

- [x] CHK031 - IPC テストが既存の `database::init_pool()` 関数に依存しているが、この関数がマイグレーション適用と FK 有効化を含むことは仕様内で前提条件として明示されているか？ <!-- spec.md Dependencies & Assumptions セクション（新規追加）に「database::init_pool() 関数に依存…マイグレーション適用（sqlx::migrate!()）と外部キー有効化（PRAGMA foreign_keys = ON）を含む」と明記済み（checklist-apply G-19, G-21, P-09） -->
- [x] CHK032 - `AppState` の公開フィールド（`pub db`, `pub sessions`）をテストで直接構築する前提は、将来の `AppState` 構造変更時の影響範囲として仕様で認識されているか？ <!-- spec.md Dependencies & Assumptions に pub フィールド依存と構造変更リスクを記載。plan.md Known Risks に setup_test_state() への集約による緩和策を定義済み（checklist-apply G-19, P-10） -->

## Notes

- Check items off as completed: `[x]`
- Add comments or findings inline
- Link to relevant resources or documentation
- Items are numbered sequentially for easy reference
