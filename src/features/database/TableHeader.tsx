import { useCallback, useState } from "react";
import { AddPropertyModal } from "./AddPropertyModal";
import { PropertyConfigPanel } from "./PropertyConfigPanel";
import styles from "./TableHeader.module.css";
import type { PropertyConfigDto, PropertyDto, PropertyTypeDto } from "./types";

const TYPE_LABELS: Record<string, string> = {
  text: "テキスト",
  number: "数値",
  date: "日付",
  select: "セレクト",
  checkbox: "チェックボックス",
};

interface TableHeaderProps {
  properties: PropertyDto[];
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
}

export function TableHeader({
  properties,
  onAddProperty,
  onUpdatePropertyName,
  onUpdatePropertyConfig,
  onDeleteProperty,
  onResetSelectOption,
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

  const handleCloseConfig = useCallback(() => {
    setEditingProperty(null);
  }, []);

  return (
    <>
      {properties.map((prop) => (
        <button
          type="button"
          key={prop.id}
          className={styles.headerCell}
          onClick={() => handleHeaderClick(prop)}
        >
          {prop.name}
          <span className={styles.typeHint}>
            {TYPE_LABELS[prop.propertyType] ?? prop.propertyType}
          </span>
        </button>
      ))}
      <button
        type="button"
        className={styles.addColumnBtn}
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
