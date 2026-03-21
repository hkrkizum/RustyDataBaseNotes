# Type-Operator Compatibility Checklist: テーブルビュー操作拡張

**Purpose**: 5 つのプロパティ型（Text, Number, Date, Select, Checkbox）とソート・フィルタ・グルーピング演算子の組み合わせに関する仕様の網羅性・明確性・一貫性を検証する
**Created**: 2026-03-22
**Feature**: [spec.md](../spec.md)

## ソート順序定義の完全性（FR-002）

- [ ] CHK001 - テキスト型の「Unicode コードポイント順」は，空文字列（`""`）と null の区別を明示しているか？空文字列は null として扱うのか，null より前に配置されるのか？ [Clarity, Spec §FR-002]
- [ ] CHK002 - 数値型のソート順序は「数値の大小順」とされているが，NaN や Infinity が格納された場合のソート位置は定義されているか？（PropertyValue バリデーションで NaN/Infinity が排除される前提なら，その参照が明示されているか） [Clarity, Spec §FR-002]
- [ ] CHK003 - 日付型で date モードと datetime モードの値が混在する場合のソート比較方法（Edge Cases に記載: date は 00:00:00 として比較）は FR-002 のソート順序定義に統合されているか，Edge Cases にのみ記載か？ [Consistency, Spec §FR-002 / Edge Cases]
- [ ] CHK004 - セレクト型のソート順序は「選択肢の定義順（position 順）」とされているが，Property の position フィールドではなく SelectOption の position（配列インデックス）を指すことが明確か？ [Ambiguity, Spec §FR-002]
- [ ] CHK005 - チェックボックス型の昇順が「未チェック（false）→チェック済み（true）」と定義されているが，チェックボックスに null 値が存在しうるか（PropertyValue が未設定の場合）の前提は明示されているか？ [Gap, Spec §FR-002]
- [ ] CHK006 - null の配置規則（昇順末尾・降順先頭）は 5 型すべてに一律適用される旨が FR-002 に記載されているが，チェックボックス型にも null が適用されるかは型の性質上（デフォルト false?）曖昧ではないか？ [Ambiguity, Spec §FR-002]

## フィルタ演算子の型別網羅性（FR-004）

- [ ] CHK007 - テキスト型の「等しい」「等しくない」が case-insensitive である要件は Clarifications と Assumptions に記載されているが，FR-004 の演算子定義に統合されていない。要件の参照先が分散していないか？ [Consistency, Spec §FR-004 / Assumptions]
- [ ] CHK008 - テキスト型の「含む」「含まない」の部分一致は，Unicode の結合文字（例: が = か + ゙）に対する正規化要件が定義されているか？ Assumptions で「全角・半角の正規化は行わない」とあるが，Unicode 正規化（NFC/NFD）への言及はないか？ [Gap, Assumptions]
- [ ] CHK009 - 数値型フィルタで「等しい」の比較は浮動小数点の近似比較（epsilon）か厳密比較かが定義されているか？（例: 0.1 + 0.2 == 0.3 の扱い） [Clarity, Spec §FR-004]
- [ ] CHK010 - 数値型フィルタの Edge Cases に「小数値・負数値が正しく比較される」と記載されているが，比較値自体の精度（桁数制限）や範囲制限は FR-004 または Assumptions で定義されているか？ [Gap, Edge Cases]
- [ ] CHK011 - 日付型フィルタの「等しい」で datetime モードの「分単位の一致」（Assumptions）は，比較対象の両方が datetime の場合にのみ適用されるのか，date 値と datetime 値の混合比較にも適用されるのかが明確か？ [Clarity, Assumptions]
- [ ] CHK012 - 日付型フィルタの「以前」「以降」は，指定日自体を含むか（inclusive / exclusive）が明示されているか？（「以降」は ≥ か > か） [Ambiguity, Spec §FR-004]
- [ ] CHK013 - セレクト型フィルタの「である」「でない」は単一選択肢との比較（Assumptions）とされているが，比較対象は option の value なのか display name なのかが明確か？ [Clarity, Assumptions]
- [ ] CHK014 - チェックボックス型のフィルタが「チェック済み」「未チェック」の 2 演算子のみで，「値が空」「値が空でない」を持たないのは，チェックボックスには null 状態がない前提と整合するか？この前提は明示されているか？ [Consistency, Spec §FR-004]

