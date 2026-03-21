# Checklist Review Report: プロパティシステムとデータベース概念の導入

**レビュー日時**: 2026-03-21
**対象チェックリスト**: api.md
**レビュー結果サマリー**:
- ✅ Covered: 12 項目
- ⚠️ Partial: 12 項目
- ❌ Gap: 11 項目
- 🔀 Misplaced: 0 項目
- **カバレッジ率**: 34%（12/35）

---

## 仕様側の問題（spec.md で対応すべき項目）

機能要件・ユーザー体験・ビジネスロジックに関するギャップ。
spec.md の修正で解決すべき項目を列挙する。

| ID | チェック項目 | 判定 | 推奨アクション |
|----|------------|------|--------------|
| G-01 | CHK001: US8「完全に削除」の IPC コマンドが未定義 | Gap | spec.md FR-006 に「完全削除は既存の `delete_page` コマンドを再利用する」旨を明記。または contracts に注記を追加 |
| G-02 | CHK005: バッチ操作のスコープ除外が未明記 | Gap | spec.md §Assumptions に「バッチ／バルク操作は本スコープ対象外」を追記 |
| G-03 | CHK024: テキスト型の空文字列の意味が未定義 | Gap | spec.md §Assumptions または data-model.md §PropertyValue の Text バリデーションに「空文字列は有効な値として保存する。値のクリアには `clear_property_value` を使用する」等の方針を追記 |
| G-04 | CHK029: フロントエンド事前バリデーションの可否 | Partial | spec.md CC-004 に「フロントエンドは UX 向上のため楽観的な事前バリデーションを行ってよい（SHOULD）。ただしバックエンドが最終的な権威とする」等の方針を追記 |

## 計画側の問題（plan.md / contracts / data-model.md で対応すべき項目）

技術的な設計・アーキテクチャ・非機能要件に関するギャップ。
plan.md または関連する技術文書の修正で解決すべき項目を列挙する。

| ID | チェック項目 | 判定 | 推奨アクション |
|----|------------|------|--------------|
| P-01 | CHK004: `CommandError` の TS interface 未定義 | Gap | contracts §DTO Definitions に `interface CommandError { kind: string; message: string }` を追加 |
| P-02 | CHK008: `PropertyConfigDto` の型ごとの制約未記載 | Partial | contracts §DTO Definitions の `PropertyConfigDto` を discriminated union に変更するか，data-model.md の JSON ワイヤーフォーマットとの対応表を追加 |
| P-03 | CHK010: TS DTO と Rust serde 形式の不整合 | Partial | contracts に「Tauri IPC は serde の internally tagged 形式で JSON を送受信する。TS 側は以下の型でマッピングする」旨の説明とマッピング表を追加 |
| P-04 | CHK012: ISO 8601 / RFC 3339 用語の不整合 | Partial | contracts と data-model.md で用語を統一（推奨: "RFC 3339 / UTC"）。タイムスタンプ精度（秒 or ミリ秒）を明記 |
| P-05 | CHK014: config 省略時のレスポンス仕様 | Gap | contracts §add_property に「config 省略時，PropertyDto.config は null を返す」または「型に応じたデフォルト config を返す」を明記 |
| P-06 | CHK016: 未設定プロパティ値の DTO 表現 | Gap | contracts §DTO Definitions の `TableRowDto.values` に「未入力のプロパティはキーが欠落する（Record に含まれない）」または「全フィールド null の PropertyValueDto が含まれる」を明記 |
| P-07 | CHK019: クロスデータベース値設定の検証 | Gap | contracts §set_property_value のエラーリストに `pageNotInDatabase`（ページが対象プロパティのデータベースに属していない）を追加。または data-model.md にクロスデータベース検証の方針を追記 |
| P-08 | CHK021: スタンドアロンページへの `remove_page_from_database` | Gap | contracts §remove_page_from_database に「database_id が NULL のページに対しては冪等に成功する（no-op）」または専用エラー kind を追加 |
| P-09 | CHK023: `list_standalone_pages` のソート順 | Gap | contracts §list_standalone_pages に「作成日時降順」等のソート順を明記 |
| P-10 | CHK026: 不正日付文字列のエラー kind | Gap | contracts §set_property_value のエラーリストに `invalidDate`（日付文字列のパース失敗）を追加 |
| P-11 | CHK033: serde ↔ TS マッピング前提の文書化 | Gap | contracts プリアンブルまたは plan.md に「Tauri IPC は Rust の serde シリアライズ結果をそのまま JSON として送受信する。TS 型はこの JSON 構造に対応する discriminated union として定義する」を追記 |
| P-12 | CHK035: 同期的 IPC 前提の文書化 | Gap | contracts プリアンブルに「すべての IPC コマンドは同期的な Request→Response パターンで動作する（ストリーミングやイベント通知は使用しない）」を追記 |
| P-13 | CHK003: エラーメッセージ形式 | Partial | contracts §Error Kind Extensions に「message フィールドはデバッグ目的の英語文字列とし，ユーザー向け表示文はフロントエンドが kind に基づいて生成する」等の方針を追記 |
| P-14 | CHK006: 型と config の不整合時の挙動 | Partial | contracts §update_property_config に「propertyType と異なる型の config を送信した場合は `invalidConfig` エラーを返す」を追記 |
| P-15 | CHK013: snake_case → camelCase 変換ルール | Partial | contracts プリアンブルに「Rust の snake_case フィールド名は TS DTO では camelCase に変換する（serde の `#[serde(rename_all = "camelCase")]`）」を追記 |
| P-16 | CHK017: エラー kind 命名規則 | Partial | contracts §Error Kind Extensions に「エラー kind は camelCase で命名する」を追記 |
| P-17 | CHK022: `delete_property` の CASCADE 件数非通知 | Partial | contracts §delete_property に「関連する PropertyValue は CASCADE で自動削除される。削除件数はレスポンスに含まない（void）」を追記 |
| P-18 | CHK031: propertyType と config の型不整合 | Partial | contracts §add_property に「propertyType と config の型が不整合の場合は `invalidConfig` エラーを返す」を追記 |
| P-19 | CHK032: PropertyName のトリミング規則 | Partial | data-model.md §Property の PropertyName バリデーションに「トリム後」の記載を追加（DatabaseTitle と同様） |
| P-20 | CHK034: 既存 PageDto との差分明示 | Partial | contracts §DTO Definitions の PageDto コメントに「既存フィールド: id, title, createdAt, updatedAt / 新規追加: databaseId」を明記 |

