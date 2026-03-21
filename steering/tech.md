# Technology Stack & Practices
<!-- Last rollup: 2026-03-22 -->

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
| tokio | 1 (sync) | 非同期ランタイム + Mutex |

### フロントエンド

| 技術 | バージョン | 用途 |
|------|----------|------|
| TypeScript | ~5.8.3 | 型安全なフロントエンド |
| React | 19.1 | UI フレームワーク |
| @tauri-apps/api | ^2 | Tauri IPC クライアント |
| Sonner | ^2.0.7 | Toast 通知 |
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

## テスト方針

- **TDD**: Red-Green-Refactor
- **ドメイン層**: 値オブジェクトバリデーション，EditorSession ロジック
- **インフラ層**: in-memory SQLite 統合テスト
- **フロントエンド**: Vitest + @testing-library/react
- **品質ゲート**: `cargo make qa`

## Clippy Lint 設定

```toml
[lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
unreachable = "deny"
```
