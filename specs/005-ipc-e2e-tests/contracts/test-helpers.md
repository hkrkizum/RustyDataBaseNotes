# Contract: テストヘルパー API

**Branch**: `005-ipc-e2e-tests` | **Date**: 2026-03-22

## IPC テストヘルパー（Rust）

### モジュール: `src-tauri/src/ipc/tests/helpers.rs`

#### `setup_test_state() -> (AppState, TempDbGuard)`

テスト用の `AppState` と一時 DB ガードを返す。

- 一時ディレクトリ `{temp_dir}/rdbn_ipc_{uuid_v7}/` を作成
- `database::init_pool()` で SQLite プールを初期化（マイグレーション適用，FK 有効化）
- `AppState` を構築（空の sessions マップ）
- `TempDbGuard` を返す（Drop でディレクトリごと削除）

**呼び出し例**:

```rust
#[tokio::test]
async fn test_create_database() {
    let (state, _guard) = setup_test_state().await;
    let result = create_database_inner(&state, "My Database".to_string()).await;
    assert!(result.is_ok());
}
```

#### `TempDbGuard`

```rust
pub(crate) struct TempDbGuard {
    path: PathBuf,
}

impl Drop for TempDbGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}
```

### 内部ロジック関数の命名規則

各コマンドハンドラに対応する内部関数は `<command_name>_inner` の命名とする。

| コマンド | 内部関数 | シグネチャ |
|----------|----------|-----------|
| `create_database` | `create_database_inner` | `(&AppState, String) -> Result<DatabaseDto, CommandError>` |
| `list_databases` | `list_databases_inner` | `(&AppState) -> Result<Vec<DatabaseDto>, CommandError>` |
| `get_database` | `get_database_inner` | `(&AppState, String) -> Result<DatabaseDto, CommandError>` |
| `update_database_title` | `update_database_title_inner` | `(&AppState, String, String) -> Result<DatabaseDto, CommandError>` |
| `delete_database` | `delete_database_inner` | `(&AppState, String) -> Result<(), CommandError>` |
| `create_page` | `create_page_inner` | `(&AppState, String) -> Result<PageDto, CommandError>` |
| `list_pages` | `list_pages_inner` | `(&AppState) -> Result<Vec<PageDto>, CommandError>` |
| `get_page` | `get_page_inner` | `(&AppState, String) -> Result<PageDto, CommandError>` |
| `update_page_title` | `update_page_title_inner` | `(&AppState, String, String) -> Result<PageDto, CommandError>` |
| `delete_page` | `delete_page_inner` | `(&AppState, String) -> Result<(), CommandError>` |
| `open_editor` | `open_editor_inner` | `(&AppState, String) -> Result<EditorStateDto, CommandError>` |
| `close_editor` | `close_editor_inner` | `(&AppState, String) -> Result<(), CommandError>` |
| `add_block` | `add_block_inner` | `(&AppState, String) -> Result<EditorStateDto, CommandError>` |
| `edit_block_content` | `edit_block_content_inner` | `(&AppState, String, String, String) -> Result<EditorStateDto, CommandError>` |
| `move_block_up` | `move_block_up_inner` | `(&AppState, String, String) -> Result<EditorStateDto, CommandError>` |
| `move_block_down` | `move_block_down_inner` | `(&AppState, String, String) -> Result<EditorStateDto, CommandError>` |
| `remove_block` | `remove_block_inner` | `(&AppState, String, String) -> Result<EditorStateDto, CommandError>` |
| `save_editor` | `save_editor_inner` | `(&AppState, String) -> Result<EditorStateDto, CommandError>` |
| `add_property` | `add_property_inner` | `(&AppState, String, String, PropertyType, Option<PropertyConfig>) -> Result<PropertyDto, CommandError>` |
| `list_properties` | `list_properties_inner` | `(&AppState, String) -> Result<Vec<PropertyDto>, CommandError>` |
| `update_property_name` | `update_property_name_inner` | `(&AppState, String, String) -> Result<PropertyDto, CommandError>` |
| `update_property_config` | `update_property_config_inner` | `(&AppState, String, PropertyConfig) -> Result<PropertyDto, CommandError>` |
| `reorder_properties` | `reorder_properties_inner` | `(&AppState, String, Vec<String>) -> Result<Vec<PropertyDto>, CommandError>` |
| `delete_property` | `delete_property_inner` | `(&AppState, String) -> Result<(), CommandError>` |
| `reset_select_option` | `reset_select_option_inner` | `(&AppState, String, String) -> Result<(), CommandError>` |
| `set_property_value` | `set_property_value_inner` | `(&AppState, String, String, serde_json::Value) -> Result<PropertyValueDto, CommandError>` |
| `clear_property_value` | `clear_property_value_inner` | `(&AppState, String, String) -> Result<(), CommandError>` |
| `add_page_to_database` | `add_page_to_database_inner` | `(&AppState, String, String) -> Result<PageDto, CommandError>` |
| `add_existing_page_to_database` | `add_existing_page_to_database_inner` | `(&AppState, String, String) -> Result<PageDto, CommandError>` |
| `list_standalone_pages` | `list_standalone_pages_inner` | `(&AppState) -> Result<Vec<PageDto>, CommandError>` |
| `remove_page_from_database` | `remove_page_from_database_inner` | `(&AppState, String) -> Result<(), CommandError>` |
| `get_table_data` | `get_table_data_inner` | `(&AppState, String) -> Result<TableDataDto, CommandError>` |
| `get_view` | `get_view_inner` | `(&AppState, String) -> Result<ViewDto, CommandError>` |
| `reset_view` | `reset_view_inner` | `(&AppState, String) -> Result<ViewDto, CommandError>` |
| `update_sort_conditions` | `update_sort_conditions_inner` | `(&AppState, String, Vec<SortConditionInput>) -> Result<ViewDto, CommandError>` |
| `update_filter_conditions` | `update_filter_conditions_inner` | `(&AppState, String, Vec<FilterConditionInput>) -> Result<ViewDto, CommandError>` |
| `update_group_condition` | `update_group_condition_inner` | `(&AppState, String, Option<GroupConditionInput>) -> Result<ViewDto, CommandError>` |
| `toggle_group_collapsed` | `toggle_group_collapsed_inner` | `(&AppState, String, Option<String>) -> Result<ViewDto, CommandError>` |

## E2E テストヘルパー（TypeScript）

### モジュール: `e2e/helpers/app.ts`

#### WebDriverIO Configuration

```typescript
// wdio.conf.ts
export const config: WebdriverIO.Config = {
    runner: 'local',
    hostname: 'localhost',
    port: 4444,
    specs: ['./specs/**/*.spec.ts'],
    capabilities: [{
        'tauri:options': {
            application: '../src-tauri/target/debug/rusty-database-notes',
        },
    }],
    framework: 'mocha',
    reporters: ['spec'],
    mochaOpts: {
        timeout: 30000,
    },
};
```

#### ヘルパー関数

| 関数 | 目的 |
|------|------|
| `clearDatabase()` | 全テーブルの行を DELETE（シナリオ前リセット） |
| `waitForApp()` | アプリのメインウィンドウが表示されるまで待機 |
| `findByTestId(id)` | `data-testid` 属性でエレメントを検索 |
