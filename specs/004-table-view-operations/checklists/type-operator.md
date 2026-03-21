# Type-Operator Compatibility Checklist: テーブルビュー操作拡張

**Purpose**: 5 つのプロパティ型（Text, Number, Date, Select, Checkbox）とソート・フィルタ・グルーピング演算子の組み合わせに関する仕様の網羅性・明確性・一貫性を検証する
**Created**: 2026-03-22
**Feature**: [spec.md](../spec.md)

## ソート順序定義の完全性（FR-002）

- [ ] CHK001 - テキスト型の「Unicode コードポイント順」は，空文字列（`""`）と null の区別を明示しているか？空文字列は null として扱うのか，null より前に配置されるのか？ [Clarity, Spec §FR-002] [Gap] <!-- spec.md FR-002 は「未設定値（null）: 昇順時は末尾，降順時は先頭」とのみ記載。空文字列と null の区別は定義されていない。US4 シナリオ6 で「空文字列の行は『未設定』グループに含まれる」とあるがソート文脈での扱いは未定義 -->
- [x] CHK002 - 数値型のソート順序は「数値の大小順」とされているが，NaN や Infinity が格納された場合のソート位置は定義されているか？（PropertyValue バリデーションで NaN/Infinity が排除される前提なら，その参照が明示されているか） [Clarity, Spec §FR-002] <!-- research.md RQ-6 で f64::total_cmp を使用と明記。total_cmp は NaN を最大値として扱い決定的な全順序を保証する。また PropertyValue の数値バリデーションは 003 スコープで既存実装済みと推定。plan.md でも既存エンティティは変更なしと記載 -->
- [ ] CHK003 - 日付型で date モードと datetime モードの値が混在する場合のソート比較方法（Edge Cases に記載: date は 00:00:00 として比較）は FR-002 のソート順序定義に統合されているか，Edge Cases にのみ記載か？ [Consistency, Spec §FR-002 / Edge Cases] [Partial] <!-- spec.md Edge Cases に「date は当日の 00:00:00 として比較」と記載があり，research.md RQ-6 でも「date モードは 00:00:00 UTC として比較」と明記。しかし FR-002 本文は「日時の早い順」とだけ記載し，date/datetime 混在の比較方法を統合していない。要件が分散している -->
- [x] CHK004 - セレクト型のソート順序は「選択肢の定義順（position 順）」とされているが，Property の position フィールドではなく SelectOption の position（配列インデックス）を指すことが明確か？ [Ambiguity, Spec §FR-002] <!-- spec.md FR-002 は「選択肢の定義順（position 順）」と記載。research.md RQ-6 で「選択肢の position 値順」と記載。data-model.md の FilterOperator は SelectOption の value を参照すると明記。003 スコープで SelectOption は PropertyConfig 内の配列として定義されており，配列インデックスが定義順に相当する。Property の position とは文脈が異なることは十分に推定可能 -->
- [ ] CHK005 - チェックボックス型の昇順が「未チェック（false）→チェック済み（true）」と定義されているが，チェックボックスに null 値が存在しうるか（PropertyValue が未設定の場合）の前提は明示されているか？ [Gap, Spec §FR-002] [Gap] <!-- spec.md FR-002 は「未設定値（null）: 昇順時は末尾，降順時は先頭に配置」を全型に適用。FR-004 はチェックボックスに IsEmpty/IsNotEmpty を持たない（IsChecked/IsUnchecked のみ）。これはチェックボックスに null が存在しない暗黙の前提を示唆するが，明示的にチェックボックスは常に false/true のいずれかであると記載されていない。Assumptions にも言及なし -->
- [ ] CHK006 - null の配置規則（昇順末尾・降順先頭）は 5 型すべてに一律適用される旨が FR-002 に記載されているが，チェックボックス型にも null が適用されるかは型の性質上（デフォルト false?）曖昧ではないか？ [Ambiguity, Spec §FR-002] [Gap] <!-- CHK005 と同根の問題。FR-002 の null 規則は型を限定せず全型に適用と読める。しかしチェックボックスの null 存在可否が未定義のため，null 規則がチェックボックスに適用されるかが曖昧。research.md RQ-6 でもチェックボックスの null 位置は「昇順末尾，降順先頭」と記載されており矛盾はないが，チェックボックスに null が発生するかの前提が明示されていない -->

