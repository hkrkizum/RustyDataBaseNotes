# Research: IPC テストおよび E2E テスト

**Branch**: `005-ipc-e2e-tests` | **Date**: 2026-03-22

## R-001: IPC テストアーキテクチャ — ハンドラ内部関数抽出方式

### Decision

各コマンドハンドラから Tauri `State<'_, AppState>` に依存しない内部ロジック関数を
`pub(crate)` として抽出し，テストはこの内部関数を直接呼び出す。

### Rationale

- **既存の問題**: `#[tauri::command]` 関数は `State<'_, AppState>` を受け取るが，
  `State` は Tauri のマネージドステート機構でしか構築できない
- **内部関数抽出のメリット**:
  - Tauri ランタイム（GUI, IPC シリアライゼーション層）を一切必要としない
  - テスト対象は ID パース，リポジトリ呼び出し，DTO 変換，エラーマッピングの全ロジック
  - `AppState` は公開フィールド（`pub db`, `pub sessions`）を持つため，テストで直接構築可能
  - SRP に適合: Tauri State 抽出の責務とビジネスロジックの責務を分離

### Pattern

```rust
// 内部ロジック関数（テスト可能）
pub(crate) async fn create_database_inner(
    state: &AppState,
    title: String,
) -> Result<DatabaseDto, CommandError> {
    let title = DatabaseTitle::try_from(title)?;
    let database = Database::new(title);
    let repo = SqlxDatabaseRepository::new(state.db.clone());
    repo.create(&database).await?;
    let view = View::new_default(database.id().clone());
    let view_repo = SqlxViewRepository::new(state.db.clone());
    view_repo.save(&view).await?;
    Ok(DatabaseDto::from(database))
}

// Tauri コマンドラッパー（1 行委譲）
#[tauri::command]
pub async fn create_database(
    state: State<'_, AppState>,
    title: String,
) -> Result<DatabaseDto, CommandError> {
    create_database_inner(&state, title).await
}
```

`State<'_, AppState>` は `Deref<Target = AppState>` を実装するため，`&state` は
auto-deref coercion により `&AppState` に変換される。

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|-----------------|
| `tauri::test::MockRuntime` | Tauri テストユーティリティの MockRuntime はランタイム依存を追加。仕様の「Tauri ランタイム不要」要件に反する |
| リポジトリ層のみテスト | 既に infrastructure 層テストが存在。IPC 層の ID パース・DTO 変換・エラーマッピングがテスト対象から漏れる |
| `State` を構築するモック | `tauri::State` はプライベートなコンストラクタのため外部から構築不可 |

---

## R-002: テストデータベースセットアップ — 一時ファイル方式

### Decision

各 IPC テストはテストごとにユニークな一時 SQLite ファイルを作成し，マイグレーションを適用し，
テスト後に削除する。

### Rationale

- **仕様要件**: FR-003「テストごとに一時 SQLite ファイルを作成」（MUST）
- **分離保証**: ファイルベースのため，テスト間のコネクションプール共有や WAL 競合を完全に排除
- **既存パターンとの整合**: `infrastructure::persistence::database` テストが同じ方式を採用

### Pattern

```rust
/// Creates a test `AppState` with a unique temporary SQLite database.
pub(crate) async fn setup_test_state() -> (AppState, TempDbGuard) {
    let db_dir = std::env::temp_dir().join(format!("rdbn_ipc_{}", uuid::Uuid::now_v7()));
    std::fs::create_dir_all(&db_dir).expect("create temp dir");
    let db_path = db_dir.join("test.db");

    let pool = database::init_pool(&db_path).await.expect("init test pool");

    let state = AppState {
        db: pool,
        sessions: tokio::sync::Mutex::new(HashMap::new()),
    };

    let guard = TempDbGuard { path: db_dir };
    (state, guard)
}

/// RAII guard that deletes the temporary database directory on drop.
pub(crate) struct TempDbGuard {
    path: std::path::PathBuf,
}

impl Drop for TempDbGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}
```

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|-----------------|
| In-memory SQLite (`:memory:`) | 仕様が明示的にファイルベースを要求。WAL モードの動作がファイルベースと異なる可能性 |
| 共有テスト DB + トランザクションロールバック | テスト並列実行時のロック競合リスク。テスト順序依存の排除が困難 |

---

## R-003: IPC テストのクレート内配置

### Decision

IPC テストは `src-tauri/src/ipc/tests/` に `#[cfg(test)]` モジュールとして配置する。
`src-tauri/tests/`（クレート外統合テスト）には配置しない。

### Rationale

- **可視性制約**: 内部ロジック関数は `pub(crate)` であり，クレート外の `tests/` ディレクトリからは
  アクセスできない
- **コンパイル効率**: クレート内テストは同一コンパイルユニットに含まれ，追加のコンパイルターゲットが
  発生しない
