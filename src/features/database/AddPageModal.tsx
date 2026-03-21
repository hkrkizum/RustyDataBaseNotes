import { useCallback, useEffect, useState } from "react";
import type { Page } from "../pages/types";
import styles from "./AddPageModal.module.css";

interface AddPageModalProps {
  onAddPage: (pageId: string) => Promise<Page | null>;
  listStandalonePages: () => Promise<Page[]>;
  onClose: () => void;
}

export function AddPageModal({
  onAddPage,
  listStandalonePages,
  onClose,
}: AddPageModalProps) {
  const [standalonePages, setStandalonePages] = useState<Page[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    listStandalonePages().then((pages) => {
      setStandalonePages(pages);
      setLoading(false);
    });
  }, [listStandalonePages]);

  const handleAdd = useCallback(
    async (pageId: string) => {
      const result = await onAddPage(pageId);
      if (result) {
        setStandalonePages((prev) => prev.filter((p) => p.id !== pageId));
      }
    },
    [onAddPage],
  );

  return (
    <>
      {/* biome-ignore lint/a11y/noStaticElementInteractions: overlay backdrop dismiss */}
      <div
        className={styles.overlay}
        role="presentation"
        onClick={onClose}
        onKeyDown={(e) => {
          if (e.key === "Escape") onClose();
        }}
      >
        {/* biome-ignore lint/a11y/noStaticElementInteractions: stopPropagation on modal container */}
        {/* biome-ignore lint/a11y/useKeyWithClickEvents: stopPropagation on modal container */}
        <div className={styles.modal} onClick={(e) => e.stopPropagation()}>
          <div className={styles.header}>
            <h3 className={styles.title}>既存ページを追加</h3>
            <button type="button" className={styles.closeBtn} onClick={onClose}>
              x
            </button>
          </div>
          <div className={styles.body}>
            {loading && <p className={styles.hint}>読み込み中...</p>}
            {!loading && standalonePages.length === 0 && (
              <p className={styles.hint}>追加可能なページがありません</p>
            )}
            {!loading && standalonePages.length > 0 && (
              <ul className={styles.list}>
                {standalonePages.map((page) => (
                  <li key={page.id} className={styles.item}>
                    <span className={styles.pageName}>{page.title}</span>
                    <button
                      type="button"
                      className={styles.addBtn}
                      onClick={() => handleAdd(page.id)}
                    >
                      追加
                    </button>
                  </li>
                ))}
              </ul>
            )}
          </div>
        </div>
      </div>
    </>
  );
}
