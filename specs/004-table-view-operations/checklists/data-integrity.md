# Data Integrity Checklist: テーブルビュー操作拡張

**Purpose**: ビュー永続化，プロパティ削除時の整合性修復，マイグレーション，トランザクション境界に関する仕様の品質を検証する
**Created**: 2026-03-22
**Feature**: [spec.md](../spec.md)

## ビュー永続化要件の完全性

- [x] CHK001 - FR-007 は「自動的に保存」と記述しているが，保存トリガーのタイミング（各操作直後 / debounce / 画面遷移時）は明確に定義されているか？ [Clarity, Spec §FR-007] <!-- Covered: contracts/view-commands.md の各コマンド（update_sort_conditions, update_filter_conditions 等）が個別の IPC コマンドとして定義されており，各操作が即座にバックエンドで永続化される設計。debounce や画面遷移時ではなく「各操作のコマンド呼び出し直後」が暗黙のタイミング。コマンド単位で ViewDto を返却する設計から，各操作直後の即時保存が明確 -->
- [ ] CHK002 - ビュー設定の復元時に，参照先プロパティが（削除ではなく）型変更されていた場合の要件は定義されているか？（例: テキスト型→数値型に変更後，テキスト用フィルタ演算子 "Contains" が残存） [Gap] <!-- Gap: FR-009 はプロパティ「削除」時のみを扱う。プロパティ型変更時の演算子不整合については spec.md, plan.md, contracts いずれにも記述なし。CC-004 のバリデーションは設定「時」のチェックであり，既存設定の型変更後の修復フローは未定義 -->
- [x] CHK003 - グループの折りたたみ状態の永続化が Assumptions セクションに記載されているが，FR 番号が割り当てられていない。FR-007 の「ソート・フィルタ・グルーピングの設定」に折りたたみ状態が含まれるか明示されているか？ [Ambiguity, Spec §FR-007] <!-- Covered: Assumptions に「グループの折りたたみ状態はビュー設定に含めて永続化する」と明記。Key Entities の View 記述でも「ソート条件，フィルタ条件，グルーピング条件，およびグループの折りたたみ状態を保持する」と明記。data-model.md でも collapsed_groups フィールドが定義済み。FR 番号は未割当だが仕様としては明確 -->
- [ ] CHK004 - US5 シナリオ 1〜3 は個別の設定種別の復元を検証しているが，3 種類すべてを同時に設定した状態での復元シナリオは定義されているか？ [Partial] <!-- Partial: US5 シナリオ 4 で「ソート・フィルタ・グルーピングがすべて設定された状態」のリセット操作はあるが，これはリセット検証であり「すべて同時設定の復元」シナリオではない。Independent Test の記述に「ソート・フィルタ・グルーピングをそれぞれ設定した後にアプリを再起動し，設定がすべて復元される」とあるが，Acceptance Scenario としては個別のみ -->
- [ ] CHK005 - 「デフォルト状態」の定義（FR-008: 作成日時順，フィルタなし，グルーピングなし）で，「作成日時順」が昇順か降順かは明示されているか？ [Gap] <!-- Gap: FR-008 は「ソートなし（作成日時順）」と記述。contracts/view-commands.md の reset_view では sort_conditions を空配列にリセットすると定義。「作成日時順」はソート条件なしのデフォルト表示順を意味するが，昇順（古い順）か降順（新しい順）かは spec.md のどこにも明示されていない -->

## プロパティ削除時の自動修復要件

