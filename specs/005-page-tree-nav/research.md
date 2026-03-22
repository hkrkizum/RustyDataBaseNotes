# Research: Page Tree Navigation

**Feature**: 005-page-tree-nav | **Date**: 2026-03-22

## R-001: デザインシステム — Tailwind CSS + shadcn/ui

### Decision

Tailwind CSS v4 + shadcn/ui（latest）を採用。shadcn CLI (`npx shadcn@latest init`) で
Vite + React プロジェクトにセットアップする。

### Rationale

- **Tailwind CSS v4**: CSS ファーストの設定方式（`@import "tailwindcss"` のみ，
  `tailwind.config.js` は不要）。Vite とのネイティブ統合により PostCSS プラグイン設定が簡素化。
- **shadcn/ui**: コンポーネントをプロジェクトにコピーする方式のため，依存関係が最小限。
  Radix UI プリミティブ上に構築され，アクセシビリティが標準装備。
  2026年3月の CLI v4 で Vite テンプレートを公式サポート。
- **組み合わせ**: shadcn/ui は Tailwind v4 を公式サポート（shadcn CLI v4）。

### Setup Steps

1. **Tailwind CSS v4 インストール**（v4 は `postcss.config.js` 不要）:
   ```bash
   pnpm add tailwindcss @tailwindcss/vite
   ```
2. **vite.config.ts に Tailwind プラグイン追加** + **パスエイリアス設定**:
   ```ts
   import tailwindcss from "@tailwindcss/vite";
   // plugins: [tailwindcss(), react()]
   // resolve: { alias: { "@": path.resolve(__dirname, "./src") } }
   ```
3. **tsconfig.json にパスエイリアス追加**:
   ```json
   { "compilerOptions": { "baseUrl": ".", "paths": { "@/*": ["./src/*"] } } }
   ```
4. **CSS エントリポイント設定**（v4 は `@import "tailwindcss"` のみ。v3 の
   `@tailwind base/components/utilities` は不要）:
   ```css
   @import "tailwindcss";
   @custom-variant dark (&:where(.dark, .dark *));
   ```
5. **shadcn/ui 初期化**:
   ```bash
   pnpm dlx shadcn@latest init
   ```
   - `components.json` 生成，`src/components/ui/` にコンポーネント配置
   - `src/lib/utils.ts` に `cn()` ヘルパー生成（`clsx` + `tailwind-merge`）
6. 必要なコンポーネントを個別追加:
   ```bash
   pnpm dlx shadcn@latest add sidebar collapsible button input \
     dropdown-menu context-menu tooltip scroll-area
   ```
7. `lucide-react` をアイコンライブラリとして使用（shadcn/ui が標準採用）
8. 既存の CSS Modules (18ファイル) + `App.css` を段階的に Tailwind ユーティリティクラスに移行

### WebKitGTK 互換性注意

Tailwind CSS v4 は最新の CSS 機能（`@layer`, `color-mix()` 等）を使用する。
Tauri on Linux は WebKitGTK をレンダリングエンジンとして使用するため，
ターゲットの WebKitGTK バージョンが Safari 16.4+ 相当以上であることを確認する必要がある。
Nix devshell の `webkitgtk_4_1` パッケージは通常この要件を満たす。

### Dark Mode

shadcn/ui の ThemeProvider パターンを使用。本プロジェクトでは OS のシステム設定に追従する
ため，`defaultTheme="system"` で固定し，手動切り替え UI は設けない。

```typescript
// ThemeProvider: "system" テーマで OS の prefers-color-scheme に追従
// document.documentElement に "light" / "dark" クラスを付与
// Tailwind v4 の dark: バリアントで自動適用
```

### Alternatives Considered

| 選択肢 | 却下理由 |
|--------|---------|
| CSS Modules 継続 | US1 でデザインシステム統一が明示要件。二重管理のコスト |
| Tailwind v3 | v4 が安定版で CLI サポートも充実。新規プロジェクトで v3 を選ぶ理由なし |
| MUI / Ant Design | 重量級。コンポーネントのカスタマイズ自由度が低く，バンドルサイズ増大 |
| Headless UI | Radix UI ベースの shadcn/ui の方が React エコシステムとの親和性が高い |

---

## R-002: ドラッグ＆ドロップライブラリ

### Decision

`@atlaskit/pragmatic-drag-and-drop` を採用。ツリー専用ユーティリティ
（`@atlaskit/pragmatic-drag-and-drop-hitbox` の `tree-item`）でツリー D&D を実装する。

### Rationale

