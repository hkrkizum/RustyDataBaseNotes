# Autosave Migration Requirements Quality Checklist: Page Tree Navigation

**Purpose**: エディタの手動保存→debounce自動保存への移行に関する要件の完全性・明確性・一貫性を検証する。isDirty廃止・UnsavedConfirmModal削除・リトライ戦略・ページ遷移時のフラッシュ動作など，移行固有のリスクを持つ要件品質を重点的に検証する
**Created**: 2026-03-22
**Feature**: [spec.md](../spec.md) | [plan.md](../plan.md) | [research.md](../research.md) | [contracts/ipc-commands.md](../contracts/ipc-commands.md)
**Focus**: 自動保存移行（手動保存廃止・debounce・リトライ・UI削除・バックエンド変更）
**Depth**: Standard | **Audience**: レビュアー（PR）

## Requirement Completeness

- [ ] CHK001 - 自動保存のトリガー条件（どのコンテンツ変更が保存をスケジュールするか）は明確に定義されているか？ブロックの追加・削除・内容変更・順序変更のすべてがトリガーに含まれることは明示されているか？ <!-- spec.md / plan.md ともに「debounce付き自動保存」のトリガー条件の詳細は未記載 -->
- [ ] CHK002 - ページ遷移時（サイドバーで別アイテムをクリック）に未保存の変更がある場合の動作（即時フラッシュ？破棄？ベストエフォート保存？）は定義されているか？ <!-- research.md R-004 に「ページ切り替え時にフラッシュ」「失敗してもナビゲーションは許可」と記載あるが，spec.md / plan.md には明示的な記載なし -->
- [ ] CHK003 - アプリ終了時（ウィンドウクローズ・OS シャットダウン）に未保存の変更がある場合の動作は定義されているか？Tauri の window close イベントでの保存フラッシュ要件はあるか？ <!-- spec/plan/research ともにアプリ終了時の未保存データ処理は未記載 -->
- [ ] CHK004 - 削除対象のUI要素・コンポーネントの完全なリストは定義されているか？（保存ボタン・未保存インジケータ・UnsavedConfirmModal・Ctrl+S ショートカット・ナビゲーション前確認ロジック） <!-- research.md R-004 に「UnsavedConfirmModal削除」「保存ボタン・未保存インジケータ削除」「Ctrl+S削除」「ナビゲーション前確認ロジック削除」と列挙。spec.md / plan.md にはリストなし -->
- [ ] CHK005 - 既存のプロパティ自動保存メカニズムと新しいエディタ自動保存の関係（共通フック化するか，独立実装か）は定義されているか？ <!-- spec.md Clarifications に「プロパティの自動保存方式に合わせてリファクタ」と記載あるが，具体的な共通化 / 独立実装の判断は未記載 -->
- [ ] CHK006 - `useAutoSave` フックの責務範囲（debounce タイマー管理・リトライ・toast 通知・アンマウント時フラッシュ）は明確に定義されているか？ <!-- plan.md に useAutoSave.ts を新規作成する記載あり。research.md に概念コードあり。責務の明示的な定義はドキュメント上になし -->
- [ ] CHK007 - 自動保存移行後の `save_editor` IPC コマンドの呼び出しパターン変更（明示的→タイマー駆動）に伴う，バックエンドの `EditorSession` の変更範囲は明確か？ <!-- contracts に isDirty 削除と記載。research.md に is_dirty()/mark_saved() は「残す」と記載。plan.md に「isDirty/mark_saved パターン廃止」と記載 — 矛盾の可能性あり -->

## Requirement Clarity

- [ ] CHK008 - デバウンス間隔が research.md（1000ms）と plan.md（500ms）で異なるが，どちらが正か確定しているか？ <!-- research.md R-004: "debounce 間隔は 1000ms"。plan.md 自動保存パラメータ: "デバウンス間隔: 500ms"。矛盾あり -->
- [ ] CHK009 - リトライ戦略が research.md（指数バックオフなし，1秒固定間隔）と plan.md（指数バックオフ 1s→2s→4s）で異なるが，どちらが正か確定しているか？ <!-- research.md R-004: "1秒後にリトライ（最大3回，指数バックオフなし）"。plan.md: "間隔は指数バックオフ（1s → 2s → 4s）"。矛盾あり -->
- [ ] CHK010 - Toast メッセージが research.md（「保存に失敗しました。再試行してください。」）と plan.md（「保存に失敗しました」）で異なるが，どちらが正か確定しているか？ <!-- research.md R-004 と plan.md 自動保存パラメータでメッセージテキストが異なる -->
- [ ] CHK011 - バックエンドの `EditorSession::is_dirty()` / `mark_saved()` メソッドの扱いが research.md（「残す」）と plan.md（「isDirty/mark_saved パターン廃止」）で矛盾しているが，どちらが正か確定しているか？ <!-- research.md R-004: "is_dirty() / mark_saved() メソッドは残す"。plan.md Project Structure: "isDirty/mark_saved パターン廃止"。contracts: EditorStateDto から isDirty 削除のみ記載 -->
- [ ] CHK012 - 「debounce 付き自動保存」がフロントエンド主導（research.md の方針）であることは spec.md / plan.md で明確に記載されているか？バックエンド側は受動的にコマンドを受け取るのみであることは明示されているか？ <!-- research.md R-004 に「保存タイミングの制御は UI 層の責務」と明記。spec.md / plan.md では明示されていない -->
- [ ] CHK013 - Ctrl+S / Cmd+S キーボードショートカット廃止後の動作は定義されているか？無反応にするのか，別の機能に割り当てるのか？ <!-- research.md に「Ctrl+S / Cmd+S キーボードショートカット削除」と記載。廃止後のキー押下時の動作（no-op / ブラウザデフォルト抑止等）は未記載 -->

