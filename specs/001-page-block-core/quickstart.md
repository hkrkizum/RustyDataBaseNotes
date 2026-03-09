# Quickstart: Page Block Core

## 目的

この quickstart は，Page Block Core の実装後に，単一 page，block 編集，自動保存，再起動復元，障害回復を短時間で検証するための手順をまとめる，

## 前提

- `nix` と `direnv` を有効化している，
- Rust 2024 ツールチェインと Node.js 実行環境が入っている，
- 依存解決に `pnpm` を使う，
- Tauri 開発実行に必要な OS 依存ライブラリが入っている，

## セットアップ

```bash
direnv allow
pnpm install
cargo fetch
```

## 開発起動

```bash
pnpm tauri dev
```

## 自動テスト

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo doc --no-deps
pnpm lint
pnpm test
pnpm playwright test
```

## 手動確認シナリオ

### 1. 初回起動

1. 保存ディレクトリに `notes.sqlite3` が存在しない状態で起動する，
2. block 0 件の空 page が表示されることを確認する，
3. title が空でも表示名が `無題` になることを確認する，

### 2. block 追加と編集

1. block を 3 件追加する，
2. title と block 本文を編集し，入力停止後 500ms 以内に保存開始表示が出ることを確認する，
3. アプリを再起動し，title，本文，block 順序が一致することを確認する，

### 3. reorder

1. 5 件以上の block を用意する，
2. 1 件を別位置へ移動する，
3. 表示順が直後に更新され，アプリ再起動後も同じ順序で復元されることを確認する，

### 4. 保存失敗

1. 保存先を一時的に書込不能にするか，保存処理を失敗させるテストダブルを有効化する，
2. title または block 本文を編集する，
3. 失敗通知が表示され，画面上の未保存編集が残ることを確認する，
4. 保存条件を復旧して，再度編集停止または reorder を行い，自動保存が再試行されることを確認する，

### 5. 破損データ回復

1. 保存ファイルを構造不正にした状態で起動する，
2. 復元失敗通知が表示されることを確認する，
3. 新しい空 page が自動生成され，編集継続できることを確認する，

## 期待される生成物

- `notes.sqlite3`
- `notes.sqlite3-wal`
- `notes.sqlite3-shm`
- `notes.sqlite3.bak`

## 備考

現時点のリポジトリには実装コードがまだ無いため，上記コマンドは実装導入後の受け入れ確認手順として扱う，
