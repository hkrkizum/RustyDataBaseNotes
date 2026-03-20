<!--
Sync Impact Report
- Version change: 1.2.0 -> 1.3.0
- Modified principles: None
- Added sections: None
- Removed sections: None
- Modified subsections:
  - Technical Standards: added home.nix description
    (Home Manager module, CLI tools, git config)
  - CLAUDE.md simplified to a pointer referencing this constitution
- Templates requiring updates:
  - ✅ .specify/templates/plan-template.md (no change needed)
  - ✅ .specify/templates/spec-template.md (no change needed)
  - ✅ .specify/templates/tasks-template.md (no change needed)
  - ⚠ .specify/templates/commands/ is absent in this repository
- Follow-up TODOs:
  - None
-->
# RustyDataBaseNotes Constitution

## Core Principles

### I. Local-First Product Integrity
本プロジェクトはローカル完結のノートブックアプリを提供するものであり，すべての機能は
ユーザーデータの完全性，復旧可能性，通信遮断を最優先に設計しなければならない
（MUST）。永続化はトランザクション境界を明示できる方式で実装し，クラッシュ時の破損を
防ぐこと。保存・移行・添付ファイル処理では，失敗時にユーザーへ明確なエラーを返し，
自動バックアップまたは同等の復旧手段を設計へ含めること。理由: 本製品の価値は
「SaaS に依存せず，データを失わずに使えること」にあるため。

### II. Domain-Faithful Information Model
仕様，UI，コードは，ブロック，ページ，データベース，ビュー，プロパティという中核語彙を
一貫して用いなければならない（MUST）。ドキュメントは巨大な単一テキストではなく，
識別可能なブロック集合として扱い，ページ階層とデータベースレコードの関係を崩す近道を
導入してはならない（MUST NOT）。新機能は，リスト，ボード，ガントチャート等の複数
ビューから再利用できるモデルを前提に設計すること。理由: ドメイン語彙と情報モデルが
崩れると，将来のビュー追加とデータ整合が同時に壊れるため。

### III. Typed Boundaries and Bounded Contexts
バックエンドは Rust と Tauri を中核にし，フロントエンドは TypeScript で実装する
（MUST）。フロントエンドとバックエンドの境界は，型付き IPC 契約，明示的なデータ構造，
およびマイグレーション可能なストレージ設計で表現しなければならない（MUST）。Cargo
ワークスペースまたはモジュール境界は境界づけられたコンテキストとして扱い，ドメイン
オブジェクトにはエンティティ，値オブジェクト，集約ルートの役割を与えること。
理由: ローカルアプリでも境界が曖昧になると，UI 変更が保存形式と一緒に壊れやすくなるため。

### IV. Test-First Delivery and Quality Gates
すべての実装は Red-Green-Refactor の順で進め，失敗するテストまたは実証可能な仕様が
存在しない状態で本実装を始めてはならない（MUST NOT）。コミット前には，少なくとも
整形，lint，関連テスト，必要なドキュメントビルドを通過させること（MUST）。機能仕様，
計画，タスクは独立に検証できるユーザーストーリー単位で分解し，各ストーリーに品質確認
手順を持たせること。理由: 本プロジェクトは機能追加と Rust 学習を両立するため，
仕様と検証が先行しない変更は保守不能になりやすい。

### V. Safe Rust and Maintainability First
アプリケーションコードで `unsafe`，`unwrap()`，`expect()`，`panic!()`，
`unreachable!()` を使用してはならない（MUST NOT）。すべての失敗可能操作は
`Result` 等で伝播し，意図的に無視する例外は理由付きコメントを残すこと。公開 API には
`///` ドキュメントコメントを付け，公開関数で `Result` を返す場合はエラー条件を説明する
こと。可読性と保守性はマイクロ最適化より優先され，複雑な抽象化や投機的最適化は，
測定結果と必要性が示されない限り導入してはならない（MUST NOT）。理由: 個人開発の
長期保守では，予測可能な失敗処理と読みやすい実装が最も大きい速度要因だからである。

## Technical Standards

- バックエンドの主要実装言語は Rust 2024 とし，デスクトップ実行基盤は Tauri を採用する。
- フロントエンドは TypeScript を用い，フレームワークは React または Vue のいずれかに統一する。
- TypeScript 依存管理は `pnpm` を使用する。
- 永続化層はマイグレーション可能であり，ローカル保存，バックアップ，自動復旧方針を備える
  こと。
- タスクランナーとして `cargo-make` を採用し，ビルド，テスト，lint 等の定型作業は
  Makefile.toml に定義して一元管理する。
- Rust のテスト実行には `cargo-nextest` を使用する。`cargo test` ではなく
  `cargo nextest run` を標準のテストコマンドとする。
- 大規模データ表示では仮想化等の手段により応答性を確保し，体感遅延を常態化させてはならない。
- アプリケーション本体は意図しない外部通信，テレメトリ送信，サーバー依存機能を導入しては
  ならない。
- ユーザー向け文書と作業メモは日本語を基本とし，Rust の公開ドキュメントコメントは英語で
  記述する。
- 日本語文書の読点は「，」（全角カンマ），句点は「。」とする。
- 開発環境は WSL2 上の Nix devshell で構成する。`flake.nix` の `devShells.default` で
  Tauri のビルド依存（rustup, cargo-tauri, GTK, WebKitGTK 等）を管理し，`.envrc` と
  direnv + nix-direnv により devshell を自動で有効にする。
- `home.nix` はポータブルな Home Manager モジュールであり，zsh, powerlevel10k,
  CLI ツール（eza, fzf, bat, fd, rg, direnv），git 設定を含む。
  `flake.nix` の `homeConfigurations` から参照される。

## Delivery Workflow

- 仕様駆動開発を基本とし，機能仕様は `specs/` に，横断的な統合仕様は `steering/` に保管する。
- すべての計画書は，ローカル完結性，データ保護，型付き境界，テスト先行，保守性優先の
  憲章チェックを通過しなければならない。
- タスク分解では，各ユーザーストーリーに対して，先に失敗するテスト，次に最小実装，
  最後にリファクタリングとドキュメント更新を置くこと。
- レビューでは，禁止構文の混入，ドメイン語彙の逸脱，未計画の外部通信，移行やバックアップの
  欠落を必ず確認する。
- 品質ゲートの既定は，`cargo fmt --all`，`cargo clippy`，`cargo nextest run`，
  `cargo doc --no-deps`，および必要なフロントエンド側の lint と test である。

## Governance

この憲章はプロジェクト内の他の慣行より優先される。改定は，変更理由，影響範囲，
テンプレート同期の有無を明記した更新として記録し，関連するテンプレートと運用文書を
同一変更内で整合させなければならない（MUST）。バージョン番号は Semantic Versioning に
従い，原則の削除または後方互換性のない再定義は MAJOR，新しい原則や必須節の追加，
または運用義務の実質的拡張は MINOR，文言整理や曖昧さ解消のみは PATCH とする。
すべての計画レビュー，実装レビュー，リリース前確認では，本憲章への適合性を確認し，
違反がある場合は例外理由と解消計画を明示しなければならない（MUST）。

**Version**: 1.3.0 | **Ratified**: 2026-03-10 | **Last Amended**: 2026-03-21
