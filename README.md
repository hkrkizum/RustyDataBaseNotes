# RustyDataBaseNotes

ローカルファーストなデータベース付きノートアプリ。
自由記述のページと構造化データベースを一つのアプリで管理できる、Notion ライクなデスクトップアプリケーション。

すべてのデータは端末のローカル SQLite に保存され、外部サービスへの通信やテレメトリは一切行わない。

## Features

- **ページ管理** — 作成・一覧・編集・削除。UUIDv7 による一意識別
- **ブロックエディタ** — テキストブロックの追加・編集・並び替え・削除。未保存状態の表示つき
- **データベース** — ページを行、プロパティを列とする構造化テーブルビュー
- **プロパティ型** — text / number / date / select / checkbox の 5 種
- **セル編集** — インライン編集で即時永続化
- **トランザクション保護** — WAL モード + トランザクションによるデータ安全性

## Architecture

```
Frontend (React 19 / TypeScript)
    │  Tauri IPC (型付き)
IPC Layer (DTO 変換・エラーシリアライズ)
    │
Domain Layer (DDD: Entity, Value Object, Aggregate, Service)
    │  Repository trait
Infrastructure (sqlx / SQLite)
```

詳細は [`steering/`](steering/) を参照。

## Tech Stack

| レイヤー | 技術 |
|----------|------|
| デスクトップ | Tauri 2 |
| フロントエンド | React 19, TypeScript ~5.8, Vite 7 |
| バックエンド | Rust 2024 edition |
| データベース | SQLite (WAL mode, sqlx 0.8) |
| テスト | cargo-nextest, Vitest, Testing Library |
| リント・フォーマット | Clippy, Biome |
| タスクランナー | cargo-make |
| 開発環境 | Nix + direnv |

## Getting Started

### 前提条件

- [Nix](https://nixos.org/) (Determinate installer 推奨) + [direnv](https://direnv.net/)
- または Rust toolchain 1.94+, Node.js, pnpm を手動インストール

### セットアップ

```bash
git clone https://github.com/hkrkizum/RustyDataBaseNotes.git
cd RustyDataBaseNotes

# 1. 開発環境の読み込み
direnv allow

# 2. Git Hooks の有効化
cargo make setup-hooks

# 3. 依存のインストール
pnpm install

# 4. 開発サーバーの起動
cargo make serve
```

## Development

### よく使うコマンド

| コマンド | 内容 |
|----------|------|
| `cargo make serve` | 開発サーバー（HMR） |
| `cargo make check` | 高速コンパイルチェック |
| `cargo make fmt` | Rust + TypeScript 一括フォーマット |
| `cargo make check-all` | 静的チェック（テスト除外） |
| `cargo make qa` | フル QA（format → lint → test → doc） |
| `cargo make test` | Rust テスト（cargo-nextest） |
| `cargo make test-ts` | TypeScript テスト（Vitest） |
| `cargo make sqlx-prepare` | DB マイグレーションリセット + .sqlx/ キャッシュ再生成 |

全タスクは [`Makefile.toml`](Makefile.toml) を参照。

### Git Hooks

`.githooks/` でフックを管理し、`cargo make setup-hooks` で有効化する。

| フック | トリガー | 実行内容 |
|--------|----------|----------|
| `pre-commit` | 開発ブランチでのコミット | `cargo make check-all` |
| `pre-merge-commit` | main へのマージ | `cargo make qa` |

### プロジェクト構成

```
src/                        # フロントエンド (React/TypeScript)
├── features/
│   ├── pages/              #   ページ CRUD
│   ├── editor/             #   ブロックエディタ
│   └── database/           #   データベース・テーブルビュー
└── components/             #   共通コンポーネント

src-tauri/                  # バックエンド (Rust)
├── src/
│   ├── domain/             #   DDD コア (Entity, VO, Service)
│   ├── infrastructure/     #   SQLite リポジトリ実装
│   └── ipc/                #   Tauri コマンドハンドラ
└── migrations/             #   SQL マイグレーション

steering/                   # アーキテクチャドキュメント
specs/                      # フィーチャー仕様書
```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
