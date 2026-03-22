import { useState } from "react";
import type { Page } from "../pages/types";
import type { DatabaseDto } from "./types";

type ListItem =
  | { kind: "page"; data: Page }
  | { kind: "database"; data: DatabaseDto };

interface DatabaseListViewProps {
  pages: Page[];
  databases: DatabaseDto[];
  loading: boolean;
  onPageClick: (page: Page) => void;
  onDatabaseClick: (database: DatabaseDto) => void;
  onRequestDeletePage: (page: Page) => void;
  onRequestDeleteDatabase?: (database: DatabaseDto) => void;
}

export function DatabaseListView({
  pages,
  databases,
  loading,
  onPageClick,
  onDatabaseClick,
  onRequestDeletePage,
  onRequestDeleteDatabase,
}: DatabaseListViewProps) {
  const [confirmDeleteDb, setConfirmDeleteDb] = useState<DatabaseDto | null>(
    null,
  );

  if (loading) {
    return (
      <div className="text-center p-8 text-muted-foreground">読み込み中...</div>
    );
  }

  // Merge and sort by createdAt DESC
  const items: ListItem[] = [
    ...databases.map((d): ListItem => ({ kind: "database", data: d })),
    ...pages
      .filter((p) => p.databaseId === null)
      .map((p): ListItem => ({ kind: "page", data: p })),
  ].sort((a, b) => b.data.createdAt.localeCompare(a.data.createdAt));

  if (items.length === 0) {
    return (
      <div className="text-center p-8 text-muted-foreground">
        <p>ページやデータベースがありません</p>
        <p className="text-sm text-muted-foreground/70">
          上のフォームから新しく作成してください
        </p>
      </div>
    );
  }

  return (
    <>
      <div className="flex flex-col gap-2">
        {items.map((item) => {
          if (item.kind === "database") {
            return (
              <div
                key={item.data.id}
                className="flex items-center gap-2 px-4 py-3 border border-border rounded-md transition-colors hover:bg-accent"
              >
                <button
                  type="button"
                  className="flex items-center gap-2 flex-1 overflow-hidden p-0 border-none bg-transparent font-inherit text-left cursor-pointer"
                  onClick={() => onDatabaseClick(item.data)}
                >
                  <span className="text-xl shrink-0">📊</span>
                  <span className="flex-1 overflow-hidden text-ellipsis whitespace-nowrap">
                    {item.data.title}
                  </span>
                  <span className="text-xs px-2 py-0.5 rounded bg-primary/10 text-primary">
                    データベース
                  </span>
                </button>
                {onRequestDeleteDatabase && (
                  <button
                    type="button"
                    className="text-sm px-2 py-0.5 border border-border rounded cursor-pointer text-muted-foreground hover:text-destructive hover:border-destructive"
                    onClick={() => setConfirmDeleteDb(item.data)}
                  >
                    削除
                  </button>
                )}
              </div>
            );
          }
          return (
            <div
              key={item.data.id}
              className="flex items-center gap-2 px-4 py-3 border border-border rounded-md transition-colors hover:bg-accent"
            >
              <button
                type="button"
                className="flex items-center gap-2 flex-1 overflow-hidden p-0 border-none bg-transparent font-inherit text-left cursor-pointer"
                onClick={() => onPageClick(item.data)}
              >
                <span className="text-xl shrink-0">📄</span>
                <span className="flex-1 overflow-hidden text-ellipsis whitespace-nowrap">
                  {item.data.title}
                </span>
              </button>
              <button
                type="button"
                className="text-sm px-2 py-0.5 border border-border rounded cursor-pointer text-muted-foreground hover:text-destructive hover:border-destructive"
                onClick={() => onRequestDeletePage(item.data)}
              >
                削除
              </button>
            </div>
          );
        })}
      </div>

      {confirmDeleteDb && (
        /* biome-ignore lint/a11y/noStaticElementInteractions: confirm overlay */
        <div
          className="fixed inset-0 bg-black/40 flex items-center justify-center z-[100]"
          role="presentation"
          onClick={() => setConfirmDeleteDb(null)}
          onKeyDown={(e) => {
            if (e.key === "Escape") setConfirmDeleteDb(null);
          }}
        >
          {/* biome-ignore lint/a11y/noStaticElementInteractions: confirm dialog */}
          {/* biome-ignore lint/a11y/useKeyWithClickEvents: confirm dialog */}
          <div
            className="bg-card rounded-lg p-6 w-[360px] shadow-lg"
            onClick={(e) => e.stopPropagation()}
          >
            <p className="m-0 mb-4 text-[0.95rem] leading-relaxed">
              データベース「{confirmDeleteDb.title}
              」を削除しますか？プロパティと値はすべて削除されます。ページ自体は残ります。
            </p>
            <div className="flex justify-end gap-2">
              <button
                type="button"
                className="px-4 py-2 border border-border rounded cursor-pointer text-sm bg-transparent hover:bg-accent"
                onClick={() => setConfirmDeleteDb(null)}
              >
                キャンセル
              </button>
              <button
                type="button"
                className="px-4 py-2 border-none rounded bg-destructive text-white cursor-pointer text-sm hover:bg-destructive/80"
                onClick={() => {
                  if (onRequestDeleteDatabase) {
                    onRequestDeleteDatabase(confirmDeleteDb);
                  }
                  setConfirmDeleteDb(null);
                }}
              >
                削除する
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
}