- [x] CHK006 - FR-009 はソート・フィルタ・グルーピング条件の自動除去を要求しているが，グルーピング対象プロパティが削除された場合の collapsed_groups のクリア要件は明示されているか？ [Gap, Spec §FR-009] <!-- Covered: data-model.md の「プロパティ削除時の自動修復フロー」に「group_condition が property_id を参照する場合は None に設定」「collapsed_groups をクリア（グルーピング解除の場合）」と明記。spec.md の FR-009 自体には collapsed_groups の言及はないが，data-model.md で明確に定義されている -->
- [x] CHK007 - プロパティ削除と条件除去が同一トランザクション内で原子的に実行される要件は CC-001 に暗示されているが，明示的に記述されているか？ [Clarity, Spec §CC-001] <!-- Covered: data-model.md の自動修復フローの末尾に「この処理は Property 削除のトランザクション内で実行し，原子性を保証する」と明記。plan.md の Constitution Check でも「ビュー設定の保存はトランザクション内で原子的に実行し，部分書き込みを防止する」と記述 -->
- [x] CHK008 - 複数のソート条件が設定されている状態で第 1 ソートキーのプロパティが削除された場合，残りの条件の優先順位がどう再構成されるかは定義されているか？ [Gap, Spec §FR-009] <!-- Covered: data-model.md の自動修復フローで「sort_conditions から property_id を参照する条件を除去」と定義。data-model.md の SortCondition で「Vec 内の位置が優先順位を表す（index 0 が最優先）」と定義されているため，該当要素を除去すれば残りの条件は自然に Vec 内の位置で優先順位が繰り上がる。暗黙的だが Vec のセマンティクスから一意に導出可能 -->
- [ ] CHK009 - フィルタ条件 20 件中 15 件が同一プロパティを参照している場合，そのプロパティ削除で 15 件が一括除去される動作は FR-009 の「該当する条件が自動的に除去」でカバーされるが，大量除去後の UX（通知やフィードバック）は定義されているか？ [Gap] <!-- Gap: FR-009 は「該当する条件が自動的に除去され，残りの条件で表示が更新される」のみ。大量除去時のユーザー通知（toast 等）や確認ダイアログについては spec.md, plan.md, contracts のいずれにも記述なし -->
- [x] CHK010 - プロパティ削除による条件除去の結果，フィルタ条件が 0 件になった場合の動作（フィルタ解除扱いになるか）は明示されているか？ [Gap, Spec §FR-009] <!-- Covered: FR-009 で「残りの条件で表示が更新される」と定義。フィルタ条件が 0 件になればフィルタなしと同義であり，contracts/view-commands.md の reset_view で filter_conditions が空配列 = フィルタなしの定義と一貫。明示的な「0 件ならフィルタ解除」の記述はないが，空配列 = フィルタなしのデータモデル設計から自明 -->

## マイグレーション要件の完全性

- [ ] CHK011 - Key Entities セクションで「マイグレーション時に既存全データベースのデフォルトビューを一括生成」と記述されているが，マイグレーション失敗時のロールバック要件は定義されているか？ [Partial] <!-- Partial: constitution.md I に「永続化はトランザクション境界を明示できる方式で実装し，クラッシュ時の破損を防ぐこと」と定義。sqlx::migrate!() は各マイグレーションをトランザクション内で実行する標準動作。しかし spec.md や plan.md にマイグレーション失敗時の具体的なロールバック要件やユーザー通知は記載なし -->
- [x] CHK012 - マイグレーションで生成するデフォルトビューの UUID 生成方式（UUIDv7 のタイムスタンプ部の扱い）は仕様レベルで定義が必要か，それとも実装詳細として委譲可能か？ [Clarity] <!-- Covered: research.md RQ-4 で「ビュー ID は UUIDv7 だが，マイグレーション SQL では SQLite の lower(hex(randomblob(16))) 等で代替 UUID を生成する（UUIDv7 のタイムスタンプ部は必須ではない）」と判断済み。実装詳細として委譲する判断が明確に記録されている -->
- [ ] CHK013 - マイグレーション時に既存データベースが 0 件の場合（databases テーブルが空），マイグレーション SQL が正常に完了する要件は暗黙的に想定されているが明記されているか？ [Gap] <!-- Gap: data-model.md で「INSERT ... SELECT で既存の全 databases に対してデフォルトビューを一括生成」と記述。INSERT ... SELECT は該当行が 0 件なら何も挿入せず正常終了するため SQL 的には問題ないが，この動作を明示的に保証する記述は spec.md, plan.md のいずれにもない -->
- [ ] CHK014 - データベース新規作成時のデフォルトビュー自動生成（eager 方式）の要件は Key Entities に記載されているが，FR 番号が割り当てられていない。これは機能要件として独立すべきか？ [Partial] <!-- Partial: spec.md Key Entities に「データベース作成時にデフォルトのテーブルビューとして自動生成される（eager）」と明記。Clarifications にも同趣旨の記述あり。contracts/view-commands.md の get_table_data でも「ビューが存在しない場合はデフォルト設定で生成して返却」と記述。ただし FR 番号は未割当で，MUST/SHOULD のレベルが不明確 -->

## トランザクション境界と原子性

