# Research: 003-database-properties

**Date**: 2026-03-21
**Status**: Complete — すべての NEEDS CLARIFICATION を解決済み

## R-001: PropertyValue の格納戦略

**Decision**: 判別カラム方式（discriminated columns）

`property_values` テーブルに型別カラム（`text_value`，`number_value`，`date_value`，
`boolean_value`）を持ち，プロパティ型に応じた1カラムのみを使用する。

**Rationale**:
- SQL レベルでのソート・フィルタが将来可能（後続スコープでテーブルビューのソート・
  フィルタ機能を予定）
- 数値型は `REAL` で格納し，SQLite ネイティブの比較演算が使える
- 各カラムは `NULL` 許容とし，対象外の型のカラムは `NULL` のまま
- セレクト型は `text_value` に選択肢の値を文字列として格納

**Alternatives considered**:
- 単一 JSON カラム: 柔軟だが SQL クエリ不可，型安全性が低い
- 単一 TEXT カラム: パース処理が煩雑，数値比較が文字列ソートになる

## R-002: Page-Database 関係の実装方法

**Decision**: `pages` テーブルに `database_id` nullable FK カラムを追加

**Rationale**:
- 仕様の「1ページ = 最大1データベース」制約を UNIQUE 制約なしで自然に表現
  （1カラムに最大1つの値しか入らない）
- スタンドアロンページは `database_id = NULL` で表現
- JOIN テーブル方式より単純で，追加インデックス1本で済む

**Alternatives considered**:
- `database_pages` JOIN テーブル: 多対多が不要なので過剰。UNIQUE 制約で1:N に
  制限するのは間接的

## R-003: セレクト型選択肢の格納方法

**Decision**: `properties` テーブルの `config` カラムに JSON として格納

`config` カラム（TEXT 型）にプロパティ型固有の設定を JSON で保存する:
- セレクト型: `{"options": [{"id": "...", "value": "未着手"}, ...]}`
- 日付型: `{"mode": "date"}` または `{"mode": "datetime"}`
- その他の型: `null` または `{}`

**Rationale**:
- 選択肢はプロパティスキーマの一部であり，独立したエンティティではない
- 別テーブルにすると JOIN が増えて取得クエリが複雑化
- 選択肢数の上限は100であり，JSON パースのコストは無視できる
- Rust 側で `PropertyConfig` enum（serde タグ付き）として型安全にデシリアライズ

**Alternatives considered**:
- `select_options` テーブル: 選択肢の独立クエリが不要なので過剰
- プロパティテーブルに `options` 専用カラム: 日付モード等の他の設定に対応できない

## R-004: プロパティ表示順の管理

**Decision**: `properties` テーブルに `position` INTEGER カラム

既存の `blocks.position` パターンに合わせ，0始まりの整数で表示順を管理する。
並び替え時はトランザクション内で対象プロパティの position を更新。

**Rationale**:
- 既存パターン（Block の position）との一貫性
- 単純な整数比較でソート可能
- 並び替え操作はプロパティ数上限50で十分軽量

**Alternatives considered**:
- 浮動小数点ギャップ方式: 上限50では不要な複雑さ
- リンクリスト方式: 全件取得が前提のため不適切

## R-005: セレクト選択肢の ID 方式

**Decision**: 各選択肢に UUIDv7 の ID を付与

選択肢を `{id, value}` のペアで管理する。`property_values.text_value` には
選択肢の `id`（UUID 文字列）を格納する。

**Rationale**:
- 選択肢の表示名変更時に property_values の更新が不要（ID は不変）
- 選択肢の削除時は ID で一括検索・リセットが可能
- 表示名の重複チェックは value フィールドで行う

**Alternatives considered**:
- 表示名をそのまま値として格納: 名前変更時に全 property_values の更新が必要

## R-006: データベース削除時のカスケード処理

**Decision**: アプリケーションレベルのトランザクションで段階的削除

1つのトランザクション内で:
1. `property_values` を対象プロパティ群の ID で削除
2. `properties` を `database_id` で削除
3. `pages.database_id` を `NULL` に更新（ページ自体は保持）
4. `databases` レコードを削除

**Rationale**:
- 仕様: ページ自体とブロックは保持（FR-011）。SQL CASCADE では
  ページまで消えてしまうため使えない
- pages.database_id の NULL 化はアプリケーション側で明示的に行う必要がある
- 原子性はトランザクションで保証

**Alternatives considered**:
- SQL ON DELETE CASCADE: pages を消したくないため不適用
- ON DELETE SET NULL for pages.database_id + CASCADE for others:
  properties → property_values は CASCADE 可能だが，
  property_values は page_id と property_id 両方の FK を持つため，
  統一的にアプリケーション側で管理する方が明確

## R-007: フロントエンド テーブルビュー実装方針

**Decision**: カスタム React コンポーネントで実装。外部テーブルライブラリは不使用

**Rationale**:
- 現時点の要件はシンプル（ソート・フィルタ・グルーピングは後続スコープ）
- 外部ライブラリは学習コストと依存の増加を伴う
- 1,000ページでのスムーズスクロールは CSS `overflow-y: auto` + 必要に応じて
  仮想化で対応（計測後に判断）
- プロパティ型ごとのインラインエディタ（テキスト入力，数値入力，日付ピッカー，
  セレクトドロップダウン，チェックボックス）はカスタムセルコンポーネントで実装

**Alternatives considered**:
- TanStack Table: 高機能だが現スコープでは過剰。後続スコープで再検討
- AG Grid: 商用ライセンスが必要，個人プロジェクトには不適

## R-008: PropertyValue の null 表現

**Decision**: 値なし = すべての型別カラムが `NULL`

新規ページ追加時の初期状態:
- テキスト: `text_value = NULL`（空文字列ではない）
- 数値: `number_value = NULL`
- 日付: `date_value = NULL`
- セレクト: `text_value = NULL`（未選択）
- チェックボックス: `boolean_value = 0`（唯一のデフォルト値あり — false）

**Rationale**:
- `NULL` は「未入力」を明確に表現
- チェックボックスのみ仕様で「未チェック（false）」が初期値と定義されている
- フロントエンドでは `null` として受け取り，各型に応じた空表示を行う

## R-009: IPC コマンド設計 — テーブルデータ取得

**Decision**: テーブルビュー用にバルク取得コマンドを1つ定義

`get_table_data(database_id)` → `TableDataDto` を返す:
```
TableDataDto {
  database: DatabaseDto,
  properties: Vec<PropertyDto>,
  rows: Vec<TableRowDto>,
}

TableRowDto {
  page: PageDto,
  values: HashMap<String, PropertyValueDto>,  // property_id → value
}
```

**Rationale**:
- テーブルビューの初期表示に必要なデータを1回の IPC で取得
- N+1 問題を回避（ページ数 × プロパティ数の個別取得を防止）
- プロパティ値は property_id をキーとした HashMap で返し，フロントエンドでの
  列マッピングを容易にする

**Alternatives considered**:
- 個別コマンド分離（get_database + list_properties + get_values_for_page × N）:
  IPC オーバーヘッドが大きく，CC-003 のパフォーマンス目標に抵触するリスク