## フィルタ演算子の型別網羅性（FR-004）

- [ ] CHK007 - テキスト型の「等しい」「等しくない」が case-insensitive である要件は Clarifications と Assumptions に記載されているが，FR-004 の演算子定義に統合されていない。要件の参照先が分散していないか？ [Consistency, Spec §FR-004 / Assumptions] [Partial] <!-- spec.md Clarifications で「区別しない（case-insensitive）。『含む』『含まない』と一貫させる」と明記。Assumptions でも「すべて大文字小文字を区別しない（case-insensitive）」と記載。research.md RQ-7 でも「String (case-insensitive)」と記載。しかし FR-004 本文の演算子リストには case-insensitive の注記がない。要件自体は存在するが記載場所が分散 -->
- [ ] CHK008 - テキスト型の「含む」「含まない」の部分一致は，Unicode の結合文字（例: が = か + ゙）に対する正規化要件が定義されているか？ Assumptions で「全角・半角の正規化は行わない」とあるが，Unicode 正規化（NFC/NFD）への言及はないか？ [Gap, Assumptions] [Gap] <!-- spec.md Assumptions に「全角・半角の正規化は行わない」と記載があるのみ。NFC/NFD 等の Unicode 正規化形式への言及はどの成果物にも存在しない。research.md RQ-6 でもテキスト比較は str::cmp（バイト列比較）としか記載されていない -->
- [ ] CHK009 - 数値型フィルタで「等しい」の比較は浮動小数点の近似比較（epsilon）か厳密比較かが定義されているか？（例: 0.1 + 0.2 == 0.3 の扱い） [Clarity, Spec §FR-004] [Gap] <!-- spec.md FR-004 は「等しい」とだけ記載。research.md RQ-7 では比較値の型が f64 と記載されているが，等値比較の方法（epsilon vs 厳密）は未定義。research.md RQ-6 でソートに f64::total_cmp を使用と記載があるが，フィルタの等値比較については言及なし -->
- [ ] CHK010 - 数値型フィルタの Edge Cases に「小数値・負数値が正しく比較される」と記載されているが，比較値自体の精度（桁数制限）や範囲制限は FR-004 または Assumptions で定義されているか？ [Gap, Edge Cases] [Gap] <!-- spec.md Edge Cases に「小数値・負数値が正しく比較される」と期待動作のみ記載。精度・桁数・範囲の制限はどの成果物にも定義されていない。data-model.md では FilterValue::Number(f64) と型のみ記載。f64 の精度限界（約15-17有効桁）への言及なし -->
- [x] CHK011 - 日付型フィルタの「等しい」で datetime モードの「分単位の一致」（Assumptions）は，比較対象の両方が datetime の場合にのみ適用されるのか，date 値と datetime 値の混合比較にも適用されるのかが明確か？ [Clarity, Assumptions] <!-- spec.md Assumptions に「date モードでは日付の一致，datetime モードでは分単位の一致」と明記。Clarifications でも「分単位の一致（同じ年月日時分であれば一致とみなす。秒は無視）」と記載。date/datetime の混合比較は Edge Cases で「date は当日の 00:00:00 として比較」と定義されており，これにより date 値は datetime 値との比較時に 00:00:00 として扱われることが導出可能。十分な明確性がある -->
- [ ] CHK012 - 日付型フィルタの「以前」「以降」は，指定日自体を含むか（inclusive / exclusive）が明示されているか？（「以降」は ≥ か > か） [Ambiguity, Spec §FR-004] [Gap] <!-- spec.md FR-004 は「以前」「以降」と記載するのみで inclusive/exclusive の定義なし。US2 シナリオ5 も「指定日以降の日付を持つページのみが表示される」と記載するが ≥ か > かは不明。view-commands.md でも "before"/"after" と記載するのみ。data-model.md でも Before/After の定義に inclusive/exclusive の言及なし -->
- [x] CHK013 - セレクト型フィルタの「である」「でない」は単一選択肢との比較（Assumptions）とされているが，比較対象は option の value なのか display name なのかが明確か？ [Clarity, Assumptions] <!-- data-model.md の FilterValue enum で SelectOption(String) に「option value」と明記。view-commands.md でも FilterValueDto の selectOption 型に「option value」と注記。Assumptions では「単一選択肢との比較」とだけ記載だが，技術契約で option value であることが明確に定義されている -->
- [ ] CHK014 - チェックボックス型のフィルタが「チェック済み」「未チェック」の 2 演算子のみで，「値が空」「値が空でない」を持たないのは，チェックボックスには null 状態がない前提と整合するか？この前提は明示されているか？ [Consistency, Spec §FR-004] [Partial] <!-- spec.md FR-004 でチェックボックスは IsChecked/IsUnchecked のみと定義。data-model.md の対応表でも Checkbox は IsChecked/IsUnchecked のみ。これは暗黙的にチェックボックスに null がないことを示唆する。しかし「チェックボックスには常に値が存在する（デフォルト false）」という前提が明示されていない。CHK005/CHK006 と関連する同根の問題 -->