- [x] CHK015 - CC-001 は「原子的に行い，部分的な書き込みを防止する」と記述しているが，「ビュー設定の保存」が具体的にどの操作単位（ソート条件更新 1 回 / フィルタ条件追加 1 回 / リセット 1 回）を指すか明確か？ [Clarity, Spec §CC-001] <!-- Covered: contracts/view-commands.md で update_sort_conditions, update_filter_conditions, update_group_condition, toggle_group_collapsed, reset_view が個別のコマンドとして定義され，各コマンドが ViewDto を返却。各コマンドが 1 つの原子的操作単位であることが契約から明確。ソート条件は「一括更新」，フィルタ条件も「一括更新」と定義 -->
- [ ] CHK016 - ソート条件更新とフィルタ条件更新が短時間に連続して発生した場合の競合解決要件は定義されているか？（シングルユーザーアプリでも非同期 IPC による競合の可能性） [Gap] <!-- Gap: spec.md, plan.md, contracts のいずれにも非同期 IPC の競合解決（楽観的ロック，シリアライズ等）についての記述なし。SQLite の WAL モードと Tauri の単一プロセスモデルで実質的な競合リスクは低いが，仕様としては未定義 -->
- [ ] CHK017 - CC-001 の「既存のテーブルデータに対する変更は一切行わない」は明確だが，views テーブル自体の書き込み失敗時にユーザーへ通知する要件はどこに定義されているか？ [Partial] <!-- Partial: CC-004 で「エラーは種類を識別可能な形でフロントエンドに伝達する」と定義。contracts/view-commands.md で各コマンドの Errors にエラー種別が列挙されている。constitution.md VII で「すべての失敗可能な操作は Result を返す」と規定。ただし書き込み失敗時の具体的なユーザー向け通知方法（toast 等）は明記されていない -->

## バリデーションと参照整合性

- [x] CHK018 - フィルタ演算子とプロパティ型の対応表（FR-004）は網羅的だが，バリデーションエラー時のエラーメッセージ要件（どの演算子がどの型で不正か）は CC-004 で十分にカバーされているか？ [Clarity, Spec §CC-004] <!-- Covered: CC-004 で「フィルタ条件のバリデーション（プロパティ型と演算子の整合性，比較値の型チェック等）はバックエンドで実施する。エラーは種類を識別可能な形でフロントエンドに伝達する」と定義。contracts/view-commands.md で invalidFilterOperator, invalidFilterValue 等の具体的エラー種別が定義済み。constitution.md VII で「エラーバリアントは十分なコンテキストを保持しなければならない」と規定 -->
- [ ] CHK019 - ソート条件の上限 5 件（Assumptions）とフィルタ条件の上限 20 件が FR 番号を持たず Assumptions セクションにのみ記載されている。これらは MUST 要件として FR に昇格すべきか？ [Partial] <!-- Partial: spec.md Assumptions に上限値が記載。contracts/view-commands.md の Validation で「conditions.length <= 5」「conditions.length <= 20」と実装要件として定義。Errors に tooManySortConditions, tooManyFilterConditions が定義済み。data-model.md でも「最大 5 件」「最大 20 件」と記述。仕様的には十分だが FR の MUST レベルでは未定義 -->
- [x] CHK020 - 同一プロパティに対する複数フィルタ条件が許可されている（FR-005, Clarifications）が，矛盾する条件（例: 数値「100 より大きい AND 50 より小さい」）の結果が 0 件になる場合，FR-010 の空結果表示要件でカバーされるか？ [Consistency, Spec §FR-005 / §FR-010] <!-- Covered: FR-010 で「フィルタが適用されている状態で表示件数が0件の場合，ユーザーにフィルタが適用中であることと解除手段を提示しなければならない」と定義。矛盾するフィルタ条件の結果が 0 件になるケースは FR-010 の空結果表示要件で正確にカバーされる。US2 シナリオ 7 でも 0 件時のメッセージ表示と解除手段の提示が検証される -->
- [ ] CHK021 - ソート条件で同一プロパティの重複が禁止されることは plan.md の contracts に記載されているが，spec.md の FR/Assumptions に明記されているか？ [Misplaced] <!-- Misplaced: spec.md の FR-003 や Assumptions には同一プロパティ重複禁止の記述なし。contracts/view-commands.md の update_sort_conditions Validation に「同一 propertyId の重複不可」，Errors に「duplicateSortProperty」が定義。data-model.md にも「同一 property_id の重複不可」と記述。技術的制約が contracts/data-model に記載され spec.md の機能要件に反映されていない -->

