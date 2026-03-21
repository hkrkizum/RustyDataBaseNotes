import { type FormEvent, useCallback, useState } from "react";
import styles from "./AddPropertyModal.module.css";
import type { PropertyConfigDto, PropertyDto, PropertyTypeDto } from "./types";

interface SelectOptionEntry {
  tempId: string;
  value: string;
}

interface AddPropertyModalProps {
  onSubmit: (
    name: string,
    propertyType: PropertyTypeDto,
    config?: PropertyConfigDto,
  ) => Promise<PropertyDto | null>;
  onClose: () => void;
}

const PROPERTY_TYPE_LABELS: Record<PropertyTypeDto, string> = {
  text: "テキスト",
  number: "数値",
  date: "日付",
  select: "セレクト",
  checkbox: "チェックボックス",
};

let nextTempId = 0;
function createTempId(): string {
  nextTempId += 1;
  return `temp-${nextTempId}`;
}

export function AddPropertyModal({ onSubmit, onClose }: AddPropertyModalProps) {
  const [name, setName] = useState("");
  const [propertyType, setPropertyType] = useState<PropertyTypeDto>("text");
  const [dateMode, setDateMode] = useState<"Date" | "DateTime">("Date");
  const [selectOptions, setSelectOptions] = useState<SelectOptionEntry[]>([]);
  const [submitting, setSubmitting] = useState(false);

  const handleAddOption = useCallback(() => {
    setSelectOptions((prev) => [
      ...prev,
      { tempId: createTempId(), value: "" },
    ]);
  }, []);

  const handleRemoveOption = useCallback((tempId: string) => {
    setSelectOptions((prev) => prev.filter((o) => o.tempId !== tempId));
  }, []);

  const handleOptionChange = useCallback((tempId: string, value: string) => {
    setSelectOptions((prev) =>
      prev.map((o) => (o.tempId === tempId ? { ...o, value } : o)),
    );
  }, []);

  const buildConfig = useCallback((): PropertyConfigDto | undefined => {
    switch (propertyType) {
      case "date":
        return { type: "Date", mode: dateMode };
      case "select": {
        const options = selectOptions
          .filter((o) => o.value.trim() !== "")
          .map((o) => ({
            id: crypto.randomUUID(),
            value: o.value.trim(),
          }));
        return { type: "Select", options };
      }
      default:
        return undefined;
    }
  }, [propertyType, dateMode, selectOptions]);

  const handleSubmit = useCallback(
    async (e: FormEvent) => {
      e.preventDefault();
      if (!name.trim()) return;
      setSubmitting(true);
      const config = buildConfig();
      const result = await onSubmit(name.trim(), propertyType, config);
      setSubmitting(false);
      if (result) {
        onClose();
      }
    },
    [name, propertyType, buildConfig, onSubmit, onClose],
  );

  return (
    <>
      {/* biome-ignore lint/a11y/noStaticElementInteractions: overlay backdrop dismiss */}
      <div
        className={styles.overlay}
        role="presentation"
        onClick={onClose}
        onKeyDown={(e) => {
          if (e.key === "Escape") onClose();
        }}
      >
        {/* biome-ignore lint/a11y/noStaticElementInteractions: stopPropagation on modal container */}
        {/* biome-ignore lint/a11y/useKeyWithClickEvents: stopPropagation on modal container */}
        <div className={styles.modal} onClick={(e) => e.stopPropagation()}>
          <div className={styles.header}>
            <h3 className={styles.title}>プロパティを追加</h3>
            <button type="button" className={styles.closeBtn} onClick={onClose}>
              x
            </button>
          </div>
          <form onSubmit={handleSubmit}>
            <div className={styles.body}>
              <div className={styles.field}>
                <label className={styles.label} htmlFor="prop-name">
                  名前
                </label>
                <input
                  id="prop-name"
                  className={styles.input}
                  type="text"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  placeholder="プロパティ名"
                />
              </div>
              <div className={styles.field}>
                <label className={styles.label} htmlFor="prop-type">
                  タイプ
                </label>
                <select
                  id="prop-type"
                  className={styles.select}
                  value={propertyType}
                  onChange={(e) =>
                    setPropertyType(e.target.value as PropertyTypeDto)
                  }
                >
                  {Object.entries(PROPERTY_TYPE_LABELS).map(
                    ([value, label]) => (
                      <option key={value} value={value}>
                        {label}
                      </option>
                    ),
                  )}
                </select>
              </div>

              {propertyType === "date" && (
                <div className={styles.field}>
                  <label className={styles.label} htmlFor="date-mode">
                    日付モード
                  </label>
                  <select
                    id="date-mode"
                    className={styles.select}
                    value={dateMode}
                    onChange={(e) =>
                      setDateMode(e.target.value as "Date" | "DateTime")
                    }
                  >
                    <option value="Date">日付のみ</option>
                    <option value="DateTime">日付と時刻</option>
                  </select>
                </div>
              )}

              {propertyType === "select" && (
                <div className={styles.field}>
                  <span className={styles.label}>選択肢</span>
                  <div className={styles.optionsSection}>
                    {selectOptions.map((opt) => (
                      <div key={opt.tempId} className={styles.optionRow}>
                        <input
                          className={styles.optionInput}
                          type="text"
                          value={opt.value}
                          onChange={(e) =>
                            handleOptionChange(opt.tempId, e.target.value)
                          }
                          placeholder="選択肢の値"
                        />
                        <button
                          type="button"
                          className={styles.removeBtn}
                          onClick={() => handleRemoveOption(opt.tempId)}
                        >
                          削除
                        </button>
                      </div>
                    ))}
                    <button
                      type="button"
                      className={styles.addOptionBtn}
                      onClick={handleAddOption}
                    >
                      + 選択肢を追加
                    </button>
                  </div>
                </div>
              )}
            </div>
            <div className={styles.footer}>
              <button
                type="button"
                className={styles.cancelBtn}
                onClick={onClose}
              >
                キャンセル
              </button>
              <button
                type="submit"
                className={styles.submitBtn}
                disabled={!name.trim() || submitting}
              >
                追加
              </button>
            </div>
          </form>
        </div>
      </div>
    </>
  );
}
