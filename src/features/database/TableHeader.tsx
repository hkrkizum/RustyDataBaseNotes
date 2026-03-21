import { useCallback, useState } from "react";
import { AddPropertyModal } from "./AddPropertyModal";
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
}

export function TableHeader({ properties, onAddProperty }: TableHeaderProps) {
  const [showModal, setShowModal] = useState(false);

  const handleOpenModal = useCallback(() => {
    setShowModal(true);
  }, []);

  const handleCloseModal = useCallback(() => {
    setShowModal(false);
  }, []);

  return (
    <>
      <div className={styles.headerRow}>
        {properties.map((prop) => (
          <div key={prop.id} className={styles.headerCell}>
            {prop.name}
            <span className={styles.typeHint}>
              {TYPE_LABELS[prop.propertyType] ?? prop.propertyType}
            </span>
          </div>
        ))}
        <button
          type="button"
          className={styles.addColumnBtn}
          onClick={handleOpenModal}
          title="プロパティを追加"
        >
          +
        </button>
      </div>
      {showModal && (
        <AddPropertyModal onSubmit={onAddProperty} onClose={handleCloseModal} />
      )}
    </>
  );
}
