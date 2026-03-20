import { useEffect, useRef, useState } from "react";
import styles from "./BlockItem.module.css";
import type { Block } from "./types";

interface BlockItemProps {
  block: Block;
  isFirst: boolean;
  isLast: boolean;
  shouldFocus?: boolean;
  onEditContent: (blockId: string, content: string) => void;
  onMoveUp: (blockId: string) => void;
  onMoveDown: (blockId: string) => void;
  onRemove: (blockId: string) => void;
}

export function BlockItem({
  block,
  isFirst,
  isLast,
  shouldFocus,
  onEditContent,
  onMoveUp,
  onMoveDown,
  onRemove,
}: BlockItemProps) {
  const [localContent, setLocalContent] = useState(block.content);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    if (shouldFocus && textareaRef.current) {
      textareaRef.current.focus();
    }
  }, [shouldFocus]);

  function handleBlur() {
    if (localContent !== block.content) {
      onEditContent(block.id, localContent);
    }
  }

  // Sync local state when backend state changes (e.g. after error recovery)
  if (
    localContent !== block.content &&
    document.activeElement !== textareaRef.current
  ) {
    setLocalContent(block.content);
  }

  return (
    <div className={styles.item}>
      <textarea
        ref={textareaRef}
        className={styles.textarea}
        value={localContent}
        onChange={(e) => setLocalContent(e.target.value)}
        onBlur={handleBlur}
        maxLength={10000}
        placeholder="テキストを入力..."
      />
      <div className={styles.actions}>
        <button
          type="button"
          className={styles.actionButton}
          onClick={() => onMoveUp(block.id)}
          disabled={isFirst}
          title="上に移動"
        >
          ↑
        </button>
        <button
          type="button"
          className={styles.actionButton}
          onClick={() => onMoveDown(block.id)}
          disabled={isLast}
          title="下に移動"
        >
          ↓
        </button>
        <button
          type="button"
          className={styles.deleteButton}
          onClick={() => onRemove(block.id)}
          title="削除"
        >
          &times;
        </button>
      </div>
    </div>
  );
}
