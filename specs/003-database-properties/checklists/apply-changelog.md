# Checklist Apply Changelog: プロパティシステムとデータベース概念の導入
**実行日時**: 2026-03-21（API チェックリスト対応）
**入力**: review-report.md（api.md チェックリスト対象）
**モード**: 差分更新（非破壊）

---

## 変更統計
- spec.md: 1箇所 補完（G-01, G-04） / 2箇所 追記（G-02, G-03）
- contracts/ipc-commands.md: 8箇所 補完（P-02, P-03, P-04, P-05, P-08, P-09, P-13, P-14, P-16, P-17, P-18, P-20） / 5箇所 追記（P-01, P-06, P-07, P-10, P-11, P-12, P-15）
- data-model.md: 2箇所 補完（G-03, P-19） / 1箇所 追記（P-07, P-10 エラーバリアント）
- 新規作成ファイル: なし
- 簡素化提案（要判断）: 0箇所

---

## 変更詳細

### spec.md の変更

| レポートID | 変更種別 | 対象箇所 | 変更内容の要約 |
|-----------|---------|---------|--------------|
| G-01 | 補完 | §FR-006 | 「完全削除は既存の `delete_page` コマンドを再利用し，新規 IPC コマンドは追加しない」を追記 |
| G-02 | 追記 | §Assumptions (末尾) | 「バッチ操作は本スコープ対象外，`set_property_value` で1ページ×1プロパティ単位」を追記 |
| G-03 | 追記 | §Assumptions (末尾) | 「テキスト型の空文字列は有効な値，値のクリアには `clear_property_value` を使用」を追記 |
| G-04 | 補完 | §CC-004 | 「フロントエンドは楽観的事前バリデーションを行ってよい（SHOULD），バックエンドが最終的な権威」を追記 |

### contracts/ipc-commands.md の変更

| レポートID | 変更種別 | 対象箇所 | 変更内容の要約 |
|-----------|---------|---------|--------------|
| P-01 | 追記 | §DTO Definitions | `CommandError { kind, message }` の TypeScript interface を追加 |
| P-02, P-03 | 補完 | §DTO Definitions (PropertyConfigDto) | serde internally tagged enum との対応説明と JSON ワイヤーフォーマット例を追記。型ごとの必須フィールドをコメントで明記 |
| P-04 | 補完 | §DTO Definitions (全タイムスタンプ) | "ISO 8601" → "RFC 3339 / UTC" に用語統一 |
| P-05 | 補完 | §add_property | config 省略時は `PropertyDto.config` が `null` を返す旨を追記 |
| P-06 | 追記 | §DTO Definitions (TableRowDto) | 未入力プロパティはキー欠落（Record に含まれない）と明記 |
| P-07 | 追記 | §set_property_value | `pageNotInDatabase` エラーを追加（クロスデータベース検証） |
| P-08 | 補完 | §remove_page_from_database | standalone ページへの呼び出しは冪等に成功（no-op）と追記 |
| P-09 | 補完 | §list_standalone_pages | 「作成日時降順で返す」を追記 |
| P-10 | 追記 | §set_property_value / §Error Kind Extensions | `invalidDate` エラーを追加（RFC 3339 パース失敗） |
| P-11, P-15 | 追記 | §プリアンブル | serde シリアライズ規則と snake_case → camelCase 変換ルールを追記 |
| P-12 | 追記 | §プリアンブル | 同期的 Request→Response パターンの明記 |
| P-13 | 補完 | §Error Kind Extensions | message フィールドはデバッグ用英語文字列，ユーザー向け表示は Frontend が生成する旨を追記 |
| P-14, P-18 | 補完 | §update_property_config / §add_property | propertyType と config の型不整合時は `invalidConfig` を返す旨を追記 |
| P-16 | 補完 | §Error Kind Extensions | エラー kind は camelCase で命名するルールを追記 |
| P-17 | 補完 | §delete_property | CASCADE で自動削除される旨と削除件数を返さない設計を追記 |
| P-20 | 補完 | §DTO Definitions (PageDto) | 既存フィールド vs 新規追加フィールドの差分をコメントで明記 |

### data-model.md の変更

| レポートID | 変更種別 | 対象箇所 | 変更内容の要約 |
|-----------|---------|---------|--------------|
| G-03 | 補完 | §PropertyValue バリデーション規則 (Text) | 空文字列は有効な値として保存，クリアには `clear_property_value` を使用する旨を追記 |
| P-07, P-10 | 追記 | §PropertyValueError | `InvalidDate` と `PageNotInDatabase` バリアントを追加 |
| P-19 | 補完 | §Property フィールド表 + バリデーション規則 | PropertyName の制約に「トリム後」を追記 |

### 要判断項目（人間のレビューが必要）

なし。過剰設計・矛盾は review-report.md で検出されなかった。

---

## 次のステップ

1. `git diff` で変更内容を確認する
2. 各ドキュメント内の `<!-- checklist-apply: ... -->` コメントを確認し，内容に問題がなければそのまま残す
3. 満足したら `git commit` する
4. `/checklist-review api` を再実行し，カバレッジ率の改善を確認する
5. `/speckit.tasks` → `/speckit.analyze` → `/speckit.implement` へ進む
