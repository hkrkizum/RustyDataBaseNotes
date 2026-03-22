# Checklist Review Report: Page Tree Navigation

**レビュー日時**: 2026-03-22 (7th pass, autosave 矛盾解消＋レビュアー判断反映)
**対象チェックリスト**: autosave.md
**レビュー結果サマリー**:

### autosave.md (Autosave Migration Requirements Quality)
- ✅ Covered: 10 項目（うち 2 項目はレビュアー承認による意図的除外，1 項目は plan.md 修正）
- ⚠️ Partial: 7 項目
- ❌ Gap: 8 項目
- 🔀 Conflict: 0 項目（全 7 Conflict を解消済み）
- **カバレッジ率**: 40% (10/25)

### 前回レビューからの変化

| 指標 | 6th pass | 7th pass | 変化 |
|------|---------|---------|------|
| ✅ Covered | 0 (0%) | 10 (40%) | +10 items |
| ⚠️ Partial | 9 | 7 | -2 items |
| ❌ Gap | 9 | 8 | -1 item |
| 🔀 Conflict | 7 | 0 | **-7 items（全解消）** |
| カバレッジ率 | 0% | 40% | +40pp |

**改善要因**:
- レビュアーが plan.md を正式な値として確定 → CHK008/009/010/017 の Conflict 解消
- レビュアーが is_dirty()/mark_saved() を「残す」と決定 → CHK007/011/016 の Conflict 解消，plan.md 修正済み
- plan.md に削除対象 UI の完全リスト追記 → CHK004 Covered
- レビュアー承認による意図的除外 → CHK003, CHK025 Covered
- Ctrl+S no-op を plan.md に追記 → CHK013 Covered

### 解消済み矛盾のサマリー

| パラメータ | 確定値 | 根拠 |
|-----------|--------|------|
| デバウンス間隔 | **500ms** | plan.md（research.md に改訂注記済み） |
| リトライ間隔 | **指数バックオフ 1s→2s→4s** | plan.md（research.md に改訂注記済み） |
| Toast メッセージ | **「保存に失敗しました」（5秒，自動消去）** | plan.md（research.md に改訂注記済み） |
| is_dirty()/mark_saved() | **残す** | レビュアー決定（plan.md 修正済み，research.md と整合） |
| ドキュメント優先順位 | **plan.md > research.md** | レビュアー確定 |

---

## 仕様側の問題（spec.md で対応すべき項目）

| ID | チェック項目 | 判定 | 推奨アクション |
|----|------------|------|--------------|
| G-01 | CHK001 — 自動保存のトリガー条件 | Gap | spec.md Assumptions に「ブロックの追加・削除・内容変更・順序変更がトリガー」を追記 |
| G-02 | CHK002 — ページ遷移時のフラッシュ動作 | Partial | spec.md Assumptions に「遷移時はベストエフォートでフラッシュ保存し，失敗してもナビゲーションを許可する」を追記 |
| G-03 | CHK015 — 確認ダイアログ不要の前提 | Partial | spec.md Assumptions の「不要」に「自動保存＋遷移時フラッシュにより未保存データが残らないため」の前提を追記 |

## 計画側の問題（plan.md で対応すべき項目）