## 演算子とプロパティ型の対応バリデーション

- [x] CHK015 - テキスト型の演算子（Equals, NotEquals, Contains, NotContains, IsEmpty, IsNotEmpty）と数値型で共通する Equals / NotEquals は，同じ演算子名だが異なるセマンティクス（case-insensitive vs 数値比較）を持つ。仕様上でこの区別は明確か？ [Ambiguity, Spec §FR-004] <!-- data-model.md の FilterOperator enum で Equals/NotEquals は共通の列挙子として定義。しかし research.md RQ-7 の型別実装表で「テキスト: String (case-insensitive)」「数値: f64」と比較値の型を明記しており，同じ演算子名でもプロパティ型に応じて異なる比較ロジックが適用されることが明確。view-commands.md の Validation でも「operator がプロパティ型と整合すること」と記載。型ごとのセマンティクスの違いは artifacts 全体で十分に定義されている -->
- [ ] CHK016 - 不正な演算子・プロパティ型の組み合わせ（例: テキスト型に GreaterThan）が指定された場合のエラー要件は CC-004 に「バリデーションはバックエンドで実施」とあるが，エラーメッセージにどの型とどの演算子が不整合かの情報を含む要件はあるか？ [Clarity, Spec §CC-004] [Partial] <!-- spec.md CC-004 に「エラーは種類を識別可能な形でフロントエンドに伝達する」と記載。view-commands.md で invalidFilterOperator エラー型を定義。plan.md Constitution Check VII で「ViewError 列挙型に十分なコンテキスト（view_id, property_id, operator 等）を保持する」と記載。ただしエラーメッセージの具体的な内容（「テキスト型に GreaterThan は使用不可」等）の要件は未定義。コンテキスト保持の方針はあるが具体性が不足 -->
- [ ] CHK017 - フィルタ演算子 IsEmpty / IsNotEmpty は 4 型（Text, Number, Date, Select）に共通だが，各型で「空」の定義（null のみか，空文字列を含むか，0 を含むか）は型ごとに明示されているか？ [Gap, Spec §FR-004] [Gap] <!-- spec.md FR-004 は IsEmpty/IsNotEmpty を4型に定義するが「空」の意味を型別に定義していない。US2 シナリオ8 で「値が未設定のプロパティ」に対する IsEmpty と記載があるが，null 以外の空値（空文字列，0）の扱いは未定義。Assumptions にも言及なし。テキスト型で空文字列が IsEmpty に該当するかは US4 シナリオ6（空文字列は「未設定」グループ）から推定可能だが，フィルタ文脈での明示的定義がない -->
- [ ] CHK018 - フィルタ条件の比較値（FilterValue）の型チェック要件は CC-004 に言及されているが，具体的にどのような不整合（例: 数値フィルタに文字列，日付フィルタに不正フォーマット）がエラーとなるかのリストは仕様に含まれているか？ [Completeness, Spec §CC-004] [Partial] <!-- spec.md CC-004 に「比較値の型チェック等」と言及。view-commands.md の Validation に「value の型が operator と整合すること」と記載。view-commands.md で invalidFilterValue エラー型を定義。data-model.md で FilterValue enum により型安全な比較値を定義（Text→String, Number→f64, Date→ISO8601 String, SelectOption→String）。型の対応は構造的に定義されているが，具体的なエラーケースのリスト（不正な ISO 8601 形式等）は明示されていない -->

