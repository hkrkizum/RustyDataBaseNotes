# Research: ページの永続化（最小縦断スライス）

**Feature Branch**: `001-page-persistence`
**Date**: 2026-03-21

## Decision 1: SQLite ドライバ

**Decision**: sqlx 0.8（runtime-tokio, sqlite, macros, migrate, chrono, uuid features）

**Rationale**:
- **コンパイル時クエリ検証** — `query!()` / `query_as!()` マクロにより SQL の構文・型をビルド時に検証。スキーマが複雑化する将来（blocks, databases, views, properties）に最大の防御線となる
- Tauri 2 は tokio 内蔵 — 非同期は追加コストではなく既存基盤の活用
- `SqlitePool` は Send + Sync + Clone — Mutex 不要で Tauri State に直接登録可能
- `sqlite` feature で SQLite をバイナリに静的リンク（bundled） — システム依存ゼロ
- マイグレーション組み込み（`sqlx::migrate!()`） — 別クレート不要
- chrono, uuid の型マッピングを feature で直接サポート
- 将来 rusqlite（同期）から移行するコストが高い（API・同期/非同期の断絶で全コマンド書き直し）ため，最初から sqlx を採用

**Alternatives considered**:
- **rusqlite**: 同期 API でシンプルだが，コンパイル時クエリ検証がなく，スキーマ複雑化時に SQL 文字列のランタイムエラーリスクが高い。後から sqlx への移行コストも高い
- **diesel**: ORM + DSL の学習コストが高く，複雑な SQLite クエリ（JSON 関数，ウィンドウ関数等）で DSL の制約に当たる
- **SeaORM**: sqlx 上の抽象層であり，このプロジェクトの規模では抽象のコストが利点を上回る
- **tauri-plugin-sql**: Rust 側からの DB アクセス API がなく，ドメインロジックを TypeScript に置くことになり境界違反

```toml
sqlx = { version = "0.8", features = [
    "runtime-tokio",
    "sqlite",
    "macros",
    "migrate",
    "chrono",
    "uuid",
] }
```

## Decision 2: マイグレーション

**Decision**: sqlx 組み込みマイグレーション（`sqlx::migrate!()` マクロ）

**Rationale**:
- sqlx に統合されており追加クレート不要
- `sqlx::migrate!()` で SQL ファイルをコンパイル時にバイナリへ埋め込み
- `_sqlx_migrations` テーブルで適用済みバージョンを追跡（チェックサム検証付き）
- マイグレーションファイルは手動作成可能（sqlx-cli は必須ではない）
- ディレクトリ構成: `migrations/{version}_{description}.sql`
- `build.rs` で `cargo:rerun-if-changed=migrations` を設定し，SQL 変更時の自動再コンパイルを保証

**開発時のコンパイル時検証**:
- `DATABASE_URL` 環境変数（または `.env` ファイル）でビルド時の DB パスを指定
- オフラインモード: `cargo sqlx prepare` で `.sqlx/` ディレクトリにクエリメタデータを保存し，CI では DB 不要でビルド可能
- `.sqlx/` ディレクトリはバージョン管理にコミットする

## Decision 3: UUID 生成

**Decision**: uuid クレート v1（`v7` + `serde` features）

**Rationale**:
- `Uuid::now_v7()` は不可謬（infallible） — `Result` も `unwrap()` も不要
- UUIDv7 は RFC 9562 標準，時刻順ソート可能（ページの作成順表示に最適）
- `serde` feature で Serialize/Deserialize を自動導出
- SQLite には TEXT 形式（36 文字ハイフン付き）で保存 — 辞書順 = 時系列順
- テスト時は `Uuid::new_v7(ts)` で決定的なタイムスタンプを指定可能

**Alternatives considered**:
- **ulid**: UUID 非互換，DB ツーリングとの統合が弱い
- **uuid7**: ニッチクレート，独自型を返す（`uuid::Uuid` との互換性なし）
- **typeid**: 仕様が若く採用実績が限定的

```toml
uuid = { version = "1", features = ["v7", "serde"] }
```