## データベース・ビューのライフサイクル

- [ ] CHK022 - View エンティティの Key Entities 記述で「データベースの削除に連動して削除される」とあるが，CASCADE 削除の要件は FR レベルで定義されているか？ [Partial] <!-- Partial: spec.md Key Entities に「データベースの削除に連動して削除される」と明記。data-model.md に「Database 削除時に View も CASCADE 削除」「REFERENCES databases(id) ON DELETE CASCADE」と SQL レベルで定義済み。ただし FR 番号は未割当で MUST レベルの機能要件としては未定義 -->
- [ ] CHK023 - 1 データベース = 1 ビューの不変条件が Assumptions に記載されているが，アプリケーション層でこの不変条件を強制する要件（重複ビュー作成の防止）は定義されているか？ [Partial] <!-- Partial: spec.md Assumptions に「1データベース=1ビュー（本スコープ）」と記載。contracts/view-commands.md では get_view, update_* 系コマンドがすべて databaseId を引数に取り，viewId を使わない設計で暗黙的に 1:1 を強制。ただし UNIQUE 制約（database_id）や重複生成防止の明示的要件は data-model.md の SQL スキーマにも記載なし -->
- [x] CHK024 - View が存在しないデータベースに対して get_table_data が呼ばれた場合の動作（plan.md: デフォルト設定で生成して返却）は spec.md に記載されているか？ [Gap] <!-- Covered: contracts/view-commands.md の get_table_data 動作変更に「ビューが存在しない場合はデフォルト設定で生成して返却」と明記。spec.md の Key Entities にも「データベース作成時にデフォルトのテーブルビューとして自動生成される（eager）」と記述。Clarifications にも eager 生成が確認されている。contracts が spec を補完する形で動作が定義されている -->

## エッジケースの仕様カバレッジ

- [ ] CHK025 - Edge Cases に「グルーピング適用中にプロパティ値を変更した場合，ページが適切なグループに即座に移動する」とあるが，collapsed_groups に旧グループ値が残存する場合の要件は定義されているか？ [Gap] <!-- Gap: spec.md Edge Cases に「ページが適切なグループに即座に移動する」と記述。research.md RQ-8 に「プロパティ値の変更でグループが増減しても，collapsed 状態は値ベースで維持される」と記述。しかし旧グループが空になり消滅した場合に collapsed_groups から旧値を除去するかどうかの要件は未定義 -->
- [ ] CHK026 - セレクト型プロパティの選択肢が追加・削除された場合（セレクト型のフィルタ条件 "Is X" の X が削除された場合），フィルタ条件の自動修復要件は FR-009（プロパティ削除）とは別に定義されているか？ [Gap] <!-- Gap: FR-009 は「プロパティが削除された場合」のみを対象とする。セレクト型の選択肢削除（プロパティ自体は存続）でフィルタ条件が「存在しない選択肢」を参照する場合の修復要件は spec.md, plan.md, data-model.md のいずれにも記述なし -->
- [ ] CHK027 - 日付型フィルタの「等しい」で datetime モードの「分単位の一致」（Clarifications, Assumptions）は，秒の切り捨て方向（floor / round）が明示されているか？ [Partial] <!-- Partial: spec.md Assumptions に「分単位の一致（同じ年月日時分であれば一致とみなす。秒は無視）」と定義。「秒は無視」は事実上 floor（切り捨て）を意味する（同じ分であれば秒の値に関わらず一致）が，「floor」「round」という用語での明示はない。実装上は「年月日時分が同一か」の比較で一意に決まるため実質的に十分 -->
- [ ] CHK028 - ビュー設定の JSON デシリアライズに失敗した場合（データ破損，スキーマ不整合）のリカバリ要件は定義されているか？ [Gap] <!-- Gap: spec.md, plan.md, contracts, data-model.md のいずれにも JSON デシリアライズ失敗時のリカバリ要件（デフォルトへのフォールバック等）は記述なし。constitution.md I に「失敗時にユーザーへ明確なエラーを返し」とあるが，具体的なビュー設定破損時の動作は未定義 -->

## Notes

- Check items off as completed: `[x]`
- Add comments or findings inline
- Link to relevant resources or documentation
- Items are numbered sequentially for easy reference
