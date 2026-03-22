# Quickstart: IPC テストおよび E2E テスト

**Branch**: `005-ipc-e2e-tests` | **Date**: 2026-03-22

## IPC テストの実行

```bash
# 全 IPC テスト実行（cargo-nextest）
cargo make test

# IPC テストのみフィルタ実行
TEST_FILTER="ipc::tests" cargo make test-filter

# ドメイン別フィルタ
TEST_FILTER="ipc::tests::database" cargo make test-filter
TEST_FILTER="ipc::tests::editor" cargo make test-filter
```

## E2E テストの実行

### 前提条件

```bash
# tauri-driver のインストール（初回のみ）
# Tauri v2 互換バージョンが必要
cargo install tauri-driver <!-- refined by checklist-apply: P-04 -->

# E2E テスト依存のインストール（初回のみ）
cd e2e && pnpm install && cd ..
```

### 実行

```bash
# E2E テスト全体実行（ビルド + tauri-driver 起動 + テスト + 停止）
cargo make e2e

# WSLg 無効環境でのヘッドレス実行
xvfb-run cargo make e2e
```

## 品質ゲート

```bash
# 日常的な QA（IPC テストを含む，E2E は含まない）
cargo make qa

# マージ前の完全検証
cargo make qa && cargo make e2e
```

## テストの追加方法

### IPC テストの追加

1. `src-tauri/src/ipc/tests/<domain>_commands_test.rs` を開く
2. `#[tokio::test]` 関数を追加:

```rust
#[tokio::test]
async fn <command>_<scenario>() {
    let (state, _guard) = setup_test_state().await;
    // Arrange: テストデータの準備
    // Act: 内部関数の呼び出し
    let result = <command>_inner(&state, /* args */).await;
    // Assert: 結果の検証
    assert!(result.is_ok());
}
```

### E2E テストの追加

1. `e2e/specs/<workflow>.spec.ts` を開く
2. テストシナリオを追加:

```typescript
describe('<workflow>', () => {
    beforeEach(async () => {
        await clearDatabase();
    });

    it('should <expected behavior>', async () => {
        // UI 操作
        // アサーション
    });
});
```
