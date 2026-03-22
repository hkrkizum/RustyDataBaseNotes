import { useCallback, useState } from "react";
import styles from "./FilterPanel.module.css";
import type { PropertyDto, SortConditionDto } from "./types";

interface SortPanelProps {
  properties: PropertyDto[];
  conditions: SortConditionDto[];
  onApply: (conditions: SortConditionDto[]) => void;
  onClose: () => void;
}

export function SortPanel({
  properties,
  conditions: initialConditions,
  onApply,
  onClose,
}: SortPanelProps) {
  const [conditions, setConditions] =
    useState<SortConditionDto[]>(initialConditions);

  const usedPropertyIds = new Set(conditions.map((c) => c.propertyId));
  const availableProperties = properties.filter(
    (p) => !usedPropertyIds.has(p.id),
  );

  const handleAdd = useCallback(() => {
    if (availableProperties.length === 0) return;
    setConditions((prev) => [
      ...prev,
      {
        propertyId: availableProperties[0].id,
        direction: "ascending" as const,
      },
    ]);
  }, [availableProperties]);

  const handleChange = useCallback(
    (index: number, condition: SortConditionDto) => {
      setConditions((prev) =>
        prev.map((c, i) => (i === index ? condition : c)),
      );
    },
    [],
  );

  const handleDelete = useCallback((index: number) => {
    setConditions((prev) => prev.filter((_, i) => i !== index));
  }, []);

  const handleMoveUp = useCallback((index: number) => {
    if (index <= 0) return;
    setConditions((prev) => {
      const next = [...prev];
      [next[index - 1], next[index]] = [next[index], next[index - 1]];
      return next;
    });
  }, []);

  const handleMoveDown = useCallback((index: number) => {
    setConditions((prev) => {
      if (index >= prev.length - 1) return prev;
      const next = [...prev];
      [next[index], next[index + 1]] = [next[index + 1], next[index]];
      return next;
    });
  }, []);

  const handleApply = useCallback(() => {
    onApply(conditions);
    onClose();
  }, [conditions, onApply, onClose]);

  const handleClearAll = useCallback(() => {
    onApply([]);
    onClose();
  }, [onApply, onClose]);

  return (
    <div className={styles.panel}>
      <div className={styles.header}>
        <span className={styles.title}>ソート</span>
        <button type="button" className={styles.closeBtn} onClick={onClose}>
          ×
        </button>
      </div>
      <div className={styles.conditionList}>
        {conditions.map((cond, i) => (
          <div key={`sort-${cond.propertyId}`} className={styles.conditionRow}>
            <select
              className={styles.select}
              value={cond.propertyId}
              onChange={(e) =>
                handleChange(i, { ...cond, propertyId: e.target.value })
              }
            >
              {properties
                .filter(
                  (p) => p.id === cond.propertyId || !usedPropertyIds.has(p.id),
                )
                .map((p) => (
                  <option key={p.id} value={p.id}>
                    {p.name}
                  </option>
                ))}
            </select>
            <select
              className={styles.select}
              value={cond.direction}
              onChange={(e) =>
                handleChange(i, {
                  ...cond,
                  direction: e.target.value as "ascending" | "descending",
                })
              }
            >
              <option value="ascending">昇順</option>
              <option value="descending">降順</option>
            </select>
            <button
              type="button"
              className={styles.deleteBtn}
              onClick={() => handleMoveUp(i)}
              disabled={i === 0}
              title="上へ"
            >
              ↑
            </button>
            <button
              type="button"
              className={styles.deleteBtn}
              onClick={() => handleMoveDown(i)}
              disabled={i === conditions.length - 1}
              title="下へ"
            >
              ↓
            </button>
            <button
              type="button"
              className={styles.deleteBtn}
              onClick={() => handleDelete(i)}
            >
              ×
            </button>
          </div>
        ))}
      </div>
      <div className={styles.actions}>
        <button
          type="button"
          className={styles.addBtn}
          onClick={handleAdd}
          disabled={availableProperties.length === 0 || conditions.length >= 5}
        >
          + ソート条件を追加
        </button>
        <div className={styles.actionRight}>
          <button
            type="button"
            className={styles.clearBtn}
            onClick={handleClearAll}
          >
            すべて解除
          </button>
          <button
            type="button"
            className={styles.applyBtn}
            onClick={handleApply}
          >
            適用
          </button>
        </div>
      </div>
    </div>
  );
}