- **ツリーファースト設計**: `tree-item` ヒットボックスユーティリティが，
  ドロップターゲットを top（兄弟として上に挿入）/ middle（子として追加）/
  bottom（兄弟として下に挿入）の3ゾーンに分割し，instruction ベースの API で
  視覚フィードバックの種類（`reorder-above`, `reorder-below`, `make-child`,
  `instruction-blocked`）を返す。本機能の要件と完全に一致。
- **最大深度バリデーション内蔵**: `tree-item` API が `maxLevel` パラメータを
  ネイティブに受け付け，深度超過時は `blocked` instruction を返す。
- **React 19 安全**: コアパッケージは React に依存しない純粋な DOM 操作。
  `findDOMNode` 不使用。フレームワーク非依存のため React バージョン互換性リスクなし。
- **Atlassian による企業サポート**: Jira / Confluence / Trello で本番使用。
  react-beautiful-dnd の後継として長期メンテナンス保証。
- **軽量**: コアパッケージ約 4.7KB。必要なパッケージのみインストール。
- **デスクトップ最適**: HTML5 Drag and Drop API ベースで OS ネイティブのドラッグ
  カーソルとプレビューを使用。Tauri デスクトップアプリに最適（タッチ非対応は問題なし）。

### Packages

```
@atlaskit/pragmatic-drag-and-drop           -- コア（draggable/droppable）
@atlaskit/pragmatic-drag-and-drop-hitbox    -- tree-item ヒット検出
@atlaskit/pragmatic-drag-and-drop-react-drop-indicator  -- ドロップインジケータ（任意）
```

### Implementation Approach

```
monitorForElements (グローバルドラッグ監視)
├── Tree Item (draggable + dropTargetForElements)
│   ├── tree-item hitbox: top zone → reorder-above
│   ├── tree-item hitbox: middle zone → make-child
│   └── tree-item hitbox: bottom zone → reorder-below
├── Drop indicator line (DropIndicator コンポーネント)
└── Blocked instruction → 禁止カーソル表示
```

- 各ツリーノードに `draggable()` と `dropTargetForElements()` を attach
- ドラッグ開始時にソースノード情報（pageId, parentId, depth）を `data` に格納
- `tree-item` の `attachInstruction()` で各ノードのドロップゾーンを設定
- `maxLevel: 5` で深度制限をヒットボックス側で処理
- `onDrag` / `onDrop` でバリデーション（循環参照，DB ページ制約）を追加実行
- バリデーション失敗時は `blocked` instruction で禁止カーソル表示
- `onDrop` 完了時に `move_page` IPC コマンドを呼び出し

### Alternatives Considered

| 選択肢 | 却下理由 |
|--------|---------|
| @dnd-kit/core | メンテナンス停滞（2023年中頃からメンテナ不在）。ツリー D&D はカスタム実装が必要で工数大。React 19 互換性が実環境で未検証 |
| @dnd-kit/react (0.x) | 新 API だが pre-release（0.1.x）。プロダクション使用にはリスクが高い |
| react-beautiful-dnd | Atlassian が非推奨宣言。pragmatic-drag-and-drop が後継 |
| react-arborist | react-dnd 依存で React 19 リスク。shadcn/ui スタイルとの統合困難。単一メンテナ |
| HTML5 Drag and Drop API 直接使用 | ツリー用ヒット検出・深度バリデーション・アクセシビリティの実装コスト大（1500-2500行推定） |

---

## R-003: サイドバーコンポーネント設計

### Decision

shadcn/ui の Sidebar コンポーネント群 + Collapsible コンポーネントを組み合わせて
ツリーナビゲーションを構築する。

### Rationale

shadcn/ui は以下の Sidebar プリミティブを提供:
- `SidebarProvider` / `useSidebar` — サイドバー状態管理（開閉）
- `Sidebar` — コンテナ（固定幅，collapsible 対応）
- `SidebarHeader` / `SidebarContent` / `SidebarFooter` — レイアウト区画
- `SidebarMenu` / `SidebarMenuItem` / `SidebarMenuButton` — メニュー項目
- `SidebarMenuSub` / `SidebarMenuSubItem` — ネスト項目
- `SidebarMenuAction` — アクションボタン（「...」メニュー等）
- `SidebarTrigger` — 開閉トグル
- `SidebarInset` — メインコンテンツ領域

Collapsible コンポーネント（Radix UI ベース）で再帰的なツリー展開/折りたたみを実現:
- `CollapsibleTrigger` — 展開/折りたたみトグル（ChevronRight アイコン回転）
- `CollapsibleContent` — 折りたたみ可能なコンテンツ領域

