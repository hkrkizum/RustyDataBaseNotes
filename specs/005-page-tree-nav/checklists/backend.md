# Backend Requirements Quality Checklist: Page Tree Navigation

**Purpose**: バックエンド要件（データモデル・ドメインロジック・IPC コントラクト）の完全性・明確性・一貫性を実装前に検証する
**Created**: 2026-03-22
**Feature**: [spec.md](../spec.md) | [data-model.md](../data-model.md) | [contracts/ipc-commands.md](../contracts/ipc-commands.md)
**Focus**: バックエンド中心（データモデル・ドメインロジック・IPC コントラクト）
**Depth**: Standard | **Audience**: 実装前の自己レビュー（著者）
**Last Reviewed**: 2026-03-22 (2nd pass, post checklist-apply)

## Requirement Completeness

- [x] CHK001 - `move_page` で現在の親と同じ `newParentId` を指定した場合の動作（no-op／エラー／通常更新）は定義されているか？ <!-- spec FR-007「移動先が現在の親と同一の場合は no-op（無操作）とする」と定義済み (G-01) -->
- [x] CHK002 - `move_page` および `delete_page` のトランザクション境界（validate → update が単一トランザクション内で実行される）は明示的に規定されているか？ <!-- contracts move_page「手順1-4は単一トランザクション内で実行する」、delete_page_with_promotion「トランザクション内で」と両方明記 (P-01) -->
- [x] CHK003 - `bulk_update_parent_id` が部分的に失敗した場合のロールバック動作は規定されているか？ <!-- data-model.md「delete_page_with_promotion のトランザクション内でのみ呼び出すこと。単体で呼び出した場合の部分失敗に対するロールバック保証はない」と制約明記 (P-02) -->
- [x] CHK004 - `list_sidebar_items` が返すフラットリストからツリー構造を構築するアルゴリズム（ソート順・グルーピングルール）はフロントエンド側の責務として明確に境界が定義されているか？ <!-- contracts「ソート順はフロントエンドに委任」、data-model.md の SidebarTreeNode「フロントエンドで付与」で明確 -->
- [x] CHK005 - `create_child_page` の深度チェックにおいて、親の深度を誰が（どのレイヤーが）計算するかのオーケストレーションは規定されているか？ <!-- contracts「PageHierarchyService::depth() で親の深度を計算し、validate_create_child() に渡す」と具体化 (P-03)。data-model.md にオーケストレーション注記あり (P-12) -->
- [x] CHK006 - `delete_page` で削除されるページの blocks が CASCADE 削除される旨は spec レベル（FR-008）で規定されているか、それとも contracts のみか？ <!-- spec FR-008「削除されたページ自身に紐づくブロックデータは CASCADE で同時に削除される」と明記 (G-02) -->

## Requirement Clarity

- [x] CHK007 - 「最大5階層」の深度カウント基準（ルート = 深度1）は spec で明示されているか？ <!-- spec FR-004「深度はルートレベルを1として計算する」と明記。data-model.md でも root = 1 (G-03) -->
- [x] CHK008 - ルートレベルの表示順は spec FR-011 と Assumptions で方向が一致しているか？ <!-- FR-011「created_at DESC（作成日時降順）」に修正済み。Assumptions「作成日時降順」と整合 (G-04) -->
- [x] CHK009 - `sort_order` が全ページで `DEFAULT 0` のまま使用される場合、同一親内の子ページ表示順の決定ルール（created_at フォールバック）は明示されているか？ <!-- Key Entities「本スコープでは表示順に created_at DESC を使用する」、Assumptions にも同様の記載あり -->
- [x] CHK010 - ON DELETE SET NULL（DB レベル）とアプリケーション層の子昇格ロジックの二重防御の優先関係・相互作用は明確に文書化されているか？ <!-- data-model.md に専用セクションで詳細に説明済み: アプリ層が先に昇格→DB レベルはフェイルセーフ -->
- [x] CHK011 - `PageDto` の `sortOrder` フィールドの IPC 境界での型（Rust `i64` → TypeScript `number`）は明示されているか？ <!-- data-model.md PageDto に「sortOrder: number」と TypeScript 側の型注記追記済み (P-04)。本スコープでは値が 0 固定のため実害なし -->

## Requirement Consistency

- [x] CHK012 - `delete_page` は contracts で「置き換え → `delete_page_with_promotion`」と記載されているが、同時に「コマンド名は `delete_page` のまま」とも記載。これは「既存コマンドの内部ロジック変更」として一貫しているか？ <!-- 本文「コマンド名は delete_page のまま維持」+ Summary「Modified」で一貫 -->
- [x] CHK013 - `find_root_pages`（parent_id IS NULL, database_id IS NULL）と既存の `find_standalone_pages`（database_id IS NULL）の役割・使い分けは明確か？ <!-- data-model.md に各メソッドのユースケース追記: find_root_pages=サイドバーのルート表示用、find_standalone_pages=全スタンドアロンページ取得用 (P-05) -->
- [x] CHK014 - 不変条件2「parent_id が指すページは database_id IS NULL」に対し、`validate_move` は移動元・移動先の両方をチェックするか？ <!-- contracts move_page「3.b. 新しい親が DB 所属でないことを確認」、data-model validate_move「if either page is a database page」で両方チェック明記 -->
- [x] CHK015 - `save_editor` / `open_editor` から `isDirty` を削除する変更の影響を受ける既存フロントエンドコードの範囲は特定されているか？ <!-- plan.md にフロントエンド影響箇所を列挙: editor/ 配下の isDirty 参照、UnsavedConfirmModal（廃止対象）、save/open_editor 呼び出し元 (P-06) -->