## 演算子とプロパティ型の対応バリデーション

- [ ] CHK015 - テキスト型の演算子（Equals, NotEquals, Contains, NotContains, IsEmpty, IsNotEmpty）と数値型で共通する Equals / NotEquals は，同じ演算子名だが異なるセマンティクス（case-insensitive vs 数値比較）を持つ。仕様上でこの区別は明確か？ [Ambiguity, Spec §FR-004]
- [ ] CHK016 - 不正な演算子・プロパティ型の組み合わせ（例: テキスト型に GreaterThan）が指定された場合のエラー要件は CC-004 に「バリデーションはバックエンドで実施」とあるが，エラーメッセージにどの型とどの演算子が不整合かの情報を含む要件はあるか？ [Clarity, Spec §CC-004]
- [ ] CHK017 - フィルタ演算子 IsEmpty / IsNotEmpty は 4 型（Text, Number, Date, Select）に共通だが，各型で「空」の定義（null のみか，空文字列を含むか，0 を含むか）は型ごとに明示されているか？ [Gap, Spec §FR-004]
- [ ] CHK018 - フィルタ条件の比較値（FilterValue）の型チェック要件は CC-004 に言及されているが，具体的にどのような不整合（例: 数値フィルタに文字列，日付フィルタに不正フォーマット）がエラーとなるかのリストは仕様に含まれているか？ [Completeness, Spec §CC-004]

## グルーピングの型別動作定義

- [ ] CHK019 - グルーピング時のグループ表示順が Assumptions に型別に定義されている（セレクト: 定義順，チェックボックス: チェック済み→未チェック，その他: 値の昇順）が，FR-006 には記載がない。この要件は FR に昇格すべきか？ [Gap, Assumptions]
- [ ] CHK020 - テキスト型グルーピングは「完全一致」（Edge Cases）とされているが，case-insensitive でグルーピングされるのか（フィルタの Equals と同じ扱い），それとも case-sensitive か？フィルタとの一貫性は定義されているか？ [Consistency, Edge Cases / Assumptions]
- [ ] CHK021 - 数値型グルーピングでは各数値がそのままグループキーになるが，浮動小数点の表示精度（例: 1.0 と 1.00 は同一グループか）は定義されているか？ [Clarity, Gap]
- [ ] CHK022 - 日付型グルーピングでは date モードと datetime モードでグループキーの粒度（日単位 / 分単位）が異なるべきだが，この区別は要件として定義されているか？ [Gap]
- [ ] CHK023 - 「未設定」グループが「常に末尾」（Assumptions）である要件はすべての型に一律適用されるが，チェックボックス型で「未設定」が存在するかどうかの前提と整合するか？（CHK005, CHK014 と関連） [Consistency, Assumptions]

## 型別エッジケースの仕様カバレッジ

- [ ] CHK024 - テキスト型のソート・フィルタで，Unicode サロゲートペア（絵文字等）を含む文字列の扱いは Rust の `str::cmp` に委譲するとしても，仕様レベルでの期待動作が定義されているか？ [Gap, Edge Case]
- [ ] CHK025 - セレクト型プロパティの選択肢が 0 件の場合（全選択肢削除後），フィルタの「である」演算子の比較対象がなくなるが，この状態での動作要件は定義されているか？ [Gap, Edge Case]
- [ ] CHK026 - 日付型フィルタの比較値の入力形式（ISO 8601）は plan.md の contracts に記載があるが，タイムゾーン情報の有無（UTC 固定か，ローカルタイムか）は spec.md で明示されているか？ [Gap, Edge Case]
- [ ] CHK027 - 数値型のソートで同値の行が存在する場合の安定ソート（元の順序維持）要件は定義されているか？FR-002 は値の順序のみ定義し，同値時の二次ソートについて言及がない [Gap, Spec §FR-002]
- [ ] CHK028 - セレクト型フィルタの「でない」演算子で，null 値（未設定）の行は「指定選択肢でない」に該当するか除外されるかの要件は明示されているか？ [Ambiguity, Spec §FR-004]

## Notes

- Check items off as completed: `[x]`
- Add comments or findings inline
- Link to relevant resources or documentation
- Items are numbered sequentially for easy reference
