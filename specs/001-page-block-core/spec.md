# Feature Specification: Page Block Core

**Feature Branch**: `001-page-block-core`
**Created**: 2026-03-10
**Status**: Draft
**Input**: User description: "単一ページ，テキストブロック，永続化を対象とした最小ドメインモデルを定義する"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - ページを作って書き始める (Priority: P1)

ユーザーとして，空のページを 1 つ作成し，そのページ内に複数のテキストブロックを追加したい。
これにより，後続の高度な機能が無くても，ノートアプリとして最小限の記述を開始できる。

**Why this priority**: すべての後続機能は，ページとブロックが安定して存在することを前提とするため。

**Independent Test**: 新規環境でページを 1 つ作成し，3 個以上のテキストブロックを追加できれば，
最小編集体験が成立していると判断できる。

**Acceptance Scenarios**:

1. **Given** アプリにページが存在しない，**When** ユーザーが新規ページを作成する，
   **Then** 一意に識別できる空のページが作成される。
2. **Given** 空のページが存在する，**When** ユーザーがテキストブロックを追加する，
   **Then** 追加した順にブロックが表示され，各ブロックは個別に識別できる。

---

### User Story 2 - ブロック順序を整える (Priority: P2)

ユーザーとして，同じページ内のテキストブロックの順序を変更したい。これにより，書いた内容を
後から整理できる。

**Why this priority**: ブロックが単なる配列ではなく，順序を持つ編集対象であることを早期に確定するため。

**Independent Test**: 5 個以上のブロックを持つページで並び替えを行い，表示順と保存対象の順序が一致すれば
検証できる。

**Acceptance Scenarios**:

1. **Given** 複数のテキストブロックを持つページがある，**When** ユーザーが 1 つのブロックを別位置へ移動する，
   **Then** ページ内のブロック順序が更新され，重複や欠落が発生しない。
2. **Given** ブロック順序を変更したページがある，**When** ユーザーが続けて別のブロックを追加する，
   **Then** 新しいブロックは更新後の順序体系に従って配置される。

---

### User Story 3 - 再起動後も内容を維持する (Priority: P3)

ユーザーとして，アプリを閉じて再度開いても，作成したページとブロックが同じ内容，同じ順序で
復元されていてほしい。これにより，ローカルノートとして安心して使い始められる。

**Why this priority**: 永続化の信頼性が無いと，最小編集体験そのものが成立しないため。

**Independent Test**: ページ作成，ブロック追加，並び替えを行った後にアプリを再起動し，同じページが
同じ順序，同じ内容で復元されれば検証できる。

**Acceptance Scenarios**:

1. **Given** 保存済みのページとブロックがある，**When** ユーザーがアプリを終了して再起動する，
   **Then** 直前に保存されたページタイトル，ブロック内容，ブロック順序が復元される。
2. **Given** 保存処理中にエラーが発生した，**When** ユーザーが次回起動する，
   **Then** 破損した中途半端な状態ではなく，最後に整合していた保存状態が復元される。

### Edge Cases

- ページタイトルが空のまま作成された場合でも，一意なページとして扱い，後から識別可能であること。
- 空文字のテキストブロックを追加した場合でも，順序管理が壊れないこと。
- 保存処理の途中でアプリが終了しても，部分的にだけ反映された順序や内容が表示されないこと。
- ローカル保存先へ書き込めない場合，既存データを壊さずに，ユーザーへ再試行可能な失敗を伝えること。
- ブロック数が増えても，最小スコープとして想定する通常利用量では編集や再表示に目立つ遅延が出ないこと。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST allow the user to create a single page as the primary container for text blocks.
- **FR-002**: System MUST assign every page a stable unique identifier that remains unchanged across reloads.
- **FR-003**: System MUST allow the user to add multiple text blocks to a page.
- **FR-004**: System MUST assign every block a stable unique identifier and associate it with exactly one page.
- **FR-005**: System MUST preserve an explicit order for blocks within a page.
- **FR-006**: System MUST allow the user to reorder blocks within the same page without losing content or duplicating blocks.
- **FR-007**: System MUST persist the page title, block content, block identifiers, and block order to local storage.
- **FR-008**: System MUST restore the last consistent saved state of the page and its blocks after application restart.
- **FR-009**: System MUST treat each user action that changes page or block state as all-or-nothing from the user's perspective.
- **FR-010**: System MUST present a clear failure message when a save cannot be completed and MUST keep the last consistent saved state intact.
- **FR-011**: System MUST operate fully offline and MUST NOT require sign-in, cloud sync, or external network access for this feature.
- **FR-012**: System MUST limit this feature to plain text blocks inside a single page and MUST NOT include databases, multiple views,
  nested pages, or advanced editing behaviors in this increment.

### Key Entities *(include if feature involves data)*

- **Page**: ユーザーがノート内容を保持する最上位単位。識別子，タイトル，作成日時，更新日時を持ち，
  配下のブロック集合を管理する。
- **Block**: ページ内に配置される最小のテキスト要素。識別子，所属ページ識別子，本文，順序位置，
  作成日時，更新日時を持つ。

## Constraints & Compliance *(mandatory)*

- **CC-001 Data Integrity**: ページ作成，ブロック追加，ブロック並び替え，保存は，ユーザーから見て
  常に整合した状態として完了または失敗しなければならない。
- **CC-002 Privacy**: 本機能はローカル環境のみで完結し，アカウント作成，通信許可，外部サービス接続を
  必須にしてはならない。
- **CC-003 Performance**: 単一ページに 200 個のテキストブロックがある状態でも，ページ再表示，
  ブロック追加，並び替え結果の反映は 1 秒以内に知覚できることを目標とする。
- **CC-004 Boundary Types**: ページとブロックの作成，更新，再読み込みで受け渡すデータは，タイトル，
  本文，識別子，順序といった明示的な項目を持ち，曖昧な巨大テキスト 1 件へ折り畳まれてはならない。
- **CC-005 Testability**: 各ユーザーストーリーは，作成，追加，並び替え，再起動復元，保存失敗時の保全を
  独立に確認できる受け入れ試験を持たなければならない。

## Assumptions

- 本機能の利用者は単一ユーザーであり，同時編集や共有は想定しない。
- 初回スコープでは，1 回の作業対象は単一ページとし，ページ間移動や階層ページは扱わない。
- テキストブロックはプレーンテキストのみを対象とし，画像，チェックボックス，データベース行は含めない。

## Dependencies

- 本機能は，ユーザーがローカル環境でアプリを起動し，単一ページの編集画面へ到達できることを前提とする。
- 本機能は，ページとブロックの保存先として利用可能なローカル保存領域が存在することを前提とする。

## Out of Scope

- データベース機能，リストビュー，ボードビュー，ガントチャート。
- スラッシュコマンド，ドラッグ＆ドロップ UI，複雑なショートカット編集。
- ネストされたページ，複数ページ管理，共有，クラウド同期。
- リッチテキスト装飾，添付ファイル，検索，フィルタ。

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 新規ユーザーが初回起動後 2 分以内に 1 つのページを作成し，10 個のテキストブロックを追加できる。
- **SC-002**: 20 回連続の保存と再起動の確認で，ページタイトル，ブロック内容，ブロック順序が毎回一致する。
- **SC-003**: 200 個のテキストブロックを含むページでも，ブロック追加または並び替えの結果が 1 秒以内に確認できる。
- **SC-004**: 保存不能な状況を再現した確認で，既存の保存済み内容が失われず，ユーザーが失敗を認識できる案内が毎回表示される。
