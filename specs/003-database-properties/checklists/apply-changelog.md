# Checklist Apply Changelog: プロパティシステムとデータベース概念の導入
**実行日時**: 2026-03-21
**入力**: review-report.md（data-integrity チェックリスト対象）
**モード**: 差分更新（非破壊）

---

## 変更統計
- spec.md: 5箇所 補完（G-01, G-02, G-04, G-05, G-07, G-08） / 2箇所 追記（G-03, G-06）
- data-model.md: 5箇所 補完（P-02, P-03, P-07, P-08, P-09） / 5箇所 追記（G-01詳細, P-01, P-10, P-11, P-12）
- contracts/ipc-commands.md: 2箇所 補完（P-05, P-06） / 1箇所 追記（P-04）
- 新規作成ファイル: なし
- 簡素化提案（要判断）: 0箇所

---

## 変更詳細

### spec.md の変更

| レポートID | 変更種別 | 対象箇所 | 変更内容の要約 |
|-----------|---------|---------|--------------|
| G-01 | 補完 | §Key Entities (Database, Property, PropertyValue) | 各エンティティの `updated_at` 更新タイミングを追記 |
| G-02 | 補完 | §Assumptions (数値型) | 「任意の IEEE 754 finite f64 を受け入れ，min/max 制約は設けない」を追記 |
| G-03 | 追記 | §Assumptions (末尾) | 文字数制限の単位を「Unicode スカラー値（Rust の char::count）」と明記 |
| G-04 | 補完 | §SC-001 | 前提条件を追記（初回ユーザー，空の状態，開発用デスクトップマシン） |
| G-05 | 補完 | §CC-003 | 測定条件を追記（ウォームキャッシュ，標準テキスト長，開発用マシン） |
| G-06 | 追記 | §Assumptions (末尾) | 並行操作の方針（single-writer，last-write-wins）を追記 |
| G-07 | 補完 | §FR-005 | 「プロパティが 0 個の場合はタイトル列のみ表示」を追記 |
| G-08 | 補完 | §Assumptions (型変更) | 「将来の型変換対応時はマイグレーション設計が別途必要」を追記 |

### data-model.md の変更

| レポートID | 変更種別 | 対象箇所 | 変更内容の要約 |
|-----------|---------|---------|--------------|
| G-01 | 追記 | §updated_at 更新トリガー一覧（新セクション） | 各エンティティの更新トリガー一覧テーブルを追加 |
| P-01 | 追記 | §Property — Position 管理 | 削除時のギャップ許容方針を追記 |
| P-02 | 補完 | §PropertyConfig enum | serde internally tagged 方式を明示し，JSON 出力例を追加 |
| P-03 | 補完 | §PropertyValue バリデーション規則 (Select) | ID 格納の設計根拠（選択肢名変更時の参照整合性）を追記 |
| P-07 | 補完 | §PropertyValue バリデーション規則 (Number) | -0.0 の正規化，subnormal/MAX/MIN f64 の受入方針を追記 |
| P-08 | 補完 | §PropertyValue バリデーション規則 (Date) | DateTime<Utc> 範囲の受入，UTC 強制を追記 |
| P-09 | 補完 | §PropertyConfig (SelectOption) | JSON 特殊文字は serde が自動エスケープするため追加サニタイズ不要と注記 |
| P-10 | 追記 | §マイグレーション設計ノート | forward-only ポリシーとバックアップ復旧方針を追記 |
| P-11 | 追記 | §マイグレーション設計ノート | PRAGMA foreign_keys = ON が CASCADE の前提条件であることを追記 |
| P-12 | 追記 | §Repository Traits — クロスリポジトリトランザクション | セレクト選択肢削除，ページ除外，DB 削除のトランザクション境界を網羅 |

### contracts/ipc-commands.md の変更

| レポートID | 変更種別 | 対象箇所 | 変更内容の要約 |
|-----------|---------|---------|--------------|
| P-04 | 追記 | §DTO Definitions | PageDto 拡張定義（databaseId: string \| null）を追加 |
| P-05 | 補完 | §reorder_properties | 全プロパティ ID の完全リストを要求し，サブセットはエラーとする旨を追記 |
| P-06 | 補完 | §clear_property_value | 値が存在しない場合は no-op（エラーなし）で正常終了する旨を追記 |

### 要判断項目（人間のレビューが必要）

なし。過剰設計・矛盾は review-report.md で検出されなかった。

---

## 次のステップ

1. `git diff` で変更内容を確認する
2. 各ドキュメント内の `<!-- checklist-apply: ... -->` コメントを確認し，内容に問題がなければコメントを残すか削除する
3. 満足したら `git commit` する
4. 必要に応じて `/checklist-review` を再実行し，カバレッジ率の改善を確認する
5. api.md チェックリストのレビュー（`/checklist-review` で対象を api.md に切り替え）を実施する
6. `/speckit.tasks` → `/speckit.analyze` → `/speckit.implement` へ進む
