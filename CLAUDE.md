# CLAUDE.md

このファイルは AI エージェント（Claude Code 等）がプロジェクトの設定意図を把握するためのドキュメントです。

## プロジェクト概要

Tauri (Rust + WebView) ベースのノートアプリケーション。

## 開発環境

WSL2 上の Podman rootless で devcontainer を動かしている。

### Nix / Home Manager

- **WSL2 ホスト**: Determinate Nix + Home Manager (`~/.config/home-manager/flake.nix`)
- **devcontainer**: 同じ Determinate Nix + Home Manager をコンテナ内でも使用し、シェル環境を統一

#### ファイル構成と役割

| ファイル | 説明 |
|---|---|
| `flake.nix` | devShell（Tauri ビルド依存）+ `homeConfigurations.devcontainer`（Home Manager 設定）を公開 |
| `home.nix` | WSL2 と devcontainer で共有するポータブルな Home Manager モジュール。zsh, powerlevel10k, modern CLI ツール (eza, fzf, bat, fd, rg, direnv), git 設定を含む |
| `.devcontainer/Dockerfile` | Ubuntu ベース + systemd + nix-installer ダウンロード + PATH 設定 |
| `.devcontainer/devcontainer.json` | Podman rootless + systemd 構成の devcontainer 定義 |
| `.devcontainer/setup.sh` | コンテナ初回起動時の Nix インストール + systemd 設定 + Home Manager 適用 |

#### devcontainer のアーキテクチャ上の制約と設計判断

1. **`/nix` は named volume でマウント**
   - Docker ビルド時に `/nix` に書いても、コンテナ起動時に空の volume で上書きされる
   - そのため Nix のインストールは Dockerfile ではなく `postCreateCommand`（setup.sh）で実行する

2. **`/etc` はコンテナ再作成で揮発する**
   - `/etc/nix/nix.conf` と `/etc/systemd/system/nix-daemon.service` は setup.sh で毎回再作成する

3. **systemd を PID 1 として使用**
   - `overrideCommand: false` + `runArgs: ["--systemd=always"]` + `CMD ["/sbin/init"]`
   - nix-daemon は systemd service として管理される

4. **Podman rootless 固有の制約**
   - `--userns=keep-id` により UID がリマップされるため `nixbld` グループが機能しない → `build-users-group =`（空）で無効化
   - ビルドサンドボックスは namespace 権限不足で動かない → `sandbox = false`
   - コンテナ内の `sudo` は user namespace 上の疑似 root であり、ホストの root 権限は持たない

5. **Home Manager の適用フロー**
   - `flake.nix` の `homeConfigurations.devcontainer` が `home.nix` を参照
   - `nix run home-manager -- switch --flake .#devcontainer` で適用
   - WSL2 固有の設定（Windows パス、credential manager、Podman）は `home.nix` から除外済み

### devShell（Tauri ビルド環境）

`flake.nix` の `devShells.default` で Tauri のビルド依存（rustup, cargo-tauri, GTK, WebKitGTK 等）を管理。
`.envrc` で `use flake` しており、direnv + nix-direnv により自動で devShell が有効になる。
