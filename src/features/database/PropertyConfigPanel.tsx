import { type FormEvent, useCallback, useState } from "react";
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
        className="fixed inset-0 bg-black/40 flex items-center justify-center z-[100]"
        role="presentation"
        onClick={onClose}
        onKeyDown={(e) => {
          if (e.key === "Escape") onClose();
        }}
      >
        {/* biome-ignore lint/a11y/noStaticElementInteractions: stopPropagation on panel */}
        {/* biome-ignore lint/a11y/useKeyWithClickEvents: stopPropagation on panel */}
        <div
          className="bg-card rounded-lg w-[380px] max-h-[80vh] flex flex-col shadow-lg"
          onClick={(e) => e.stopPropagation()}
        >
          <div className="flex items-center justify-between px-5 py-4 border-b border-border">
            <h3 className="m-0 text-lg">プロパティ設定</h3>
            <button
              type="button"
              className="bg-transparent border-none text-xl cursor-pointer text-muted-foreground px-2 py-0.5 hover:text-foreground"
              onClick={onClose}
            >
              x
            </button>
          </div>
          <form onSubmit={handleSave}>
            <div className="px-5 py-4 overflow-y-auto flex flex-col gap-4">
              <div className="flex flex-col gap-1.5">
                <label
                  className="text-sm font-semibold text-muted-foreground"
                  htmlFor="prop-edit-name"
                >
                  名前
                </label>
                <input
                  id="prop-edit-name"
                  className="px-3 py-2 border border-border rounded text-[0.95rem] outline-none focus:border-ring"
                  type="text"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                />
              </div>

              <div className="text-sm text-muted-foreground py-1">
                タイプ:{" "}
                {TYPE_LABELS[property.propertyType] ?? property.propertyType}
              </div>

              {property.propertyType === "date" && (
                <div className="flex flex-col gap-1.5">
                  <label
                    className="text-sm font-semibold text-muted-foreground"
                    htmlFor="edit-date-mode"
                  >
                    日付モード
                  </label>
                  <select
                    id="edit-date-mode"
                    className="px-3 py-2 border border-border rounded text-[0.95rem] bg-card"
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
                <div className="flex flex-col gap-1.5">
                  <span className="text-sm font-semibold text-muted-foreground">
                    選択肢
                  </span>
                  <div className="flex flex-col gap-2">
                    {selectOptions.map((opt) => (
                      <div key={opt.id} className="flex items-center gap-2">
                        <input
                          className="flex-1 px-2.5 py-1.5 border border-border rounded text-sm"
                          type="text"
                          value={opt.value}
                          onChange={(e) =>
                            handleOptionChange(opt.id, e.target.value)
                          }
                          placeholder="選択肢の値"
                        />
                        <button
                          type="button"
                          className="bg-transparent border border-border rounded cursor-pointer text-sm text-muted-foreground px-2.5 py-1 hover:text-destructive hover:border-destructive"
                          onClick={() => handleRemoveOption(opt.id)}
                        >
                          削除
                        </button>
                      </div>
                    ))}
                    <button
                      type="button"
                      className="px-3 py-1.5 border border-dashed border-border rounded bg-transparent cursor-pointer text-sm text-muted-foreground self-start hover:border-foreground hover:text-foreground"
                      onClick={handleAddOption}
                    >
                      + 選択肢を追加
                    </button>
                  </div>
                </div>
              )}
            </div>
            <div className="px-5 py-3 border-t border-border flex justify-between items-center">
              <button
                type="button"
                className="px-4 py-2 border border-destructive rounded bg-transparent cursor-pointer text-sm text-destructive hover:bg-destructive/10"
                onClick={() => setShowDeleteConfirm(true)}
              >
                削除
              </button>
              <button
                type="submit"
                className="px-4 py-2 border-none rounded bg-primary text-primary-foreground cursor-pointer text-sm hover:bg-primary/90 disabled:bg-primary/50 disabled:cursor-not-allowed"
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
          className="fixed inset-0 bg-black/50 flex items-center justify-center z-[200]"
          role="presentation"
          onClick={() => setShowDeleteConfirm(false)}
          onKeyDown={(e) => {
            if (e.key === "Escape") setShowDeleteConfirm(false);
          }}
        >
          {/* biome-ignore lint/a11y/noStaticElementInteractions: confirm dialog */}
          {/* biome-ignore lint/a11y/useKeyWithClickEvents: confirm dialog */}
          <div
            className="bg-card rounded-lg p-6 w-[320px] shadow-lg"
            onClick={(e) => e.stopPropagation()}
          >
            <p className="m-0 mb-4 text-[0.95rem] leading-relaxed">
              プロパティ「{property.name}
              」を削除しますか？この操作は取り消せません。関連するすべての値も削除されます。
            </p>
            <div className="flex justify-end gap-2">
              <button
                type="button"
                className="px-4 py-2 border border-border rounded cursor-pointer text-sm bg-transparent hover:bg-accent"
                onClick={() => setShowDeleteConfirm(false)}
              >
                キャンセル
              </button>
              <button
                type="button"
                className="px-4 py-2 border-none rounded bg-destructive text-white cursor-pointer text-sm hover:bg-destructive/80"
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
