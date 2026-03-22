# Checklist Apply Changelog: Page Tree Navigation
**実行日時**: 2026-03-22
**入力**: review-report.md
**モード**: 差分更新（非破壊）

---

## 変更統計
- spec.md: 2箇所 追記 (G-01, G-02, G-05) / 2箇所 補完 (G-03, G-04) / 0箇所 移動受入
- plan.md: 1箇所 追記 (P-11) / 1箇所 補完 (P-06) / 0箇所 移動受入
- data-model.md: 3箇所 追記 (P-02, P-08, P-12) / 4箇所 補完 (P-04, P-05×2, P-09)
- contracts/ipc-commands.md: 2箇所 追記 (P-01, P-07) / 2箇所 補完 (P-03, P-10)
- 新規作成ファイル: なし
- 簡素化提案（要判断）: 0箇所

---

## 変更詳細

### spec.md の変更

| レポートID | 変更種別 | 対象箇所 | 変更内容の要約 |
|-----------|---------|---------|--------------|
| G-01 | 追記 | FR-007 | 移動先が現在の親と同一の場合は no-op（無操作）とする旨を追加 |
| G-02 | 追記 | FR-008 | 削除ページ自身のブロックデータは CASCADE で削除される旨を追加 |
| G-03 | 補完 | FR-004 | 「ルートレベルを深度1として計算する」と基準を明記 |
| G-04 | 補完 | FR-011 | 「created_at 順」→「created_at DESC（作成日時降順）」に修正し Assumptions との整合を確保 |
| G-05 | 追記 | CC-003 | 階層操作（子作成・移動・削除）の性能目標（500ms以内）を追加 |

### contracts/ipc-commands.md の変更

| レポートID | 変更種別 | 対象箇所 | 変更内容の要約 |
|-----------|---------|---------|--------------|
| P-01 | 追記 | move_page 動作説明末尾 | 手順1-4は単一トランザクション内で実行する旨を追記 |
| P-03 | 補完 | create_child_page 手順3 | `PageHierarchyService::depth()` → `validate_create_child()` の呼び出しフローを具体化 |
| P-07 | 追記 | move_page 手順3 | `newParentId` が null の場合の動作（DB チェックのみ→ルート昇格）を明記 |
| P-10 | 補完 | create_child_page エラー | 「処理中に並行して親が削除された場合，FK 制約違反を NotFound に変換」を追記 |

### data-model.md の変更

| レポートID | 変更種別 | 対象箇所 | 変更内容の要約 |
|-----------|---------|---------|--------------|
| P-02 | 追記 | bulk_update_parent_id ドキュメント | delete_page_with_promotion トランザクション内でのみ呼び出す制約を注記 |
| P-04 | 補完 | PageDto sortOrder コメント | TypeScript 側の型注記（`sortOrder: number`）を追加 |
| P-05 | 補完 | find_root_pages, find_standalone_pages | 各メソッドのユースケースを追記 |
| P-08 | 追記 | Recursive CTE セクション | depth < 10 到達時のフェイルセーフ動作（不完全チェーン検出→CircularReference エラー）を追記 |
| P-09 | 補完 | validate_move ドキュメント | self-reference check（page_id == new_parent_id）の明示的検出を追記 |
| P-12 | 追記 | PageHierarchyService | IPC コマンドハンドラがページロード→サービスに渡すオーケストレーション注記を追記 |

### plan.md の変更

| レポートID | 変更種別 | 対象箇所 | 変更内容の要約 |
|-----------|---------|---------|--------------|
| P-06 | 補完 | Project Structure 後 | isDirty 削除のフロントエンド影響箇所（コンポーネント・フック・モーダル）を列挙 |
| P-11 | 追記 | Performance Goals | `list_sidebar_items` バックエンドクエリ合計 ≤50ms @500ページ を追加 |

### 要判断項目（人間のレビューが必要）

なし。review-report.md の過剰設計指摘（CHK019, CHK011, CHK024）は以下の理由で自動適用を見送り:
- CHK019（メモリ影響）: 実質的に無視できるレベル（数十KB）。plan.md が「投機的最適化なし」と明言済み
- CHK011（sort_order 型安全性）: 本スコープでは値が 0 固定。将来スコープで検討
- CHK024（書き込み性能目標）: G-05 として spec CC-003 に控えめな目標（500ms）を追記済み

### Constitution 準拠補完

review-report.md で指摘された未カバー原則:
- **VI. Rust ドキュメント標準**: plan.md §Constitution Check (L68-71) で既に言及済み。チェックリスト項目としての未設置は checklist の粒度問題であり，spec/plan の追記は不要と判断
- **IV. Test-First（テストシナリオ網羅性）**: CC-005 でテスト要件を列挙済み。個別テストケースの入力条件・期待結果の明確性は tasks.md / 実装時に詳細化する領域と判断

---

## 次のステップ

1. `git diff` で変更内容を確認する
2. 満足したら `git commit` する
3. 必要に応じて `/checklist-review` を再実行し，カバレッジ率の改善を確認する
