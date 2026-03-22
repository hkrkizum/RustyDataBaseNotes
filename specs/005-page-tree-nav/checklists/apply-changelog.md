# Checklist Apply Changelog: Page Tree Navigation
**実行日時**: 2026-03-22 (2nd apply — frontend-ux)
**入力**: review-report.md (3rd pass, frontend-ux section)
**モード**: 差分更新（非破壊）

---

## 変更統計（累計）
- spec.md: 10箇所 補完 (FG-01〜FG-19) / 1箇所 追記 (FG-19: US2 scenario) / 前回分 5箇所
- plan.md: 1箇所 追記 (Frontend Design Decisions セクション, FP-01〜FP-12) / 前回分 2箇所
- data-model.md: 前回分 7箇所（今回変更なし）
- contracts/ipc-commands.md: 前回分 4箇所（今回変更なし）
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

---

## Frontend/UX 変更詳細（2nd apply）

### spec.md の変更

| レポートID | 変更種別 | 対象箇所 | 変更内容の要約 |
|-----------|---------|---------|--------------|
| FG-01 | 補完 | FR-010 | 「240–260px」に「実装時に決定」の注記を追加 |
| FG-02 | 補完 | FR-006 | インデント・5階層視認性・トランケーション要件を追記 |
| FG-03 | 補完 | FR-006 | テキストオーバーフロー処理（ellipsis）を追記（FG-02 と同時） |
| FG-04 | 補完 | FR-013 | 「...」ボタン「アイテム右端に即時表示」を明記 |
| FG-05 | 補完 | FR-013 | インライン編集バリデーション UX（空文字→復帰，maxLength 255）を追記 |
| FG-06 | 補完 | FR-001 | サイドバーのスクロール動作・ヘッダー固定表示を追記 |
| FG-07 | 補完 | FR-002 | アクティブアイテムへの自動スクロールを追記 |
| FG-08 | 補完 | FR-002 | shadcn/ui「具体的コンポーネント選択は実装時に決定」と明記 |
| FG-09 | 補完 | FR-013 | 「ドロップダウン等」→「ドロップダウンメニュー」に確定 |
| FG-10 | 補完 | FR-007 | D&D の意味「ドロップ先の子になる」を明記，同一親内並び替えは後続 |
| FG-11 | 補完 | FR-011 | 表示順を「ルートレベルおよび同一親内の子ページ」に拡張 |
| FG-12 | 補完 | FR-013 | 自動遷移時に last-opened-item を更新する旨を追記 |
| FG-13 | 補完 | FR-013 | 削除確認ダイアログ＋子ページ昇格の説明を追記 |
| FG-14 | 補完 | FR-002 | DB 所属ページのクリック→エディタ遷移を明記 |
| FG-15 | 補完 | FR-013 | 最大深度で「子ページ作成」メニュー非表示を追記 |
| FG-16 | 補完 | FR-007 | 無効ドロップ先に「自分自身」を追加 |
| FG-17 | 補完 | FR-013 | フォーカスアウト時は確定として扱う旨を追記 |
| FG-18 | 補完 | FR-014, FR-017 | localStorage 破損時のデフォルト値フォールバックを追記 |
| FG-19 | 追記 | US2 Scenario 5 | 空状態→新規作成→自動遷移→インライン編集のシナリオを追加 |

### plan.md の変更

| レポートID | 変更種別 | 対象箇所 | 変更内容の要約 |
|-----------|---------|---------|--------------|
| FP-01 | 追記 | Frontend Design Decisions > D&D 詳細 | auto-scroll addon の使用を追記 |
| FP-02 | 追記 | Frontend Design Decisions > 自動保存 | デバウンス間隔 500ms を追記 |
| FP-03 | 追記 | Frontend Design Decisions > 自動保存 | リトライ間隔（指数バックオフ）・toast 内容を追記 |
| FP-04 | 追記 | Frontend Design Decisions > レイアウト | サイドバー非表示時フル幅展開を追記 |
| FP-05 | 追記 | Frontend Design Decisions > D&D 詳細 | D&D 中のコンテキストメニュー無効化を追記 |
| FP-06 | 追記 | Frontend Design Decisions > 更新戦略 | 楽観的更新＋エラー時ロールバック＋再取得を追記 |
| FP-07 | 追記 | Frontend Design Decisions > D&D 詳細 | ルート昇格のドロップ UI を追記 |
| FP-08 | 追記 | Frontend Design Decisions > ショートカット | テキスト入力中のイベント伝播停止を追記 |
| FP-09 | 追記 | Frontend Design Decisions > 起動時復元 | 折りたたまれた祖先の自動展開を追記 |
| FP-10 | 追記 | Frontend Design Decisions > パフォーマンス | React Profiler 計測方法を追記 |
| FP-11 | 追記 | Frontend Design Decisions > 品質検証 | ビジュアルレビュー 6パターンを追記 |
| FP-12 | 追記 | Frontend Design Decisions > 品質検証 | 旧スタイリング方式の grep 検証を追記 |