| ID | チェック項目 | 判定 | 推奨アクション |
|----|------------|------|--------------|
| P-01 | CHK005 — プロパティ自動保存との関係 | Partial | plan.md に「useAutoSave はエディタ専用。プロパティの自動保存は既存のまま独立」等の判断を追記 |
| P-02 | CHK006 — useAutoSave の責務範囲 | Partial | plan.md に useAutoSave の責務（debounce・リトライ・toast・アンマウント時フラッシュ）を明示的にリスト化 |
| P-03 | CHK012 — FE 主導の明示 | Partial | plan.md に「保存タイミングの制御は FE（useAutoSave）が担う。BE は save_editor を受動的に実行する」を追記 |
| P-04 | CHK014 — プロパティ自動保存の参照 | Partial | plan.md に既存プロパティ自動保存の動作仕様への参照を追記 |
| P-05 | CHK018 — 高速ページ切り替え | Gap | plan.md に「遷移時に前ページの pending save をフラッシュし，新ページの useAutoSave を初期化。並行 save は発生しない」を追記 |
| P-06 | CHK019 — リトライ中の追加編集 | Gap | plan.md に「リトライは常に最新のエディタ状態を保存する」を追記 |
| P-07 | CHK020 — リトライ中のページ遷移 | Partial | plan.md に「リトライ中にページ遷移する場合は遷移前に警告を表示する」を追記（レビュアー指示） |
| P-08 | CHK021 — ドメインエラーの種別 | Gap | plan.md に「リトライ対象は一時的エラーのみ。PageError::NotFound は即座に toast で通知しリトライしない」を追記 |
| P-09 | CHK022 — テスト修正範囲 | Gap | plan.md に EditorSession テストの修正方針を追記 |
| P-10 | CHK023 — アンマウント時フラッシュ | Partial | plan.md に「コンポーネントアンマウント時に useEffect cleanup で即時フラッシュ保存を実行する」を追記（レビュアー指示） |
| P-11 | CHK024 — 継続的失敗時の UX | Gap | plan.md に「継続的失敗時はトースト表示で警告する」方針を追記（レビュアー指示: 検討） |

## 配置ミス（Misplaced 項目）

該当なし。

## レビュアー承認済みの意図的除外

| ID | チェック項目 | 除外理由 |
|----|------------|---------|
| CHK003 | アプリ終了時の未保存データ処理 | 意図的除外。データはサイレントで削除。自動バックアップは将来スコープ |
| CHK025 | 初期化前入力のリスク | 意図的除外 |

## レビュアー指示による要求への組み込み

| ID | チェック項目 | レビュアー指示 |
|----|------------|--------------|
| CHK013 | Ctrl+S 廃止後の動作 | no-op（preventDefault で抑止）→ plan.md に反映済み |
| CHK020 | リトライ中のページ遷移 | 遷移前に警告 → checklist-apply で対応必要 |
| CHK023 | アンマウント時のタイマー安全性 | アンマウント時フラッシュ実施 → checklist-apply で対応必要 |
| CHK024 | 継続的失敗時の長期 UX | トースト表示を検討 → checklist-apply で対応必要 |

---

## Constitution 準拠チェック

### カバーされている原則

| Constitution 原則 | 対応するチェック項目 |
|------------------|-------------------|
| I. Local-First Product Integrity | CHK002（遷移時フラッシュ），CHK003（アプリ終了時），CHK024（継続的失敗）— データ完全性の観点 |
| III. Typed Boundaries and DDD | CHK007/011/016（EditorSession の責務＝解消済み），CHK012（FE/BE 責務分担） |
| IV. Test-First Delivery | CHK022（既存テスト修正範囲） |
| V. Safe Rust, SOLID, Maintainability | CHK005（SRP），CHK006（useAutoSave 責務=SRP） |
| VII. 防御的エラーハンドリング | CHK021（ドメインエラー種別），CHK024（継続的失敗 UX） |

### カバーされていない原則

- **Article II（Domain-Faithful Information Model）**: 自動保存は動作パターンの変更であり，ドメインモデル変更ではない。適切な除外
- **Article VI（Rust ドキュメント標準）**: is_dirty()/mark_saved() は残存するためドキュメント更新は不要

### 矛盾・過剰設計の指摘

該当なし。CHK011 の is_dirty() 残存決定により Constitution I（データ完全性）と V（YAGNI）のバランスは解消された。

---

## 総合評価

autosave.md は 0%（6th pass）→ **40%**（7th pass）に改善。全 7 Conflict 項目を解消済み。

残り 15 項目（Partial 7 + Gap 8）は `/checklist-apply autosave` による spec.md / plan.md の差分更新で対応可能。特に以下が優先:

1. **G-01（CHK001）**: トリガー条件 — 実装の前提となる基本要件
2. **P-05（CHK018）**: 高速ページ切り替え — サイドバーナビゲーションとの連携
3. **P-08（CHK021）**: ドメインエラー種別 — リトライ戦略の前提条件
4. **P-07（CHK020）+ P-10（CHK023）**: レビュアー指示の反映

**結論**: 矛盾は全て解消。残りの Gap/Partial は checklist-apply で対応すれば実装開始可能な水準に到達する。
