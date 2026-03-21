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
}

export function DatabaseListView({
  pages,
  databases,
  loading,
  onPageClick,
  onDatabaseClick,
  onRequestDeletePage,
}: DatabaseListViewProps) {
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
    <div className={styles.list}>
      {items.map((item) => {
        if (item.kind === "database") {
          return (
            <button
              type="button"
              key={item.data.id}
              className={styles.item}
              onClick={() => onDatabaseClick(item.data)}
            >
              <span className={styles.icon}>📊</span>
              <span className={styles.title}>{item.data.title}</span>
              <span className={styles.badge}>データベース</span>
            </button>
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
  );
}
