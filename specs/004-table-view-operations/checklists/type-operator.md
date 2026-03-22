# Type-Operator Compatibility Checklist: テーブルビュー操作拡張

**Purpose**: 5 つのプロパティ型（Text, Number, Date, Select, Checkbox）とソート・フィルタ・グルーピング演算子の組み合わせに関する仕様の網羅性・明確性・一貫性を検証する
**Created**: 2026-03-22
**Updated**: 2026-03-22 (final re-review after two rounds of checklist-apply)
**Feature**: [spec.md](../spec.md)

## ソート順序定義の完全性（FR-002）

- [x] CHK001 - テキスト型の「Unicode コードポイント順」は，空文字列（`""`）と null の区別を明示しているか？空文字列は null として扱うのか，null より前に配置されるのか？ [Clarity, Spec §FR-002] <!-- spec.md FR-002 に「テキスト型の空文字列は未設定値（null）と同一扱いとする」と明記（checklist-apply: G-11 で追加）。空文字列は null と同一として扱われ，昇順末尾・降順先頭に配置される。US4 シナリオ6 の「空文字列の行は『未設定』グループに含まれる」とも整合 -->
- [x] CHK002 - 数値型のソート順序は「数値の大小順」とされているが，NaN や Infinity が格納された場合のソート位置は定義されているか？（PropertyValue バリデーションで NaN/Infinity が排除される前提なら，その参照が明示されているか） [Clarity, Spec §FR-002] <!-- research.md RQ-6 で f64::total_cmp を使用と明記。total_cmp は NaN を最大値として扱い決定的な全順序を保証する。また PropertyValue の数値バリデーションは 003 スコープで既存実装済みと推定。plan.md でも既存エンティティは変更なしと記載 -->
- [x] CHK003 - 日付型で date モードと datetime モードの値が混在する場合のソート比較方法（Edge Cases に記載: date は 00:00:00 として比較）は FR-002 のソート順序定義に統合されているか，Edge Cases にのみ記載か？ [Consistency, Spec §FR-002 / Edge Cases] <!-- spec.md FR-002 に「date モードと datetime モードの値が混在する場合，date モードの値は当日の 00:00:00 として比較する」と直接統合された（checklist-apply R2: TO-CHK003 で追加）。Edge Cases にも同内容が記載されており冗長だが，FR-002 本文に権威的な定義が存在する。research.md RQ-6 でも「date モードは 00:00:00 UTC として比較」と整合 -->
- [x] CHK004 - セレクト型のソート順序は「選択肢の定義順（position 順）」とされているが，Property の position フィールドではなく SelectOption の position（配列インデックス）を指すことが明確か？ [Ambiguity, Spec §FR-002] <!-- spec.md FR-002 は「選択肢の定義順（position 順）」と記載。research.md RQ-6 で「選択肢の position 値順」と記載。data-model.md の FilterOperator は SelectOption の value を参照すると明記。003 スコープで SelectOption は PropertyConfig 内の配列として定義されており，配列インデックスが定義順に相当する。Property の position とは文脈が異なることは十分に推定可能 -->
- [x] CHK005 - チェックボックス型の昇順が「未チェック（false）→チェック済み（true）」と定義されているが，チェックボックスに null 値が存在しうるか（PropertyValue が未設定の場合）の前提は明示されているか？ [Gap, Spec §FR-002] <!-- spec.md FR-004 に「チェックボックスは常に true/false のいずれかであり，未設定値は存在しない」と明記（checklist-apply: TO-CHK005/006/014 で追加）。data-model.md のグルーピングキー粒度表でも「チェックボックス: true / false の 2 グループ, null は存在しない」と記載。前提が明示的に定義された -->
- [x] CHK006 - null の配置規則（昇順末尾・降順先頭）は 5 型すべてに一律適用される旨が FR-002 に記載されているが，チェックボックス型にも null が適用されるかは型の性質上（デフォルト false?）曖昧ではないか？ [Ambiguity, Spec §FR-002] <!-- spec.md FR-004 に「チェックボックスは常に true/false のいずれかであり，未設定値は存在しない」と明記。これにより FR-002 の null 配置規則はチェックボックスには適用されない（null が発生しないため）ことが明確。CHK005 と同根の問題が解消された -->

## フィルタ演算子の型別網羅性（FR-004）

