# Technology Stack & Practices
<!-- Last rollup: 2026-03-23, 005-page-tree-nav -->

## 技術スタック

<!-- rollup: init, 2026-03-22 -->

### バックエンド

| 技術 | バージョン | 用途 |
|------|----------|------|
| Rust | 2024 edition (toolchain 1.94.0) | アプリケーション本体 |
| Tauri | 2 | デスクトップアプリ基盤・IPC |
| sqlx | 0.8 | SQLite アクセス・コンパイル時クエリ検証 |
| SQLite | bundled (WAL mode) | 永続化 |
| uuid | 1 (v7, serde) | UUIDv7 識別子生成 |
| chrono | 0.4 (serde) | 日時型 |
| thiserror | 2 | 宣言的エラー型定義 |
| serde | 1 (derive) | シリアライズ/デシリアライズ |
| serde_json | 1 | JSON 操作（View 条件の永続化等） <!-- rollup: 004-table-view-operations, 2026-03-22 --> |
| tokio | 1 (sync) | 非同期ランタイム + Mutex |

### フロントエンド

| 技術 | バージョン | 用途 |
|------|----------|------|
| TypeScript | ~5.8.3 | 型安全なフロントエンド |
| React | 19.1 | UI フレームワーク |
| @tauri-apps/api | ^2 | Tauri IPC クライアント |
| Sonner | ^2.0.7 | Toast 通知 |
| Tailwind CSS | v4 | ユーティリティファーストCSS（CSS Modules を全廃し移行） <!-- rollup: 005-page-tree-nav, 2026-03-23 --> |
| @tailwindcss/vite | — | Tailwind CSS の Vite プラグイン <!-- rollup: 005-page-tree-nav, 2026-03-23 --> |
| shadcn/ui | latest | UI コンポーネントライブラリ（Radix UI ベース，コピー方式） <!-- rollup: 005-page-tree-nav, 2026-03-23 --> |
| @atlaskit/pragmatic-drag-and-drop | — | ツリー D&D（autoScrollForElements 含む） <!-- rollup: 005-page-tree-nav, 2026-03-23 --> |
| lucide-react | — | アイコン（ページ/DB 区別等） <!-- rollup: 005-page-tree-nav, 2026-03-23 --> |
| clsx + tailwind-merge | — | cn() ヘルパー（shadcn/ui 標準） <!-- rollup: 005-page-tree-nav, 2026-03-23 --> |
| Vite | ^7.0.4 | ビルドツール・HMR |

### 開発ツール

| 技術 | 用途 |
|------|------|
| cargo-make | タスクランナー（`Makefile.toml`） |
| cargo-nextest | Rust テスト実行 |
| Biome ^2.4.8 | TypeScript lint + format |
| Vitest ^4.1.0 | フロントエンドテスト |
| Nix + direnv | 開発環境構築（WSL2） |

## 実装パターン

<!-- rollup: init, 2026-03-22 -->

| パターン | 適用場所 | 説明 |
|---------|---------|------|
| DDD | `domain/` 全体 | エンティティ・値オブジェクト・集約ルート・リポジトリトレイト |
| Repository | `domain/` → `infrastructure/` | ドメインがトレイトを定義，インフラが実装 |
| 値オブジェクト | PageId, PageTitle 等 | 生成時バリデーション，不変 |
| DTO 変換 | `ipc/dto.rs` | ドメイン型 → camelCase DTO |
| EditorSession | `domain::editor` | インメモリ状態管理，DB 非依存 |
| 即時保存 | プロパティ値 | セル編集は即時永続化 |
| 一括保存 | ブロックエディタ | トランザクション内で一括永続化 |
| CASCADE 削除 | FK 制約 | 親削除→子自動削除 |
| 自動保存（debounce） | `hooks/useAutoSave` | 500ms debounce + 指数バックオフリトライ（最大3回）。アンマウント時フラッシュ <!-- rollup: 005-page-tree-nav, 2026-03-23 --> |
| 楽観的更新 | `features/sidebar` | 操作後ローカル即時反映，エラー時ロールバック + 再取得 <!-- rollup: 005-page-tree-nav, 2026-03-23 --> |
| localStorage 状態永続化 | サイドバー | ツリー展開/折りたたみ，サイドバー表示/非表示，最後に開いたアイテムを localStorage で保持 <!-- rollup: 005-page-tree-nav, 2026-03-23 --> |
| ドメインサービス | `domain::page::hierarchy` | PageHierarchyService — 純粋ロジック（リポジトリ非依存），IPC がデータをロードして渡す <!-- rollup: 005-page-tree-nav, 2026-03-23 --> |
| Recursive CTE | `page_repository` | 祖先チェーン取得。深度安全上限付き <!-- rollup: 005-page-tree-nav, 2026-03-23 --> |

## テスト方針

- **TDD**: Red-Green-Refactor
- **ドメイン層**: 値オブジェクトバリデーション，EditorSession ロジック
- **インフラ層**: in-memory SQLite 統合テスト
- **フロントエンド**: Vitest + @testing-library/react
- **品質ゲート**: `cargo make qa`
- **カバレッジ**: `cargo make coverage`（Rust: cargo-llvm-cov, TS: Vitest v8）

### コードカバレッジ（2026-03-22, branch: 004-table-view-operations）

**Rust** — 235 tests, line coverage **75.68%**

| レイヤー | Line Coverage | 備考 |
|---------|--------------|------|
| domain/ | ~95% | entity, filter, sort, group, editor session |
| infrastructure/persistence/ | ~85% | in-memory SQLite 統合テスト |
| ipc/ | 0% | Tauri コマンドハンドラ（E2E テスト未実装） |
| lib.rs / main.rs | 0% | アプリエントリーポイント |

**TypeScript** — 15 tests, line coverage **1.71%**

| ファイル | Line Coverage | 備考 |
|---------|--------------|------|
| filterUtils.ts | 84.21% | ユニットテスト済 |
| React コンポーネント群 | 0% | E2E / integration test 未実装 |

## Clippy Lint 設定

```toml
[lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
unreachable = "deny"
```
