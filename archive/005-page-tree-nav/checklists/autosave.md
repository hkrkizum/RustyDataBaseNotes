# Autosave Migration Requirements Quality Checklist: Page Tree Navigation

**Purpose**: エディタの手動保存→debounce自動保存への移行に関する要件の完全性・明確性・一貫性を検証する。isDirty廃止・UnsavedConfirmModal削除・リトライ戦略・ページ遷移時のフラッシュ動作など，移行固有のリスクを持つ要件品質を重点的に検証する
**Created**: 2026-03-22
**Feature**: [spec.md](../spec.md) | [plan.md](../plan.md) | [research.md](../research.md) | [contracts/ipc-commands.md](../contracts/ipc-commands.md)
**Focus**: 自動保存移行（手動保存廃止・debounce・リトライ・UI削除・バックエンド変更）
**Depth**: Standard | **Audience**: レビュアー（PR）

## Requirement Completeness

- [x] CHK001 - 自動保存のトリガー条件（どのコンテンツ変更が保存をスケジュールするか）は明確に定義されているか？ブロックの追加・削除・内容変更・順序変更のすべてがトリガーに含まれることは明示されているか？ <!-- spec.md Assumptions に「トリガーはブロックの追加・削除・内容変更・順序変更など，エディタコンテンツに対するすべての変更」と明記。plan.md useAutoSave 責務 #1 にも記載 -->
- [x] CHK002 - ページ遷移時（サイドバーで別アイテムをクリック）に未保存の変更がある場合の動作（即時フラッシュ？破棄？ベストエフォート保存？）は定義されているか？ <!-- spec.md Assumptions に「ベストエフォートでフラッシュ保存し，保存失敗時もナビゲーションを許可する」と明記。plan.md ページ遷移時の動作にも詳細記載 -->
- [x] CHK003 - アプリ終了時（ウィンドウクローズ・OS シャットダウン）に未保存の変更がある場合の動作は定義されているか？Tauri の window close イベントでの保存フラッシュ要件はあるか？ <!-- 意図的除外。データはサイレントで削除。自動バックアップは将来スコープ（レビュアー承認済み） -->
- [x] CHK004 - 削除対象のUI要素・コンポーネントの完全なリストは定義されているか？（保存ボタン・未保存インジケータ・UnsavedConfirmModal・Ctrl+S ショートカット・ナビゲーション前確認ロジック） <!-- plan.md「自動保存移行の影響箇所」に完全リスト記載 -->
- [x] CHK005 - 既存のプロパティ自動保存メカニズムと新しいエディタ自動保存の関係（共通フック化するか，独立実装か）は定義されているか？ <!-- plan.md 自動保存設計に「useAutoSave はエディタ専用。既存のプロパティ自動保存（features/database/ 内で update_property_value を直接呼び出す方式）は現行のまま独立」と明記 -->
- [x] CHK006 - `useAutoSave` フックの責務範囲（debounce タイマー管理・リトライ・toast 通知・アンマウント時フラッシュ）は明確に定義されているか？ <!-- plan.md「useAutoSave の責務」に5つの責務を明示的にリスト化: デバウンス・リトライ・エラー種別・toast・アンマウントフラッシュ -->
- [x] CHK007 - 自動保存移行後の `save_editor` IPC コマンドの呼び出しパターン変更（明示的→タイマー駆動）に伴う，バックエンドの `EditorSession` の変更範囲は明確か？ <!-- plan.md: is_dirty()/mark_saved() メソッドは残存，EditorStateDto::is_dirty のみ削除（レビュアー承認済み） -->

## Requirement Clarity

- [x] CHK008 - デバウンス間隔が research.md（1000ms）と plan.md（500ms）で異なるが，どちらが正か確定しているか？ <!-- plan.md 500ms を正とし確定。research.md に改訂注記済み（レビュアー承認済み） -->
- [x] CHK009 - リトライ戦略が research.md（指数バックオフなし，1秒固定間隔）と plan.md（指数バックオフ 1s→2s→4s）で異なるが，どちらが正か確定しているか？ <!-- plan.md の指数バックオフを正とし確定。research.md に改訂注記済み（レビュアー承認済み） -->
- [x] CHK010 - Toast メッセージが research.md（「保存に失敗しました。再試行してください。」）と plan.md（「保存に失敗しました」）で異なるが，どちらが正か確定しているか？ <!-- plan.md の「保存に失敗しました」（5秒，自動消去）を正とし確定（レビュアー承認済み） -->
- [x] CHK011 - バックエンドの `EditorSession::is_dirty()` / `mark_saved()` メソッドの扱いが research.md（「残す」）と plan.md（「isDirty/mark_saved パターン廃止」）で矛盾しているが，どちらが正か確定しているか？ <!-- is_dirty()/mark_saved() を残す方針で確定。plan.md を修正済み（レビュアー承認済み） -->
- [x] CHK012 - 「debounce 付き自動保存」がフロントエンド主導（research.md の方針）であることは spec.md / plan.md で明確に記載されているか？バックエンド側は受動的にコマンドを受け取るのみであることは明示されているか？ <!-- plan.md 自動保存設計に「保存タイミングの制御は FE（useAutoSave）が担う。BE は save_editor を受動的に実行するのみ」と明記 -->
- [x] CHK013 - Ctrl+S / Cmd+S キーボードショートカット廃止後の動作は定義されているか？無反応にするのか，別の機能に割り当てるのか？ <!-- plan.md に「Ctrl+S / Cmd+S → no-op（preventDefault で抑止）」と明記（レビュアー承認済み） -->

