import { type FormEvent, useCallback, useState } from "react";
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
        className="fixed inset-0 bg-black/40 flex items-center justify-center z-[100]"
        role="presentation"
        onClick={onClose}
        onKeyDown={(e) => {
          if (e.key === "Escape") onClose();
        }}
      >
        {/* biome-ignore lint/a11y/noStaticElementInteractions: stopPropagation on modal container */}
        {/* biome-ignore lint/a11y/useKeyWithClickEvents: stopPropagation on modal container */}
        <div
          className="bg-card rounded-lg w-[420px] max-h-[80vh] flex flex-col shadow-lg"
          onClick={(e) => e.stopPropagation()}
        >
          <div className="flex items-center justify-between px-5 py-4 border-b border-border">
            <h3 className="m-0 text-lg">プロパティを追加</h3>
            <button
              type="button"
              className="bg-transparent border-none text-xl cursor-pointer text-muted-foreground px-2 py-0.5 hover:text-foreground"
              onClick={onClose}
            >
              x
            </button>
          </div>
          <form onSubmit={handleSubmit}>
            <div className="px-5 py-4 overflow-y-auto flex flex-col gap-4">
              <div className="flex flex-col gap-1.5">
                <label
                  className="text-sm font-semibold text-muted-foreground"
                  htmlFor="prop-name"
                >
                  名前
                </label>
                <input
                  id="prop-name"
                  className="px-3 py-2 border border-border rounded text-[0.95rem] outline-none focus:border-ring"
                  type="text"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  placeholder="プロパティ名"
                />
              </div>
              <div className="flex flex-col gap-1.5">
                <label
                  className="text-sm font-semibold text-muted-foreground"
                  htmlFor="prop-type"
                >
                  タイプ
                </label>
                <select
                  id="prop-type"
                  className="px-3 py-2 border border-border rounded text-[0.95rem] bg-card"
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
                <div className="flex flex-col gap-1.5">
                  <label
                    className="text-sm font-semibold text-muted-foreground"
                    htmlFor="date-mode"
                  >
                    日付モード
                  </label>
                  <select
                    id="date-mode"
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

              {propertyType === "select" && (
                <div className="flex flex-col gap-1.5">
                  <span className="text-sm font-semibold text-muted-foreground">
                    選択肢
                  </span>
                  <div className="flex flex-col gap-2">
                    {selectOptions.map((opt) => (
                      <div key={opt.tempId} className="flex items-center gap-2">
                        <input
                          className="flex-1 px-2.5 py-1.5 border border-border rounded text-sm"
                          type="text"
                          value={opt.value}
                          onChange={(e) =>
                            handleOptionChange(opt.tempId, e.target.value)
                          }
                          placeholder="選択肢の値"
                        />
                        <button
                          type="button"
                          className="bg-transparent border border-border rounded cursor-pointer text-sm text-muted-foreground px-2.5 py-1 hover:text-destructive hover:border-destructive"
                          onClick={() => handleRemoveOption(opt.tempId)}
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
            <div className="px-5 py-3 border-t border-border flex justify-end gap-2">
              <button
                type="button"
                className="px-4 py-2 border border-border rounded bg-transparent cursor-pointer text-sm hover:bg-accent"
                onClick={onClose}
              >
                キャンセル
              </button>
              <button
                type="submit"
                className="px-4 py-2 border-none rounded bg-primary text-primary-foreground cursor-pointer text-sm hover:bg-primary/90 disabled:bg-primary/50 disabled:cursor-not-allowed"
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
