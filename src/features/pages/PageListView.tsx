import { PageItem } from "./PageItem";
import styles from "./PageListView.module.css";
import type { Page } from "./types";

interface PageListViewProps {
  pages: Page[];
  loading: boolean;
  onUpdateTitle: (id: string, title: string) => Promise<unknown>;
  onRequestDelete: (page: Page) => void;
}

export function PageListView({
  pages,
  loading,
  onUpdateTitle,
  onRequestDelete,
}: PageListViewProps) {
  if (loading) {
    return <div className={styles.empty}>読み込み中...</div>;
  }

  if (pages.length === 0) {
    return (
      <div className={styles.empty}>
        <p>ページがありません</p>
        <p className={styles.hint}>
          上のフォームから新しいページを作成してください
        </p>
      </div>
    );
  }

  return (
    <div className={styles.list}>
      {pages.map((page) => (
        <PageItem
          key={page.id}
          page={page}
          onUpdateTitle={onUpdateTitle}
          onRequestDelete={onRequestDelete}
        />
      ))}
    </div>
  );
}
