import { useCallback, useState } from "react";
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
  onRemoveFromDatabase?: (pageId: string) => Promise<boolean>;
  onDeletePage?: (pageId: string) => Promise<boolean>;
}

export function TableRow({
  page,
  properties,
  values,
  onPageClick,
  onSaveValue,
  onClearValue,
  onRemoveFromDatabase,
  onDeletePage,
}: TableRowProps) {
  const [showMenu, setShowMenu] = useState(false);
  const [showConfirm, setShowConfirm] = useState<"remove" | "delete" | null>(
    null,
  );

  const handleTitleClick = useCallback(() => {
    onPageClick(page);
  }, [page, onPageClick]);

  const handleRemove = useCallback(async () => {
    if (onRemoveFromDatabase) {
      await onRemoveFromDatabase(page.id);
    }
    setShowConfirm(null);
    setShowMenu(false);
  }, [page.id, onRemoveFromDatabase]);

  const handleDelete = useCallback(async () => {
    if (onDeletePage) {
      await onDeletePage(page.id);
    }
    setShowConfirm(null);
    setShowMenu(false);
  }, [page.id, onDeletePage]);

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
        {(onRemoveFromDatabase || onDeletePage) && (
          <button
            type="button"
            className={styles.menuBtn}
            onClick={() => setShowMenu((prev) => !prev)}
            title="操作"
          >
            ...
          </button>
        )}
        {showMenu && (
          <div className={styles.contextMenu}>
            {onRemoveFromDatabase && (
              <button
                type="button"
                className={styles.menuItem}
                onClick={() => {
                  setShowConfirm("remove");
                  setShowMenu(false);
                }}
              >
                データベースから除外
              </button>
            )}
            {onDeletePage && (
              <button
                type="button"
                className={styles.menuItemDanger}
                onClick={() => {
                  setShowConfirm("delete");
                  setShowMenu(false);
                }}
              >
                完全に削除
              </button>
            )}
          </div>
        )}
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

      {showConfirm && (
        /* biome-ignore lint/a11y/noStaticElementInteractions: confirm overlay */
        <div
          className={styles.confirmOverlay}
          role="presentation"
          onClick={() => setShowConfirm(null)}
          onKeyDown={(e) => {
            if (e.key === "Escape") setShowConfirm(null);
          }}
        >
          {/* biome-ignore lint/a11y/noStaticElementInteractions: confirm dialog */}
          {/* biome-ignore lint/a11y/useKeyWithClickEvents: confirm dialog */}
          <div
            className={styles.confirmDialog}
            onClick={(e) => e.stopPropagation()}
          >
            <p className={styles.confirmMessage}>
              {showConfirm === "remove"
                ? `ページ「${page.title}」をデータベースから除外しますか？ページ自体は残りますが、プロパティ値は削除されます。`
                : `ページ「${page.title}」を完全に削除しますか？この操作は取り消せません。`}
            </p>
            <div className={styles.confirmActions}>
              <button
                type="button"
                className={styles.cancelBtn}
                onClick={() => setShowConfirm(null)}
              >
                キャンセル
              </button>
              <button
                type="button"
                className={styles.confirmBtn}
                onClick={showConfirm === "remove" ? handleRemove : handleDelete}
              >
                {showConfirm === "remove" ? "除外する" : "削除する"}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
