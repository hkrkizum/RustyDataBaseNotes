import type { DatabaseDto } from "./types";
import styles from "./TableView.module.css";

interface TableViewProps {
  database: DatabaseDto;
  onNavigateBack: () => void;
}

export function TableView({ database, onNavigateBack }: TableViewProps) {
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
      <div className={styles.emptyState}>
        <p>ページがありません</p>
        <p className={styles.hint}>ページを追加してテーブルを構築しましょう</p>
      </div>
    </div>
  );
}