## Scenario Coverage

- [x] CHK016 - `move_page` で `newParentId: null`（ルート昇格）の場合に適用されるバリデーションは明確に規定されているか？ <!-- contracts「newParentId が null の場合: 手順2（DB ページチェック）のみ適用し、parent_id を NULL に更新（ルートに昇格）。循環参照・深度チェックはスキップする」と明記 (P-07) -->
- [x] CHK017 - 不変条件1「database_id IS NOT NULL → parent_id IS NULL」が既存データで違反している場合の対処は規定されているか？ <!-- migration 0007 は parent_id を NULL デフォルトで追加。既存行は全て parent_id=NULL となり不変条件1は自動的に充足。データ検証不要 -->
- [x] CHK018 - Recursive CTE の安全上限（depth < 10）に達した場合の動作は規定されているか？データ破損で循環が発生した場合のフェイルセーフは定義されているか？ <!-- data-model.md「depth < 10 到達時は CTE が停止し不完全な祖先チェーンを返す→アプリ層で件数と MAX_DEPTH を比較→期待値超過時は CircularReference エラー」と明記 (P-08) -->
- [ ] CHK019 - `ancestor_chain` / `depth` / `max_descendant_depth` がインメモリの `&[Page]` を受け取る設計で、全ページをメモリにロードする前提だが、500+ ページ規模でのメモリ影響は考慮されているか？ [Partial] <!-- plan.md「500ページ規模では仮想化は不要」で規模は考慮済み。階層操作時の全ページロードのメモリ影響は未分析だが、Page 構造体 ×500 ≈ 数十KB で実害なし (YAGNI)。過剰設計指摘あり -->

## Edge Case Coverage

- [x] CHK020 - ページを自分自身の子に移動しようとした場合（`pageId == newParentId`）の動作は規定されているか？ <!-- data-model.md validate_move「自己参照チェック（page_id == new_parent_id）を最初に実行し、ancestors_of_target に依存しない明示的な検出を行う」と明記 (P-09) -->
- [x] CHK021 - 子ページを持つ親ページが、同時に別のページの子でもある場合の削除時の昇格先（祖父母への昇格）はどの深度まで正しく動作するか？ <!-- contracts「削除対象の parent_id を取得（昇格先）→ 子の parent_id を一括更新」で任意深度に対応。CC-005 でテスト要件あり -->
- [x] CHK022 - `create_child_page` で指定された `parentId` が、リクエスト処理中に並行して削除された場合の動作は規定されているか？ <!-- contracts create_child_page エラー「処理中に並行して親が削除された場合、FK 制約違反を NotFound に変換」と明記 (P-10) -->

## Non-Functional Requirements

- [x] CHK023 - CC-003 のパフォーマンス目標（サイドバー 200ms @500 ページ）に対して、`list_sidebar_items` のクエリ性能要件はバックエンド側で規定されているか？ <!-- plan.md Performance Goals「list_sidebar_items: バックエンドクエリ合計 ≤ 50ms @500ページ」と明記 (P-11)。idx_pages_parent_id インデックスも data-model で定義済み -->
- [x] CHK024 - 階層操作（move/create_child/delete）のレスポンスタイム目標は規定されているか？ <!-- spec CC-003「階層操作（子作成・移動・削除）は500ms以内に完了する」と追記済み (G-05) -->

## Dependencies & Assumptions

- [x] CHK025 - マイグレーション 0007 の `ALTER TABLE ... ADD COLUMN` が SQLite の制約（既存行への DEFAULT 適用）と互換であることは検証済みか？ <!-- SQLite 標準機能。parent_id は NULL 許容（DEFAULT NULL）、sort_order は DEFAULT 0。プロジェクトに先行マイグレーション 6 件あり同パターン実績あり -->
- [x] CHK026 - `PageHierarchyService` がステートレスで設計されているが、全メソッドが `&[Page]` を引数に取るため、呼び出し元が適切なページセットを渡す責務が明確か？ <!-- data-model.md「IPC コマンドハンドラが PageRepository 経由で必要なページセットをロードし、PageHierarchyService の各メソッドに &[Page] として渡す責務を持つ」と明記 (P-12) -->

## Notes

- Check items off as completed: `[x]`
- Add comments or findings inline
- Items are numbered sequentially for easy reference
- [Gap] = 仕様に記載がない要件、[Partial] = 関連記述はあるが詳細度不足、[Conflict] = 矛盾する記載
