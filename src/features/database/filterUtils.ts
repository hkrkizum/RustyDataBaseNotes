import type {
  FilterOperatorDto,
  FilterValueDto,
  PropertyTypeDto,
} from "./types";

/** プロパティ型に応じたフィルタ値のデフォルト値を返す */
export function getDefaultFilterValue(
  propType: PropertyTypeDto,
): FilterValueDto {
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

/** ユーザー入力文字列をプロパティ型に応じた FilterValueDto に変換する */
export function parseFilterValue(
  rawValue: string,
  propType: PropertyTypeDto,
): FilterValueDto | null {
  switch (propType) {
    case "text":
      return { type: "text", value: rawValue };
    case "number": {
      if (rawValue === "") return null;
      const n = Number(rawValue);
      if (Number.isNaN(n)) return null;
      return { type: "number", value: n };
    }
    case "date":
      return { type: "date", value: rawValue };
    case "select":
      return { type: "selectOption", value: rawValue };
    case "checkbox":
      return null;
    default:
      return null;
  }
}

/** FilterValueDto の表示用文字列を返す */
export function getFilterDisplayValue(value: FilterValueDto | null): string {
  if (!value) return "";
  return String(value.value);
}

/** 値を必要としないオペレーター一覧 */
export const NO_VALUE_OPERATORS: readonly FilterOperatorDto[] = [
  "isEmpty",
  "isNotEmpty",
  "isChecked",
  "isUnchecked",
];

/** 既存 value の型がプロパティ型と整合するか判定する */
export function isValueTypeCompatible(
  value: FilterValueDto,
  propType: PropertyTypeDto,
): boolean {
  return (
    (propType === "text" && value.type === "text") ||
    (propType === "number" && value.type === "number") ||
    (propType === "date" && value.type === "date") ||
    (propType === "select" && value.type === "selectOption")
  );
}
