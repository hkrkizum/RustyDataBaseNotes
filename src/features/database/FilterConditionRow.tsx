import { useCallback } from "react";
import styles from "./FilterPanel.module.css";
import {
  getDefaultFilterValue,
  getFilterDisplayValue,
  isValueTypeCompatible,
  NO_VALUE_OPERATORS,
  parseFilterValue,
} from "./filterUtils";
import type {
  FilterConditionDto,
  FilterOperatorDto,
  PropertyDto,
  PropertyTypeDto,
} from "./types";

const OPERATORS_BY_TYPE: Record<PropertyTypeDto, FilterOperatorDto[]> = {
  text: [
    "equals",
    "notEquals",
    "contains",
    "notContains",
    "isEmpty",
    "isNotEmpty",
  ],
  number: [
    "equals",
    "notEquals",
    "greaterThan",
    "lessThan",
    "greaterOrEqual",
    "lessOrEqual",
    "isEmpty",
    "isNotEmpty",
  ],
  date: ["equals", "before", "after", "isEmpty", "isNotEmpty"],
  select: ["is", "isNot", "isEmpty", "isNotEmpty"],
  checkbox: ["isChecked", "isUnchecked"],
};

const OPERATOR_LABELS: Record<FilterOperatorDto, string> = {
  equals: "等しい",
  notEquals: "等しくない",
  contains: "含む",
  notContains: "含まない",
  greaterThan: "より大きい",
  lessThan: "より小さい",
  greaterOrEqual: "以上",
  lessOrEqual: "以下",
  before: "より前",
  after: "以降",
  is: "である",
  isNot: "でない",
  isChecked: "チェック済み",
  isUnchecked: "未チェック",
  isEmpty: "空",
  isNotEmpty: "空でない",
};

interface FilterConditionRowProps {
  condition: FilterConditionDto;
  properties: PropertyDto[];
  index: number;
  onChange: (index: number, condition: FilterConditionDto) => void;
  onDelete: (index: number) => void;
}

export function FilterConditionRow({
  condition,
  properties,
  index,
  onChange,
  onDelete,
}: FilterConditionRowProps) {
  const selectedProp = properties.find((p) => p.id === condition.propertyId);
  const propType = selectedProp?.propertyType ?? "text";
  const validOperators = OPERATORS_BY_TYPE[propType] ?? [];

  const handlePropertyChange = useCallback(
    (propertyId: string) => {
      const prop = properties.find((p) => p.id === propertyId);
      const newType = prop?.propertyType ?? "text";
      const ops = OPERATORS_BY_TYPE[newType] ?? [];
      const defaultOp = ops[0] ?? "equals";
      onChange(index, {
        propertyId,
        operator: defaultOp,
        value: NO_VALUE_OPERATORS.includes(defaultOp)
          ? null
          : getDefaultFilterValue(newType),
      });
    },
    [properties, index, onChange],
  );

  const handleOperatorChange = useCallback(
    (operator: FilterOperatorDto) => {
      let newValue = condition.value;
      if (NO_VALUE_OPERATORS.includes(operator)) {
        newValue = null;
      } else if (!newValue || !isValueTypeCompatible(newValue, propType)) {
        newValue = getDefaultFilterValue(propType);
      }
      onChange(index, { ...condition, operator, value: newValue });
    },
    [condition, propType, index, onChange],
  );

  const handleValueChange = useCallback(
    (rawValue: string) => {
      const value = parseFilterValue(rawValue, propType);
      onChange(index, { ...condition, value });
    },
    [condition, propType, index, onChange],
  );

  const showValueInput = !NO_VALUE_OPERATORS.includes(condition.operator);

  return (
    <div className={styles.conditionRow}>
      <select
        className={styles.select}
        value={condition.propertyId}
        onChange={(e) => handlePropertyChange(e.target.value)}
      >
        {properties.map((p) => (
          <option key={p.id} value={p.id}>
            {p.name}
          </option>
        ))}
      </select>
      <select
        className={styles.select}
        value={condition.operator}
        onChange={(e) =>
          handleOperatorChange(e.target.value as FilterOperatorDto)
        }
      >
        {validOperators.map((op) => (
          <option key={op} value={op}>
            {OPERATOR_LABELS[op]}
          </option>
        ))}
      </select>
      {showValueInput &&
        (propType === "select" ? (
          <select
            className={styles.select}
            value={getFilterDisplayValue(condition.value)}
            onChange={(e) => handleValueChange(e.target.value)}
          >
            <option value="">（選択してください）</option>
            {selectedProp?.config?.type === "Select" &&
              selectedProp.config.options.map((opt) => (
                <option key={opt.id} value={opt.id}>
                  {opt.value}
                </option>
              ))}
          </select>
        ) : (
          <input
            className={styles.valueInput}
            type={
              propType === "number"
                ? "number"
                : propType === "date"
                  ? "datetime-local"
                  : "text"
            }
            value={getFilterDisplayValue(condition.value)}
            onChange={(e) => handleValueChange(e.target.value)}
            placeholder="値"
          />
        ))}
      <button
        type="button"
        className={styles.deleteBtn}
        onClick={() => onDelete(index)}
      >
        ×
      </button>
    </div>
  );
}
