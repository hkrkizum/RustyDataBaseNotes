import { useCallback, useState } from "react";
import { FilterConditionRow } from "./FilterConditionRow";
import { getDefaultFilterValue } from "./filterUtils";
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
        value: needsValue
          ? getDefaultFilterValue(defaultProp.propertyType)
          : null,
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
    <div className="border border-border rounded-md bg-card p-3 mb-4 shadow-sm">
      <div className="flex justify-between items-center mb-2">
        <span className="font-semibold text-sm text-foreground">フィルタ</span>
        <button
          type="button"
          className="bg-transparent border-none text-xl cursor-pointer text-muted-foreground px-1"
          onClick={onClose}
        >
          ×
        </button>
      </div>
      <div className="flex flex-col gap-1.5 mb-2">
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
      <div className="flex justify-between items-center">
        <button
          type="button"
          className="px-2.5 py-1 border border-dashed border-border rounded bg-transparent cursor-pointer text-sm text-muted-foreground hover:border-foreground hover:text-foreground"
          onClick={handleAdd}
        >
          + 条件を追加
        </button>
        <div className="flex gap-1.5">
          <button
            type="button"
            className="px-2.5 py-1 border border-border rounded bg-transparent cursor-pointer text-sm"
            onClick={handleClearAll}
          >
            すべて解除
          </button>
          <button
            type="button"
            className="px-2.5 py-1 border border-primary rounded bg-primary text-primary-foreground cursor-pointer text-sm hover:bg-primary/90"
            onClick={handleApply}
          >
            適用
          </button>
        </div>
      </div>
    </div>
  );
}
