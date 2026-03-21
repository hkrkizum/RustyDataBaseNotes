# Research: テーブルビュー操作拡張

**Feature**: 004-table-view-operations
**Date**: 2026-03-22

## RQ-1: ビュー設定の永続化形式（JSON blob vs 正規化テーブル）

### Decision: JSON blob（views テーブル内の JSON カラム）

### Rationale
- ビュー設定は常に一括で読み書きされる（個別条件のクエリは不要）
- 既存の PropertyConfig が JSON カラムで格納されるパターンと一致する
- 正規化テーブル（sort_conditions, filter_conditions 等）は結合クエリが増え，
  1 データベース = 1 ビューの現スコープでは過剰な正規化となる
- JSON blob でも serde による型安全なシリアライズ / デシリアライズが可能
- プロパティ削除時の孤立条件除去はアプリケーション層で対応（JSON パース後にフィルタリング）

### Alternatives Considered
- **正規化テーブル**: FK CASCADE でプロパティ削除時の自動クリーンアップが可能だが，
  テーブル数が 4〜5 増え，マイグレーションと読み込みクエリが複雑化する。
  将来複数ビューに拡張する際にも JSON blob のままスケールする
- **フロントエンド localStorage**: Tauri アプリのデータ一元管理に反し，
  バックアップ・移行の対象外となるため不採用

---

## RQ-2: ソート・フィルタ・グルーピングの計算場所

### Decision: バックエンド（Rust）で全処理を実行

### Rationale
- 既存の `get_table_data` コマンドが全ページ + プロパティ値を一括で読み込む構造を持つ
- Rust での Vec ソート・フィルタリングは 1,000 件規模で十分高速（μs〜ms オーダー）
- ドメインロジック（型別比較，null ハンドリング等）をバックエンドに集約することで
  テスト容易性が向上し，フロントエンドはプレゼンテーションに専念できる
- フロントエンドでの再計算を避けることで React の再レンダリング負荷を軽減

### Alternatives Considered
- **フロントエンド（TypeScript）**: 小規模データでは動作するが，ドメインロジックが
  フロントエンドに漏洩し，Rust 側のテストで検証できなくなる。Constitution III 違反のリスク
- **SQL 動的クエリ**: ORDER BY / WHERE を動的生成する方法。sqlx のコンパイル時チェックと
  相性が悪く，SQL インジェクションリスクも増す。JSON 格納のプロパティ値に対する
  型別ソートは SQL では複雑になる

---

## RQ-3: get_table_data の拡張方針

### Decision: 既存 `get_table_data` を拡張し，自動的にデフォルトビューの設定を適用

### Rationale
- 現スコープでは 1 データベース = 1 ビューのため，ビュー ID を明示的に渡す必要がない
- フロントエンドの既存呼び出し（`invoke("get_table_data", { databaseId })`）を維持でき，
  移行コストが最小
- レスポンス型を拡張: `TableDataDto` に `view` フィールドを追加し，
  現在のビュー設定をフロントエンドに伝達する

### Alternatives Considered
- **新規 `get_view_data` コマンド**: ビュー ID を明示的に受け取る設計。
  将来の複数ビュー対応時に必要になるが，現スコープでは不要な抽象。
  複数ビュー対応時にコマンドを追加すればよい（YAGNI）

---

## RQ-4: マイグレーションでの既存データベース対応

### Decision: SQL マイグレーション内で既存全データベースのデフォルトビューを一括生成

### Rationale
- 仕様書で「マイグレーション時に既存全データベースのデフォルトビューを一括生成」と明記
- `INSERT INTO views ... SELECT` で既存 databases の ID を参照し，デフォルト設定の
  ビューを生成する
- アプリケーション起動時のマイグレーション実行で自動的に適用される
- ビュー ID は UUIDv7 だが，マイグレーション SQL では SQLite の `lower(hex(randomblob(16)))`
  等で代替 UUID を生成する（UUIDv7 のタイムスタンプ部は必須ではない）