- [x] CHK007 - テキスト型の「等しい」「等しくない」が case-insensitive である要件は Clarifications と Assumptions に記載されているが，FR-004 の演算子定義に統合されていない。要件の参照先が分散していないか？ [Consistency, Spec §FR-004 / Assumptions] <!-- spec.md Clarifications で「区別しない（case-insensitive）。『含む』『含まない』と一貫させる」と明記。Assumptions でも「すべて大文字小文字を区別しない（case-insensitive）」と記載。research.md RQ-7 でも「String (case-insensitive)」と記載。FR-004 本文には case-insensitive の注記がないが，spec.md 内の Clarifications と Assumptions で明確に定義されており，同一ドキュメント内での参照として十分。要件の存在と内容は明確 -->
- [x] CHK008 - テキスト型の「含む」「含まない」の部分一致は，Unicode の結合文字（例: が = か + ゙）に対する正規化要件が定義されているか？ Assumptions で「全角・半角の正規化は行わない」とあるが，Unicode 正規化（NFC/NFD）への言及はないか？ [Gap, Assumptions] <!-- spec.md Assumptions に「Unicode の NFC/NFD 正規化は行わない。Rust の str 型が保証する UTF-8 表現に委譲する」と明記（checklist-apply: TO-CHK008/024 で追加）。NFC/NFD 非対応の方針が YAGNI として明示的に定義された -->
- [x] CHK009 - 数値型フィルタで「等しい」の比較は浮動小数点の近似比較（epsilon）か厳密比較かが定義されているか？（例: 0.1 + 0.2 == 0.3 の扱い） [Clarity, Spec §FR-004] <!-- research.md RQ-8a に「f64 の == 演算子による厳密比較を採用」と明記（checklist-apply: P-03 で追加）。ユーザー入力値同士の比較では蓄積誤差が発生しないことが根拠として示されており，epsilon 比較は過剰な複雑性として不採用の判断が記録されている -->
- [x] CHK010 - 数値型フィルタの Edge Cases に「小数値・負数値が正しく比較される」と記載されているが，比較値自体の精度（桁数制限）や範囲制限は FR-004 または Assumptions で定義されているか？ [Gap, Edge Cases] <!-- spec.md Assumptions に「数値型フィルタの比較値の精度・範囲は IEEE 754 倍精度浮動小数点（f64）の型制約に委譲する。桁数制限や範囲制限は明示的に設けない」と明記（checklist-apply R2: TO-CHK010 で追加）。精度・範囲を f64 の型制約に委譲する方針が明示的に定義されており，意図的に制限を設けないという判断が記録されている -->
- [x] CHK011 - 日付型フィルタの「等しい」で datetime モードの「分単位の一致」（Assumptions）は，比較対象の両方が datetime の場合にのみ適用されるのか，date 値と datetime 値の混合比較にも適用されるのかが明確か？ [Clarity, Assumptions] <!-- spec.md Assumptions に「date モードでは日付の一致，datetime モードでは分単位の一致」と明記。Clarifications でも「分単位の一致（同じ年月日時分であれば一致とみなす。秒は無視）」と記載。date/datetime の混合比較は Edge Cases で「date は当日の 00:00:00 として比較」と定義されており，これにより date 値は datetime 値との比較時に 00:00:00 として扱われることが導出可能。十分な明確性がある -->
- [x] CHK012 - 日付型フィルタの「以前」「以降」は，指定日自体を含むか（inclusive / exclusive）が明示されているか？（「以降」は ≥ か > か） [Ambiguity, Spec §FR-004] <!-- spec.md FR-004 に「以前（指定日を含まない），以降（指定日を含む）」と明記（checklist-apply: G-03 で追加）。Before は exclusive（<），After は inclusive（>=）であることが FR-004 本文に統合されている -->
- [x] CHK013 - セレクト型フィルタの「である」「でない」は単一選択肢との比較（Assumptions）とされているが，比較対象は option の value なのか display name なのかが明確か？ [Clarity, Assumptions] <!-- data-model.md の FilterValue enum で SelectOption(String) に「option value」と明記。view-commands.md でも FilterValueDto の selectOption 型に「option value」と注記。Assumptions では「単一選択肢との比較」とだけ記載だが，技術契約で option value であることが明確に定義されている -->
- [x] CHK014 - チェックボックス型のフィルタが「チェック済み」「未チェック」の 2 演算子のみで，「値が空」「値が空でない」を持たないのは，チェックボックスには null 状態がない前提と整合するか？この前提は明示されているか？ [Consistency, Spec §FR-004] <!-- spec.md FR-004 に「チェックボックスは常に true/false のいずれかであり，未設定値は存在しない」と明記（checklist-apply: TO-CHK005/006/014 で追加）。IsEmpty/IsNotEmpty がチェックボックスに不要な理由が前提として明示され，演算子リストとの整合性が確保されている -->

## 演算子とプロパティ型の対応バリデーション

