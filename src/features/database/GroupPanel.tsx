import { useCallback } from "react";
import type { GroupConditionDto, PropertyDto } from "./types";

interface GroupPanelProps {
  properties: PropertyDto[];
  currentCondition: GroupConditionDto | null;
  onApply: (condition: GroupConditionDto | null) => void;
  onClose: () => void;
}

export function GroupPanel({
  properties,
  currentCondition,
  onApply,
  onClose,
}: GroupPanelProps) {
  const handleChange = useCallback(
    (propertyId: string) => {
      if (propertyId === "") {
        onApply(null);
      } else {
        onApply({ propertyId });
      }
      onClose();
    },
    [onApply, onClose],
  );

  const handleClear = useCallback(() => {
    onApply(null);
    onClose();
  }, [onApply, onClose]);

  return (
    <div className="border border-border rounded-md bg-card p-3 mb-4 shadow-sm">
      <div className="flex justify-between items-center mb-2">
        <span className="font-semibold text-sm text-foreground">
          グルーピング
        </span>
        <button
          type="button"
          className="bg-transparent border-none text-xl cursor-pointer text-muted-foreground px-1"
          onClick={onClose}
        >
          ×
        </button>
      </div>
      <div className="flex flex-col gap-1.5 mb-2">
        <div className="flex gap-1.5 items-center">
          <select
            className="px-2 py-1 border border-border rounded text-sm min-w-[100px]"
            value={currentCondition?.propertyId ?? ""}
            onChange={(e) => handleChange(e.target.value)}
          >
            <option value="">なし</option>
            {properties.map((p) => (
              <option key={p.id} value={p.id}>
                {p.name}
              </option>
            ))}
          </select>
        </div>
      </div>
      {currentCondition && (
        <div className="flex justify-between items-center">
          <button
            type="button"
            className="px-2.5 py-1 border border-border rounded bg-transparent cursor-pointer text-sm"
            onClick={handleClear}
          >
            グルーピング解除
          </button>
        </div>
      )}
    </div>
  );
}