### Alternatives Considered
- **アプリケーション層での遅延生成**: データベース読み込み時にビューが存在しなければ生成。
  ロジックが分散し，レースコンディションのリスクがある。マイグレーションで一括生成が単純

---

## RQ-5: グルーピング時のレスポンス構造

### Decision: `TableDataDto` のレスポンスにグループ情報を追加し，行の順序でグループを表現

### Rationale
- グルーピング有効時のレスポンス: rows はグループ順 → グループ内ソート順で返却
- 追加フィールド `groups: Option<Vec<GroupInfo>>` で各グループのメタデータを伝達
  - `GroupInfo { value: Option<String>, display_value: String, count: usize, is_collapsed: bool }`
- フロントエンドはグループ情報を使って行にグループヘッダーを挿入し，
  折りたたみ状態を制御する
- グルーピングなしの場合は `groups: null` で既存の動作を維持

### Alternatives Considered
- **ネストした構造 `groups: Vec<{ header, rows }>`**: フロントエンドの行レンダリングが
  2 重ループになり，仮想化スクロール等との相性が悪い
- **フロントエンドでグルーピング**: ドメインロジックの分散を避けるため不採用

---

## RQ-6: プロパティ型別のソート比較戦略

### Decision: 各プロパティ型に対応する比較関数を Rust で実装

### Rationale

| プロパティ型 | 比較方法 | null 位置 |
|---|---|---|
| テキスト | Unicode コードポイント順（`str::cmp`），case-sensitive | 昇順末尾，降順先頭 |
| 数値 | `f64::total_cmp` | 昇順末尾，降順先頭 |
| 日付 | `DateTime<Utc>` の比較（date モードは 00:00:00 UTC として比較） | 昇順末尾，降順先頭 |
| セレクト | 選択肢の `position` 値順 | 昇順末尾，降順先頭 |
| チェックボックス | `false < true`（昇順: 未チェック先） | 昇順末尾，降順先頭 |

- `Ordering` を返す比較関数を各型で実装し，`Vec::sort_by` のクロージャで合成
- 複数ソートは第 1 キーの結果が `Equal` の場合に第 2 キーで比較する連鎖パターン

---

## RQ-7: フィルタ演算子と型の整合性バリデーション

### Decision: バックエンドでフィルタ条件の設定時にバリデーションを実施

### Rationale
- フィルタ条件追加時に PropertyType と演算子の組み合わせを検証
- 比較値の型チェック（例: 数値フィルタに文字列が渡された場合はエラー）
- バリデーションエラーは `ViewError::InvalidFilterOperator` 等で返却
- フロントエンドでも演算子リストをプロパティ型に応じて制限するが，
  バックエンドが最終的な権威

### フィルタ評価の型別実装

| プロパティ型 | 演算子 | 比較値の型 |
|---|---|---|
| テキスト | Equals, NotEquals, Contains, NotContains, IsEmpty, IsNotEmpty | String (case-insensitive) |
| 数値 | Equals, NotEquals, GreaterThan, LessThan, GreaterOrEqual, LessOrEqual, IsEmpty, IsNotEmpty | f64 |
| 日付 | Equals, Before, After, IsEmpty, IsNotEmpty | String (ISO 8601) |
| セレクト | Is, IsNot, IsEmpty, IsNotEmpty | String (option value) |
| チェックボックス | IsChecked, IsUnchecked | なし（演算子のみ） |

---

## RQ-8: グループ折りたたみ状態の永続化

### Decision: `collapsed_groups` を View エンティティの JSON フィールドとして格納

### Rationale
- 折りたたみ状態はグループ値（プロパティ値の文字列表現）の集合として表現
- `HashSet<String>` で管理し，JSON 配列としてシリアライズ
- グルーピングプロパティ変更時に collapsed_groups をクリアする
- プロパティ値の変更でグループが増減しても，collapsed 状態は値ベースで維持される

### Alternatives Considered
- **フロントエンドの sessionStorage**: アプリ再起動で失われるため不採用
  （仕様 FR-007 で永続化が必須）
- **別テーブル**: 1 ビューに対する少量のデータであり，JSON で十分