- [x] CHK015 - テキスト型の演算子（Equals, NotEquals, Contains, NotContains, IsEmpty, IsNotEmpty）と数値型で共通する Equals / NotEquals は，同じ演算子名だが異なるセマンティクス（case-insensitive vs 数値比較）を持つ。仕様上でこの区別は明確か？ [Ambiguity, Spec §FR-004] <!-- data-model.md の FilterOperator enum で Equals/NotEquals は共通の列挙子として定義。しかし research.md RQ-7 の型別実装表で「テキスト: String (case-insensitive)」「数値: f64」と比較値の型を明記しており，同じ演算子名でもプロパティ型に応じて異なる比較ロジックが適用されることが明確。view-commands.md の Validation でも「operator がプロパティ型と整合すること」と記載。型ごとのセマンティクスの違いは artifacts 全体で十分に定義されている -->
- [x] CHK016 - 不正な演算子・プロパティ型の組み合わせ（例: テキスト型に GreaterThan）が指定された場合のエラー要件は CC-004 に「バリデーションはバックエンドで実施」とあるが，エラーメッセージにどの型とどの演算子が不整合かの情報を含む要件はあるか？ [Clarity, Spec §CC-004] <!-- spec.md CC-004 に「エラーは種類を識別可能な形でフロントエンドに伝達する」と記載。view-commands.md で invalidFilterOperator エラー型を定義。plan.md Constitution Check VII で「ViewError 列挙型に十分なコンテキスト（view_id, property_id, operator 等）を保持する」と記載。エラー型（invalidFilterOperator）とコンテキストフィールド（property_id, operator）が仕様レベルで定義されており，フロントエンドがユーザー向けメッセージを構成するための情報は十分。ユーザー向けメッセージの文面は UI 実装の詳細であり仕様で規定する必要はない -->
- [x] CHK017 - フィルタ演算子 IsEmpty / IsNotEmpty は 4 型（Text, Number, Date, Select）に共通だが，各型で「空」の定義（null のみか，空文字列を含むか，0 を含むか）は型ごとに明示されているか？ [Gap, Spec §FR-004] <!-- spec.md FR-004 に「『値が空』『値が空でない』における『空』の定義: 未設定値（null）のみを対象とする。テキスト型の空文字列，数値型の 0，日付型のエポック値は『空』に含まない」と明記（checklist-apply: G-04 で追加）。型横断で「空」の定義が明確に統一されている -->
- [ ] CHK018 - フィルタ条件の比較値（FilterValue）の型チェック要件は CC-004 に言及されているが，具体的にどのような不整合（例: 数値フィルタに文字列，日付フィルタに不正フォーマット）がエラーとなるかのリストは仕様に含まれているか？ [Completeness, Spec §CC-004] [Partial] <!-- spec.md CC-004 に「比較値の型チェック等」と言及。view-commands.md の Validation に「value の型が operator と整合すること」と記載。view-commands.md で invalidFilterValue エラー型を定義。data-model.md の FilterValue enum と view-commands.md の FilterValueDto で型安全な比較値を定義（Text→String, Number→f64, Date→ISO 8601 String, SelectOption→option value）。IPC 境界で Tagged Union（{ type, value }）により型レベルで不正な組み合わせを防ぐ設計は明確。ただし DTO デシリアライズ時の境界ケース（不正な ISO 8601 文字列，空文字列の比較値，NaN/Infinity の数値等）に対する個別のバリデーション要件・エラー応答は明示されていない -->

## グルーピングの型別動作定義

- [x] CHK019 - グルーピング時のグループ表示順が Assumptions に型別に定義されている（セレクト: 定義順，チェックボックス: チェック済み→未チェック，その他: 値の昇順）が，FR-006 には記載がない。この要件は FR に昇格すべきか？ [Gap, Assumptions] <!-- spec.md FR-006 に「グループの表示順は以下の通りとする（MUST）」として型別の表示順が統合された（checklist-apply: M-02/G-16 で移動）。セレクト型: 選択肢の定義順，チェックボックス型: チェック済み→未チェックの順，テキスト・数値・日付型: 値の昇順，「未設定」グループは常に末尾。ユーザーに直接影響する表示順の要件が FR に正しく配置されている -->
- [x] CHK020 - テキスト型グルーピングは「完全一致」（Edge Cases）とされているが，case-insensitive でグルーピングされるのか（フィルタの Equals と同じ扱い），それとも case-sensitive か？フィルタとの一貫性は定義されているか？ [Consistency, Edge Cases / Assumptions] <!-- spec.md Assumptions に「テキスト型のグルーピングは case-insensitive（大文字小文字を区別しない）でグループ化する。フィルタの等値比較と一貫させる」と明記（checklist-apply: G-12 で追加）。data-model.md のグルーピングキー粒度表でも「テキスト: case-insensitive 完全一致, フィルタの Equals と同じ扱い」と記載。フィルタとの一貫性が明示されている -->
- [x] CHK021 - 数値型グルーピングでは各数値がそのままグループキーになるが，浮動小数点の表示精度（例: 1.0 と 1.00 は同一グループか）は定義されているか？ [Clarity, Gap] <!-- data-model.md のグルーピングキー粒度表に「数値: f64 の内部表現で同一判定, 1.0 と 1.00 は同一グループ」と明記（checklist-apply: P-04 で追加）。f64 の内部表現で同値判定することが明確に定義されている -->
- [x] CHK022 - 日付型グルーピングでは date モードと datetime モードでグループキーの粒度（日単位 / 分単位）が異なるべきだが，この区別は要件として定義されているか？ [Gap] <!-- data-model.md のグルーピングキー粒度表に「日付 (date): 日単位で同一判定, 年月日が一致すれば同一グループ」「日付 (datetime): 分単位で同一判定, 年月日時分が一致すれば同一グループ」と明記（checklist-apply: P-04 で追加）。date/datetime の粒度の違いが明確に定義されている -->
- [x] CHK023 - 「未設定」グループが「常に末尾」（Assumptions）である要件はすべての型に一律適用されるが，チェックボックス型で「未設定」が存在するかどうかの前提と整合するか？（CHK005, CHK014 と関連） [Consistency, Assumptions] <!-- spec.md FR-004 に「チェックボックスは常に true/false のいずれかであり，未設定値は存在しない」と明記。data-model.md のグルーピングキー粒度表でも「チェックボックス: true / false の 2 グループ, null は存在しない」と記載。FR-006 の「未設定グループは常に末尾」のルールはチェックボックスには適用されない（null が存在しないため）ことが前提の明示により整合的に導出可能 -->

