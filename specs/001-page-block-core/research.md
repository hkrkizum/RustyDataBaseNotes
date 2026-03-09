# Research: Page Block Core

## 技術判断 1: フロントエンドは React 19 + TypeScript 5.x を採用する

- Decision: Tauri 2 の Webview フロントエンドは React 19 + TypeScript 5.x で構成する，
- Rationale: 200 ブロック規模では通常の制御入力で十分に扱え，必要に応じて React の `useDeferredValue` や `useTransition` 系で派生描画だけを遅延できる。将来の複数 view 導入時も，ページ集約を維持したまま UI を増やしやすい，
- Alternatives considered: Vue 3 は有力だが，現時点では既存コードが無く，React 側の運用指針と将来の UI 最適化手段を優先した，

## 技術判断 2: 永続化は SQLite + `rusqlite` を採用する

- Decision: 永続化は Tauri の app-local-data 配下に置く SQLite `notes.sqlite3` とし，Rust 側で `rusqlite` を使ってトランザクション保存する，
- Rationale: 単一ページでも，ページとブロックを別表で保持することで，page と block を巨大テキストへ折り畳まずに順序と識別子を維持できる。`rusqlite` の `Transaction` は明示的 `commit` までロールバック既定であり，保存失敗時に最後の整合済み状態を保ちやすい，
- Alternatives considered: JSON 単一ファイルは順序保持自体は可能だが，マイグレーションと部分破損時の検査性で不利。Tauri Store は軽量だが，ページとブロックの関係や将来の拡張を考えると情報モデルが弱い，

## 技術判断 3: 保存境界は「1 ページ集約スナップショット」の IPC に絞る

- Decision: フロントエンドとバックエンドの主契約は `load_page_core` と `persist_page_snapshot` の 2 コマンドを基本とし，編集操作はフロントエンドのドラフト状態で行ってから完全なページ集約を保存する，
- Rationale: 本 increment は常にページ 1 件だけを扱うため，操作ごとに細粒度コマンドを増やすより，ページ集約スナップショットを型付き DTO で往復させる方が，原子保存と障害時の再試行を単純に保てる，
- Alternatives considered: 追加，編集，並び替えごとの個別 IPC は将来の複数集約では有効だが，現段階では境界数が増え，保存失敗時の再同期処理が複雑になる，

## 技術判断 4: 自動保存はフロントエンドのドラフト状態とバックエンドの整合済み状態を分離する

- Decision: ページタイトル編集とブロック本文編集は入力停止 500ms のデバウンスで保存し，ブロック追加と並び替えは操作完了時に即時保存する。保存失敗時はドラフトを保持し，次の保存契機で同じ集約を再送する，
- Rationale: 仕様が求める「未保存編集は画面に残すが，再起動後は最後の整合済み状態だけを復元する」を満たすには，UI のドラフト状態と永続化済みスナップショットを明確に分離する必要がある，
- Alternatives considered: 各キーストローク即時保存は入力体験を悪化させやすく，手動保存は要件違反になる，

## 技術判断 5: 復旧方針は WAL + 側車バックアップ + 破損ファイル隔離とする

- Decision: SQLite は WAL モードで運用し，各コミット成功後に `notes.sqlite3.bak` を更新する。起動時に読取不能または形式不正が判明した場合は，破損ファイルを隔離して空ページで再初期化する，
- Rationale: WAL と明示的トランザクションで保存途中の部分反映を避けられ，側車バックアップで「最後の整合済み状態」を別ファイルでも保持できる。破損ファイルの隔離により，空ページ起動と後調査の両立ができる，
- Alternatives considered: バックアップなし運用は憲章の復旧要件を満たしにくい。保存失敗時だけコピーを取る方式は，平常時の復旧基点が弱い，

## 技術判断 6: テストは Rust 単体 + React 単体 + E2E で分担する

- Decision: ドメイン不変条件と永続化整合性は Rust テスト，編集 UI と自動保存スケジューラはフロントエンド単体テスト，再起動復元と失敗通知は E2E で検証する，
- Rationale: 失敗理由ごとに最短の検査層を分けることで，保存失敗や破損データ回復のような高価なシナリオも継続的に確認しやすい，
- Alternatives considered: E2E のみでは失敗原因の特定が遅い。単体テストのみでは Tauri 境界と起動復元を保証できない，

## 参考ソース

- Tauri の型付きコマンド境界と JSON 直列化要件: https://v2.tauri.app/ja/develop/calling-rust/
- Tauri IPC の境界説明: https://v2.tauri.app/ja/concept/inter-process-communication/
- React `startTransition`: https://react.dev/reference/react/startTransition
- React `useDeferredValue`: https://react.dev/reference/react/useDeferredValue
- `rusqlite::Transaction`: https://docs.rs/rusqlite/latest/rusqlite/struct.Transaction.html
