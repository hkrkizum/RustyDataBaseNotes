import { useState } from "react";
import type { Page } from "../pages/types";
import styles from "./DatabaseListView.module.css";
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
    return <div className={styles.empty}>読み込み中...</div>;
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
      <div className={styles.empty}>
        <p>ページやデータベースがありません</p>
        <p className={styles.hint}>上のフォームから新しく作成してください</p>
      </div>
    );
  }

  return (
    <>
      <div className={styles.list}>
        {items.map((item) => {
          if (item.kind === "database") {
            return (
              <div key={item.data.id} className={styles.item}>
                <button
                  type="button"
                  className={styles.itemBtn}
                  onClick={() => onDatabaseClick(item.data)}
                >
                  <span className={styles.icon}>📊</span>
                  <span className={styles.title}>{item.data.title}</span>
                  <span className={styles.badge}>データベース</span>
                </button>
                {onRequestDeleteDatabase && (
                  <button
                    type="button"
                    className={styles.deleteBtn}
                    onClick={() => setConfirmDeleteDb(item.data)}
                  >
                    削除
                  </button>
                )}
              </div>
            );
          }
          return (
            <div key={item.data.id} className={styles.item}>
              <button
                type="button"
                className={styles.itemBtn}
                onClick={() => onPageClick(item.data)}
              >
                <span className={styles.icon}>📄</span>
                <span className={styles.title}>{item.data.title}</span>
              </button>
              <button
                type="button"
                className={styles.deleteBtn}
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
          className={styles.confirmOverlay}
          role="presentation"
          onClick={() => setConfirmDeleteDb(null)}
          onKeyDown={(e) => {
            if (e.key === "Escape") setConfirmDeleteDb(null);
          }}
        >
          {/* biome-ignore lint/a11y/noStaticElementInteractions: confirm dialog */}
          {/* biome-ignore lint/a11y/useKeyWithClickEvents: confirm dialog */}
          <div
            className={styles.confirmDialog}
            onClick={(e) => e.stopPropagation()}
          >
            <p className={styles.confirmMessage}>
              データベース「{confirmDeleteDb.title}
              」を削除しますか？プロパティと値はすべて削除されます。ページ自体は残ります。
            </p>
            <div className={styles.confirmActions}>
              <button
                type="button"
                className={styles.confirmCancelBtn}
                onClick={() => setConfirmDeleteDb(null)}
              >
                キャンセル
              </button>
              <button
                type="button"
                className={styles.confirmDeleteBtn}
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
