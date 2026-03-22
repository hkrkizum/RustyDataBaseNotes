# Rollup Log

## Rollup: 004-table-view-operations (2026-03-22)

### steering/ の更新内容

| ファイル | 変更種別 | 変更内容の要約 |
|---------|---------|--------------|
| product.md | 追記 | ソート・フィルタ・グルーピング・ビュー永続化の4ユースケースを追加，View ドメイン説明を更新 |
| architecture.md | 更新 | domain::view モジュール追加，Database→View 関係追加，テーブルデータ取得フロー追加，テーブル数を6に更新 |
| tech.md | 追記 | serde_json を技術スタックに追加 |
| current-state.md | 更新 | 004 の6機能を追加，既知制約を整理（ソート等の制約事項を追加），直近変更を更新 |

### specs/ のライフサイクル遷移

| spec | 遷移 | specs/ | archive/ |
|------|------|--------|----------|
| 004-table-view-operations | Active → Merged | ARCHIVED.md（ポインタ） | 全成果物を保管（10ファイル + 2ディレクトリ） |

### steering/ バジェット状況

| ファイル | 行数 | バジェット | 使用率 | 状態 |
|---------|------|----------|--------|------|
| product.md | 46 | 200 | 23% | OK |
| architecture.md | 117 | 350 | 33% | OK |
| tech.md | 93 | 200 | 47% | OK |
| current-state.md | 59 | 250 | 24% | OK |
| **合計** | **315** | **1000** | **32%** | OK |

---

## Init (2026-03-22)

steering/ を初期構築。情報源:

- specs/001-page-persistence（spec.md, plan.md）
- specs/002-block-editor（spec.md, plan.md）
- specs/003-database-properties（spec.md, plan.md, data-model.md）
- .specify/memory/constitution.md v1.5.0
- コードベースのディレクトリ構造，Cargo.toml，package.json

### 生成されたファイル

| ファイル | 行数 | バジェット | 使用率 |
|---------|------|----------|--------|
| config.md | — | — | 設定ファイル |
| product.md | 40 | 200 | 20% |
| architecture.md | 105 | 350 | 30% |
| tech.md | 80 | 200 | 40% |
| current-state.md | 40 | 250 | 16% |
