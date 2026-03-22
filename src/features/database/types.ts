/** A persisted database returned from the backend. */
export interface DatabaseDto {
  id: string;
  title: string;
  createdAt: string;
  updatedAt: string;
}

/** Property type enum values. */
export type PropertyTypeDto =
  | "text"
  | "number"
  | "date"
  | "select"
  | "checkbox";

/** A select option within a property. */
export interface SelectOptionDto {
  id: string;
  value: string;
}

/**
 * Property config - discriminated union matching Rust's serde internally tagged enum.
 * Note: The tag uses PascalCase ("Text", "Number") while PropertyTypeDto uses camelCase.
 */
export type PropertyConfigDto =
  | { type: "Text" }
  | { type: "Number" }
  | { type: "Date"; mode: "Date" | "DateTime" }
  | { type: "Select"; options: SelectOptionDto[] }
  | { type: "Checkbox" };

/** A persisted property (column) returned from the backend. */
export interface PropertyDto {
  id: string;
  databaseId: string;
  name: string;
  propertyType: PropertyTypeDto;
  config: PropertyConfigDto | null;
  position: number;
  createdAt: string;
  updatedAt: string;
}

/** A persisted property value returned from the backend. */
export interface PropertyValueDto {
  id: string;
  pageId: string;
  propertyId: string;
  textValue: string | null;
  numberValue: number | null;
  dateValue: string | null;
  booleanValue: boolean | null;
  createdAt: string;
  updatedAt: string;
}

/** Input for setting a property value - discriminated union. */
export type PropertyValueInputDto =
  | { type: "text"; value: string }
  | { type: "number"; value: number }
  | { type: "date"; value: string }
  | { type: "select"; optionId: string }
  | { type: "checkbox"; value: boolean };

/** A table row with page data and property values. */
export interface TableRowDto {
  page: import("../pages/types").Page;
  values: Record<string, PropertyValueDto>;
}

/** Full table view data. */
export interface TableDataDto {
  database: DatabaseDto;
  properties: PropertyDto[];
  rows: TableRowDto[];
  view: ViewDto;
  groups: GroupInfoDto[] | null;
}

// ---------------------------------------------------------------------------
// View types
// ---------------------------------------------------------------------------

/** View settings returned from the backend. */
export interface ViewDto {
  id: string;
  databaseId: string;
  name: string;
  viewType: "table";
  sortConditions: SortConditionDto[];
  filterConditions: FilterConditionDto[];
  groupCondition: GroupConditionDto | null;
  collapsedGroups: string[];
  createdAt: string;
  updatedAt: string;
}

/** A single sort condition. */
export interface SortConditionDto {
  propertyId: string;
  direction: "ascending" | "descending";
}

/** Filter operator type. */
export type FilterOperatorDto =
  | "equals"
  | "notEquals"
  | "contains"
  | "notContains"
  | "greaterThan"
  | "lessThan"
  | "greaterOrEqual"
  | "lessOrEqual"
  | "before"
  | "after"
  | "is"
  | "isNot"
  | "isChecked"
  | "isUnchecked"
  | "isEmpty"
  | "isNotEmpty";

/** A type-safe filter comparison value. */
export type FilterValueDto =
  | { type: "text"; value: string }
  | { type: "number"; value: number }
  | { type: "date"; value: string }
  | { type: "selectOption"; value: string };

/** A single filter condition. */
export interface FilterConditionDto {
  propertyId: string;
  operator: FilterOperatorDto;
  value: FilterValueDto | null;
}

/** A grouping condition. */
export interface GroupConditionDto {
  propertyId: string;
}

/** Group information in table data response. */
export interface GroupInfoDto {
  value: string | null;
  displayValue: string;
  count: number;
  isCollapsed: boolean;
}

/** Structured error returned from IPC commands. */
export interface CommandError {
  kind: string;
  message: string;
}