### Tree Rendering Strategy

```
Sidebar
├── SidebarHeader
│   ├── App title
│   └── Create button (+ dropdown: ページ / データベース)
├── SidebarContent
│   └── SidebarMenu (ルートレベル items, created_at DESC)
│       ├── SidebarMenuItem (スタンドアロンページ - 子なし)
│       │   └── SidebarMenuButton (FileText icon + title)
│       ├── SidebarMenuItem (スタンドアロンページ - 子あり)
│       │   ├── Collapsible
│       │   │   ├── CollapsibleTrigger (ChevronRight + FileText + title)
│       │   │   └── CollapsibleContent
│       │   │       └── SidebarMenuSub (再帰: 子ページ)
│       │   └── SidebarMenuAction (「...」ボタン)
│       └── SidebarMenuItem (データベース)
│           ├── Collapsible
│           │   ├── CollapsibleTrigger (ChevronRight + Table2 + title)
│           │   └── CollapsibleContent
│           │       └── SidebarMenuSub (DB 所属ページ一覧)
│           └── SidebarMenuAction (「...」ボタン)
└── SidebarTrigger (トグルボタン)
```

### Alternatives Considered

| 選択肢 | 却下理由 |
|--------|---------|
| カスタムサイドバー実装 | shadcn/ui が高品質な Sidebar コンポーネントを提供済み。車輪の再発明 |
| react-arborist | スタイルシステムが shadcn/ui と競合。統合コスト大 |

---

## R-004: エディタ自動保存パターン

### Decision

フロントエンド側で debounce 付き自動保存を実装する。debounce 間隔は **~~1000ms~~ 500ms**（※ plan.md で改訂）。
バックエンドの `EditorSession::is_dirty()` / `mark_saved()` メソッドは残す（save_editor の変更検出に使用）。`EditorStateDto::is_dirty` フィールドのみ IPC レスポンスから削除する。

### Rationale

- **1000ms debounce**: ユーザーの入力が途切れてから1秒後に保存。タイピング中の
  頻繁な IPC 呼び出しを防ぎつつ，データ損失リスクを最小化するバランス。
- **フロントエンド主導**: 保存タイミングの制御は UI 層の責務。バックエンドは
  「保存を実行する」コマンドに徹する。
- **既存の `save_editor` IPC コマンドを再利用**: コマンド自体は変更不要。
  呼び出しタイミングが「ユーザーの明示的操作」から「debounce タイマー発火」に変わるのみ。

### Implementation

```typescript
// useAutoSave hook (概念)
function useAutoSave(pageId: string, saveInterval = 1000) {
  const timerRef = useRef<number>();

  const scheduleSave = useCallback(() => {
    clearTimeout(timerRef.current);
    timerRef.current = window.setTimeout(async () => {
      try {
        await invoke("save_editor", { pageId });
      } catch (err) {
        // リトライロジック（最大3回）
        // 全失敗時は toast.warning()
      }
    }, saveInterval);
  }, [pageId, saveInterval]);

  // コンポーネントアンマウント時 or ページ切り替え時にフラッシュ
  useEffect(() => {
    return () => {
      clearTimeout(timerRef.current);
      // 即時保存（ベストエフォート）
    };
  }, [pageId]);

  return { scheduleSave };
}
```

### Backend Changes

- `EditorSession::is_dirty()` / `EditorSession::mark_saved()` メソッドは**残す**。
  `save_editor` コマンドが「変更がある場合のみ DB 書き込み」を判断するために必要。
- `EditorStateDto::is_dirty` フィールドは IPC レスポンスから**削除**（フロントエンドで不使用）。
- `save_editor` コマンドのシグネチャ・動作は変更なし。

### Frontend Removals

- `UnsavedConfirmModal` コンポーネント削除
- `EditorToolbar` の保存ボタン・未保存インジケータ削除
- Ctrl+S / Cmd+S キーボードショートカット削除
- `BlockEditor` のナビゲーション前確認ロジック削除

### Retry Strategy

- 保存失敗時: ~~1秒後にリトライ（最大3回，指数バックオフなし）~~ サイレントリトライ最大3回，間隔は指数バックオフ（1s → 2s → 4s）（※ plan.md で改訂）
- 全リトライ失敗時: ~~`toast.warning("保存に失敗しました。再試行してください。")`~~ `toast.warning("保存に失敗しました")`（表示時間: 5秒，自動消去）（※ plan.md で改訂）
- ユーザーがページを切り替えた場合: 最終保存を試行し，失敗してもナビゲーションは許可

### Alternatives Considered

