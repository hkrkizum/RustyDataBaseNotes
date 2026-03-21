import { type ChangeEvent, useCallback, useState } from "react";
import styles from "./PropertyCell.module.css";
import type {
  PropertyConfigDto,
  PropertyDto,
  PropertyValueDto,
  PropertyValueInputDto,
} from "./types";

interface PropertyCellProps {
  property: PropertyDto;
  value: PropertyValueDto | undefined;
  pageId: string;
  onSave: (
    pageId: string,
    propertyId: string,
    value: PropertyValueInputDto,
  ) => Promise<unknown>;
  onClear: (pageId: string, propertyId: string) => Promise<unknown>;
}

export function PropertyCell({
  property,
  value,
  pageId,
  onSave,
  onClear,
}: PropertyCellProps) {
  switch (property.propertyType) {
    case "text":
      return (
        <TextCell
          pageId={pageId}
          propertyId={property.id}
          currentValue={value?.textValue ?? ""}
          onSave={onSave}
          onClear={onClear}
        />
      );
    case "number":
      return (
        <NumberCell
          pageId={pageId}
          propertyId={property.id}
          currentValue={value?.numberValue ?? null}
          onSave={onSave}
          onClear={onClear}
        />
      );
    case "date":
      return (
        <DateCell
          pageId={pageId}
          propertyId={property.id}
          currentValue={value?.dateValue ?? null}
          config={property.config}
          onSave={onSave}
          onClear={onClear}
        />
      );
    case "select":
      return (
        <SelectCell
          pageId={pageId}
          propertyId={property.id}
          currentValue={value?.textValue ?? null}
          config={property.config}
          onSave={onSave}
          onClear={onClear}
        />
      );
    case "checkbox":
      return (
        <CheckboxCell
          pageId={pageId}
          propertyId={property.id}
          currentValue={value?.booleanValue ?? false}
          onSave={onSave}
        />
      );
    default:
      return <span className={styles.cell}>-</span>;
  }
}

// --- Text Cell ---

function TextCell({
  pageId,
  propertyId,
  currentValue,
  onSave,
  onClear,
}: {
  pageId: string;
  propertyId: string;
  currentValue: string;
  onSave: PropertyCellProps["onSave"];
  onClear: PropertyCellProps["onClear"];
}) {
  const [draft, setDraft] = useState(currentValue);

  const handleBlur = useCallback(() => {
    const trimmed = draft.trim();
    if (trimmed === currentValue) return;
    if (trimmed === "") {
      void onClear(pageId, propertyId);
    } else {
      void onSave(pageId, propertyId, { type: "text", value: trimmed });
    }
  }, [draft, currentValue, pageId, propertyId, onSave, onClear]);

  return (
    <input
      className={styles.textInput}
      type="text"
      value={draft}
      onChange={(e) => setDraft(e.target.value)}
      onBlur={handleBlur}
    />
  );
}

// --- Number Cell ---

function NumberCell({
  pageId,
  propertyId,
  currentValue,
  onSave,
  onClear,
}: {
  pageId: string;
  propertyId: string;
  currentValue: number | null;
  onSave: PropertyCellProps["onSave"];
  onClear: PropertyCellProps["onClear"];
}) {
  const [draft, setDraft] = useState(currentValue?.toString() ?? "");

  const handleBlur = useCallback(() => {
    const trimmed = draft.trim();
    if (trimmed === "") {
      if (currentValue !== null) {
        void onClear(pageId, propertyId);
      }
      return;
    }
    const num = Number(trimmed);
    if (!Number.isFinite(num)) return;
    if (num === currentValue) return;
    void onSave(pageId, propertyId, { type: "number", value: num });
  }, [draft, currentValue, pageId, propertyId, onSave, onClear]);

  return (
    <input
      className={styles.numberInput}
      type="number"
      value={draft}
      onChange={(e) => setDraft(e.target.value)}
      onBlur={handleBlur}
    />
  );
}

// --- Date Cell ---

function toInputDate(rfc3339: string | null, isDateTime: boolean): string {
  if (!rfc3339) return "";
  const d = new Date(rfc3339);
  if (isDateTime) {
    // datetime-local wants "YYYY-MM-DDThh:mm"
    const pad = (n: number) => n.toString().padStart(2, "0");
    return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}T${pad(d.getHours())}:${pad(d.getMinutes())}`;
  }
  // date wants "YYYY-MM-DD"
  return rfc3339.slice(0, 10);
}

function DateCell({
  pageId,
  propertyId,
  currentValue,
  config,
  onSave,
  onClear,
}: {
  pageId: string;
  propertyId: string;
  currentValue: string | null;
  config: PropertyConfigDto | null;
  onSave: PropertyCellProps["onSave"];
  onClear: PropertyCellProps["onClear"];
}) {
  const isDateTime = config?.type === "Date" && config.mode === "DateTime";
  const inputType = isDateTime ? "datetime-local" : "date";

  const handleChange = useCallback(
    (e: ChangeEvent<HTMLInputElement>) => {
      const val = e.target.value;
      if (!val) {
        void onClear(pageId, propertyId);
        return;
      }
      // Convert to RFC3339 UTC
      const dt = new Date(val);
      const rfc3339 = dt.toISOString();
      void onSave(pageId, propertyId, { type: "date", value: rfc3339 });
    },
    [pageId, propertyId, onSave, onClear],
  );

  return (
    <input
      className={styles.dateInput}
      type={inputType}
      value={toInputDate(currentValue, isDateTime)}
      onChange={handleChange}
    />
  );
}

// --- Select Cell ---

function SelectCell({
  pageId,
  propertyId,
  currentValue,
  config,
  onSave,
  onClear,
}: {
  pageId: string;
  propertyId: string;
  currentValue: string | null;
  config: PropertyConfigDto | null;
  onSave: PropertyCellProps["onSave"];
  onClear: PropertyCellProps["onClear"];
}) {
  const options = config?.type === "Select" ? config.options : [];

  const handleChange = useCallback(
    (e: ChangeEvent<HTMLSelectElement>) => {
      const val = e.target.value;
      if (val === "") {
        void onClear(pageId, propertyId);
      } else {
        void onSave(pageId, propertyId, {
          type: "select",
          optionId: val,
        });
      }
    },
    [pageId, propertyId, onSave, onClear],
  );

  return (
    <select
      className={styles.selectInput}
      value={currentValue ?? ""}
      onChange={handleChange}
    >
      <option value="">--</option>
      {options.map((opt) => (
        <option key={opt.id} value={opt.id}>
          {opt.value}
        </option>
      ))}
    </select>
  );
}

// --- Checkbox Cell ---

function CheckboxCell({
  pageId,
  propertyId,
  currentValue,
  onSave,
}: {
  pageId: string;
  propertyId: string;
  currentValue: boolean;
  onSave: PropertyCellProps["onSave"];
}) {
  const handleChange = useCallback(() => {
    void onSave(pageId, propertyId, {
      type: "checkbox",
      value: !currentValue,
    });
  }, [currentValue, pageId, propertyId, onSave]);

  return (
    <input
      className={styles.checkboxInput}
      type="checkbox"
      checked={currentValue}
      onChange={handleChange}
    />
  );
}
