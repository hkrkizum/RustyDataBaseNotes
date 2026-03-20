import styles from "./DeleteConfirmModal.module.css";
import type { Page } from "./types";

interface DeleteConfirmModalProps {
  page: Page;
  onConfirm: () => void;
  onCancel: () => void;
}

export function DeleteConfirmModal({
  page,
  onConfirm,
  onCancel,
}: DeleteConfirmModalProps) {
  return (
    <div className={styles.overlay} onClick={onCancel}>
      <div className={styles.modal} onClick={(e) => e.stopPropagation()}>
        <h3 className={styles.title}>ページを削除しますか？</h3>
        <p className={styles.message}>
          「{page.title}」を削除します。この操作は元に戻せません。
        </p>
        <div className={styles.actions}>
          <button className={styles.cancelButton} onClick={onCancel}>
            キャンセル
          </button>
          <button className={styles.confirmButton} onClick={onConfirm}>
            削除
          </button>
        </div>
      </div>
    </div>
  );
}
