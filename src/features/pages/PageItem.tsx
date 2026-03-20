import { useState, useRef, useEffect } from "react";
import styles from "./PageItem.module.css";
import type { Page } from "./types";

interface PageItemProps {
  page: Page;
  onUpdateTitle: (id: string, title: string) => Promise<unknown>;
  onRequestDelete: (page: Page) => void;
}

export function PageItem({
  page,
  onUpdateTitle,
  onRequestDelete,
}: PageItemProps) {
  const [editing, setEditing] = useState(false);
  const [editValue, setEditValue] = useState(page.title);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (editing && inputRef.current) {
      inputRef.current.focus();
      inputRef.current.select();
    }
  }, [editing]);

  function startEditing() {
    setEditValue(page.title);
    setEditing(true);
  }

  async function confirmEdit() {
    const trimmed = editValue.trim();
    if (!trimmed || trimmed === page.title) {
      setEditing(false);
      return;
    }
    const result = await onUpdateTitle(page.id, trimmed);
    if (result) {
      setEditing(false);
    }
  }

  function cancelEdit() {
    setEditing(false);
    setEditValue(page.title);
  }

  function handleKeyDown(e: React.KeyboardEvent) {
    if (e.key === "Enter") {
      confirmEdit();
    } else if (e.key === "Escape") {
      cancelEdit();
    }
  }

  const createdDate = new Date(page.createdAt).toLocaleDateString("ja-JP", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });

  return (
    <div className={styles.item}>
      {editing ? (
        <input
          ref={inputRef}
          className={styles.editInput}
          type="text"
          value={editValue}
          onChange={(e) => setEditValue(e.target.value)}
          onBlur={confirmEdit}
          onKeyDown={handleKeyDown}
          maxLength={255}
        />
      ) : (
        <span
          className={styles.title}
          onClick={startEditing}
          title="クリックして編集"
        >
          {page.title}
        </span>
      )}
      <span className={styles.date}>{createdDate}</span>
      <button
        className={styles.deleteButton}
        onClick={() => onRequestDelete(page)}
        title="削除"
      >
        &times;
      </button>
    </div>
  );
}
