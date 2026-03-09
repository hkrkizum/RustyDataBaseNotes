# Implementation Plan: Page Block Core

**Branch**: `001-page-block-core` | **Date**: 2026-03-10 | **Spec**: [/home/hikaru/Develop/RustyDataBaseNotes/specs/001-page-block-core/spec.md](/home/hikaru/Develop/RustyDataBaseNotes/specs/001-page-block-core/spec.md)
**Input**: Feature specification from `/specs/001-page-block-core/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See
`.specify/templates/plan-template.md` for the execution workflow.

## Summary

単一ページとフラットなテキストブロック集合を最小のドメイン境界として定義し，React 19 + Tauri 2 + Rust 2024 で，ローカル専用の自動保存付きエディタを構成する。保存は 1 ページ集約のスナップショットを SQLite にトランザクション保存し，成功済み状態だけをバックアップ側ファイルへ複製することで，保存失敗時は最後の整合済み状態を維持しつつ，画面上の未保存編集をセッション内に保持する，

## Technical Context

<!--
  Replace this section with project-specific facts for the feature.
  The default expectations for this repository are:
  - Backend: Rust 2024 on Tauri
  - Frontend: TypeScript with React or Vue
  - Package manager: pnpm
  - Storage: local-first persistence with migrations and backup strategy
-->

**Language/Version**: Rust 2024，TypeScript 5.x，React 19
**Primary Dependencies**: Tauri 2.x，React 19，Vite，Serde，rusqlite
**Storage**: Tauri の app-local-data 配下に置く SQLite `notes.sqlite3`，WAL モード，`src-tauri/migrations/` の順序付き SQL マイグレーション，成功済み状態を複製する `notes.sqlite3.bak`
**Testing**: `cargo test`，`cargo clippy`，`cargo doc --no-deps`，`pnpm test`，`pnpm lint`，`pnpm playwright test`
**Target Platform**: デスクトップアプリ。Windows を優先対象とし，Linux と macOS は同一保存モデルでのスモーク確認を行う
**Project Type**: desktop-app
**Performance Goals**: 保存済み 200 ブロックの起動復元を 1 秒以内に知覚可能，ブロック追加と並び替え反映を 1 秒以内，タイトルと本文編集は入力停止後 500ms 以内に保存開始
**Constraints**: 完全オフライン，単一ページのみ，ブロック削除なし，外部通信なし，`unsafe` と `unwrap()` と `expect()` と `panic!()` 禁止
**Scale/Scope**: 常時管理ページ 1 件，ブロック 0-200 件，単一ユーザー，単一ウィンドウ

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Gate Status**: PASS。Phase 0 調査後および Phase 1 設計後の再評価でも，憲章違反は発生していない，

- **Local-First Product Integrity**: すべての保存はローカル SQLite に限定し，ページ集約の保存を 1 トランザクションで完了させる。コミット成功後のみ `notes.sqlite3.bak` を更新し，保存失敗時は最後の整合済み状態を保持する。起動時に読取不能または形式不正が検出された場合は，失敗通知を返したうえで破損ファイルを隔離し，新しい空ページを生成して継続する，
- **Domain-Faithful Information Model**: 本 increment の実体は page と block のみとし，database，view，property は将来の語彙として予約し，ページ属性や巨大テキストへ折り畳まない。ブロックは単一ページ配下のフラットな順序付き集合として扱う，
- **Typed Boundaries and Bounded Contexts**: Rust 側は `domain::page_block` 集約，`application::page_block` ユースケース，`infrastructure::sqlite` 永続化，`ipc::page_block` コマンド境界に分割する。TypeScript 側は `PageSnapshotDto`，`BlockDto`，`EditorSessionState`，`PersistPageSnapshotRequest` を Rust の `serde` DTO と 1 対 1 対応させる。ストレージ変更は `pages` と `blocks` と `save_metadata` の初回マイグレーションを追加する，
- **Test-First Delivery and Quality Gates**: 実装前に，初回起動で空ページ生成，ブロック追加，500ms 自動保存，並び替え保存，再起動復元，保存失敗時の未保存保持，保存失敗後リトライ，破損データ起動回復の失敗テストを先に作る。品質ゲートは `cargo fmt --all`，`cargo clippy`，`cargo test`，`cargo doc --no-deps`，`pnpm lint`，`pnpm test`，`pnpm playwright test` とする，
- **Safe Rust and Maintainability First**: `unsafe` と強制アンラップ系は使わず，すべての永続化失敗を `Result` と明示的なエラー DTO で返す。公開 Rust API と IPC で共有される型には `///` ドキュメントコメントを付与し，エラー条件を記述する，

## Project Structure

### Documentation (this feature)

```text
specs/001-page-block-core/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
src/
├── app/
├── features/
│   └── page-block-core/
├── components/
└── lib/

src-tauri/
├── src/
│   ├── application/
│   │   └── page_block/
│   ├── domain/
│   │   └── page_block/
│   ├── infrastructure/
│   │   └── sqlite/
│   └── ipc/
│       └── page_block.rs
└── migrations/

tests/
├── integration/
└── e2e/
```

**Structure Decision**: 現在のリポジトリには実装ディレクトリが未作成のため，本 feature で初期構造を定義する。フロントエンドは `src/features/page-block-core/` に UI と自動保存用フックを閉じ込め，Tauri 呼び出しは `src/lib/` に分離する。バックエンドは page-block 集約ごとに domain，application，infrastructure，ipc を分けて，UI 変更が保存形式へ直接漏れない境界を作る，

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | N/A | N/A |
