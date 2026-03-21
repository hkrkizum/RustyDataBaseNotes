import styles from "./EditorToolbar.module.css";

interface EditorToolbarProps {
  pageTitle: string;
  isDirty: boolean;
  onBack: () => void;
  onSave: () => void;
}

export function EditorToolbar({
  pageTitle,
  isDirty,
  onBack,
  onSave,
}: EditorToolbarProps) {
  return (
    <div className={styles.toolbar}>
      <button type="button" className={styles.backButton} onClick={onBack}>
        ← 戻る
      </button>
      <h2 className={styles.title}>{pageTitle}</h2>
      <span className={styles.dirtyIndicator}>{isDirty ? "未保存" : ""}</span>
      <button
        type="button"
        className={styles.saveButton}
        onClick={onSave}
        disabled={!isDirty}
      >
        保存
      </button>
    </div>
  );
}
