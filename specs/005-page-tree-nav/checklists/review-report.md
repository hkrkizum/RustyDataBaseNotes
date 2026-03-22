# Checklist Review Report: Page Tree Navigation

**レビュー日時**: 2026-03-22
**対象チェックリスト**: backend.md
**レビュー結果サマリー**:
- ✅ Covered: 8 項目 (CHK004, CHK009, CHK010, CHK012, CHK014, CHK017, CHK021, CHK025)
- ⚠️ Partial: 14 項目 (CHK002, CHK003, CHK005, CHK007, CHK008, CHK011, CHK013, CHK015, CHK016, CHK019, CHK020, CHK023, CHK026)
- ❌ Gap: 4 項目 (CHK001, CHK006, CHK018, CHK024)
- 🔀 Misplaced: 0 項目
- **カバレッジ率**: 30.8% (8/26)

---

## 仕様側の問題（spec.md で対応すべき項目）

機能要件・ユーザー体験・ビジネスロジックに関するギャップ。

| ID | チェック項目 | 判定 | 推奨アクション |
|----|------------|------|--------------|
| G-01 | CHK001 — `move_page` 同一親指定時の動作 | Gap | spec.md FR-007 に「移動先が現在の親と同一の場合は no-op とする」旨を追記 |
| G-02 | CHK006 — 削除ページの blocks CASCADE 削除 | Gap | spec.md FR-008 に「削除されたページ自身のブロックデータは CASCADE で削除される」旨を追記（子ページの昇格と区別して明記） |
| G-03 | CHK007 — 深度カウント基準 | Partial | spec.md FR-004 に「ルートレベル = 深度1 として最大深度5」と基準を明記 |
| G-04 | CHK008 — 表示順の方向 | Partial | spec.md FR-011 の「created_at 順」を「created_at DESC（作成日時降順）」に修正し、Assumptions との整合を確保 |
| G-05 | CHK024 — 書き込み操作の性能目標 | Gap | CC-003 に階層操作（move/create_child/delete）のレスポンスタイム目標を追記するか、意図的に除外する場合はその旨を明記 |

## 計画側の問題（plan.md / data-model.md / contracts で対応すべき項目）

技術的な設計・アーキテクチャ・非機能要件に関するギャップ。

| ID | チェック項目 | 判定 | 推奨アクション |
|----|------------|------|--------------|
| P-01 | CHK002 — move_page のトランザクション境界 | Partial | contracts move_page に「手順2-4は単一トランザクション内で実行する」旨を明記（delete_page と同様の記法で） |
| P-02 | CHK003 — bulk_update_parent_id の部分失敗 | Partial | data-model.md のリポジトリメソッドに「delete_page_with_promotion のトランザクション内でのみ呼び出す」制約を注記 |
| P-03 | CHK005 — 深度チェックのオーケストレーション | Partial | contracts create_child_page の手順3を「PageHierarchyService::depth() で親の深度を計算し、validate_create_child() に渡す」に具体化 |
| P-04 | CHK011 — sortOrder の IPC 型マッピング | Partial | contracts PageDto に TypeScript 側の型注記（`sortOrder: number`）を追加。本スコープでは値が 0 固定のため優先度低 |
| P-05 | CHK013 — find_root_pages vs find_standalone_pages | Partial | data-model.md に各メソッドのユースケースを追記: `find_root_pages` = サイドバーのルート表示用、`find_standalone_pages` = 既存の全スタンドアロンページ取得用 |
| P-06 | CHK015 — isDirty 削除の FE 影響範囲 | Partial | plan.md Project Structure にフロントエンド側の影響箇所（isDirty を参照するコンポーネント・フック）を追記 |
| P-07 | CHK016 — move_page null 時のバリデーション | Partial | contracts move_page に「newParentId が null の場合: 手順2（DB ページチェック）のみ適用、parent_id を NULL に更新」を明記 |
| P-08 | CHK018 — CTE 安全上限到達時の動作 | Gap | data-model.md の CTE セクションに「depth < 10 到達時はクエリが停止し、不完全な祖先チェーンが返される。アプリ層で件数と MAX_DEPTH を比較し、異常を検出する」等のフェイルセーフ仕様を追記 |
| P-09 | CHK020 — 自己参照（pageId == newParentId） | Partial | data-model.md validate_move に「self-reference check: page_id == new_parent_id の場合は CircularReference を返す」を追記。ancestors_of_target による暗黙的検出に依存しない明示的チェック |
| P-10 | CHK022 — 並行削除時の競合 | Gap | contracts create_child_page のエラーに「FK 制約違反（親が削除済み）→ PageError::NotFound に変換」を追記 |
| P-11 | CHK023 — list_sidebar_items のクエリ性能 | Partial | plan.md Performance Goals に「list_sidebar_items: 全アイテム取得クエリ合計 ≤ 50ms @500ページ」等のバックエンド性能バジェットを追記（任意） |
| P-12 | CHK026 — PageHierarchyService の呼び出し責務 | Partial | data-model.md に「IPC コマンドハンドラが PageRepository 経由で必要なページセットをロードし、PageHierarchyService に渡す」旨のオーケストレーション注記を追記 |