| 選択肢 | 却下理由 |
|--------|---------|
| バックエンド主導の定期保存 | Tauri の IPC は呼び出し側が主導。バックエンドからの push は複雑 |
| ~~500ms debounce~~ | ~~高速タイピング時の IPC 呼び出し頻度が高すぎる~~ ※ plan.md で 500ms に改訂。実測で問題ないと判断 |
| 2000ms debounce | データ損失のウィンドウが大きすぎる |
| lodash.debounce | 外部依存の追加。useRef + setTimeout で十分 |

---

## R-005: ページ階層のドメインモデル設計

### Decision

`Page` エンティティに `parent_id: Option<PageId>` と `sort_order: i64` を追加。
階層操作のバリデーションは専用のドメインサービス `PageHierarchyService` に集約する。

### Rationale

- **`parent_id` as `Option<PageId>`**: ルートレベルページは `None`。自己参照外部キー制約で
  参照整合性を DB レベルで保証。
- **ドメインサービス分離**: 循環参照検出・深度制限チェック・DB ページ制約は，単一の Page
  エンティティの責務を超える（複数ページのグラフ走査が必要）。SRP に従い
  `PageHierarchyService` として分離する。
- **階層クエリ**: SQLite は再帰 CTE（`WITH RECURSIVE`）をサポート。祖先チェーン取得・
  子孫ツリー取得に使用する。

### Hierarchy Validation Rules

1. **循環参照検出**: ターゲット親から根まで祖先を辿り，移動対象ページが含まれないことを確認
2. **深度制限**: 移動対象の最大子孫深度 + ターゲット親の深度 + 1 ≤ 5
3. **DB ページ制約**: `database_id.is_some()` のページは `parent_id` を持てず，子も持てない
4. **自己参照禁止**: ページ自身を親にできない（循環参照の特殊ケース）

### SQL Migration (0007)

```sql
ALTER TABLE pages ADD COLUMN parent_id TEXT REFERENCES pages(id) ON DELETE SET NULL;
ALTER TABLE pages ADD COLUMN sort_order INTEGER NOT NULL DEFAULT 0;
CREATE INDEX idx_pages_parent_id ON pages(parent_id);
```

**`ON DELETE SET NULL`** を選択: 親ページ削除時に子の `parent_id` を NULL にし，
アプリケーション層で正しい昇格先（削除された親の parent_id）に更新する。
これにより DB レベルでは孤児ページが発生せず，アプリケーション層でビジネスロジック
（祖父母への昇格）を実行する余地を確保する。

### Alternatives Considered

| 選択肢 | 却下理由 |
|--------|---------|
| adjacency list + closure table | 本スコープの規模（500ページ，5階層）では過剰。再帰 CTE で十分 |
| nested set model | 書き込みコストが高い（移動時に多数の行を更新） |
| materialized path | 文字列操作による深度計算が脆弱。型安全性が低い |
| ON DELETE CASCADE | 子ページのデータ削除は仕様で明示的に禁止（FR-008） |

---

## R-006: サイドバーデータ取得パターン

### Decision

新規 IPC コマンド `list_sidebar_items` で，サイドバー表示に必要な全データを
一括取得する。レスポンスはフラットなリストとし，フロントエンドでツリー構造に変換する。

### Rationale

- **一括取得**: サイドバーは起動時とデータ変更時に全ツリーを表示する必要がある。
  ページ単位のフェッチはラウンドトリップが多く，500ページでは非効率。
- **フラットリスト → ツリー変換**: バックエンドはフラットな `SidebarItemDto[]` を返し，
  フロントエンドの `useSidebar` フックで `parentId` を基にツリー構造を構築する。
  これにより IPC のシリアライズが単純化され，バックエンドのクエリも効率的。
- **ページ + データベースの統合**: 1回の IPC 呼び出しでスタンドアロンページ，
  データベース，データベース所属ページをすべて取得。

### Response Shape

```typescript
type SidebarItemDto = {
  id: string;
  title: string;
  itemType: "page" | "database";
  parentId: string | null;       // ページの親ページ ID
  databaseId: string | null;     // データベース所属ページの場合
  createdAt: string;
};
```

### Alternatives Considered

| 選択肢 | 却下理由 |
|--------|---------|
| 既存の `list_pages` + `list_databases` を個別呼び出し | 2回の IPC ラウンドトリップ。parent_id 情報が既存の PageDto にない |
| バックエンドでツリー構造を返す | 再帰的な JSON 構造はシリアライズ/デシリアライズが複雑。フラットの方が効率的 |
| GraphQL | プロジェクトに GraphQL を導入する正当性がない |
