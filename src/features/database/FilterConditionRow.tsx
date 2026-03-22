import { useCallback } from "react";
import styles from "./FilterPanel.module.css";
import type {
  FilterConditionDto,
  FilterOperatorDto,
  FilterValueDto,
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

const NO_VALUE_OPERATORS: FilterOperatorDto[] = [
  "isEmpty",
  "isNotEmpty",
  "isChecked",
  "isUnchecked",
];

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
          : getDefaultValue(newType),
      });
    },
    [properties, index, onChange],
  );

  const handleOperatorChange = useCallback(
    (operator: FilterOperatorDto) => {
      onChange(index, {
        ...condition,
        operator,
        value: NO_VALUE_OPERATORS.includes(operator)
          ? null
          : (condition.value ?? getDefaultValue(propType)),
      });
    },
    [condition, propType, index, onChange],
  );

  const handleValueChange = useCallback(
    (rawValue: string) => {
      let value: FilterValueDto | null = null;
      if (propType === "text") value = { type: "text", value: rawValue };
      else if (propType === "number")
        value = { type: "number", value: Number(rawValue) || 0 };
      else if (propType === "date") value = { type: "date", value: rawValue };
      else if (propType === "select")
        value = { type: "selectOption", value: rawValue };
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
      {showValueInput && (
        <input
          className={styles.valueInput}
          type={
            propType === "number"
              ? "number"
              : propType === "date"
                ? "datetime-local"
                : "text"
          }
          value={getDisplayValue(condition.value)}
          onChange={(e) => handleValueChange(e.target.value)}
          placeholder="値"
        />
      )}
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

function getDefaultValue(propType: PropertyTypeDto): FilterValueDto {
  switch (propType) {
    case "number":
      return { type: "number", value: 0 };
    case "date":
      return { type: "date", value: new Date().toISOString() };
    case "select":
      return { type: "selectOption", value: "" };
    default:
      return { type: "text", value: "" };
  }
}

function getDisplayValue(value: FilterValueDto | null): string {
  if (!value) return "";
  return String(value.value);
}
