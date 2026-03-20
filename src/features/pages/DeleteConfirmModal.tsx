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
    <>
      {/* biome-ignore lint/a11y/noStaticElementInteractions: overlay backdrop dismiss */}
      <div
        className={styles.overlay}
        role="presentation"
        onClick={onCancel}
        onKeyDown={(e) => {
          if (e.key === "Escape") onCancel();
        }}
      >
        {/* biome-ignore lint/a11y/noStaticElementInteractions: stopPropagation on modal container */}
        {/* biome-ignore lint/a11y/useKeyWithClickEvents: stopPropagation on modal container */}
        <div className={styles.modal} onClick={(e) => e.stopPropagation()}>
          <h3 className={styles.title}>ページを削除しますか？</h3>
          <p className={styles.message}>
            「{page.title}」を削除します。この操作は元に戻せません。
          </p>
          <div className={styles.actions}>
            <button
              type="button"
              className={styles.cancelButton}
              onClick={onCancel}
            >
              キャンセル
            </button>
            <button
              type="button"
              className={styles.confirmButton}
              onClick={onConfirm}
            >
              削除
            </button>
          </div>
        </div>
      </div>
    </>
  );
}