### 要判断項目（人間のレビューが必要）

なし。全項目を自動適用済み。

### spec.md への FP-09 要件反映について

FP-09（起動時復元の祖先自動展開）は plan.md への技術設計追記に加え，
FR-017 にもユーザー向け要件として反映済み（「復元対象が折りたたまれた子ページの場合，
祖先ノードを自動展開してからスクロール」）。

---

---

## Autosave 変更詳細（3rd apply）

**実行日時**: 2026-03-22
**入力**: review-report.md (7th pass, autosave section)
**モード**: 差分更新（非破壊）

### 変更統計
- spec.md: 1箇所 補完（Assumptions の自動保存項目を拡充: G-01, G-02, G-03）
- plan.md: 1箇所 大幅拡充（「自動保存パラメータ」→「自動保存設計」に改題＋責務・遷移動作・テスト方針を追記: P-01〜P-11）
- 新規作成ファイル: なし
- 簡素化提案（要判断）: 0箇所

### spec.md の変更

| レポートID | 変更種別 | 対象箇所 | 変更内容の要約 |
|-----------|---------|---------|--------------|
| G-01 | 補完 | Assumptions 自動保存項目 | トリガー条件「ブロックの追加・削除・内容変更・順序変更」を追記 |
| G-02 | 補完 | Assumptions 自動保存項目 | 遷移時フラッシュ「ベストエフォートで保存，失敗してもナビゲーション許可」を追記 |
| G-03 | 補完 | Assumptions 自動保存項目 | 確認ダイアログ不要の前提「自動保存＋遷移時フラッシュにより」を明記 |

### plan.md の変更

| レポートID | 変更種別 | 対象箇所 | 変更内容の要約 |
|-----------|---------|---------|--------------|
| P-01 | 追記 | 自動保存設計 | useAutoSave はエディタ専用，プロパティ自動保存は既存のまま独立 |
| P-02 | 追記 | 自動保存設計 > useAutoSave の責務 | 5つの責務（debounce・リトライ・エラー種別・toast・アンマウントフラッシュ）をリスト化 |
| P-03 | 追記 | 自動保存設計 | FE 主導の方針を明示（BE は save_editor を受動的に実行） |
| P-04 | 追記 | 自動保存設計 | 既存プロパティ自動保存の動作仕様（update_property_value 直接呼出）への参照 |
| P-05 | 追記 | 自動保存設計 > ページ遷移時の動作 | 遷移時フラッシュ＋新 useAutoSave インスタンス初期化，並行 save なし |
| P-06 | 追記 | 自動保存設計 > useAutoSave 責務 #2 | リトライは常に最新のエディタ状態を保存する |
| P-07 | 追記 | 自動保存設計 > ページ遷移時の動作 | リトライ中の遷移時は toast で警告（レビュアー指示） |
| P-08 | 追記 | 自動保存設計 > useAutoSave 責務 #3 | 一時的エラーはリトライ，永続的エラー（NotFound等）は即座に通知 |
| P-09 | 追記 | 自動保存設計 > テスト修正方針 | EditorSession テスト維持，useAutoSave Vitest 追加，手動保存テスト削除 |
| P-10 | 追記 | 自動保存設計 > useAutoSave 責務 #5 | アンマウント時 useEffect cleanup でフラッシュ（レビュアー指示） |
| P-11 | 追記 | 自動保存設計 > useAutoSave 責務 #4 | 継続的失敗時は変更のたびに toast 表示（レビュアー指示） |

### 要判断項目（人間のレビューが必要）

なし。全項目を自動適用済み。

---

## 次のステップ

1. `git diff` で変更内容を確認する
2. 満足したら `git commit` する
3. `/checklist-review autosave` を再実行し，カバレッジ率の改善を確認する