## 配置ミス（Misplaced 項目）

該当なし。spec.md と plan.md / data-model.md / contracts の責務分担は適切。

## 意図的な除外の確認

以下の Gap / Partial 項目について、意図的に対象外としている場合は理由を記録してください。

| ID | チェック項目 | 除外理由（人間が記入） |
|----|------------|---------------------|
| G-01 | move_page 同一親指定時の動作 | |
| G-02 | 削除ページの blocks CASCADE 削除 | |
| G-05 | 書き込み操作の性能目標 | |
| P-08 | CTE 安全上限到達時のフェイルセーフ | |
| P-10 | 並行削除時の競合エラーパス | |

---

## Constitution 準拠チェック

### カバーされている原則

| Constitution 原則 | 対応するチェック項目 |
|------------------|-------------------|
| I. Local-First Product Integrity | CHK002 (トランザクション境界), CHK003 (ロールバック), CHK010 (二重防御), CHK017 (データ整合), CHK022 (競合安全性) |
| II. Domain-Faithful Information Model | CHK012 (コマンド命名一貫性), CHK013 (リポジトリメソッド名の区別) |
| III. Typed Boundaries and DDD | CHK004 (FE/BE 責務境界), CHK011 (IPC 型マッピング), CHK014 (バリデーション範囲) |
| IV. Test-First Delivery and Quality Gates | CHK021 (テストシナリオ) |
| V. Safe Rust, SOLID, Maintainability | CHK005 (オーケストレーション=SRP), CHK026 (呼び出し責務=DIP) |
| VII. 防御的エラーハンドリング | CHK018 (CTE フェイルセーフ), CHK020 (自己参照検出), CHK016 (null ケースバリデーション) |

### カバーされていない原則

以下の constitution 原則に対応するチェック項目が不足しています:

- **VI. Rust ドキュメント標準**: 新規公開アイテム（`PageHierarchyService`, `validate_move`, `validate_create_child`, 新規エラーバリアント等）のドキュメントコメント要件が spec/plan で言及されているか（plan §Constitution Check で触れられているが、チェックリスト項目としては未設置）
- **IV. Test-First（テストシナリオの網羅性）**: 個別のテストケース（循環参照、深度超過、DB ページ制約）の要件品質は未検査。CC-005 で列挙はされているが、各テストの入力条件・期待結果の明確性は未評価

### 矛盾・過剰設計の指摘

| チェック項目 | 関連する原則 | 指摘内容 |
|------------|------------|---------|
| CHK019 | V. Maintainability (YAGNI) | 500ページ × Page 構造体のメモリ影響は実質的に無視できるレベル（数十KB）。plan.md が「投機的最適化なし」と明言しており、この項目の優先度は最低。実装時に計測で判断すれば十分 |
| CHK011 | V. Maintainability (YAGNI) | sort_order は本スコープで常に 0。i64 → number の型安全性は将来スコープで sort_order を実際に使用する際に検討すれば十分 |
| CHK024 | V. Maintainability (YAGNI) | デスクトップアプリの単一ユーザー・ローカル SQLite 環境で、書き込み操作の性能目標を事前に定義するのは過剰の可能性。ただし、CC-003 で読み取り性能のみ定義されている非対称性は指摘として有効 |
