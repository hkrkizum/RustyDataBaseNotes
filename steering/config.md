# Steering Configuration

## Project Profile

- **scale**: medium
  <!-- auto-detected | 手動で上書き可能 -->
  <!-- small: 個人ツール・単一モジュール・CLI -->
  <!-- medium: 中規模アプリ・数モジュール -->
  <!-- large: マイクロサービス・大規模システム -->
  <!-- enterprise: 多チーム・数十サービス -->
- **detected_basis**: ディレクトリ 7個，ソースファイル 65個，specs/ に 3 feature

## Budget

steering/ 全体の合計行数バジェットと，各ファイルへの配分率。
バジェットはプロジェクト規模に応じた目安であり，ハードリミットではない。
超過時は警告を表示し，集約を提案するが自動では行わない。

- **total_budget**: 1000
- **product_pct**: 20
- **architecture_pct**: 35
- **tech_pct**: 20
- **current_state_pct**: 25

<!-- 配分率の合計は 100 にすること -->
<!-- 例: API中心のプロジェクトなら architecture_pct を上げ，product_pct を下げる -->
<!-- 例: 技術スタックが複雑なら tech_pct を上げる -->

## Lifecycle

- **merged_pointer_threshold**: 10
  <!-- specs/ 内の ARCHIVED.md ポインタがこの数を超えたら整理を提案 -->

## Rollup Behavior

- **auto_commit**: false
  <!-- true にすると rollup 完了後に自動で git commit する -->
- **language**: ja
  <!-- steering/ ドキュメントの記述言語 -->