## 型別エッジケースの仕様カバレッジ

- [x] CHK024 - テキスト型のソート・フィルタで，Unicode サロゲートペア（絵文字等）を含む文字列の扱いは Rust の `str::cmp` に委譲するとしても，仕様レベルでの期待動作が定義されているか？ [Gap, Edge Case] <!-- spec.md Assumptions に「Unicode の NFC/NFD 正規化は行わない。Rust の str 型が保証する UTF-8 表現に委譲する」と明記（checklist-apply: TO-CHK008/024 で追加）。Rust の str は有効な UTF-8 を保証するためサロゲートペアは発生しない。絵文字等の複合コードポイントの比較動作は Rust の str 実装に委譲する方針が YAGNI として明示されている -->
- [x] CHK025 - セレクト型プロパティの選択肢が 0 件の場合（全選択肢削除後），フィルタの「である」演算子の比較対象がなくなるが，この状態での動作要件は定義されているか？ [Gap, Edge Case] <!-- spec.md Edge Cases に「セレクト型プロパティの選択肢が全削除された場合（0 件），『である』『でない』フィルタは FR-009 により自動除去される。『値が空』『値が空でない』は引き続き動作すること」と明記（checklist-apply R2: TO-CHK025 で追加）。FR-009 の自動除去メカニズムにより，選択肢 0 件の状態では Is/IsNot フィルタが存在しないことが保証される。IsEmpty/IsNotEmpty の継続動作も明示されている -->
- [x] CHK026 - 日付型フィルタの比較値の入力形式（ISO 8601）は plan.md の contracts に記載があるが，タイムゾーン情報の有無（UTC 固定か，ローカルタイムか）は spec.md で明示されているか？ [Gap, Edge Case] <!-- spec.md Assumptions に「日付型の内部表現は UTC で統一する。フロントエンドでの表示はローカルタイムに変換するが，ソート・フィルタの比較は UTC ベースで行う」と明記（checklist-apply R2: TO-CHK026 で追加）。research.md RQ-6 でも「DateTime<Utc> の比較」と整合。UTC 固定の方針が仕様レベルで明確に定義されている -->
- [x] CHK027 - 数値型のソートで同値の行が存在する場合の安定ソート（元の順序維持）要件は定義されているか？FR-002 は値の順序のみ定義し，同値時の二次ソートについて言及がない [Gap, Spec §FR-002] <!-- spec.md FR-002 に「同値の行はデフォルト表示順（作成日時順）を維持する（安定ソート）」と明記（checklist-apply: G-10 で追加）。安定ソート要件が FR-002 本文に統合されている -->
- [x] CHK028 - セレクト型フィルタの「でない」演算子で，null 値（未設定）の行は「指定選択肢でない」に該当するか除外されるかの要件は明示されているか？ [Ambiguity, Spec §FR-004] <!-- spec.md FR-004 に「でない（未設定値の行は『でない』の結果に含む）」と明記（checklist-apply: G-05 で追加）。null（未設定）は「指定選択肢でない」と解釈され，結果に含まれることが FR-004 本文で明確に定義されている -->

## Notes

- Check items off as completed: `[x]`
- Add comments or findings inline
- Link to relevant resources or documentation
- Items are numbered sequentially for easy reference