- **既存パターンとの整合**: プロジェクト内の既存テスト（domain, infrastructure 層）はすべて
  `#[cfg(test)]` モジュールとして実装されている

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|-----------------|
| `src-tauri/tests/` に配置 | `pub(crate)` 関数にアクセスできない。`pub` に変更すると不要な公開 API が増える |
| `ipc` モジュールを `pub` に変更 | ライブラリクレートとしての公開 API が不必要に拡大する |

---

## R-004: E2E テストフレームワーク — tauri-driver + WebDriverIO

### Decision

E2E テストは `tauri-driver`（Tauri v2 対応）を WebDriver サーバーとして使用し，
WebDriverIO（Node.js）をクライアントとしてテストを記述・実行する。

### Rationale

- **仕様要件**: FR-006「`tauri-driver` + WebDriverIO を使用」（MUST）
- **Tauri 公式サポート**: `tauri-driver` は Tauri v2 の公式 WebDriver ラッパー。
  Linux 環境では WebKitGTK の WebDriver 実装を利用する
- **WebDriverIO**: 成熟した WebDriver クライアント。TypeScript サポート，豊富なセレクタ API，
  wait/retry 機構を備える

### Setup

```text
1. tauri-driver のインストール:
   cargo install tauri-driver

2. アプリのデバッグビルド:
   cargo build --manifest-path src-tauri/Cargo.toml

3. E2E 依存のインストール:
   cd e2e && pnpm install

4. テスト実行:
   cargo make e2e
   （内部: tauri-driver 起動 → WebDriverIO テスト実行 → tauri-driver 停止）
```

### WSL2 環境での GUI テスト

- **WSLg** (Windows 11 標準搭載): X11/Wayland 描画を自動的にサポート。
  WSLg が有効であれば Tauri アプリの GUI テストが WSL2 内で直接動作する
- **ヘッドレス代替**: WSLg が無効な環境では `xvfb-run` を使用:
  `xvfb-run cargo make e2e`

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|-----------------|
| Playwright + Tauri | Tauri v2 の WebView2/WebKitGTK に対する Playwright サポートが限定的。公式サポートなし |
| Cypress | Electron ベースのテストランナー。Tauri の WebView を直接制御できない |
| Selenium | WebDriverIO が同じ WebDriver プロトコルを使用しつつ，より簡潔な API を提供 |

---

## R-005: E2E テストのデータベース分離

### Decision

E2E テストスイート開始時に一時 DB を作成し，各シナリオ前にデータリセット
（テーブルクリアまたはマイグレーション再適用）を行う。

### Rationale

- **仕様要件**: CC-001「スイート開始時に一時 DB を作成し，各シナリオ前にデータリセットする」
- **パフォーマンス**: E2E テストはアプリ起動コストが高いため，テストごとのアプリ再起動は避ける。
  データベースのみリセットすることでスイート全体の実行時間を短縮する

### Pattern

- E2E テスト用の環境変数 `RDBN_DB_PATH` で一時 DB パスを指定
- アプリの `setup()` フックで `RDBN_DB_PATH` が設定されている場合はそのパスを使用
- 各テストシナリオの `beforeEach` で全テーブルの行を DELETE（マイグレーション再適用より高速）
- スイート終了後に一時 DB ファイルを削除

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|-----------------|
| テストごとにアプリ再起動 | アプリビルド・起動コストが高く，E2E スイート全体の実行時間が数十分に膨らむ |
| 本番 DB を使用してリストア | 本番データへの影響リスク。仕様の分離要件に違反 |

---

## R-006: Makefile.toml への E2E タスク統合

### Decision

`cargo make e2e` タスクを追加し，以下の手順を自動化する:
1. デバッグビルド
2. `tauri-driver` をバックグラウンド起動
3. WebDriverIO テスト実行
4. `tauri-driver` 停止・クリーンアップ

### Rationale

- **仕様要件**: CC-005「E2E テストは独立タスク `cargo make e2e` として提供」（MUST）
- **QA パイプラインとの分離**: `qa` タスクには含めず，マージ前または手動で実行する
  （仕様 Clarification: 「独立タスクとして分離し，qa には含めず手動またはマージ前に実行」）

### Pattern

```toml
[tasks.e2e]
description = "Run E2E tests (tauri-driver + WebDriverIO)"
script = [
  "cargo build --manifest-path src-tauri/Cargo.toml",
  "tauri-driver &",
  "DRIVER_PID=$!",
  "sleep 2",
  "cd e2e && pnpm wdio run wdio.conf.ts || EXIT_CODE=$?",
  "kill $DRIVER_PID 2>/dev/null || true",
  "exit ${EXIT_CODE:-0}",
]
```

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|-----------------|
| `qa` タスクに統合 | E2E テストはビルド・起動コストが高く，日常的な QA チェックを遅延させる。仕様で明示的に分離が指定されている |
| npm scripts のみ | cargo-make で統一されているプロジェクト慣行に反する |
