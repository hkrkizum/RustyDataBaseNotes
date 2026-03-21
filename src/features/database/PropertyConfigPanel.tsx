import { type FormEvent, useCallback, useState } from "react";
import styles from "./PropertyConfigPanel.module.css";
import type { PropertyConfigDto, PropertyDto, SelectOptionDto } from "./types";

const TYPE_LABELS: Record<string, string> = {
  text: "テキスト",
  number: "数値",
  date: "日付",
  select: "セレクト",
  checkbox: "チェックボックス",
};

interface PropertyConfigPanelProps {
  property: PropertyDto;
  onUpdateName: (id: string, name: string) => Promise<PropertyDto | null>;
  onUpdateConfig: (
    id: string,
    config: PropertyConfigDto,
  ) => Promise<PropertyDto | null>;
  onDelete: (id: string) => Promise<boolean>;
  onResetSelectOption: (
    propertyId: string,
    optionId: string,
  ) => Promise<boolean>;
  onClose: () => void;
}

export function PropertyConfigPanel({
  property,
  onUpdateName,
  onUpdateConfig,
  onDelete,
  onResetSelectOption,
  onClose,
}: PropertyConfigPanelProps) {
  const [name, setName] = useState(property.name);
  const [dateMode, setDateMode] = useState<"Date" | "DateTime">(
    property.config?.type === "Date" ? property.config.mode : "Date",
  );
  const [selectOptions, setSelectOptions] = useState<SelectOptionDto[]>(() => {
    if (property.config?.type === "Select") {
      return [...property.config.options];
    }
    return [];
  });
  const [saving, setSaving] = useState(false);
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);

  const handleAddOption = useCallback(() => {
    setSelectOptions((prev) => [
      ...prev,
      { id: crypto.randomUUID(), value: "" },
    ]);
  }, []);

  const handleRemoveOption = useCallback(
    async (optionId: string) => {
      await onResetSelectOption(property.id, optionId);
      setSelectOptions((prev) => prev.filter((o) => o.id !== optionId));
    },
    [property.id, onResetSelectOption],
  );

  const handleOptionChange = useCallback((optionId: string, value: string) => {
    setSelectOptions((prev) =>
      prev.map((o) => (o.id === optionId ? { ...o, value } : o)),
    );
  }, []);

  const handleSave = useCallback(
    async (e: FormEvent) => {
      e.preventDefault();
      setSaving(true);

      // Update name if changed
      if (name.trim() !== property.name) {
        const result = await onUpdateName(property.id, name.trim());
        if (!result) {
          setSaving(false);
          return;
        }
      }

      // Update config if applicable
      if (property.propertyType === "select") {
        const filteredOptions = selectOptions.filter(
          (o) => o.value.trim() !== "",
        );
        const config: PropertyConfigDto = {
          type: "Select",
          options: filteredOptions,
        };
        const result = await onUpdateConfig(property.id, config);
        if (!result) {
          setSaving(false);
          return;
        }
      } else if (property.propertyType === "date") {
        const config: PropertyConfigDto = { type: "Date", mode: dateMode };
        const result = await onUpdateConfig(property.id, config);
        if (!result) {
          setSaving(false);
          return;
        }
      }

      setSaving(false);
      onClose();
    },
    [
      name,
      property,
      selectOptions,
      dateMode,
      onUpdateName,
      onUpdateConfig,
      onClose,
    ],
  );

  const handleDelete = useCallback(async () => {
    const success = await onDelete(property.id);
    if (success) {
      onClose();
    }
  }, [property.id, onDelete, onClose]);

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
        {/* biome-ignore lint/a11y/noStaticElementInteractions: stopPropagation on panel */}
        {/* biome-ignore lint/a11y/useKeyWithClickEvents: stopPropagation on panel */}
        <div className={styles.panel} onClick={(e) => e.stopPropagation()}>
          <div className={styles.header}>
            <h3 className={styles.title}>プロパティ設定</h3>
            <button type="button" className={styles.closeBtn} onClick={onClose}>
              x
            </button>
          </div>
          <form onSubmit={handleSave}>
            <div className={styles.body}>
              <div className={styles.field}>
                <label className={styles.label} htmlFor="prop-edit-name">
                  名前
                </label>
                <input
                  id="prop-edit-name"
                  className={styles.input}
                  type="text"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                />
              </div>

              <div className={styles.typeInfo}>
                タイプ:{" "}
                {TYPE_LABELS[property.propertyType] ?? property.propertyType}
              </div>

              {property.propertyType === "date" && (
                <div className={styles.field}>
                  <label className={styles.label} htmlFor="edit-date-mode">
                    日付モード
                  </label>
                  <select
                    id="edit-date-mode"
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

              {property.propertyType === "select" && (
                <div className={styles.field}>
                  <span className={styles.label}>選択肢</span>
                  <div className={styles.optionsSection}>
                    {selectOptions.map((opt) => (
                      <div key={opt.id} className={styles.optionRow}>
                        <input
                          className={styles.optionInput}
                          type="text"
                          value={opt.value}
                          onChange={(e) =>
                            handleOptionChange(opt.id, e.target.value)
                          }
                          placeholder="選択肢の値"
                        />
                        <button
                          type="button"
                          className={styles.removeBtn}
                          onClick={() => handleRemoveOption(opt.id)}
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
                className={styles.deleteBtn}
                onClick={() => setShowDeleteConfirm(true)}
              >
                削除
              </button>
              <button
                type="submit"
                className={styles.saveBtn}
                disabled={!name.trim() || saving}
              >
                保存
              </button>
            </div>
          </form>
        </div>
      </div>

      {showDeleteConfirm && (
        /* biome-ignore lint/a11y/noStaticElementInteractions: confirm overlay */
        <div
          className={styles.confirmOverlay}
          role="presentation"
          onClick={() => setShowDeleteConfirm(false)}
          onKeyDown={(e) => {
            if (e.key === "Escape") setShowDeleteConfirm(false);
          }}
        >
          {/* biome-ignore lint/a11y/noStaticElementInteractions: confirm dialog */}
          {/* biome-ignore lint/a11y/useKeyWithClickEvents: confirm dialog */}
          <div
            className={styles.confirmDialog}
            onClick={(e) => e.stopPropagation()}
          >
            <p className={styles.confirmMessage}>
              プロパティ「{property.name}
              」を削除しますか？この操作は取り消せません。関連するすべての値も削除されます。
            </p>
            <div className={styles.confirmActions}>
              <button
                type="button"
                className={styles.cancelBtn}
                onClick={() => setShowDeleteConfirm(false)}
              >
                キャンセル
              </button>
              <button
                type="button"
                className={styles.confirmDeleteBtn}
                onClick={handleDelete}
              >
                削除する
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
}