## Requirement Consistency

- [ ] CHK014 - spec.md の Assumptions（「プロパティと同様の自動保存方式に統一する」）と，plan.md / research.md の自動保存設計が一貫しているか？プロパティの自動保存がどのような方式であるかの参照は十分か？ <!-- 「プロパティの自動保存方式」の具体的な動作仕様への参照がない -->
- [ ] CHK015 - 「画面遷移時の未保存確認ダイアログは不要になる」（spec.md Assumptions）と，ページ遷移時の未保存データ処理戦略は整合しているか？確認ダイアログ不要の前提条件（自動保存が常に成功する or フラッシュ保存する）は明確か？ <!-- spec.md Assumptions に「不要」と記載あるが，その前提（フラッシュ保存 or データロス許容）は未明記 -->
- [ ] CHK016 - `save_editor` コマンドが「変更がある場合のみ DB 書き込み」を判断する仕組み（research.md で言及）は，isDirty 廃止の方針と両立するか？バックエンド側のダーティチェック責務は明確か？ <!-- research.md: "is_dirty() で変更有無を判断して DB 書き込み"。plan.md: "isDirty/mark_saved パターン廃止"。両立の方法が不明 -->
- [ ] CHK017 - frontend-ux.md CHK012（デバウンス間隔 500ms として Covered）と research.md R-004（1000ms）の不整合は認識されているか？ <!-- frontend-ux チェックリストは plan.md の 500ms を根拠に Covered としたが，research.md との不整合は未解決 -->

## Scenario Coverage

- [ ] CHK018 - 高速なページ切り替え（ページ A 編集→即座にページ B に遷移→即座にページ C に遷移）時の自動保存キューの振る舞いは定義されているか？複数ページの pending save が同時に存在するケースは想定されているか？ <!-- spec/plan/research ともに複数ページの pending save シナリオは未記載 -->
- [ ] CHK019 - 自動保存のリトライ中にユーザーがさらに編集を続けた場合の動作（リトライ対象のデータは古い内容？最新の内容？）は定義されているか？ <!-- spec/plan/research ともにリトライ中の追加編集シナリオは未記載 -->
- [ ] CHK020 - 自動保存がリトライ中（最大3回未完了）にユーザーがページを遷移した場合の動作は定義されているか？リトライをキャンセルして遷移するか，リトライ完了を待つか？ <!-- research.md に「失敗してもナビゲーションは許可」と記載あるが，リトライ中の遷移シナリオは未記載 -->
- [ ] CHK021 - 自動保存の save_editor 呼び出しがドメインエラー（PageError::NotFound 等，ページ削除済み）を返した場合の UX は定義されているか？一時的エラーと永続的エラーの区別はあるか？ <!-- spec/plan に自動保存固有のエラーハンドリング区分はなし。リトライ対象エラーの種別は未記載 -->
- [ ] CHK022 - 自動保存移行と既存の手動保存テスト（spec.md が「既存ユニットテストが充実」と記載）の関係は定義されているか？既存テストの修正・削除範囲は明確か？ <!-- spec.md Clarifications に「既存ユニットテストが充実しておりリファクタ耐性あり」と記載。移行後のテスト変更計画は未記載 -->

## Edge Case Coverage

- [ ] CHK023 - デバウンスタイマー発火時にコンポーネントがすでにアンマウントされていた場合の安全性要件は定義されているか？ <!-- research.md の概念コードに useEffect cleanup があるが，要件としての定義はなし -->
- [ ] CHK024 - 自動保存が継続的に失敗し続ける場合（DB ファイル破損・ディスクフル等）の長期的な UX は定義されているか？toast が繰り返し表示され続けるシナリオは想定されているか？ <!-- 3回リトライ後 toast のみ記載。その後の再試行ポリシー（次の変更時に再度リトライ？手動リロード要求？）は未記載 -->
- [ ] CHK025 - エディタを開いた直後（コンテンツロード完了前）にユーザーが入力を開始した場合，自動保存が不完全なデータを保存してしまうリスクは考慮されているか？ <!-- spec/plan/research ともにエディタの初期化完了と自動保存開始のタイミング関係は未記載 -->

## Notes

- Check items off as completed: `[x]`
- Add comments or findings inline
- Items are numbered sequentially for easy reference
- [Gap] = 仕様に記載がない要件，[Partial] = 関連記述はあるが詳細度不足，[Conflict] = ドキュメント間で矛盾する記載
- research.md は Phase 0 の技術調査であり，plan.md / spec.md と矛盾する場合は後者が優先される前提だが，その優先関係自体が明示されていない点に注意
- 本チェックリストは**自動保存移行**に特化。サイドバー UI・ページ階層・D&D は frontend-ux.md / backend.md でカバー済み