## Decision 4: エラーハンドリング

**Decision**: thiserror + 構造化シリアライズ（境界コンテキスト別エラー型）

**Rationale**:
- Tauri 2 は `#[tauri::command]` のエラー型に `serde::Serialize` を要求 — `anyhow::Error` は非対応
- `thiserror` は `#[from]` で `?` 変換を自動化，`Display` を自動導出
- 3 層構成: ドメインエラー → インフラエラー → コマンド境界エラー（Principle III 準拠）
- コマンド境界エラーを `{ "kind": "titleTooLong", "message": "..." }` 形式でシリアライズ
- フロントエンドはエラー種別に応じた Toast 表示が可能（FR-007 準拠）

**Architecture**:
- `domain::page::PageError` — ドメインルール違反（TitleEmpty, TitleTooLong, NotFound）
- `infrastructure::persistence::StorageError` — 技術的失敗（sqlx::Error，IO エラー）
- `ipc::CommandError` — IPC 境界のシリアライズ可能なエラー（上記を `#[from]` で合成）

```toml
thiserror = "2"
```

## Decision 5: フロントエンド Toast 通知

**Decision**: sonner

**Rationale**:
- React 19 を明示的にサポート（`peerDependencies: ^18 || ^19`）
- ゼロランタイム依存 — 最小のフットプリント
- Provider 不要 — `<Toaster />` を 1 箇所に置くだけで `toast()` をどこからでも呼べる
- `toast.promise()` パターンが Tauri `invoke` 呼び出しに最適

**Alternatives considered**:
- **react-hot-toast**: goober（CSS-in-JS）に依存，React 19 対応が暗黙的
- **react-toastify**: React 19 で状態管理バグが報告されている（Issue #1275）

```bash
pnpm add sonner
```

## Decision 6: 状態管理

**Decision**: React 19 組み込みフック（useState, useEffect）

**Rationale**:
- データフローが単純: Tauri IPC でフェッチ → 表示 → IPC で変更 → ローカル状態更新
- リモート API のキャッシュ問題がない（ローカル完結）— TanStack Query の価値が薄い
- 状態はコンポーネントローカルまたは 1 階層 — useState で十分
- 投機的な状態管理ライブラリは Principle V の保守性優先に反する

**Alternatives considered**:
- **Zustand / Jotai**: コンポーネント間の複雑な状態共有が必要になった時点で導入を検討
- **TanStack Query**: リモート API 向けの設計であり，ローカル IPC には過剰

## Decision 7: CSS アプローチ

**Decision**: CSS Modules（Vite 組み込みサポート）

**Rationale**:
- Vite が `.module.css` を設定なしでサポート — プラグインもコンフィグ変更も不要
- コンポーネントごとにスコープ付きクラス名 — 名前衝突を防止
- 標準 CSS 構文 — 新しいパラダイム不要
- 既存の `App.css` はグローバルリセット/トークンとして維持可能

**Alternatives considered**:
- **Tailwind CSS**: PostCSS パイプラインと設定ファイルが必要，このスライスの規模では過剰
- **Plain CSS（現状）**: コンポーネント増加に伴いクラス名衝突のリスクが高まる

## Decision 8: IPC 型安全性

**Decision**: `@tauri-apps/api/core` の `invoke<T>` + 手動 TypeScript インターフェース

**Rationale**:
- 4 コマンドの段階でコード生成は過剰
- 手動インターフェースは仕様の TypeScript 側の型として spec と 1:1 で対応
- 将来コマンド数が 10 を超えた段階で tauri-specta の導入を検討

## Decision 9: データベースファイルの保存場所

**Decision**: Tauri の `app_data_dir()` 配下

**Rationale**:
- Tauri 2 では `Manager` トレイトの `app.path().app_data_dir()` で取得
- OS ごとの標準データディレクトリに配置（XDG_DATA_HOME 等）
- `Result<PathBuf>` を返すため `?` で伝播可能
- ディレクトリが存在しない場合は `std::fs::create_dir_all()` で作成

**File path**: `{app_data_dir}/rustydatabasenotes.db`