## グルーピングの型別動作定義

- [ ] CHK019 - グルーピング時のグループ表示順が Assumptions に型別に定義されている（セレクト: 定義順，チェックボックス: チェック済み→未チェック，その他: 値の昇順）が，FR-006 には記載がない。この要件は FR に昇格すべきか？ [Gap, Assumptions] [Misplaced] <!-- spec.md Assumptions にグループ表示順の型別定義が詳細に記載:「セレクト型: 選択肢の定義順」「チェックボックス型: チェック済み → 未チェックの順」「テキスト・数値・日付型: 値の昇順」「『未設定』グループは常に末尾」。しかし FR-006 はグルーピングの基本機能（ヘッダー表示，折りたたみ，未設定グループ）のみ定義し，グループ表示順への言及がない。ユーザーに直接影響する表示順の要件が Assumptions に置かれている（機能要件が Assumptions に misplaced） -->
- [ ] CHK020 - テキスト型グルーピングは「完全一致」（Edge Cases）とされているが，case-insensitive でグルーピングされるのか（フィルタの Equals と同じ扱い），それとも case-sensitive か？フィルタとの一貫性は定義されているか？ [Consistency, Edge Cases / Assumptions] [Gap] <!-- spec.md US4 シナリオ6 に「テキスト値の完全一致でグループ化される」と記載。しかし case-sensitive/insensitive の指定がない。フィルタの Equals は case-insensitive（Assumptions），ソートは Unicode コードポイント順（case-sensitive，FR-002）。グルーピングの文字列比較がどちらに従うかは未定義。Assumptions にもグルーピングの case sensitivity への言及なし -->
- [ ] CHK021 - 数値型グルーピングでは各数値がそのままグループキーになるが，浮動小数点の表示精度（例: 1.0 と 1.00 は同一グループか）は定義されているか？ [Clarity, Gap] [Gap] <!-- 数値型グルーピングの動作はどの成果物にも明示的に定義されていない。spec.md Assumptions のグループ表示順で「数値型: 値の昇順」と記載があるが，グループキーの同一性判定（浮動小数点の精度）については言及なし。1.0 と 1.00 が同一グループかどうかは f64 の内部表現では同値だが，表示精度の定義がない -->
- [ ] CHK022 - 日付型グルーピングでは date モードと datetime モードでグループキーの粒度（日単位 / 分単位）が異なるべきだが，この区別は要件として定義されているか？ [Gap] [Gap] <!-- 日付型グルーピングのグループキー粒度はどの成果物にも定義されていない。spec.md Assumptions では「日付型: 値の昇順」（表示順のみ）。フィルタの Equals では date=日付一致，datetime=分単位一致と定義されているが（Assumptions），グルーピングのキー粒度には言及なし。datetime モードで分単位でグループ化するのか日単位でグループ化するのかが不明 -->
- [ ] CHK023 - 「未設定」グループが「常に末尾」（Assumptions）である要件はすべての型に一律適用されるが，チェックボックス型で「未設定」が存在するかどうかの前提と整合するか？（CHK005, CHK014 と関連） [Consistency, Assumptions] [Gap] <!-- spec.md Assumptions に「『未設定』グループは常に末尾」と記載（型の限定なし）。しかし CHK005/CHK006/CHK014 で指摘の通り，チェックボックスの null 存在可否が未定義。US4 シナリオ3 では「チェック済み」「未チェック」の2グループのみ記載され，「未設定」グループへの言及なし。チェックボックスに null がないなら「未設定」グループ末尾ルールは無関係だが，その前提が明示されていない -->

