# Quickstart: ページの永続化（最小縦断スライス）

**Feature Branch**: `001-page-persistence`
**Date**: 2026-03-21

## 前提条件

- Nix devshell が有効であること（`direnv allow` 済み）
- `rustup`, `cargo-make`, `cargo-nextest`, `pnpm` が利用可能であること

## 開発サーバーの起動

```bash
cargo make serve
```

Tauri 開発サーバーが起動し，フロントエンド（Vite HMR）とバックエンド（Rust）の
ホットリロードが有効になる。

## 品質ゲートの実行

```bash
cargo make qa
```

以下を順次実行する:
1. `cargo fmt --all` — コードフォーマット
2. `cargo clippy --workspace` — Lint（`unwrap_used = deny` 等）
3. `cargo nextest run` — テスト
4. `cargo doc --no-deps` — ドキュメントビルド

## 個別コマンド

```bash
# フォーマット
cargo make fmt

# フォーマットチェック（CI 用）
cargo make fmt-check

# Lint
cargo make clippy

# テスト実行
cargo make test

# 特定テストの実行
cargo make test-filter -- "test_name_pattern"

# ドキュメントテスト
cargo make doc-test

# ドキュメント生成 & 表示
cargo make doc
```

## フロントエンド

```bash
# 依存インストール
pnpm install

# 開発サーバー（Tauri なし，フロントエンドのみ）
pnpm dev

# ビルド
pnpm build
```

## プロジェクト構成（本機能）

```
src-tauri/
├── src/
│   ├── main.rs                    # エントリポイント
│   ├── lib.rs                     # Tauri Builder 構成
│   ├── domain/
│   │   ├── mod.rs
│   │   └── page/
│   │       ├── mod.rs
│   │       ├── entity.rs          # Page, PageId, PageTitle
│   │       ├── error.rs           # PageError
│   │       └── repository.rs      # PageRepository トレイト
│   ├── infrastructure/
│   │   ├── mod.rs
│   │   └── persistence/
│   │       ├── mod.rs
│   │       ├── database.rs        # SqlitePool 初期化，マイグレーション
│   │       ├── error.rs           # StorageError
│   │       └── page_repository.rs # SqlxPageRepository
│   └── ipc/
│       ├── mod.rs
│       ├── error.rs               # CommandError + Serialize impl
│       ├── dto.rs                 # PageDto
│       └── page_commands.rs       # #[tauri::command] ハンドラ
├── migrations/
│   └── 0001_create_pages.sql
└── Cargo.toml

src/
├── main.tsx                       # React エントリポイント
├── App.tsx                        # ルートコンポーネント
├── App.css                        # グローバルスタイル
├── features/
│   └── pages/
│       ├── PageListView.tsx       # 一覧画面
│       ├── PageListView.module.css
│       ├── PageItem.tsx           # 一覧行（インライン編集）
│       ├── PageItem.module.css
│       ├── CreatePageForm.tsx     # 新規作成フォーム
│       ├── CreatePageForm.module.css
│       ├── DeleteConfirmModal.tsx # 削除確認ダイアログ
│       ├── DeleteConfirmModal.module.css
│       ├── usePages.ts           # ページ CRUD カスタムフック
│       └── types.ts              # Page, CommandError 型定義
└── components/
    └── toast/
        └── Toaster.tsx            # sonner Toaster ラッパー
```

## 依存関係の追加

### Rust (src-tauri/Cargo.toml)

```toml
[dependencies]
sqlx = { version = "0.8", features = [
    "runtime-tokio",
    "sqlite",
    "macros",
    "migrate",
    "chrono",
    "uuid",
] }
uuid = { version = "1", features = ["v7", "serde"] }
thiserror = "2"
chrono = { version = "0.4", features = ["serde"] }
```

### Frontend (package.json)

```bash
pnpm add sonner
```

## コンパイル時クエリ検証の設定

sqlx の `query!()` / `query_as!()` マクロはビルド時に `DATABASE_URL` の DB に接続して
SQL を検証する。開発時は `.env` ファイルで設定する:

```bash
# src-tauri/.env
DATABASE_URL=sqlite:dev.db
```

初回は開発用 DB を作成しマイグレーションを適用する:

```bash
cd src-tauri
# sqlx-cli がある場合
sqlx database create
sqlx migrate run

# sqlx-cli がない場合は，アプリを一度起動すれば DB が作成される
cargo make serve
```

### オフラインモード（CI 用）

CI など DB が利用できない環境では，事前にクエリメタデータを保存する:

```bash
cd src-tauri
cargo sqlx prepare
```

生成された `.sqlx/` ディレクトリをバージョン管理にコミットする。
`SQLX_OFFLINE=true` が設定されている場合，マクロは `.sqlx/` から検証情報を読み取る。

## データベースの場所

SQLite ファイルは Tauri の `app_data_dir()` 配下に作成される:

- **Linux**: `~/.local/share/com.vscode.rustydatabasenotes/rustydatabasenotes.db`
- **macOS**: `~/Library/Application Support/com.vscode.rustydatabasenotes/rustydatabasenotes.db`
- **Windows**: `C:\Users\<User>\AppData\Roaming\com.vscode.rustydatabasenotes\rustydatabasenotes.db`

開発中にデータベースをリセットするには上記ファイルを削除する。