## 配置ミス（Misplaced 項目）

該当なし。

## 意図的な除外の確認

以下の Gap 項目について，意図的に対象外としている場合は理由を記録してください。
（人間が判断して記入するセクション）

| ID | チェック項目 | 除外理由（人間が記入） |
|----|------------|---------------------|
| G-01 | CHK001: US8 完全削除の IPC コマンド | |
| G-02 | CHK005: バッチ操作のスコープ除外 | |
| G-03 | CHK024: テキスト型空文字列の意味 | |
| P-01 | CHK004: CommandError の TS interface | |
| P-05 | CHK014: config 省略時のレスポンス | |
| P-06 | CHK016: 未設定値の DTO 表現 | |
| P-07 | CHK019: クロスデータベース値設定 | |
| P-08 | CHK021: standalone ページの除外操作 | |
| P-09 | CHK023: list_standalone_pages ソート順 | |
| P-10 | CHK026: 不正日付エラー kind | |
| P-11 | CHK033: serde ↔ TS マッピング前提 | |
| P-12 | CHK035: 同期的 IPC 前提 | |

---

## Constitution 準拠チェック

### カバーされている原則

| Constitution 原則 | 対応するチェック項目 |
|------------------|-------------------|
| Article I: Local-First Product Integrity | CHK005（バッチ操作不要 = ローカル完結で十分な性能），CHK009（全プロパティ完全リスト = データ整合性） |
| Article II: Domain-Faithful Information Model | CHK011（セレクト値の ID 格納 = 参照整合性），CHK016（未設定値の表現 = ドメイン概念の忠実な表現） |
| Article III: Typed Boundaries and DDD | CHK004（CommandError 型定義），CHK007（TypeMismatch エラー），CHK008（PropertyConfigDto），CHK010（シリアライズ形式），CHK013（命名規則），CHK030（バリデーション規則の型別列挙），CHK033（serde マッピング） |
| Article V: Safe Rust, SOLID, Maintainability | CHK005（バッチ操作未導入 = YAGNI 準拠），CHK022（void レスポンス = 簡素な設計） |
| Article VII: Defensive Error Handling | CHK003（エラーメッセージ形式），CHK006（型不整合エラー），CHK015（エラー kind の論理的整合），CHK019（クロスDB 検証），CHK021（standalone ページ），CHK026（不正日付エラー），CHK028（選択肢削除フロー） |

### カバーされていない原則

以下の constitution 原則に対応するチェック項目が不足しています:

- **Article IV（Test-First Delivery）**: API チェックリストにテスト戦略に関する検証項目がない。IPC コマンドの統合テスト網羅性，エラーパスのテストカバレッジ等の項目を検討すべき。ただし，テスト専用チェックリストの管轄であり API チェックリストのスコープ外とも言える。**対応不要。**
- **Article VI（Rust Documentation Standards）**: API 契約のドキュメント品質（DTO の doc コメント，コマンドハンドラの doc コメント）に関する検証項目がない。同様にスコープ外の可能性が高い。**対応不要。**

### 矛盾・過剰設計の指摘

| チェック項目 | 関連する原則 | 指摘内容 |
|------------|------------|---------|
| （該当なし） | Article V（Simplicity / YAGNI） | API チェックリストに過剰設計を示す項目は検出されなかった。CHK005（バッチ操作）が正しくスコープ外とされている点は YAGNI に適合。CHK035（同期 IPC 前提）も Tauri の標準パターンを踏襲しており簡素性を維持している |
