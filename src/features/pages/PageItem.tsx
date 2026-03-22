import { useEffect, useRef, useState } from "react";
import type { Page } from "./types";

interface PageItemProps {
  page: Page;
  onUpdateTitle: (id: string, title: string) => Promise<unknown>;
  onRequestDelete: (page: Page) => void;
  onPageClick: (page: Page) => void;
}

export function PageItem({
  page,
  onUpdateTitle,
  onRequestDelete,
  onPageClick,
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
    <div className="flex items-center gap-3 px-3 py-2 border-b border-border last:border-b-0">
      {editing ? (
        <input
          ref={inputRef}
          className="flex-1 p-1 border border-ring rounded-sm text-[length:inherit] focus:outline-none"
          type="text"
          value={editValue}
          onChange={(e) => setEditValue(e.target.value)}
          onBlur={confirmEdit}
          onKeyDown={handleKeyDown}
          maxLength={255}
        />
      ) : (
        <button
          type="button"
          className="flex-1 cursor-pointer p-1 rounded-sm bg-transparent border-none font-[inherit] text-left hover:bg-accent"
          onClick={() => onPageClick(page)}
          onDoubleClick={startEditing}
          title="クリックで開く / ダブルクリックで編集"
        >
          {page.title}
        </button>
      )}
      <span className="text-muted-foreground text-xs whitespace-nowrap">
        {createdDate}
      </span>
      <button
        type="button"
        className="bg-transparent border-none text-muted-foreground/50 text-xl cursor-pointer px-1 leading-none hover:text-destructive"
        onClick={() => onRequestDelete(page)}
        title="削除"
      >
        &times;
      </button>
    </div>
  );
}
