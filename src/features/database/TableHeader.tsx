import { useCallback, useState } from "react";
import { AddPropertyModal } from "./AddPropertyModal";
import { PropertyConfigPanel } from "./PropertyConfigPanel";
import type {
  PropertyConfigDto,
  PropertyDto,
  PropertyTypeDto,
  SortConditionDto,
  ViewDto,
} from "./types";

const TYPE_LABELS: Record<string, string> = {
  text: "テキスト",
  number: "数値",
  date: "日付",
  select: "セレクト",
  checkbox: "チェックボックス",
};

interface TableHeaderProps {
  properties: PropertyDto[];
  view: ViewDto | null;
  onAddProperty: (
    name: string,
    propertyType: PropertyTypeDto,
    config?: PropertyConfigDto,
  ) => Promise<PropertyDto | null>;
  onUpdatePropertyName: (
    id: string,
    name: string,
  ) => Promise<PropertyDto | null>;
  onUpdatePropertyConfig: (
    id: string,
    config: PropertyConfigDto,
  ) => Promise<PropertyDto | null>;
  onDeleteProperty: (id: string) => Promise<boolean>;
  onResetSelectOption: (
    propertyId: string,
    optionId: string,
  ) => Promise<boolean>;
  onSortClick: (conditions: SortConditionDto[]) => void;
}

export function TableHeader({
  properties,
  view,
  onAddProperty,
  onUpdatePropertyName,
  onUpdatePropertyConfig,
  onDeleteProperty,
  onResetSelectOption,
  onSortClick,
}: TableHeaderProps) {
  const [showModal, setShowModal] = useState(false);
  const [editingProperty, setEditingProperty] = useState<PropertyDto | null>(
    null,
  );

  const handleOpenModal = useCallback(() => {
    setShowModal(true);
  }, []);

  const handleCloseModal = useCallback(() => {
    setShowModal(false);
  }, []);

  const handleHeaderClick = useCallback((prop: PropertyDto) => {
    setEditingProperty(prop);
  }, []);

  const handleSortClick = useCallback(
    (e: React.MouseEvent, prop: PropertyDto) => {
      e.stopPropagation();
      const currentSort = view?.sortConditions?.find(
        (s) => s.propertyId === prop.id,
      );

      if (!currentSort) {
        // none → ascending
        onSortClick([{ propertyId: prop.id, direction: "ascending" }]);
      } else if (currentSort.direction === "ascending") {
        // ascending → descending
        onSortClick([{ propertyId: prop.id, direction: "descending" }]);
      } else {
        // descending → none
        onSortClick([]);
      }
    },
    [view, onSortClick],
  );

  const getSortIndicator = useCallback(
    (propId: string): string => {
      const sort = view?.sortConditions?.find((s) => s.propertyId === propId);
      if (!sort) return "";
      return sort.direction === "ascending" ? " ▲" : " ▼";
    },
    [view],
  );

  const handleCloseConfig = useCallback(() => {
    setEditingProperty(null);
  }, []);

  return (
    <>
      {properties.map((prop) => (
        <button
          type="button"
          key={prop.id}
          className="px-2.5 py-2 text-sm font-semibold text-muted-foreground min-w-[150px] flex-1 border-r border-border overflow-hidden text-ellipsis whitespace-nowrap flex items-center bg-transparent cursor-pointer font-inherit text-left hover:bg-accent"
          onClick={() => handleHeaderClick(prop)}
        >
          {/* biome-ignore lint/a11y/useSemanticElements: nested sort trigger */}
          <span
            role="button"
            tabIndex={0}
            className="cursor-pointer"
            onClick={(e) => handleSortClick(e, prop)}
            onKeyDown={(e) => {
              if (e.key === "Enter")
                handleSortClick(e as unknown as React.MouseEvent, prop);
            }}
          >
            {prop.name}
            {getSortIndicator(prop.id)}
          </span>
          <span className="font-normal text-muted-foreground/60 text-xs ml-1">
            {TYPE_LABELS[prop.propertyType] ?? prop.propertyType}
          </span>
        </button>
      ))}
      <button
        type="button"
        className="px-3 py-2 bg-transparent border-none cursor-pointer text-base text-muted-foreground min-w-[40px] text-center flex items-center justify-center hover:text-ring hover:bg-ring/10"
        onClick={handleOpenModal}
        title="プロパティを追加"
      >
        +
      </button>
      {showModal && (
        <AddPropertyModal onSubmit={onAddProperty} onClose={handleCloseModal} />
      )}
      {editingProperty && (
        <PropertyConfigPanel
          property={editingProperty}
          onUpdateName={onUpdatePropertyName}
          onUpdateConfig={onUpdatePropertyConfig}
          onDelete={onDeleteProperty}
          onResetSelectOption={onResetSelectOption}
          onClose={handleCloseConfig}
        />
      )}
    </>
  );
}
