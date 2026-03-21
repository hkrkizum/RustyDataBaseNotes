import { type FormEvent, useCallback, useState } from "react";
import type { Page } from "../pages/types";
import { AddPageModal } from "./AddPageModal";
import styles from "./TableView.module.css";
import type { DatabaseDto } from "./types";
import { useTableData } from "./useTableData";

interface TableViewProps {
  database: DatabaseDto;
  onNavigateBack: () => void;
}

export function TableView({ database, onNavigateBack }: TableViewProps) {
  const [newTitle, setNewTitle] = useState("");
  const [pages, setPages] = useState<Page[]>([]);
  const [showModal, setShowModal] = useState(false);

  const { addPageToDatabase, addExistingPageToDatabase, listStandalonePages } =
    useTableData(database.id);

  const handleCreatePage = useCallback(
    async (e: FormEvent) => {
      e.preventDefault();
      const trimmed = newTitle.trim();
      if (!trimmed) return;
      const page = await addPageToDatabase(trimmed);
      if (page) {
        setPages((prev) => [page, ...prev]);
        setNewTitle("");
      }
    },
    [newTitle, addPageToDatabase],
  );

  const handleAddExisting = useCallback(
    async (pageId: string): Promise<Page | null> => {
      const page = await addExistingPageToDatabase(pageId);
      if (page) {
        setPages((prev) => [page, ...prev]);
      }
      return page;
    },
    [addExistingPageToDatabase],
  );

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <button
          type="button"
          className={styles.backBtn}
          onClick={onNavigateBack}
        >
          ← 戻る
        </button>
        <h2 className={styles.title}>{database.title}</h2>
      </div>
      <div className={styles.actions}>
        <form className={styles.form} onSubmit={handleCreatePage}>
          <input
            className={styles.input}
            type="text"
            placeholder="新しいページ名"
            value={newTitle}
            onChange={(e) => setNewTitle(e.target.value)}
          />
          <button type="submit" className={styles.submitBtn}>
            追加
          </button>
        </form>
        <button
          type="button"
          className={styles.existingBtn}
          onClick={() => setShowModal(true)}
        >
          既存ページを追加
        </button>
      </div>
      {pages.length === 0 ? (
        <div className={styles.emptyState}>
          <p>ページがありません</p>
          <p className={styles.hint}>
            ページを追加してテーブルを構築しましょう
          </p>
        </div>
      ) : (
        <ul className={styles.pageList}>
          {pages.map((page) => (
            <li key={page.id} className={styles.pageItem}>
              {page.title}
            </li>
          ))}
        </ul>
      )}
      {showModal && (
        <AddPageModal
          onAddPage={handleAddExisting}
          listStandalonePages={listStandalonePages}
          onClose={() => setShowModal(false)}
        />
      )}
    </div>
  );
}
