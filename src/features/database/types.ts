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
}

/** Structured error returned from IPC commands. */
export interface CommandError {
  kind: string;
  message: string;
}
