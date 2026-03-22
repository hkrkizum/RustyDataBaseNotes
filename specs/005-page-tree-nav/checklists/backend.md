# Backend Requirements Quality Checklist: Page Tree Navigation

**Purpose**: バックエンド要件（データモデル・ドメインロジック・IPC コントラクト）の完全性・明確性・一貫性を実装前に検証する
**Created**: 2026-03-22
**Feature**: [spec.md](../spec.md) | [data-model.md](../data-model.md) | [contracts/ipc-commands.md](../contracts/ipc-commands.md)
**Focus**: バックエンド中心（データモデル・ドメインロジック・IPC コントラクト）
**Depth**: Standard | **Audience**: 実装前の自己レビュー（著者）

## Requirement Completeness

- [ ] CHK001 - `move_page` で現在の親と同じ `newParentId` を指定した場合の動作（no-op／エラー／通常更新）は定義されているか？ [Gap] <!-- spec/contracts ともに未記載。move_page の動作定義は newParentId が異なる場合のみ -->
- [ ] CHK002 - `move_page` および `delete_page` のトランザクション境界（validate → update が単一トランザクション内で実行される）は明示的に規定されているか？ [Partial, Spec §CC-001] <!-- CC-001 で「アトミック」と記載。delete は contracts で「トランザクション内で」と明記。move_page は明記なし -->
- [ ] CHK003 - `bulk_update_parent_id` が部分的に失敗した場合のロールバック動作は規定されているか？ [Partial] <!-- delete_page_with_promotion のトランザクション内で使用される前提だが、単体メソッドとしてのロールバック仕様は未記載 -->
- [x] CHK004 - `list_sidebar_items` が返すフラットリストからツリー構造を構築するアルゴリズム（ソート順・グルーピングルール）はフロントエンド側の責務として明確に境界が定義されているか？ [Completeness, Spec §FR-011] <!-- contracts「ソート順はフロントエンドに委任」、data-model.md の SidebarTreeNode「フロントエンドで付与」で明確 -->
- [ ] CHK005 - `create_child_page` の深度チェックにおいて、親の深度を誰が（どのレイヤーが）計算するかのオーケストレーションは規定されているか？ [Partial] <!-- contracts で手順1-5を記載、data-model に depth() メソッドあり。ただし IPC→ドメインサービスの呼び出し関係は暗黙的 -->
- [ ] CHK006 - `delete_page` で削除されるページの blocks が CASCADE 削除される旨は spec レベル（FR-008）で規定されているか、それとも contracts のみか？ [Gap, Spec §FR-008] <!-- spec FR-008 は子ページの昇格のみ。削除ページ自身の blocks の扱いは contracts のみに記載 -->

## Requirement Clarity

- [ ] CHK007 - 「最大5階層」の深度カウント基準（ルート = 深度1）は spec で明示されているか？data-model.md では `root = 1` と記載があるが、spec §FR-004 では「最大5階層」とのみ記載 [Partial, Spec §FR-004] <!-- data-model.md で「root = 1」「深度 ≤ 5」と明確。ただし spec 本文には基準の記載なし -->
- [ ] CHK008 - ルートレベルの表示順は「created_at 順」（FR-011）と「作成日時降順」（Assumptions）で方向が一致しているか？FR-011 の「created_at 順」に昇順/降順の指定はあるか？ [Partial, Spec §FR-011] <!-- FR-011「created_at 順」は方向未指定、Assumptions「作成日時降順」で補完されるが FR 本文の明確性が不足 -->
- [x] CHK009 - `sort_order` が全ページで `DEFAULT 0` のまま使用される場合、同一親内の子ページ表示順の決定ルール（created_at フォールバック）は明示されているか？ [Clarity, Spec §Key Entities] <!-- Key Entities「本スコープでは表示順に created_at DESC を使用する」、Assumptions にも同様の記載あり -->
- [x] CHK010 - ON DELETE SET NULL（DB レベル）とアプリケーション層の子昇格ロジックの二重防御の優先関係・相互作用は明確に文書化されているか？ [Clarity, data-model.md §ON DELETE SET NULL の理由] <!-- data-model.md に専用セクションで詳細に説明済み: アプリ層が先に昇格→DB レベルはフェイルセーフ -->
- [ ] CHK011 - `PageDto` の `sortOrder` フィールドの IPC 境界での型（Rust `i64` → TypeScript `number`）は明示されているか？JavaScript の `Number.MAX_SAFE_INTEGER` 超過の懸念はないか？ [Partial, contracts §PageDto] <!-- Rust 側 i64 は定義済み。TS 側の型は未明示。本スコープでは値が 0 固定のため実害なし -->

## Requirement Consistency

- [x] CHK012 - `delete_page` は contracts で「置き換え → `delete_page_with_promotion`」と記載されているが、同時に「コマンド名は `delete_page` のまま」とも記載。これは「既存コマンドの内部ロジック変更」として一貫しているか？Command Summary テーブルの "Modified" 分類と矛盾はないか？ [Consistency, contracts §delete_page] <!-- セクション見出しは紛らわしいが、本文「コマンド名は delete_page のまま維持」+ Summary「Modified」で一貫。見出しの表現のみ改善余地あり -->
- [ ] CHK013 - `find_root_pages`（parent_id IS NULL, database_id IS NULL）と既存の `find_standalone_pages`（database_id IS NULL）の役割・使い分けは明確か？重複・混乱の余地はないか？ [Partial, data-model.md §Repository Changes] <!-- SQL 条件は異なるが、各メソッドのユースケース（いつどちらを使うか）は未記載 -->
- [x] CHK014 - 不変条件2「parent_id が指すページは database_id IS NULL」は、DB 所属ページを親にできないことを意味するが、`validate_move` のエラー条件は移動元のチェックのみか、移動先（新しい親）もチェックするか明確に規定されているか？ [Consistency, data-model.md §Invariants] <!-- contracts move_page「3.b. 新しい親が DB 所属でないことを確認」、data-model validate_move「if either page is a database page」で両方チェック明記 -->
- [ ] CHK015 - `save_editor` / `open_editor` から `isDirty` を削除する変更は、spec の自動保存移行要件（Assumptions）と contracts（EditorStateDto 変更）で一致しているが、この変更の影響を受ける既存フロントエンドコードの範囲は特定されているか？ [Partial, contracts §save_editor] <!-- バックエンド変更は plan で特定（session.rs, editor_commands.rs）。フロントエンドの isDirty 消費箇所は未列挙 -->

