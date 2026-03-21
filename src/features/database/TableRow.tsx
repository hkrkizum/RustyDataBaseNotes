import { useCallback } from "react";
import type { Page } from "../pages/types";
import { PropertyCell } from "./PropertyCell";
import styles from "./TableRow.module.css";
import type {
  PropertyDto,
  PropertyValueDto,
  PropertyValueInputDto,
} from "./types";

interface TableRowProps {
  page: Page;
  properties: PropertyDto[];
  values: Record<string, PropertyValueDto>;
  onPageClick: (page: Page) => void;
  onSaveValue: (
    pageId: string,
    propertyId: string,
    value: PropertyValueInputDto,
  ) => Promise<unknown>;
  onClearValue: (pageId: string, propertyId: string) => Promise<unknown>;
}

export function TableRow({
  page,
  properties,
  values,
  onPageClick,
  onSaveValue,
  onClearValue,
}: TableRowProps) {
  const handleTitleClick = useCallback(() => {
    onPageClick(page);
  }, [page, onPageClick]);

  return (
    <div className={styles.row}>
      <div className={styles.titleCell}>
        <button
          type="button"
          className={styles.titleLink}
          onClick={handleTitleClick}
        >
          {page.title}
        </button>
      </div>
      {properties.map((prop) => (
        <div key={prop.id} className={styles.valueCell}>
          <PropertyCell
            property={prop}
            value={values[prop.id]}
            pageId={page.id}
            onSave={onSaveValue}
            onClear={onClearValue}
          />
        </div>
      ))}
    </div>
  );
}