## 型別エッジケースの仕様カバレッジ

- [ ] CHK024 - テキスト型のソート・フィルタで，Unicode サロゲートペア（絵文字等）を含む文字列の扱いは Rust の `str::cmp` に委譲するとしても，仕様レベルでの期待動作が定義されているか？ [Gap, Edge Case] [Gap] <!-- Unicode サロゲートペアや絵文字の扱いはどの成果物にも言及されていない。research.md RQ-6 で str::cmp 使用と記載があるが，これは実装判断であり仕様レベルの定義ではない。Rust の str は有効な UTF-8 を保証するためサロゲートペアは発生しないが，絵文字（複数コードポイントの結合）の比較動作は仕様として未定義 -->
- [ ] CHK025 - セレクト型プロパティの選択肢が 0 件の場合（全選択肢削除後），フィルタの「である」演算子の比較対象がなくなるが，この状態での動作要件は定義されているか？ [Gap, Edge Case] [Gap] <!-- 選択肢 0 件の状態でのフィルタ動作はどの成果物にも定義されていない。spec.md Edge Cases にも言及なし。view-commands.md の Validation でも選択肢の存在チェックは未記載 -->
- [ ] CHK026 - 日付型フィルタの比較値の入力形式（ISO 8601）は plan.md の contracts に記載があるが，タイムゾーン情報の有無（UTC 固定か，ローカルタイムか）は spec.md で明示されているか？ [Gap, Edge Case] [Partial] <!-- data-model.md で FilterValue::Date(String) に「ISO 8601 形式」と記載。view-commands.md でも FilterValueDto の date 型に「ISO 8601」と注記。research.md RQ-6 で「DateTime<Utc> の比較（date モードは 00:00:00 UTC として比較）」と UTC への言及あり。しかし spec.md 本文にはタイムゾーンの明示的な記載なし。research.md の「UTC」記載から推定可能だが，仕様レベルでの明確な定義ではない -->
- [ ] CHK027 - 数値型のソートで同値の行が存在する場合の安定ソート（元の順序維持）要件は定義されているか？FR-002 は値の順序のみ定義し，同値時の二次ソートについて言及がない [Gap, Spec §FR-002] [Gap] <!-- spec.md FR-002 はプロパティ型ごとの値順序と null 配置のみ定義。同値時の安定ソート（元の順序維持）要件は未定義。spec.md Edge Cases に「すべてのプロパティ値が未設定のカラムでソートした場合，表示順が変化しないこと（全行が同値のため，デフォルト順を維持）」と記載があり，暗黙的に安定ソートを期待しているが，FR-002 に明示的な安定ソート要件がない。research.md RQ-6 の Vec::sort_by は Rust では unstable sort だが sort_stable_by も選択可能 -->
- [ ] CHK028 - セレクト型フィルタの「でない」演算子で，null 値（未設定）の行は「指定選択肢でない」に該当するか除外されるかの要件は明示されているか？ [Ambiguity, Spec §FR-004] [Gap] <!-- spec.md FR-004 はセレクト型の「でない」演算子を定義するが，null 値の扱いは未記載。null（未設定）は「指定選択肢でない」と解釈すれば結果に含まれ，「値がある行のうち指定選択肢でないもの」と解釈すれば除外される。この区別はどの成果物にも定義されていない。Notion では「でない」は null を含む動作をする -->

## Notes

- Check items off as completed: `[x]`
- Add comments or findings inline
- Link to relevant resources or documentation
- Items are numbered sequentially for easy reference