## Scenario Coverage

- [ ] CHK016 - `move_page` で `newParentId: null`（ルート昇格）の場合に適用されるバリデーション（DB ページチェックのみ？循環参照チェックはスキップ？）は明確に規定されているか？ [Partial, contracts §move_page] <!-- contracts の手順3「newParentId が指定された場合」の条件分岐から暗黙的に推論可能だが、null 時の明示的な記載なし -->
- [x] CHK017 - 不変条件1「database_id IS NOT NULL → parent_id IS NULL」が既存データで違反している場合の対処（マイグレーション時のデータ検証）は規定されているか？ [Coverage] <!-- migration 0007 は parent_id を NULL デフォルトで追加。既存行は全て parent_id=NULL となり不変条件1は自動的に充足。データ検証不要 -->
- [ ] CHK018 - Recursive CTE の安全上限（depth < 10）に達した場合の動作（エラー返却？切り詰め？）は規定されているか？データ破損で循環が発生した場合のフェイルセーフは定義されているか？ [Gap, data-model.md §Recursive CTE] <!-- CTE に「安全上限」コメントがあるのみ。上限到達時の動作・エラーハンドリングは未定義 -->
- [ ] CHK019 - `ancestor_chain` / `depth` / `max_descendant_depth` がインメモリの `&[Page]` を受け取る設計で、全ページをメモリにロードする前提だが、500+ ページ規模でのメモリ影響は考慮されているか？ [Partial, Spec §CC-003] <!-- plan.md「500ページ規模では仮想化は不要」で規模は考慮済み。ただし階層操作時の全ページロードのメモリ影響は未分析（実害は極小） -->

## Edge Case Coverage

- [ ] CHK020 - ページを自分自身の子に移動しようとした場合（`pageId == newParentId`）の動作は規定されているか？不変条件5（自己参照禁止）はドメインサービスで検証されるか？ [Partial, data-model.md §Invariants] <!-- 不変条件5「parent_id = id は禁止」は記載あり。ただし validate_move の ancestors_of_target は親の祖先チェーンであり、自己参照ケースを捕捉するかは実装依存 -->
- [x] CHK021 - 子ページを持つ親ページが、同時に別のページの子でもある場合の削除時の昇格先（祖父母への昇格）はどの深度まで正しく動作するか、テストシナリオとして規定されているか？ [Edge Case, Spec §FR-008] <!-- contracts「削除対象の parent_id を取得（昇格先）→ 子の parent_id を一括更新」で任意深度に対応。CC-005 でテスト要件あり -->
- [ ] CHK022 - `create_child_page` で指定された `parentId` が、リクエスト処理中に並行して削除された場合の動作は規定されているか？（デスクトップアプリでも非同期 IPC で発生しうる） [Gap] <!-- 未記載。FK 制約で暗黙的に保護されるが、このエラーパスの明示的な仕様はなし -->

## Non-Functional Requirements

- [ ] CHK023 - CC-003 のパフォーマンス目標（サイドバー 200ms @500 ページ）に対して、`list_sidebar_items` のクエリ性能要件（インデックス活用・クエリ数上限）はバックエンド側で規定されているか？ [Partial, Spec §CC-003] <!-- idx_pages_parent_id インデックスは data-model で定義済み。ただしバックエンドのクエリ時間バジェット（例: クエリ合計 50ms 以内）は未定義 -->
- [ ] CHK024 - 階層操作（move/create_child/delete）のレスポンスタイム目標は規定されているか？CC-003 はサイドバー描画のみで、書き込み操作の性能目標が未定義ではないか？ [Gap, Spec §CC-003] <!-- CC-003 は読み取り操作のみ（レンダリング 200ms、展開 50ms、遷移 100ms）。書き込み操作の目標なし -->

## Dependencies & Assumptions

- [x] CHK025 - マイグレーション 0007 の `ALTER TABLE ... ADD COLUMN` が SQLite の制約（既存行への DEFAULT 適用）と互換であることは検証済みか？ [Assumption, data-model.md §Migration 0007] <!-- SQLite 標準機能。parent_id は NULL 許容（DEFAULT NULL）、sort_order は DEFAULT 0。プロジェクトに先行マイグレーション 6 件あり同パターン実績あり -->
- [ ] CHK026 - `PageHierarchyService` がステートレス（`struct PageHierarchyService;`）で設計されているが、全メソッドが `&[Page]` を引数に取るため、呼び出し元が適切なページセットを渡す責務が明確か？ [Partial, data-model.md §PageHierarchyService] <!-- サービス API は明確。IPC コマンド層がページをロードして渡す想定だが、オーケストレーション層の責務マッピングは未記載 -->

## Notes

- Check items off as completed: `[x]`
- Add comments or findings inline
- Items are numbered sequentially for easy reference
- [Gap] = 仕様に記載がない要件、[Ambiguity] = 曖昧な記載、[Conflict] = 矛盾する記載
