import { useCallback, useState } from "react";
import { FilterConditionRow } from "./FilterConditionRow";
import styles from "./FilterPanel.module.css";
import type { FilterConditionDto, PropertyDto } from "./types";

interface FilterPanelProps {
  properties: PropertyDto[];
  conditions: FilterConditionDto[];
  onApply: (conditions: FilterConditionDto[]) => void;
  onClose: () => void;
}

export function FilterPanel({
  properties,
  conditions: initialConditions,
  onApply,
  onClose,
}: FilterPanelProps) {
  const [conditions, setConditions] =
    useState<FilterConditionDto[]>(initialConditions);

  const handleAdd = useCallback(() => {
    if (properties.length === 0) return;
    const defaultProp = properties[0];
    const defaultOp =
      defaultProp.propertyType === "checkbox" ? "isChecked" : "equals";
    const needsValue = ![
      "isEmpty",
      "isNotEmpty",
      "isChecked",
      "isUnchecked",
    ].includes(defaultOp);
    setConditions((prev) => [
      ...prev,
      {
        propertyId: defaultProp.id,
        operator: defaultOp as FilterConditionDto["operator"],
        value: needsValue ? { type: "text", value: "" } : null,
      },
    ]);
  }, [properties]);

  const handleChange = useCallback(
    (index: number, condition: FilterConditionDto) => {
      setConditions((prev) =>
        prev.map((c, i) => (i === index ? condition : c)),
      );
    },
    [],
  );

  const handleDelete = useCallback((index: number) => {
    setConditions((prev) => prev.filter((_, i) => i !== index));
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
        <span className={styles.title}>フィルタ</span>
        <button type="button" className={styles.closeBtn} onClick={onClose}>
          ×
        </button>
      </div>
      <div className={styles.conditionList}>
        {conditions.map((cond, i) => (
          <FilterConditionRow
            key={`filter-${cond.propertyId}-${i.toString()}`}
            condition={cond}
            properties={properties}
            index={i}
            onChange={handleChange}
            onDelete={handleDelete}
          />
        ))}
      </div>
      <div className={styles.actions}>
        <button type="button" className={styles.addBtn} onClick={handleAdd}>
          + 条件を追加
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
