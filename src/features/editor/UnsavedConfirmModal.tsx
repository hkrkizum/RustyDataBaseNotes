import styles from "./UnsavedConfirmModal.module.css";

interface UnsavedConfirmModalProps {
  onDiscard: () => void;
  onCancel: () => void;
}

export function UnsavedConfirmModal({
  onDiscard,
  onCancel,
}: UnsavedConfirmModalProps) {
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
          <h3 className={styles.title}>未保存の変更があります</h3>
          <p className={styles.message}>
            未保存の変更があります。破棄しますか？
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
              className={styles.discardButton}
              onClick={onDiscard}
            >
              破棄
            </button>
          </div>
        </div>
      </div>
    </>
  );
}