## Requirement Consistency

- [x] CHK014 - spec.md の Assumptions（「プロパティと同様の自動保存方式に統一する」）と，plan.md / research.md の自動保存設計が一貫しているか？プロパティの自動保存がどのような方式であるかの参照は十分か？ <!-- plan.md に「既存のプロパティ自動保存（features/database/ 内で update_property_value を直接呼び出す方式）」と具体的な動作仕様を記載 -->
- [x] CHK015 - 「画面遷移時の未保存確認ダイアログは不要になる」（spec.md Assumptions）と，ページ遷移時の未保存データ処理戦略は整合しているか？確認ダイアログ不要の前提条件（自動保存が常に成功する or フラッシュ保存する）は明確か？ <!-- spec.md に「自動保存＋遷移時フラッシュにより未保存確認ダイアログは不要」と前提を明記。plan.md にフラッシュ動作の詳細も記載 -->
- [x] CHK016 - `save_editor` コマンドが「変更がある場合のみ DB 書き込み」を判断する仕組み（research.md で言及）は，isDirty 廃止の方針と両立するか？バックエンド側のダーティチェック責務は明確か？ <!-- is_dirty()/mark_saved() を残す方針で確定。矛盾解消済み -->
- [x] CHK017 - frontend-ux.md CHK012（デバウンス間隔 500ms として Covered）と research.md R-004（1000ms）の不整合は認識されているか？ <!-- plan.md 500ms を正とし確定。research.md に改訂注記済み -->

## Scenario Coverage

- [x] CHK018 - 高速なページ切り替え（ページ A 編集→即座にページ B に遷移→即座にページ C に遷移）時の自動保存キューの振る舞いは定義されているか？複数ページの pending save が同時に存在するケースは想定されているか？ <!-- plan.md ページ遷移時の動作に「前ページの pending save をフラッシュ。新ページで新 useAutoSave インスタンス初期化。並行 save は発生しない」と明記 -->
- [x] CHK019 - 自動保存のリトライ中にユーザーがさらに編集を続けた場合の動作（リトライ対象のデータは古い内容？最新の内容？）は定義されているか？ <!-- plan.md useAutoSave 責務 #2 に「リトライは常に最新のエディタ状態を保存する（古いスナップショットではない）」と明記 -->
- [x] CHK020 - 自動保存がリトライ中（最大3回未完了）にユーザーがページを遷移した場合の動作は定義されているか？リトライをキャンセルして遷移するか，リトライ完了を待つか？ <!-- plan.md ページ遷移時の動作に「進行中のリトライをキャンセルし遷移を許可。未保存の変更が失われる可能性がある場合は遷移前に toast で警告」と明記 -->
- [x] CHK021 - 自動保存の save_editor 呼び出しがドメインエラー（PageError::NotFound 等，ページ削除済み）を返した場合の UX は定義されているか？一時的エラーと永続的エラーの区別はあるか？ <!-- plan.md useAutoSave 責務 #3 に「一時的エラーはリトライ対象。永続的エラー（PageError::NotFound 等）は即座に toast で通知しリトライしない」と明記 -->
- [x] CHK022 - 自動保存移行と既存の手動保存テスト（spec.md が「既存ユニットテストが充実」と記載）の関係は定義されているか？既存テストの修正・削除範囲は明確か？ <!-- plan.md テスト修正方針に BE テスト維持・FE useAutoSave テスト追加・手動保存テスト削除の3分類を明記 -->

## Edge Case Coverage

- [x] CHK023 - デバウンスタイマー発火時にコンポーネントがすでにアンマウントされていた場合の安全性要件は定義されているか？ <!-- plan.md useAutoSave 責務 #5 に「useEffect cleanup でデバウンスタイマーをキャンセルし，即時フラッシュ保存を実行する（ベストエフォート）」と明記 -->
- [x] CHK024 - 自動保存が継続的に失敗し続ける場合（DB ファイル破損・ディスクフル等）の長期的な UX は定義されているか？toast が繰り返し表示され続けるシナリオは想定されているか？ <!-- plan.md useAutoSave 責務 #4 に「継続的に保存が失敗する場合，変更のたびに toast を表示する」と明記 -->
- [x] CHK025 - エディタを開いた直後（コンテンツロード完了前）にユーザーが入力を開始した場合，自動保存が不完全なデータを保存してしまうリスクは考慮されているか？ <!-- 意図的除外（レビュアー承認済み） -->

## Notes

- Check items off as completed: `[x]`
- Add comments or findings inline
- Items are numbered sequentially for easy reference
- research.md は Phase 0 の技術調査であり，plan.md が正式な値。矛盾があった場合は plan.md を優先（確定済み）
- 本チェックリストは**自動保存移行**に特化。サイドバー UI・ページ階層・D&D は frontend-ux.md / backend.md でカバー済み
