import styles from "./BlockItem.module.css";
import type { Block } from "./types";

interface BlockItemProps {
  block: Block;
  isFirst: boolean;
  isLast: boolean;
  onEditContent: (blockId: string, content: string) => void;
  onMoveUp: (blockId: string) => void;
  onMoveDown: (blockId: string) => void;
  onRemove: (blockId: string) => void;
}

export function BlockItem({
  block,
  isFirst,
  isLast,
  onEditContent: _onEditContent,
  onMoveUp,
  onMoveDown,
  onRemove,
}: BlockItemProps) {
  return (
    <div className={styles.item}>
      <div className={styles.content}>
        {block.content || (
          <span className={styles.placeholder}>空のブロック</span>
        )}
      </div>
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
