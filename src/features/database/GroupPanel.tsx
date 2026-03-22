import { useCallback } from "react";
import styles from "./FilterPanel.module.css";
import type { GroupConditionDto, PropertyDto } from "./types";

interface GroupPanelProps {
  properties: PropertyDto[];
  currentCondition: GroupConditionDto | null;
  onApply: (condition: GroupConditionDto | null) => void;
  onClose: () => void;
}

export function GroupPanel({
  properties,
  currentCondition,
  onApply,
  onClose,
}: GroupPanelProps) {
  const handleChange = useCallback(
    (propertyId: string) => {
      if (propertyId === "") {
        onApply(null);
      } else {
        onApply({ propertyId });
      }
      onClose();
    },
    [onApply, onClose],
  );

  const handleClear = useCallback(() => {
    onApply(null);
    onClose();
  }, [onApply, onClose]);

  return (
    <div className={styles.panel}>
      <div className={styles.header}>
        <span className={styles.title}>グルーピング</span>
        <button type="button" className={styles.closeBtn} onClick={onClose}>
          ×
        </button>
      </div>
      <div className={styles.conditionList}>
        <div className={styles.conditionRow}>
          <select
            className={styles.select}
            value={currentCondition?.propertyId ?? ""}
            onChange={(e) => handleChange(e.target.value)}
          >
            <option value="">なし</option>
            {properties.map((p) => (
              <option key={p.id} value={p.id}>
                {p.name}
              </option>
            ))}
          </select>
        </div>
      </div>
      {currentCondition && (
        <div className={styles.actions}>
          <button
            type="button"
            className={styles.clearBtn}
            onClick={handleClear}
          >
            グルーピング解除
          </button>
        </div>
      )}
    </div>
  );
}
